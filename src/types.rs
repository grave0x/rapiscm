use std::fmt;
use std::path::PathBuf;

use crate::analytics::TrackerSignature;
use serde::Serialize;

/// What the user pointed us at.
#[derive(Debug, Clone)]
pub enum Target {
    /// An OpenAPI spec file on disk.
    Spec(PathBuf),
    /// A URL to scan.
    Url(reqwest::Url),
}

impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Target::Spec(p) => write!(f, "spec:{}", p.display()),
            Target::Url(u) => write!(f, "url:{u}"),
        }
    }
}

/// Authentication configuration parsed from `--auth`.
#[derive(Debug, Clone)]
pub enum AuthConfig {
    Bearer(String),
    Basic { username: String, password: String },
    Header { name: String, value: String },
}

/// A single API endpoint to hit.
#[derive(Debug, Clone)]
pub struct Endpoint {
    pub method: reqwest::Method,
    pub url: reqwest::Url,
    pub headers: Vec<(String, String)>,
    pub body: Option<serde_json::Value>,
    /// Expected success status from the spec, if known.
    pub expected_status: Option<u16>,
    pub tags: Vec<String>,
}

/// The result of hitting a single endpoint.
#[derive(Debug, Clone, Serialize)]
pub struct ResponseResult {
    pub endpoint_method: String,
    pub endpoint_url: String,
    pub status_code: u16,
    pub response_time_ms: u64,
    pub response_size: usize,
    pub response_headers: Vec<(String, String)>,
    pub response_body: Vec<u8>,
    /// Expected status from spec, if known.
    pub expected_status: Option<u16>,
    /// ISO-8601 timestamp, if recorded.
    pub timestamp: Option<String>,
    pub checks: Vec<Check>,
    pub error: Option<String>,
    pub tags: Vec<String>,
    /// Trackers / analytics detected in the response.
    pub trackers: Vec<TrackerSignature>,
}

/// A single check result (security header, CORS, etc.).
#[derive(Debug, Clone, Serialize)]
pub struct Check {
    pub name: String,
    pub passed: bool,
    pub severity: Severity,
    pub message: String,
}

#[derive(Debug, Clone, Serialize)]
pub enum Severity {
    Info,
    Warn,
    Critical,
}

/// Convert AuthConfig into an HTTP header (name, value) pair.
pub fn auth_to_header(auth: &Option<AuthConfig>) -> Option<(String, String)> {
    match auth {
        Some(AuthConfig::Bearer(token)) => {
            Some(("Authorization".into(), format!("Bearer {token}")))
        }
        Some(AuthConfig::Basic { username, password }) => {
            let encoded = base64_encode(&format!("{username}:{password}"));
            Some(("Authorization".into(), format!("Basic {encoded}")))
        }
        Some(AuthConfig::Header { name, value }) => Some((name.clone(), value.clone())),
        None => None,
    }
}

/// Minimal base64 encode (no external crate dependency).
pub fn base64_encode(input: &str) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let bytes = input.as_bytes();
    let mut result = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0] as u32;
        let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
        let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    Table,
    Json,
    Markdown,
}

/// A domain discovered for a company/organization.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscoveredDomain {
    pub domain: String,
    /// Which discovery sources found this domain.
    pub sources: Vec<String>,
    /// TLS certificate subject names from crt.sh.
    pub cert_subjects: Vec<String>,
    /// Autonomous System Number, if known.
    pub asn: Option<u32>,
    /// ASN organization name.
    pub asn_org: Option<String>,
    /// IP ranges (CIDR) associated with this domain's infrastructure.
    pub ip_ranges: Vec<String>,
    /// Organization name from registry / RDAP data.
    pub org_name: Option<String>,
}

/// API keys for gated discovery sources, loaded from config file.
#[derive(Debug, Clone, Default)]
pub struct ApiKeys {
    pub google_api_key: Option<String>,
    pub google_cx: Option<String>,
    pub shodan_api_key: Option<String>,
}
