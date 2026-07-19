//! RDAP (Registration Data Access Protocol) reverse WHOIS search.
//!
//! Uses ARIN RDAP bootstrap to find organizations matching a name,
//! then extracts associated IP network descriptions and domain-like names.
//!
//! Endpoints:
//!   <https://rdap.arin.net/registry/entities?fn={org}>

use crate::error::{Error, Result};

/// Search RDAP for domains/IPs associated with an organization.
/// Returns a list of domain-like names and network names extracted from RDAP data.
pub async fn rdap_discover(org: &str) -> Result<Vec<String>> {
    let url = format!(
        "https://rdap.arin.net/registry/entities?fn={}",
        urlencoding(org)
    );

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(15))
        .build()
        .map_err(|e| Error::DiscoveryHttp {
            src: "rdap",
            detail: e.to_string(),
        })?;

    let resp = client
        .get(&url)
        .send()
        .await
        .map_err(|e| Error::DiscoveryHttp {
            src: "rdap",
            detail: format!("HTTP request: {e}"),
        })?;

    // ARIN RDAP returns 200 with empty list if no results
    if !resp.status().is_success() {
        return Ok(Vec::new());
    }

    // The ARIN RDAP entity search returns a flat JSON with entity_search_results
    // but the structure varies. Try to parse as an RdapSearchResponse.
    let body = resp.text().await.map_err(|e| Error::DiscoveryHttp {
        src: "rdap",
        detail: format!("read body: {e}"),
    })?;

    // Try JSON parse; if it fails, return empty (not an error — the API may return
    // a different structure for some queries).
    let parsed: serde_json::Value = match serde_json::from_str(&body) {
        Ok(v) => v,
        Err(_) => return Ok(Vec::new()),
    };

    let mut results: Vec<String> = Vec::new();

    // Entity search results can be in `entity_search_results` or top-level
    let entities = parsed
        .get("entity_search_results")
        .or_else(|| parsed.get("entities"))
        .and_then(|v| v.as_array().cloned())
        .unwrap_or_default();

    for entity in &entities {
        // Extract org name from vcard
        if let Some(org_name) = extract_org_from_vcard(entity) {
            // Split into words, treat as potential domain parts
            for word in org_name.split_whitespace() {
                let clean: String = word
                    .chars()
                    .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-')
                    .collect();
                if clean.contains('.') && !clean.contains("//") {
                    results.push(clean.to_lowercase());
                }
            }
        }

        // Follow links to networks
        if let Some(links) = entity.get("links").and_then(|v| v.as_array()) {
            for link in links {
                let href = match link.get("href").and_then(|v| v.as_str()) {
                    Some(h) => h.to_string(),
                    None => continue,
                };
                if (href.contains("/registry/ip/") || href.contains("/registry/nets/"))
                    && let Ok(Some(names)) = fetch_network_names(&client, &href).await
                {
                    results.extend(names);
                }
            }
        }
    }

    results.sort();
    results.dedup();
    Ok(results)
}

/// Extract organization name from a vCard array.
fn extract_org_from_vcard(entity: &serde_json::Value) -> Option<String> {
    let vcard = entity.get("vcard_array")?.as_array()?;
    for item in vcard {
        let arr = item.as_array()?;
        if arr.len() < 4 {
            continue;
        }
        // vCard format: ["org", {}, "text", "Org Name"]
        if (arr[0].as_str() == Some("org") || arr[0].as_str() == Some("fn"))
            && let Some(val) = arr[3].as_str()
        {
            let val = val.trim();
            if !val.is_empty() {
                return Some(val.to_string());
            }
        }
    }
    None
}

/// Fetch a linked RDAP object and extract domain-like names.
async fn fetch_network_names(client: &reqwest::Client, href: &str) -> Result<Option<Vec<String>>> {
    let resp = match client
        .get(href)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
    {
        Ok(r) => r,
        Err(_) => return Ok(None),
    };

    if !resp.status().is_success() {
        return Ok(None);
    }

    let body: serde_json::Value = match resp.json().await {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let mut names: Vec<String> = Vec::new();

    // RDAP network objects have `name` and `description` fields
    if let Some(name) = body.get("name").and_then(|v| v.as_str())
        && name.contains('.')
    {
        names.push(name.to_lowercase());
    }
    if let Some(desc) = body.get("description").and_then(|v| v.as_array()) {
        for line in desc {
            if let Some(text) = line.as_str() {
                for word in text.split_whitespace() {
                    let clean: String = word
                        .chars()
                        .filter(|c| c.is_alphanumeric() || *c == '.' || *c == '-')
                        .collect();
                    if clean.contains('.') {
                        names.push(clean.to_lowercase());
                    }
                }
            }
        }
    }

    if names.is_empty() {
        Ok(None)
    } else {
        Ok(Some(names))
    }
}

fn urlencoding(s: &str) -> String {
    s.replace('%', "%25")
        .replace(' ', "%20")
        .replace('&', "%26")
        .replace('=', "%3D")
        .replace('+', "%2B")
}
