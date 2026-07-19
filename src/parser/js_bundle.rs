//! JS bundle scanner — extracts API endpoints from JavaScript bundles.
//!
//! Goes beyond basic URL extraction by:
//! - Fetching `<script src>` bundles from HTML pages
//! - Parsing minified JS for route configs, API client calls, and path patterns
//! - Extracting API paths from template literals, concatenation, and configuration objects
//! - Discovering GraphQL query/mutation names
//! - Recognizing framework-specific route patterns (Next.js, Express, etc.)

use std::collections::HashSet;

use regex::Regex;
use reqwest::Url;

/// An API endpoint path extracted from a JS bundle with context.
#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct JsApiEndpoint {
    /// The HTTP method (or None if unknown).
    pub method: Option<String>,
    /// The full URL or path (relative paths are relative to the bundle origin).
    pub path: String,
    /// Where this was found (file URL, pattern type).
    pub source: String,
    /// Confidence: "high" (explicit route), "medium" (likely), "low" (possible).
    pub confidence: &'static str,
}

/// Fetch JS bundles from a page HTML and extract API endpoints.
pub async fn scan_bundles(
    client: &reqwest::Client,
    html: &str,
    base_url: &Url,
) -> anyhow::Result<Vec<JsApiEndpoint>> {
    let mut all = Vec::new();

    // Step 1: find all <script src> tags
    let bundle_urls = extract_script_srcs(html, base_url);

    for bundle_url in &bundle_urls {
        tracing::debug!("scanning JS bundle: {bundle_url}");
        match fetch_bundle(client, bundle_url).await {
            Ok(code) => {
                let found = extract_from_bundle(&code, bundle_url, base_url);
                all.extend(found);
            }
            Err(e) => tracing::warn!("failed to fetch JS bundle {bundle_url}: {e}"),
        }
    }

    Ok(all)
}

/// Extract API endpoints from JS code.
pub fn extract_from_bundle(js: &str, bundle_url: &Url, base_url: &Url) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    let mut seen = HashSet::new();

    for ep in extract_api_paths(js) {
        if seen.insert(ep.path.clone()) {
            results.push(ep);
        }
    }

    // Also run the basic extract_js from the existing module
    for url in crate::extract::js::extract_js(js, base_url) {
        let path = if url.origin() == base_url.origin() {
            url.path().to_string()
        } else {
            url.to_string()
        };
        if seen.insert(path.clone()) {
            results.push(JsApiEndpoint {
                method: None,
                path,
                source: bundle_url.to_string(),
                confidence: "medium",
            });
        }
    }

    results
}

async fn fetch_bundle(client: &reqwest::Client, url: &Url) -> anyhow::Result<String> {
    let resp = client.get(url.as_str()).send().await?;
    let bytes = resp.bytes().await?;
    Ok(String::from_utf8_lossy(&bytes).to_string())
}

fn extract_script_srcs(html: &str, base: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    // Match <script src="..."> and <script src='...'>
    for quote in ['"', '\''] {
        let pattern = format!("<script[^>]*src={quote}");
        let re = match Regex::new(&pattern) {
            Ok(r) => r,
            Err(_) => continue,
        };
        for m in re.find_iter(html) {
            let after = &html[m.end()..];
            if let Some(end) = after.find(quote) {
                let src = &after[..end];
                if let Ok(url) = base.join(src) {
                    urls.push(url);
                }
            }
        }
    }
    urls
}

fn extract_api_paths(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();

    // 1. Route configuration objects: { path: '/api/...' }, { url: '/api/...' }
    results.extend(find_route_config(js));
    // 2. Fetch / XMLHttpRequest calls with string paths
    results.extend(find_fetch_calls(js));
    // 3. API client patterns: api.get(...), client.post(...), axios.put(...)
    results.extend(find_api_client_calls(js));
    // 4. Template literals: `/api/${resource}`
    results.extend(find_template_literals(js));
    // 5. String concatenation: '/api/' + resource
    results.extend(find_concat_patterns(js));
    // 6. GraphQL operation strings
    results.extend(find_graphql_ops(js));
    // 7. Import paths that look like API routes
    results.extend(find_import_paths(js));
    // 8. String arrays with API paths
    results.extend(find_string_arrays(js));
    // 9. next/link href paths
    results.extend(find_router_links(js));
    // 10. Environment variable based URLs
    results.extend(find_env_urls(js));
    // 11. minified webpack/rollup path references
    results.extend(find_minified_paths(js));

    results
}

/// Find route configuration objects: `{path: '/api/...'}`, `{url: '/api/...'}`
fn find_route_config(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    // Match path:/url keys followed by a string
    for key in &["path", "url", "endpoint", "route", "uri", "target"] {
        // { path: '/api/users' } or "path":"/api/users"
        for quote in ['"', '\'', '`'] {
            let patterns = [
                format!("{key}:{quote}"),
                format!("{key}: {quote}"),
                format!("\"{key}\":{quote}"),
                format!("'{key}':{quote}"),
            ];
            for pat in &patterns {
                let mut pos = 0;
                while let Some(start) = js[pos..].find(pat.as_str()) {
                    let val_start = pos + start + pat.len();
                    if let Some(end) = js[val_start..].find(quote) {
                        let val = &js[val_start..val_start + end];
                        if looks_like_api_path(val) {
                            results.push(JsApiEndpoint {
                                method: extract_method_nearby(js, pos + start, val),
                                path: val.to_string(),
                                source: "route_config".into(),
                                confidence: "high",
                            });
                        }
                    }
                    pos = val_start + 1;
                }
            }
        }
    }
    results
}

/// Find fetch(`/api/...`) calls
fn find_fetch_calls(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    for func in &["fetch(", "fetch (", "request(", "xhr.open("] {
        for quote in ['"', '\'', '`'] {
            let pattern = format!("{func}{quote}");
            let mut pos = 0;
            while let Some(start) = js[pos..].find(&pattern) {
                let val_start = pos + start + pattern.len();
                if let Some(end) = js[val_start..].find(quote) {
                    let val = &js[val_start..val_start + end];
                    if looks_like_api_path(val) {
                        let method = extract_method_nearby(js, pos + start, val);
                        results.push(JsApiEndpoint {
                            method,
                            path: val.to_string(),
                            source: "fetch_call".into(),
                            confidence: "high",
                        });
                    }
                }
                pos = val_start + 1;
            }
        }
    }
    results
}

/// Find api.get(...), client.post(...), axios.put(...) etc.
fn find_api_client_calls(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    let http_methods = [
        "get", "post", "put", "patch", "delete", "head", "options", "request",
    ];
    // Match patterns like: api.get('/users'), client.post('/api/users'), axios.put(`/api/...`)
    for quote in ['"', '\'', '`'] {
        for method in &http_methods {
            // .get(  .post(  .put(  .patch(  .delete(
            let patterns = [format!(".{method}({quote}"), format!(".{method} ({quote}")];
            for pat in &patterns {
                let mut pos = 0;
                while let Some(start) = js[pos..].find(pat.as_str()) {
                    let val_start = pos + start + pat.len();
                    if let Some(end) = js[val_start..].find(quote) {
                        let val = &js[val_start..val_start + end];
                        if looks_like_api_path(val) {
                            // Infer method from the call pattern
                            let inferred = if *method == "request" {
                                None
                            } else {
                                Some(method.to_uppercase())
                            };
                            results.push(JsApiEndpoint {
                                method: inferred,
                                path: val.to_string(),
                                source: format!("api_client_{method}"),
                                confidence: "high",
                            });
                        }
                    }
                    pos = val_start + 1;
                }
            }
        }
    }
    results
}

/// Find template literals that look like API paths: `/api/${resource}`
fn find_template_literals(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    // Match backtick strings containing both /api/ and ${...}
    let mut pos = 0;
    while let Some(start) = js[pos..].find('`') {
        let template_start = pos + start + 1;
        if let Some(end) = js[template_start..].find('`') {
            let template = &js[template_start..template_start + end];
            // Check if it looks like an API path with interpolation
            if template.contains("/api/")
                || template.contains("/v1/")
                || template.contains("/v2/")
                || template.contains("/rest/")
                || template.contains("/graphql")
            {
                // Extract the static prefix parts
                let static_paths = extract_static_prefixes(template);
                for sp in static_paths {
                    if looks_like_api_path(&sp) {
                        results.push(JsApiEndpoint {
                            method: None,
                            path: sp,
                            source: "template_literal".into(),
                            confidence: "medium",
                        });
                    }
                }
            }
        }
        pos = template_start + 1;
    }
    results
}

/// Extract static prefixes from a template literal.
/// `/api/users/${id}/posts` → ["/api/users/", "/api/users//posts"]
fn extract_static_prefixes(template: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut in_interp = false;
    for c in template.chars() {
        if c == '$' {
            in_interp = true;
            if !current.is_empty() {
                parts.push(current.clone());
            }
            current.clear();
        } else if c == '}' && in_interp {
            in_interp = false;
        } else if !in_interp {
            current.push(c);
        }
    }
    if !current.is_empty() {
        parts.push(current);
    }
    parts
}

/// Find string concatenation patterns: '/api/' + something
fn find_concat_patterns(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    // Look for string + string where one part is an API path.
    // Matches patterns like "/api/" +  or  "/rest/api/v2/" +  or "/api/resource" +
    // Uses a simpler approach: find any quoted string followed by + that looks like an API path
    for quote in ['"', '\'', '`'] {
        let mut pos = 0;
        while let Some(start) = js[pos..].find(quote) {
            let val_start = pos + start + 1;
            if let Some(end) = js[val_start..].find(quote) {
                let val = &js[val_start..val_start + end];
                // Check for concatenation: "+" or " +" follows the closing quote
                let after = &js[val_start + end + 1..];
                let trimmed = after.trim_start();
                if trimmed.starts_with('+') && looks_like_api_path(val) {
                    results.push(JsApiEndpoint {
                        method: None,
                        path: val.to_string(),
                        source: "concat".into(),
                        confidence: "medium",
                    });
                }
            }
            pos = val_start + 1;
        }
    }
    results
}

/// Find GraphQL operation strings
fn find_graphql_ops(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    // Match strings containing "query" or "mutation" followed by a name
    for quote in ['"', '\'', '`'] {
        // gql`...`, graphql`...`, or raw strings with query/mutation
        let gql_patterns = [
            format!("gql{quote}"),
            format!("graphql{quote}"),
            format!("{quote}query "),
            format!("{quote}mutation "),
        ];
        for pat in &gql_patterns {
            let mut pos = 0;
            while let Some(start) = js[pos..].find(pat.as_str()) {
                if pat.starts_with("gql") || pat.starts_with("graphql") {
                    // Tagged template — find matching backtick
                    if quote == '`' {
                        // Already handled by template literal finder
                        pos = start + 1;
                        continue;
                    }
                    let val_start = pos + start + pat.len();
                    if let Some(end) = js[val_start..].find(quote) {
                        let val = &js[val_start..val_start + end];
                        extract_graphql_endpoint(val, &mut results);
                    }
                } else if pat.contains("query ") || pat.contains("mutation ") {
                    // Direct string: "query { ... }" or 'mutation { ... }'
                    let val_start = pos + start + pat.len();
                    if let Some(end) = js[val_start..].find(quote) {
                        let val = &js[val_start..val_start + end];
                        // Extract the operation name if present
                        let cleaned = val.trim();
                        if cleaned.starts_with('{') || cleaned.is_empty() {
                            // Anonymous query — still useful
                            results.push(JsApiEndpoint {
                                method: Some("POST".into()),
                                path: "/graphql".into(),
                                source: "gql_anonymous".into(),
                                confidence: "medium",
                            });
                        } else {
                            let name = cleaned
                                .split(|c: char| c.is_whitespace() || c == '(' || c == '{')
                                .next()
                                .unwrap_or(cleaned);
                            results.push(JsApiEndpoint {
                                method: Some("POST".into()),
                                path: format!("/graphql?op={name}"),
                                source: "gql_operation".into(),
                                confidence: "high",
                            });
                        }
                    }
                }
                pos = start + 1;
            }
        }
    }
    results
}

fn extract_graphql_endpoint(val: &str, results: &mut Vec<JsApiEndpoint>) {
    // Find query/mutation names in a gql template
    for line in val.lines() {
        let trimmed = line.trim();
        if let Some(name) = trimmed
            .strip_prefix("query ")
            .or_else(|| trimmed.strip_prefix("mutation "))
        {
            let op_name = name
                .split(|c: char| c.is_whitespace() || c == '(' || c == '{')
                .next()
                .unwrap_or(name);
            if !op_name.is_empty() {
                results.push(JsApiEndpoint {
                    method: Some("POST".into()),
                    path: format!("/graphql?op={op_name}"),
                    source: "gql".into(),
                    confidence: "high",
                });
            }
        }
    }
}

/// Find import/require paths that look like API routes
fn find_import_paths(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    for quote in ['"', '\''] {
        let patterns = [
            format!("from {quote}"),
            format!("require({quote}"),
            format!("import({quote}"),
        ];
        for pat in &patterns {
            let mut pos = 0;
            while let Some(start) = js[pos..].find(pat.as_str()) {
                let val_start = pos + start + pat.len();
                if let Some(end) = js[val_start..].find(quote) {
                    let val = &js[val_start..val_start + end];
                    if (val.starts_with('/') || val.starts_with("./") || val.starts_with("../"))
                        && (val.contains("/api/") || val.contains("/rest/") || val.contains("/v1/"))
                    {
                        results.push(JsApiEndpoint {
                            method: None,
                            path: val.to_string(),
                            source: "import".into(),
                            confidence: "medium",
                        });
                    }
                }
                pos = val_start + 1;
            }
        }
    }
    results
}

/// Find string arrays with API paths: `['/api/users', '/api/posts']`
fn find_string_arrays(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    for quote in ['"', '\'', '`'] {
        // Match [..., '...', '...'] or [..."..."...]
        let mut pos = 0;
        while let Some(start) = js[pos..].find(&format!("[{quote}")) {
            let arr_start = pos + start;
            // Find the closing bracket
            if let Some(end) = js[arr_start..].find(']') {
                let content = &js[arr_start..arr_start + end + 1];
                // Extract all string literals
                let mut inner = content;
                while let Some(q) = inner.find(quote) {
                    let s = q + 1;
                    if let Some(e) = inner[s..].find(quote) {
                        let val = &inner[s..s + e];
                        if looks_like_api_path(val) {
                            results.push(JsApiEndpoint {
                                method: None,
                                path: val.to_string(),
                                source: "string_array".into(),
                                confidence: "medium",
                            });
                        }
                    }
                    inner = &inner[s + 1..];
                }
            }
            pos = arr_start + 1;
        }
    }
    results
}

/// Find Next.js / React Router link paths
fn find_router_links(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    for key in &["href", "to", "link", "navigate"] {
        for quote in ['"', '\'', '`'] {
            let patterns = [
                format!("{key}={quote}"),
                format!("{key}:{quote}"),
                format!("\"{key}\":{quote}"),
            ];
            for pat in &patterns {
                let mut pos = 0;
                while let Some(start) = js[pos..].find(pat.as_str()) {
                    let val_start = pos + start + pat.len();
                    if let Some(end) = js[val_start..].find(quote) {
                        let val = &js[val_start..val_start + end];
                        if (val.starts_with('/')
                            && val.len() > 1
                            && !val.contains('.')
                            && val.chars().filter(|&c| c == '/').count() >= 2)
                            || val.starts_with("/api/")
                        {
                            results.push(JsApiEndpoint {
                                method: None,
                                path: val.to_string(),
                                source: "router_link".into(),
                                confidence: "low",
                            });
                        }
                    }
                    pos = val_start + 1;
                }
            }
        }
    }
    results
}

/// Find env-var-based URL construction: `process.env.API_URL + '/users'`
fn find_env_urls(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    let env_patterns = [
        "process.env.API_URL",
        "process.env.NEXT_PUBLIC_API_URL",
        "import.meta.env.VITE_API_URL",
        "process.env.REACT_APP_API_URL",
        "process.env.GATSBY_API_URL",
        "process.env.NUXT_ENV_API_URL",
    ];
    for env_var in &env_patterns {
        if js.contains(env_var) {
            // Found an env-based API URL construction
            results.push(JsApiEndpoint {
                method: None,
                path: format!("<env:{env_var}>"),
                source: "env_url".into(),
                confidence: "low",
            });
            // Also find paths concatenated near it
            let mut pos = 0;
            while let Some(start) = js[pos..].find(env_var) {
                let context = &js[pos + start..(pos + start + 200).min(js.len())];
                for quote in ['"', '\'', '`'] {
                    let concat_pat = format!("+ {quote}");
                    if let Some(cpos) = context.find(&concat_pat) {
                        let after = &context[cpos + concat_pat.len()..];
                        if let Some(end) = after.find(quote) {
                            let val = &after[..end];
                            if looks_like_api_path(val) {
                                results.push(JsApiEndpoint {
                                    method: None,
                                    path: val.to_string(),
                                    source: "env_concat".into(),
                                    confidence: "medium",
                                });
                            }
                        }
                    }
                }
                pos = start + 1;
            }
        }
    }
    results
}

/// Find webpack/rollup minified path references
fn find_minified_paths(js: &str) -> Vec<JsApiEndpoint> {
    let mut results = Vec::new();
    // Look for patterns like "/api/..." in minified code where it's clearly an API endpoint
    // These often appear as string literals in route arrays or configs
    let re = match Regex::new(r#""((?:/[a-zA-Z0-9_\-/]+)?/api/[a-zA-Z0-9_\-/]*)""#) {
        Ok(r) => r,
        Err(_) => return results,
    };
    for cap in re.captures_iter(js) {
        if let Some(path) = cap.get(1) {
            let p = path.as_str();
            if p.contains("/api/") || p.contains("/rest/") {
                results.push(JsApiEndpoint {
                    method: None,
                    path: p.to_string(),
                    source: "minified".into(),
                    confidence: "medium",
                });
            }
        }
    }
    results
}

/// Check if a string looks like an API path
fn looks_like_api_path(s: &str) -> bool {
    if s.is_empty() || s.len() < 3 || s.len() > 256 {
        return false;
    }
    // Accept absolute URLs or paths starting with /
    if !s.starts_with('/') && !s.starts_with("http://") && !s.starts_with("https://") {
        return false;
    }
    // Must contain at least one /
    if s.chars().filter(|&c| c == '/').count() < 1 {
        return false;
    }
    // Ignore obvious non-API paths
    let skip_patterns = [
        ".html",
        ".css",
        ".js",
        ".png",
        ".jpg",
        ".svg",
        ".ico",
        ".woff",
        ".woff2",
        ".ttf",
        ".eot",
        ".json",
        ".xml",
        ".map",
        "/static/",
        "/assets/",
        "/images/",
        "/fonts/",
        "/cdn/",
        "/_next/",
        "/_nuxt/",
        "/__webpack_hmr",
    ];
    let lower = s.to_lowercase();
    for pat in &skip_patterns {
        if lower.contains(pat) {
            return false;
        }
    }
    // Must contain at least one API indicator or have a reasonable path depth
    let api_indicators = [
        "/api/", "/rest/", "/v1/", "/v2/", "/v3/", "/graphql", "/auth/", "/oauth/", "/token",
        "/login", "/users", "/admin", "/health", "/status", "/metrics", "/swagger", "/openapi",
        "/docs", "/backend", "/service", "/rpc", "/query",
    ];
    if let Some(first) = s.chars().next()
        && first == '/'
        && s.len() > 1
    {
        // Check for api indicators
        if api_indicators.iter().any(|i| lower.contains(i)) {
            return true;
        }
        // Or path depth >= 2
        return s.chars().filter(|&c| c == '/').count() >= 2;
    }
    // Absolute URLs — check api indicators via path component
    if s.starts_with("http://") || s.starts_with("https://") {
        return api_indicators.iter().any(|i| lower.contains(i))
            || lower.chars().filter(|&c| c == '/').count() >= 3;
    }
    false
}

/// Try to extract HTTP method from context near a path
fn extract_method_nearby(js: &str, path_pos: usize, _path: &str) -> Option<String> {
    let before = &js[path_pos.saturating_sub(80)..path_pos];
    for method in &[
        "POST", "post", "GET", "get", "PUT", "put", "PATCH", "patch", "DELETE", "delete",
    ] {
        // .method( or 'method': "POST" etc
        if before.contains(&format!("\"{method}\""))
            || before.contains(&format!("'{method}'"))
            || before.contains(&format!("{method},"))
        {
            return Some(method.to_uppercase());
        }
    }
    None
}

/// Convert JsApiEndpoint results to regular URL endpoints, ready for scanning.
pub fn to_scan_urls(endpoints: &[JsApiEndpoint], base_url: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    for ep in endpoints {
        if ep.path.starts_with("http://") || ep.path.starts_with("https://") {
            if let Ok(u) = Url::parse(&ep.path) {
                urls.push(u);
            }
        } else if ep.path.starts_with('/')
            && let Ok(u) = base_url.join(&ep.path)
        {
            urls.push(u);
        }
    }
    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_script_srcs_basic() {
        let html = r#"<html><script src="/bundle.js"></script><script src='https://cdn.example.com/app.js'></script></html>"#;
        let base = Url::parse("https://example.com").unwrap();
        let urls = extract_script_srcs(html, &base);
        assert_eq!(urls.len(), 2);
        assert!(urls.iter().any(|u| u.as_str().ends_with("/bundle.js")));
        assert!(urls.iter().any(|u| u.as_str().contains("cdn.example.com")));
    }

    #[test]
    fn test_find_route_config() {
        let js = r#"
            const routes = [
                { path: '/api/users', component: Users },
                { path: '/api/posts', component: Posts },
            ];
            const API = { url: '/api/v2/data' };
        "#;
        let results = find_route_config(js);
        assert!(results.iter().any(|r| r.path == "/api/users"));
        assert!(results.iter().any(|r| r.path == "/api/posts"));
        assert!(results.iter().any(|r| r.path == "/api/v2/data"));
    }

    #[test]
    fn test_find_fetch_calls() {
        let js = r#"
            fetch('/api/users').then(r => r.json());
            fetch("https://api.example.com/v2/data");
        "#;
        let results = find_fetch_calls(js);
        assert!(results.iter().any(|r| r.path.contains("/api/users")));
        assert!(results.iter().any(|r| r.path.contains("/v2/data")));
    }

    #[test]
    fn test_find_api_client_calls() {
        let js = r#"
            api.get('/api/users');
            client.post('/api/posts', data);
            axios.put('https://api.example.com/v2/items/1');
            http.delete('/api/sessions/1');
        "#;
        let results = find_api_client_calls(js);
        assert!(
            results
                .iter()
                .any(|r| r.path == "/api/users" && r.method.as_deref() == Some("GET"))
        );
        assert!(
            results
                .iter()
                .any(|r| r.path == "/api/posts" && r.method.as_deref() == Some("POST"))
        );
        assert!(
            results
                .iter()
                .any(|r| r.path.contains("/v2/items/1") && r.method.as_deref() == Some("PUT"))
        );
        assert!(
            results.iter().any(
                |r| r.path.contains("/api/sessions/1") && r.method.as_deref() == Some("DELETE")
            )
        );
    }

    #[test]
    fn test_find_template_literals() {
        let js = r#"
            const url = `/api/users/${userId}/posts`;
            fetch(`/api/v2/items/${id}`);
        "#;
        let results = find_template_literals(js);
        assert!(!results.is_empty());
    }

    #[test]
    fn test_find_graphql_ops() {
        let js = r#"
            const query = "query GetUsers { users { id name } }";
            const mutation = "mutation CreateUser($input: UserInput!) { createUser(input: $input) { id } }";
        "#;
        let results = find_graphql_ops(js);
        assert!(results.iter().any(|r| r.path.contains("GetUsers")));
        assert!(results.iter().any(|r| r.path.contains("CreateUser")));
    }

    #[test]
    fn test_looks_like_api_path() {
        assert!(looks_like_api_path("/api/users"));
        assert!(looks_like_api_path("/v1/items"));
        assert!(looks_like_api_path("/graphql"));
        assert!(!looks_like_api_path("/static/js/bundle.js"));
        assert!(!looks_like_api_path("/images/logo.png"));
        assert!(!looks_like_api_path("/"));
    }

    #[test]
    fn test_find_string_arrays() {
        let js = r#"const apis = ['/api/users', '/api/posts', '/api/comments']"#;
        let results = find_string_arrays(js);
        assert!(results.iter().any(|r| r.path == "/api/users"));
        assert!(results.iter().any(|r| r.path == "/api/posts"));
        assert!(results.iter().any(|r| r.path == "/api/comments"));
    }

    #[test]
    fn test_find_concat_patterns() {
        let js = r#"const url = "/rest/api/v2/" + resource + "/items"#;
        let results = find_concat_patterns(js);
        assert!(
            results
                .iter()
                .any(|r| r.path == "/rest/api/v2/" || r.path.contains("/rest/api/v2/"))
        );
    }

    #[test]
    fn test_extract_static_prefixes() {
        let result = extract_static_prefixes("/api/users/${id}/posts");
        assert!(result.iter().any(|p| p.contains("/api/users/")));
    }

    #[test]
    fn test_find_env_urls() {
        let js = r#"const apiUrl = process.env.NEXT_PUBLIC_API_URL + '/api/users';"#;
        let results = find_env_urls(js);
        assert!(results.iter().any(|r| r.path.contains("<env:")));
        // Also should find the concatenated path
        assert!(
            results
                .iter()
                .any(|r| r.path == "/api/users" || r.source == "env_concat")
        );
    }

    #[test]
    fn test_find_minified_paths() {
        let js = r#"createRoute({path:"/api/rest/v1/users"})"#;
        let results = find_minified_paths(js);
        assert!(
            results
                .iter()
                .any(|r| r.path.contains("/api/rest/v1/users"))
        );
    }
}
