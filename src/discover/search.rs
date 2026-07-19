//! Google Custom Search API — dork-based domain discovery.
//!
//! Uses the Google Programmable Search API to find domains related to an org.
//! Requires `google_api_key` and `google_cx` in config.toml.
//!
//! Queries: `site:* "{org}"` — finds pages mentioning the org on various sites.

use crate::error::{Error, Result};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct GoogleSearchResponse {
    #[serde(default)]
    items: Vec<GoogleSearchItem>,
}

#[derive(Debug, Deserialize)]
struct GoogleSearchItem {
    #[serde(default)]
    link: String,
    #[serde(default)]
    #[serde(rename = "displayLink")]
    display_link: String,
}

/// Search Google for domains related to an organization.
/// Returns a list of unique domains found in search results.
pub async fn google_search(org: &str, api_key: &str, cx: &str) -> Result<Vec<String>> {
    let query = format!("site:* \"{org}\"");
    let url = format!(
        "https://www.googleapis.com/customsearch/v1?key={}&cx={}&q={}&num=10",
        api_key,
        cx,
        urlencoding(&query)
    );

    let resp = reqwest::get(&url).await.map_err(|e| Error::DiscoveryHttp {
        src: "search",
        detail: e.to_string(),
    })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(Error::DiscoveryHttp {
            src: "search",
            detail: format!("HTTP {status}: {body}"),
        });
    }

    let search: GoogleSearchResponse = resp.json().await.map_err(|e| Error::DiscoveryParse {
        src: "search",
        detail: format!("JSON parse: {e}"),
    })?;

    let mut domains: Vec<String> = Vec::new();
    for item in &search.items {
        // Extract domain from displayLink or link
        let domain = if !item.display_link.is_empty() {
            item.display_link.clone()
        } else {
            extract_domain(&item.link)
        };
        if !domain.is_empty() && !domains.contains(&domain) {
            domains.push(domain.to_lowercase());
        }
    }

    Ok(domains)
}

/// Extract domain from a full URL.
fn extract_domain(url: &str) -> String {
    let url = url
        .trim_start_matches("https://")
        .trim_start_matches("http://")
        .trim_start_matches("www.");
    url.split('/')
        .next()
        .unwrap_or("")
        .split(':')
        .next()
        .unwrap_or("")
        .to_string()
}

fn urlencoding(s: &str) -> String {
    s.replace('%', "%25")
        .replace(' ', "%20")
        .replace('&', "%26")
        .replace('=', "%3D")
        .replace('+', "%2B")
        .replace('"', "%22")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_domain() {
        assert_eq!(
            extract_domain("https://www.example.com/page"),
            "example.com"
        );
        assert_eq!(
            extract_domain("http://sub.example.com:8080/path"),
            "sub.example.com"
        );
        assert_eq!(extract_domain("example.com"), "example.com");
    }
}
