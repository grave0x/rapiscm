//! URL extraction from HTTP response headers.

use reqwest::Url;

/// Extracts URLs from common response headers (Location, Link, Content-Location).
pub fn extract_headers(headers: &[(String, String)], base: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    for (name, value) in headers {
        let lower = name.to_lowercase();
        if (lower == "location" || lower == "content-location")
            && let Ok(u) = base.join(value)
        {
            urls.push(u);
        }
        if lower == "link" {
            for part in value.split(',') {
                if let Some(inner) = part.trim().strip_prefix('<')
                    && let Some(end) = inner.find('>')
                    && let Ok(u) = base.join(&inner[..end])
                {
                    urls.push(u);
                }
            }
        }
    }
    urls
}
