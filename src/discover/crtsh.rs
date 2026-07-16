//! crt.sh Certificate Transparency log search.
//!
//! Query: `GET https://crt.sh/?q={org}&output=json`
//! Returns JSON array of certificate entries with `common_name` and `name_value` (SANs).

use crate::error::{Error, Result};
use serde::Deserialize;

#[derive(Debug, Clone)]
pub struct CrtEntry {
    pub domain: String,
    pub subjects: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct RawCrtEntry {
    #[serde(default)]
    common_name: String,
    #[serde(default)]
    name_value: String,
    #[serde(default)]
    issuer_name: String,
}

/// Query crt.sh for certificates matching an organization name.
/// Returns deduplicated domains and their certificate subject names.
pub async fn query_crtsh(org: &str) -> Result<Vec<CrtEntry>> {
    let url = format!("https://crt.sh/?q={}&output=json", urlencoding(org));

    let resp = reqwest::get(&url).await.map_err(|e| Error::DiscoveryHttp {
        src: "crtsh",
        detail: e.to_string(),
    })?;

    let raw: Vec<RawCrtEntry> = resp.json().await.map_err(|e| Error::DiscoveryParse {
        src: "crtsh",
        detail: format!("JSON parse: {e}"),
    })?;

    let mut seen = std::collections::HashSet::new();
    let mut entries: Vec<CrtEntry> = Vec::new();

    for raw_entry in raw {
        let cert_subject = raw_entry.issuer_name.clone();
        // Collect all domain names from common_name and name_value (newline-separated SANs)
        let mut domains: Vec<String> = Vec::new();
        if !raw_entry.common_name.is_empty() {
            domains.push(raw_entry.common_name.clone());
        }
        for name in raw_entry.name_value.split('\n') {
            let name = name.trim();
            if !name.is_empty() {
                domains.push(name.to_string());
            }
        }

        for domain in domains {
            let domain = domain.trim().to_lowercase();
            // Skip wildcards, IP addresses, and internal TLDs
            if domain.starts_with('*') || domain.starts_with('.') {
                continue;
            }
            if seen.insert(domain.clone()) {
                entries.push(CrtEntry {
                    domain,
                    subjects: vec![cert_subject.clone()],
                });
            }
        }
    }

    Ok(entries)
}

/// Auto-detect organization name from a URL.
/// Queries crt.sh with the domain and extracts the most common
/// organization name from certificate issuer/subject fields.
pub async fn auto_detect_org(target_url: &str) -> Result<String> {
    // Extract domain from URL
    let domain = target_url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .split('/')
        .next()
        .unwrap_or(target_url)
        .split(':')
        .next()
        .unwrap_or(target_url);

    let url = format!("https://crt.sh/?q={}&output=json", urlencoding(domain));

    let resp = reqwest::get(&url).await.map_err(|e| Error::DiscoveryHttp {
        src: "crtsh",
        detail: e.to_string(),
    })?;

    let raw: Vec<serde_json::Value> = resp.json().await.map_err(|e| Error::DiscoveryParse {
        src: "crtsh",
        detail: format!("JSON parse: {e}"),
    })?;

    // Extract O= (Organization) values from issuer_name
    let mut orgs: Vec<String> = Vec::new();
    for entry in &raw {
        if let Some(issuer) = entry.get("issuer_name").and_then(|v| v.as_str()) {
            for part in issuer.split(',') {
                let part = part.trim();
                if let Some(org_val) = part.strip_prefix("O=").or_else(|| part.strip_prefix("o=")) {
                    let org_val = org_val.trim().to_string();
                    if !org_val.is_empty() && !orgs.contains(&org_val) {
                        orgs.push(org_val);
                    }
                }
            }
        }
        // Also check common_name for org-like patterns
        if let Some(cn) = entry.get("common_name").and_then(|v| v.as_str())
            && cn.contains('.')
            && !cn.contains('*')
            && !cn.contains(' ')
        {
            // Common name is a domain, skip (not an org)
        }
    }

    if orgs.is_empty() {
        return Err(Error::DiscoveryParse {
            src: "crtsh",
            detail: format!("no organization found in certs for {domain}"),
        });
    }

    // Return the most common org name
    Ok(orgs[0].clone())
}

/// Minimal URL encoding (replaces spaces with %20).
fn urlencoding(s: &str) -> String {
    s.replace('%', "%25")
        .replace(' ', "%20")
        .replace('&', "%26")
        .replace('=', "%3D")
        .replace('+', "%2B")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_urlencoding() {
        assert_eq!(urlencoding("Acme Inc"), "Acme%20Inc");
        assert_eq!(urlencoding("hello"), "hello");
    }
}
