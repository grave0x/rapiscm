/// URL pattern matching and wordlist for API endpoint discovery.
use reqwest::Url;

/// Common API path wordlist for probing.
pub fn api_wordlist() -> Vec<&'static str> {
    vec![
        "/api",
        "/api/v1",
        "/api/v2",
        "/v1",
        "/v2",
        "/v3",
        "/graphql",
        "/rest",
        "/swagger.json",
        "/openapi.json",
        "/api-docs",
        "/health",
        "/status",
        "/metrics",
        "/ping",
    ]
}

/// Heuristic: does the URL path look like an API endpoint?
pub fn is_api_endpoint(url: &Url) -> bool {
    let path = url.path().to_lowercase();
    if path.contains("/api/")
        || path.contains("/v1/")
        || path.contains("/v2/")
        || path.contains("/v3/")
        || path.contains("/rest/")
        || path.contains("/graphql")
        || path.ends_with("/api")
        || path.ends_with("/swagger.json")
        || path.ends_with("/openapi.json")
        || path.ends_with("/health")
        || path.ends_with("/status")
        || path.ends_with("/metrics")
        || path.ends_with("/ping")
    {
        return true;
    }
    false
}

/// Check if two URLs share the same origin (scheme + host + port).
pub fn same_origin(a: &Url, b: &Url) -> bool {
    a.scheme() == b.scheme() && a.host_str() == b.host_str() && a.port() == b.port()
}

/// Normalize URL for dedup: lowercase scheme+host, remove fragment, sort query params.
pub fn normalize(url: &Url) -> Url {
    let mut result = url.clone();
    result.set_fragment(None);
    // Lowercase scheme and host
    if let Ok(parsed) = Url::parse(&url.as_str().to_lowercase()) {
        result = parsed;
        result.set_fragment(None);
    }
    result
}

/// Dedup a sorted list of URLs, removing exact normalized duplicates.
pub fn dedup(urls: Vec<Url>) -> Vec<Url> {
    let mut seen = std::collections::HashSet::new();
    urls.into_iter()
        .filter(|u| seen.insert(normalize(u)))
        .collect()
}

/// Fingerprint a URL path by replacing variable segments with placeholders.
/// e.g., /users/123/posts/a1b2c3 → /users/{n}/posts/{hash}
pub fn fingerprint_path(path: &str) -> String {
    path.split('/')
        .map(|segment| {
            if segment.is_empty() {
                return segment.to_string();
            }
            if segment.chars().all(|c| c.is_ascii_digit()) {
                "{n}".to_string()
            } else if segment.len() == 36 && segment.chars().filter(|&c| c == '-').count() == 4 {
                "{uuid}".to_string()
            } else if segment.len() >= 6 && segment.chars().all(|c| c.is_ascii_hexdigit()) {
                "{hex}".to_string()
            } else if segment.contains('-') && segment.chars().any(|c| c.is_ascii_digit()) {
                "{token}".to_string()
            } else {
                segment.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_api_endpoint() {
        assert!(is_api_endpoint(
            &reqwest::Url::parse("https://example.com/api/users").unwrap()
        ));
        assert!(!is_api_endpoint(
            &reqwest::Url::parse("https://example.com/about").unwrap()
        ));
    }

    #[test]
    fn test_same_origin() {
        let a = Url::parse("https://example.com/api").unwrap();
        let b = Url::parse("https://example.com/other").unwrap();
        let c = Url::parse("https://other.com/api").unwrap();
        assert!(same_origin(&a, &b));
        assert!(!same_origin(&a, &c));
    }

    #[test]
    fn test_fingerprint_path() {
        assert_eq!(
            fingerprint_path("/users/123/posts/a1b2c3"),
            "/users/{n}/posts/{hex}"
        );
        assert_eq!(fingerprint_path("/api/v1/health"), "/api/v1/health");
        assert_eq!(
            fingerprint_path("/items/550e8400-e29b-41d4-a716-446655440000"),
            "/items/{uuid}"
        );
    }

    #[test]
    fn test_dedup() {
        let urls = vec![
            Url::parse("https://example.com/api").unwrap(),
            Url::parse("https://example.com/api").unwrap(),
            Url::parse("https://example.com/other").unwrap(),
        ];
        let deduped = dedup(urls);
        assert_eq!(deduped.len(), 2);
    }

    #[test]
    fn test_api_wordlist_all_valid() {
        let base = reqwest::Url::parse("https://example.com").unwrap();
        for path in api_wordlist() {
            assert!(base.join(path).is_ok(), "invalid wordlist path: {path}");
        }
    }
}
