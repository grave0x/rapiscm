//! Structured API documentation output (llm-api style).
//!
//! Produces a markdown document organized by domain, path, status, and purpose,
//! matching the format used by the `llm-api` project.

use crate::types::ResponseResult;
use std::collections::BTreeMap;

/// Format scan results as structured API documentation.
pub fn format_doc(results: &[ResponseResult]) -> String {
    let mut out = String::new();

    // Title
    if let Some(first) = results.first()
        && let Ok(url) = reqwest::Url::parse(&first.endpoint_url)
        && let Some(host) = url.host_str()
    {
        out.push_str(&format!("# API Endpoints — {}\n\n", host));
        out.push_str(&format!("Base URL: `{}://{}`\n\n", url.scheme(), host));
    }

    out.push_str("## Endpoints\n\n");
    out.push_str("| Method | Path | Status | Time | Tags |\n");
    out.push_str("|--------|------|--------|------|------|\n");

    // Group by path for cleaner output
    let mut grouped: BTreeMap<String, Vec<&ResponseResult>> = BTreeMap::new();
    for r in results {
        let path = extract_path(&r.endpoint_url);
        grouped.entry(path).or_default().push(r);
    }

    for entries in grouped.values() {
        for r in entries {
            let method = &r.endpoint_method;
            let path = extract_path(&r.endpoint_url);
            let status = if r.status_code > 0 {
                r.status_code.to_string()
            } else {
                "ERR".into()
            };
            let time = if r.response_time_ms < 1000 {
                format!("{}ms", r.response_time_ms)
            } else {
                format!("{:.1}s", r.response_time_ms as f64 / 1000.0)
            };
            let tags = if r.tags.is_empty() {
                "-".into()
            } else {
                r.tags.join(", ")
            };
            out.push_str(&format!(
                "| `{}` | `{}` | {} | {} | {} |\n",
                method, path, status, time, tags
            ));
        }
    }

    // Security checks section
    out.push_str("\n## Security Checks\n\n");
    let total = results.len();
    let passed = results.iter().flat_map(|r| &r.checks).filter(|c| c.passed).count();
    let failed = results.iter().flat_map(|r| &r.checks).filter(|c| !c.passed).count();
    out.push_str(&format!("- **Total endpoints:** {}\n", total));
    out.push_str(&format!("- **Checks passed:** {}\n", passed));
    out.push_str(&format!("- **Checks failed:** {}\n", failed));
    out.push_str("\n### Failed Checks\n\n");
    for r in results {
        let failed_checks: Vec<_> = r.checks.iter().filter(|c| !c.passed).collect();
        if !failed_checks.is_empty() {
            let path = extract_path(&r.endpoint_url);
            out.push_str(&format!("**{} {}**\n\n", r.endpoint_method, path));
            for c in &failed_checks {
                out.push_str(&format!("- ✗ **{}**: {}\n", c.name, c.message));
            }
            out.push('\n');
        }
    }

    // Tracker/analytics section
    let has_trackers = results.iter().any(|r| !r.trackers.is_empty());
    if has_trackers {
        out.push_str("\n## Trackers & Analytics\n\n");
        for r in results {
            if !r.trackers.is_empty() {
                let path = extract_path(&r.endpoint_url);
                out.push_str(&format!("**{}**\n\n", path));
                for t in &r.trackers {
                    out.push_str(&format!("- `{}` ({})\n", t.name, t.category.as_str()));
                }
                out.push('\n');
            }
        }
    }

    // Infrastructure notes
    out.push_str("\n## Infrastructure\n\n");
    out.push_str("- **Auth**: ");
    if results.iter().any(|r| r.status_code == 401 || r.status_code == 403) {
        out.push_str("Authentication required (some endpoints return 401/403)\n");
    } else {
        out.push_str("No authentication observed\n");
    }

    let methods: Vec<_> = {
        let mut m: Vec<_> = results.iter().map(|r| r.endpoint_method.as_str()).collect();
        m.sort();
        m.dedup();
        m
    };
    out.push_str(&format!("- **Methods**: {}\n", methods.join(", ")));

    let statuses: Vec<_> = {
        let mut s: Vec<_> = results.iter().map(|r| r.status_code).collect();
        s.sort();
        s.dedup();
        s
    };
    out.push_str(&format!(
        "- **Observed status codes**: {}\n",
        statuses.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(", ")
    ));

    out
}

fn extract_path(url: &str) -> String {
    if let Ok(parsed) = reqwest::Url::parse(url) {
        let path = parsed.path().to_string();
        if let Some(query) = parsed.query() {
            format!("{}?{}", path, query)
        } else {
            path
        }
    } else {
        url.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn result(method: &str, url: &str, status: u16) -> ResponseResult {
        ResponseResult {
            endpoint_method: method.into(),
            endpoint_url: url.into(),
            status_code: status,
            response_time_ms: 100,
            response_size: 64,
            response_headers: vec![],
            response_body: vec![],
            expected_status: None,
            timestamp: None,
            checks: vec![],
            error: None,
            tags: vec![],
            trackers: vec![],
        }
    }

    #[test]
    fn test_format_doc_empty() {
        let out = format_doc(&[]);
        assert!(out.is_empty() || out.contains("## Endpoints"));
    }

    #[test]
    fn test_format_doc_contains_method() {
        let out = format_doc(&[result("GET", "https://api.example.com/users", 200)]);
        assert!(out.contains("GET"));
        assert!(out.contains("/users"));
    }

    #[test]
    fn test_format_doc_contains_security_checks() {
        let out = format_doc(&[result("POST", "https://api.example.com/login", 401)]);
        assert!(out.contains("Security Checks"));
        assert!(out.contains("Authentication required"));
    }

    #[test]
    fn test_extract_path() {
        assert_eq!(extract_path("https://example.com/api/users"), "/api/users");
        assert_eq!(extract_path("https://example.com/api?page=1"), "/api?page=1");
    }

    #[test]
    fn test_extract_path_invalid() {
        assert_eq!(extract_path("not a url"), "not a url");
    }
}
