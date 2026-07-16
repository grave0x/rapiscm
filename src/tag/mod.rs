/// Tag engine — classifies endpoints into categories based on URL, method,
/// response characteristics, and spec metadata.
use crate::types::{Endpoint, ResponseResult, Severity};

pub type Tags = Vec<String>;

/// Generate tags for an endpoint from its URL, method, and spec metadata.
pub fn tag_endpoint(ep: &Endpoint) -> Tags {
    let mut tags = Tags::new();
    let path = ep.url.path().to_lowercase();
    let method = ep.method.as_str();

    // Method-based tags
    match method {
        "GET" => tags.push("read".into()),
        "POST" => tags.push("create".into()),
        "PUT" | "PATCH" => tags.push("update".into()),
        "DELETE" => tags.push("delete".into()),
        _ => {}
    }

    // Path-based tags
    if path.contains("/api/") || path == "/api" {
        tags.push("rest".into());
    }
    if path.contains("/v1/") || path.starts_with("/v1") || path.contains("/v1") {
        tags.push("v1".into());
    }
    if path.contains("/v2/") || path.starts_with("/v2") {
        tags.push("v2".into());
    }
    if path.contains("/v3/") || path.starts_with("/v3") {
        tags.push("v3".into());
    }
    if path.contains("/graphql") || path.contains("/graphiql") {
        tags.push("graphql".into());
    }
    if path.contains("/admin") || path.contains("/administrator") {
        tags.push("admin".into());
    }
    if path.contains("/health")
        || path.contains("/healthz")
        || path.contains("/readyz")
        || path.contains("/ping")
    {
        tags.push("health".into());
    }
    if path.contains("/auth")
        || path.contains("/login")
        || path.contains("/oauth")
        || path.contains("/token")
    {
        tags.push("auth".into());
    }
    if path.contains("/users")
        || path.contains("/user")
        || path.contains("/accounts")
        || path.contains("/profile")
    {
        tags.push("users".into());
    }
    if path.contains("/upload")
        || path.contains("/download")
        || path.contains("/export")
        || path.contains("/import")
    {
        tags.push("file".into());
    }
    if path.contains("/ws")
        || path.contains("/websocket")
        || path.contains("/socket.io")
        || path.contains("/events")
    {
        tags.push("websocket".into());
    }
    if path.contains("/docs")
        || path.contains("/swagger")
        || path.contains("/openapi")
        || path.contains("/api-docs")
    {
        tags.push("docs".into());
    }
    if path.contains("/.env") || path.contains("/.git") {
        tags.push("exposed".into());
    }
    if path.contains("/debug") || path.contains("/trace") || path.contains("/actuator") {
        tags.push("debug".into());
    }

    // Spec metadata tags (if available from parser)
    if ep.expected_status.is_some() {
        tags.push("in-spec".into());
    }

    tags
}

/// Generate tags for a response result from response characteristics.
pub fn tag_response(result: &ResponseResult) -> Tags {
    let mut tags = Tags::new();

    // Status-based tags
    match result.status_code / 100 {
        2 => tags.push("success".into()),
        3 => tags.push("redirect".into()),
        4 => tags.push("client-error".into()),
        5 => tags.push("server-error".into()),
        _ => {}
    }

    // Timing-based tags
    if result.response_time_ms > 2000 {
        tags.push("slow".into());
    }
    if result.response_time_ms > 10000 {
        tags.push("timeout".into());
    }

    // Check-based tags
    for check in &result.checks {
        if !check.passed {
            match check.severity {
                Severity::Critical => tags.push(format!(
                    "fail-{}",
                    check.name.to_lowercase().replace(' ', "-")
                )),
                Severity::Warn => tags.push(format!(
                    "warn-{}",
                    check.name.to_lowercase().replace(' ', "-")
                )),
                Severity::Info => {}
            }
        }
    }

    // Content-type based
    for (k, v) in &result.response_headers {
        if k.eq_ignore_ascii_case("content-type") {
            if v.contains("json") {
                tags.push("json".into());
            }
            if v.contains("xml") {
                tags.push("xml".into());
            }
            if v.contains("html") {
                tags.push("html".into());
            }
            break;
        }
    }

    tags
}

/// Filter endpoints by tag rules.
pub fn filter_by_tags(items: &[Tags], include: &[String], exclude: &[String]) -> Vec<bool> {
    items
        .iter()
        .map(|tags| {
            // Include: all must match (AND)
            if !include.is_empty() && !include.iter().all(|t| tags.iter().any(|tag| tag == t)) {
                return false;
            }
            // Exclude: any match → reject
            if !exclude.is_empty() && exclude.iter().any(|t| tags.iter().any(|tag| tag == t)) {
                return false;
            }
            true
        })
        .collect()
}

/// Filter a slice of Endpoints in-place by tag rules.
pub fn filter_endpoints(endpoints: &mut Vec<Endpoint>, include: &[String], exclude: &[String]) {
    if include.is_empty() && exclude.is_empty() {
        return;
    }
    let tags: Vec<Tags> = endpoints.iter().map(|ep| ep.tags.clone()).collect();
    let mask = filter_by_tags(&tags, include, exclude);
    let mut i = 0;
    endpoints.retain(|_| {
        let keep = mask[i];
        i += 1;
        keep
    });
}

#[cfg(test)]
mod tests {
    use super::*;
    use reqwest::Url;

    fn endpoint(path: &str) -> Endpoint {
        Endpoint {
            method: reqwest::Method::GET,
            url: Url::parse(&format!("https://example.com{}", path)).unwrap(),
            headers: vec![],
            body: None,
            expected_status: None,
            tags: vec![],
        }
    }

    #[test]
    fn test_rest_tag() {
        let ep = endpoint("/api/v2/users");
        let tags = tag_endpoint(&ep);
        assert!(tags.contains(&"rest".into()));
        assert!(tags.contains(&"v2".into()));
        assert!(tags.contains(&"users".into()));
    }

    #[test]
    fn test_graphql_tag() {
        let ep = endpoint("/graphql");
        let tags = tag_endpoint(&ep);
        assert!(tags.contains(&"graphql".into()));
    }

    #[test]
    fn test_health_tag() {
        let ep = endpoint("/healthz");
        let tags = tag_endpoint(&ep);
        assert!(tags.contains(&"health".into()));
    }

    #[test]
    fn test_method_tags() {
        let mut ep = endpoint("/api/users");
        ep.method = reqwest::Method::DELETE;
        let tags = tag_endpoint(&ep);
        assert!(tags.contains(&"delete".into()));
    }

    #[test]
    fn test_filter_include() {
        let items = vec![
            vec!["rest".into(), "v2".into()],
            vec!["rest".into(), "v1".into()],
        ];
        let mask = filter_by_tags(&items, &["rest".into(), "v2".into()], &[]);
        assert!(mask[0]);
        assert!(!mask[1]);
    }

    #[test]
    fn test_filter_exclude() {
        let items = vec![vec!["rest".into()], vec!["admin".into()]];
        let mask = filter_by_tags(&items, &[], &["admin".into()]);
        assert!(mask[0]);
        assert!(!mask[1]);
    }
}
