//! Core data types: Endpoint, ResponseResult, Check, OutputFormat.

use std::fmt;
use std::path::PathBuf;

use crate::analytics::TrackerSignature;
use serde::{Deserialize, Serialize};

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
    /// Bearer token authentication (adds `Authorization: Bearer <token>`).
    Bearer(String),
    /// HTTP Basic authentication (base64-encoded username:password).
    Basic { username: String, password: String },
    /// Custom header-based authentication.
    Header { name: String, value: String },
}

/// A single API endpoint to hit.
#[derive(Debug, Clone)]
pub struct Endpoint {
    /// HTTP method (GET, POST, PUT, etc).
    pub method: reqwest::Method,
    /// Target URL.
    pub url: reqwest::Url,
    /// Additional headers to send with the request.
    pub headers: Vec<(String, String)>,
    /// Request body (for POST/PUT/PATCH).
    pub body: Option<serde_json::Value>,
    /// Expected success status from the spec, if known.
    pub expected_status: Option<u16>,
    /// Tags classifying this endpoint (method, path category, etc).
    pub tags: Vec<String>,
}

/// The result of hitting a single endpoint.
///
/// NOTE: `Deserialize` is manual — `trackers` is always deserialized as empty
/// (tracker signatures use `&'static str` fields from a static database).
#[derive(Debug, Clone, Serialize)]
pub struct ResponseResult {
    /// HTTP method used.
    pub endpoint_method: String,
    /// URL that was requested.
    pub endpoint_url: String,
    /// HTTP response status code.
    pub status_code: u16,
    /// Round-trip time in milliseconds.
    pub response_time_ms: u64,
    /// Response body size in bytes.
    pub response_size: usize,
    /// Response headers as (name, value) pairs.
    pub response_headers: Vec<(String, String)>,
    /// Raw response body bytes.
    pub response_body: Vec<u8>,
    /// Expected status from spec, if known.
    pub expected_status: Option<u16>,
    /// ISO-8601 timestamp, if recorded.
    pub timestamp: Option<String>,
    /// Security and compliance checks applied to this response.
    pub checks: Vec<Check>,
    /// Error message if the request failed entirely.
    pub error: Option<String>,
    /// Tags classifying this response.
    pub tags: Vec<String>,
    /// Trackers / analytics detected in the response.
    pub trackers: Vec<TrackerSignature>,
}

impl<'de> Deserialize<'de> for ResponseResult {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct ResponseResultHelper {
            endpoint_method: String,
            endpoint_url: String,
            status_code: u16,
            response_time_ms: u64,
            response_size: usize,
            response_headers: Vec<(String, String)>,
            response_body: Vec<u8>,
            expected_status: Option<u16>,
            timestamp: Option<String>,
            checks: Vec<Check>,
            error: Option<String>,
            tags: Vec<String>,
        }
        let h = ResponseResultHelper::deserialize(deserializer)?;
        Ok(ResponseResult {
            endpoint_method: h.endpoint_method,
            endpoint_url: h.endpoint_url,
            status_code: h.status_code,
            response_time_ms: h.response_time_ms,
            response_size: h.response_size,
            response_headers: h.response_headers,
            response_body: h.response_body,
            expected_status: h.expected_status,
            timestamp: h.timestamp,
            checks: h.checks,
            error: h.error,
            tags: h.tags,
            trackers: Vec::new(),
        })
    }
}

/// A single check result (security header, CORS, etc.).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Check {
    /// Check name (e.g. "HSTS", "CSP", "CORS").
    pub name: String,
    /// Whether the check passed.
    pub passed: bool,
    /// Severity level if the check failed.
    pub severity: Severity,
    /// Human-readable description of the result.
    pub message: String,
}

/// Severity level for check results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Severity {
    /// Informational — no action required.
    Info,
    /// Warning — potential issue worth reviewing.
    Warn,
    /// Critical — security or compliance issue.
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

/// Output format for scan results.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum OutputFormat {
    /// Terminal table with colored status codes.
    Table,
    /// Pretty-printed JSON.
    Json,
    /// Markdown table.
    Markdown,
    /// Structured API documentation (llm-api style).
    Doc,
}

/// A domain discovered for a company/organization.
#[derive(Debug, Clone, serde::Serialize)]
pub struct DiscoveredDomain {
    /// Discovered domain name.
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
    /// Google Custom Search API key.
    pub google_api_key: Option<String>,
    /// Google Custom Search engine ID.
    pub google_cx: Option<String>,
    /// Shodan API key.
    pub shodan_api_key: Option<String>,
}
