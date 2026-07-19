//! Deep spec — technical breakdown of scan results.
//!
//! When `--deep-spec` is set, produces a YAML report with:
//! - Endpoint dependency graph (which endpoints link to which)
//! - Response structure fingerprint (field names, types, patterns)
//! - API style detection (RESTful, RPC, GraphQL)
//! - Authentication requirements per endpoint
//! - Payload analysis (request/response body schemas)

use crate::types::ResponseResult;

/// Deep spec analysis result.
#[derive(serde::Serialize)]
pub struct DeepSpec {
    /// API style detected.
    pub api_style: Vec<String>,
    /// Authentication coverage.
    pub auth: AuthAnalysis,
    /// Endpoint breakdown.
    pub endpoints: Vec<EndpointSpec>,
    /// Unique response content types observed.
    pub content_types: Vec<String>,
    /// HTTP methods in use.
    pub methods: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct AuthAnalysis {
    pub total_endpoints: usize,
    pub with_auth: usize,
    pub without_auth: usize,
    pub auth_required_endpoints: Vec<String>,
    pub public_endpoints: Vec<String>,
}

#[derive(serde::Serialize)]
pub struct EndpointSpec {
    pub method: String,
    pub url: String,
    pub status: u16,
    pub response_time_ms: u64,
    pub response_size: usize,
    pub has_json_body: bool,
    pub has_html_body: bool,
    pub body_keys: Vec<String>,
    pub tags: Vec<String>,
    pub checks_passed: usize,
    pub checks_failed: usize,
}

/// Run deep analysis on scan results and produce a YAML spec.
pub fn analyze(results: &[ResponseResult]) -> DeepSpec {
    let api_style = detect_api_style(results);
    let auth = analyze_auth(results);
    let content_types = collect_content_types(results);
    let methods = collect_methods(results);

    let endpoints: Vec<EndpointSpec> = results
        .iter()
        .map(|r| {
            let body_str = String::from_utf8_lossy(&r.response_body);
            let (has_json, has_html, keys) = analyze_body(&body_str);
            EndpointSpec {
                method: r.endpoint_method.clone(),
                url: r.endpoint_url.clone(),
                status: r.status_code,
                response_time_ms: r.response_time_ms,
                response_size: r.response_size,
                has_json_body: has_json,
                has_html_body: has_html,
                body_keys: keys,
                tags: r.tags.clone(),
                checks_passed: r.checks.iter().filter(|c| c.passed).count(),
                checks_failed: r.checks.iter().filter(|c| !c.passed).count(),
            }
        })
        .collect();

    DeepSpec {
        api_style,
        auth,
        endpoints,
        content_types,
        methods,
    }
}

/// Render deep spec as YAML string.
pub fn to_yaml(spec: &DeepSpec) -> Result<String, serde_yaml::Error> {
    serde_yaml::to_string(spec)
}

fn detect_api_style(results: &[ResponseResult]) -> Vec<String> {
    let mut styles = Vec::new();
    let urls: Vec<&str> = results.iter().map(|r| r.endpoint_url.as_str()).collect();

    if urls.iter().any(|u| u.contains("/graphql")) {
        styles.push("graphql".into());
    }
    let path_count = |s: &str| -> usize { urls.iter().filter(|u| u.contains(s)).count() };
    if path_count("/api/") as f64 > urls.len() as f64 * 0.3 {
        styles.push("restful".into());
    }
    if path_count("/rpc") > 0 || path_count("/jsonrpc") > 0 {
        styles.push("rpc".into());
    }
    if results.iter().any(|r| r.endpoint_method == "POST")
        && urls.iter().any(|u| u.contains("/graphql"))
    {
        styles.push("graphql:operations".into());
    }
    if styles.is_empty() {
        styles.push("unknown".into());
    }
    styles
}

fn analyze_auth(results: &[ResponseResult]) -> AuthAnalysis {
    let total = results.len();
    let mut protected = Vec::new();
    let mut public = Vec::new();

    for r in results {
        let path = extract_path(&r.endpoint_url);
        if r.status_code == 401 || r.status_code == 403 {
            protected.push(path);
        } else {
            public.push(path);
        }
    }

    AuthAnalysis {
        total_endpoints: total,
        with_auth: protected.len(),
        without_auth: public.len(),
        auth_required_endpoints: protected,
        public_endpoints: public,
    }
}

fn collect_content_types(results: &[ResponseResult]) -> Vec<String> {
    let mut types: Vec<String> = Vec::new();
    for r in results {
        for (k, v) in &r.response_headers {
            if k.to_lowercase() == "content-type" {
                let ct = v.split(';').next().unwrap_or(v).trim().to_string();
                if !types.contains(&ct) {
                    types.push(ct);
                }
            }
        }
    }
    types.sort();
    types
}

fn collect_methods(results: &[ResponseResult]) -> Vec<String> {
    let mut methods: Vec<String> = results.iter().map(|r| r.endpoint_method.clone()).collect();
    methods.sort();
    methods.dedup();
    methods
}

fn analyze_body(body: &str) -> (bool, bool, Vec<String>) {
    let has_json = body.trim().starts_with('{') || body.trim().starts_with('[');
    let has_html = body.contains("<html") || body.contains("<!DOCTYPE");
    let keys = if has_json {
        extract_json_keys(body)
    } else {
        vec![]
    };
    (has_json, has_html, keys)
}

fn extract_json_keys(body: &str) -> Vec<String> {
    let mut keys = Vec::new();
    if let Ok(val) = serde_json::from_str::<serde_json::Value>(body) {
        match val {
            serde_json::Value::Object(map) => {
                for k in map.keys() {
                    keys.push(k.clone());
                }
            }
            serde_json::Value::Array(arr) => {
                if let Some(first) = arr.first()
                    && let serde_json::Value::Object(map) = first
                {
                    for k in map.keys() {
                        keys.push(k.clone());
                    }
                }
            }
            _ => {}
        }
    }
    keys.sort();
    keys.dedup();
    keys
}

fn extract_path(url: &str) -> String {
    if let Ok(parsed) = reqwest::Url::parse(url) {
        parsed.path().to_string()
    } else {
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Check, Severity};

    fn sample_result(url: &str, status: u16, method: &str, json_body: bool) -> ResponseResult {
        let body = if json_body {
            br#"{"id":1,"name":"test","email":"a@b.com"}"#.to_vec()
        } else {
            br#"<html><body>OK</body></html>"#.to_vec()
        };
        ResponseResult {
            endpoint_method: method.into(),
            endpoint_url: url.into(),
            status_code: status,
            response_time_ms: 42,
            response_size: body.len(),
            response_headers: vec![(
                "content-type".into(),
                if json_body {
                    "application/json".into()
                } else {
                    "text/html".into()
                },
            )],
            response_body: body,
            expected_status: None,
            timestamp: None,
            checks: vec![Check {
                name: "hsts".into(),
                passed: true,
                severity: Severity::Info,
                message: "ok".into(),
            }],
            error: None,
            tags: vec![],
            trackers: vec![],
        }
    }

    #[test]
    fn test_detect_rest_style() {
        let results = vec![
            sample_result("https://api.example.com/api/users", 200, "GET", true),
            sample_result("https://api.example.com/api/posts", 200, "POST", true),
        ];
        let spec = analyze(&results);
        assert!(spec.api_style.iter().any(|s| s == "restful"));
    }

    #[test]
    fn test_detect_graphql() {
        let results = vec![sample_result(
            "https://api.example.com/graphql",
            200,
            "POST",
            true,
        )];
        let spec = analyze(&results);
        assert!(spec.api_style.iter().any(|s| s.contains("graphql")));
    }

    #[test]
    fn test_auth_analysis() {
        let results = vec![
            sample_result("https://api.example.com/admin", 401, "GET", false),
            sample_result("https://api.example.com/public", 200, "GET", false),
        ];
        let spec = analyze(&results);
        assert_eq!(spec.auth.with_auth, 1);
        assert_eq!(spec.auth.without_auth, 1);
    }

    #[test]
    fn test_content_types() {
        let results = vec![
            sample_result("https://api.example.com/json", 200, "GET", true),
            sample_result("https://api.example.com/html", 200, "GET", false),
        ];
        let spec = analyze(&results);
        assert!(spec.content_types.contains(&"application/json".to_string()));
        assert!(spec.content_types.contains(&"text/html".to_string()));
    }

    #[test]
    fn test_extract_json_keys() {
        let body = r#"{"id":1,"name":"test","items":[1,2,3]}"#;
        let keys = extract_json_keys(body);
        assert!(keys.contains(&"id".to_string()));
        assert!(keys.contains(&"name".to_string()));
        assert!(keys.contains(&"items".to_string()));
        assert_eq!(keys.len(), 3);
    }

    #[test]
    fn test_to_yaml() {
        let results = vec![
            sample_result("https://example.com/api/users", 200, "GET", true),
            sample_result("https://example.com/api/posts", 200, "GET", true),
        ];
        let spec = analyze(&results);
        let yaml = to_yaml(&spec).unwrap();
        assert!(yaml.contains("api_style"));
        assert!(yaml.contains("restful"));
        assert!(yaml.contains("endpoints"));
    }
}
