//! Shodan org-name search (favicon hash search planned).
//!
//! Searches Shodan for hosts matching an organization name, then extracts
//! associated domains. Requires a Shodan API key in config.toml.
//!
//! A future enhancement would implement the two-pass favicon approach:
//!   1. Find known domains for the org
//!   2. Get their favicon hashes
//!   3. Search Shodan for other hosts with matching hashes

use crate::error::{Error, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct ShodanSearchResponse {
    #[serde(default)]
    matches: Vec<ShodanMatch>,
    #[serde(default)]
    #[expect(dead_code)]
    total: usize,
}

#[derive(Debug, Deserialize)]
struct ShodanMatch {
    #[serde(default)]
    hostnames: Vec<String>,
    #[serde(default)]
    #[expect(dead_code)]
    ip_str: String,
    #[serde(default)]
    #[expect(dead_code)]
    port: u16,
    #[serde(default)]
    http: Option<ShodanHttp>,
}

#[derive(Debug, Deserialize)]
struct ShodanHttp {
    #[serde(default)]
    host: String,
}

/// Search Shodan for domains associated with an organization.
/// Uses Shodan's text search API with the org name as query.
pub async fn shodan_favicon(org: &str, api_key: &str) -> Result<Vec<String>> {
    let query = format!("org:\"{}\" http.title:\"*\"", org.replace('"', "\\\""));
    let url = format!(
        "https://api.shodan.io/shodan/host/search?key={}&query={}&limit=100",
        api_key,
        urlencoding(&query)
    );

    let resp = reqwest::get(&url).await.map_err(|e| Error::DiscoveryHttp {
        src: "shodan",
        detail: e.to_string(),
    })?;

    if !resp.status().is_success() {
        let status = resp.status();
        if status.as_u16() == 403 {
            tracing::warn!("Shodan API: invalid or missing API key");
            return Ok(Vec::new());
        }
        let body = resp.text().await.unwrap_or_default();
        return Err(Error::DiscoveryHttp {
            src: "shodan",
            detail: format!("HTTP {status}: {body}"),
        });
    }

    let search: ShodanSearchResponse = resp.json().await.map_err(|e| Error::DiscoveryParse {
        src: "shodan",
        detail: format!("JSON parse: {e}"),
    })?;

    let mut domains: Vec<String> = Vec::new();
    for m in &search.matches {
        for hostname in &m.hostnames {
            let domain = hostname.trim().to_lowercase();
            if !domain.is_empty() && !domains.contains(&domain) {
                domains.push(domain);
            }
        }
        // Also check http.host field
        if let Some(http) = &m.http {
            let host = http.host.trim().to_lowercase();
            if !host.is_empty() && !domains.contains(&host) {
                domains.push(host);
            }
        }
    }

    Ok(domains)
}

fn urlencoding(s: &str) -> String {
    s.replace('%', "%25")
        .replace(' ', "%20")
        .replace('&', "%26")
        .replace('=', "%3D")
        .replace('+', "%2B")
        .replace('"', "%22")
}
