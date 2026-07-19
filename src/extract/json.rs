//! URL extraction from JSON response bodies.

/// Extract URLs from JSON responses by walking the value tree
/// and collecting string values that look like URLs.
use reqwest::Url;

pub fn extract_json(body: &[u8], base: &Url) -> Vec<Url> {
    let Ok(val) = serde_json::from_slice::<serde_json::Value>(body) else {
        return vec![];
    };
    let mut urls = Vec::new();
    walk_value(&val, base, &mut urls);
    urls
}

fn walk_value(val: &serde_json::Value, base: &Url, out: &mut Vec<Url>) {
    match val {
        serde_json::Value::String(s) => {
            try_add_url(s, base, out);
        }
        serde_json::Value::Object(map) => {
            // Keys named like URL fields are prioritized
            let url_keys = [
                "url",
                "link",
                "href",
                "endpoint",
                "redirect",
                "callback",
                "webhook",
                "self",
                "next",
                "prev",
                "first",
                "last",
                "avatar_url",
                "profile_url",
                "html_url",
            ];
            // Scan URL-keyed fields first
            for key in &url_keys {
                if let Some(serde_json::Value::String(v)) = map.get(*key) {
                    try_add_url(v, base, out);
                }
            }
            // Then scan all values recursively
            for (_k, v) in map {
                walk_value(v, base, out);
            }
        }
        serde_json::Value::Array(arr) => {
            for v in arr {
                walk_value(v, base, out);
            }
        }
        _ => {}
    }
}

fn try_add_url(s: &str, base: &Url, out: &mut Vec<Url>) {
    // Try as absolute URL first
    if let Ok(url) = Url::parse(s)
        && (url.scheme() == "http" || url.scheme() == "https")
    {
        out.push(url);
        return;
    }
    // Try as relative URL
    if (s.starts_with('/') || s.starts_with("./") || s.starts_with("../"))
        && let Ok(url) = base.join(s)
    {
        out.push(url);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_absolute_urls() {
        let base = Url::parse("https://example.com").unwrap();
        let json = br#"{"url":"https://api.example.com/users","next":"/api/v2/users"}"#;
        let urls = extract_json(json, &base);
        assert!(urls.iter().any(|u| u.as_str().contains("api.example.com")));
        assert!(urls.iter().any(|u| u.as_str().contains("/api/v2/users")));
    }

    #[test]
    fn test_extract_nested() {
        let base = Url::parse("https://example.com").unwrap();
        let json = br#"{"data":{"links":{"self":"/api/v1/users"}}}"#;
        let urls = extract_json(json, &base);
        assert!(urls.iter().any(|u| u.as_str().contains("/api/v1/users")));
    }

    #[test]
    fn test_empty_on_invalid() {
        let base = Url::parse("https://example.com").unwrap();
        let urls = extract_json(b"not json", &base);
        assert!(urls.is_empty());
    }
}
