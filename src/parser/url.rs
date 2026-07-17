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
    fn test_api_wordlist_all_valid() {
        let base = reqwest::Url::parse("https://example.com").unwrap();
        for path in api_wordlist() {
            assert!(base.join(path).is_ok(), "invalid wordlist path: {path}");
        }
    }
}
