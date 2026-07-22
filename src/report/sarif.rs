//! SARIF 2.1.0 export (Static Analysis Results Interchange Format).
//!
//! Produces a SARIF JSON document compatible with GitHub Code Scanning,
//! GitLab SAST, and other SARIF-consuming tools.
//!
//! Each failed security check becomes a SARIF result with ruleId, level,
//! message, and locations (pointing to the scanned endpoint).

use crate::types::ResponseResult;

/// Render scan results as a SARIF 2.1.0 JSON string.
pub fn to_sarif(results: &[ResponseResult], target_description: &str, version: &str) -> String {
    let mut sarif_results = Vec::new();

    for (i, r) in results.iter().enumerate() {
        for check in &r.checks {
            if check.passed {
                continue;
            }

            let (level, rule_id) = map_severity(&check.name, &check.severity);
            let physical_location = serde_json::json!({
                "artifactLocation": {
                    "uri": r.endpoint_url.replace('\\', "/"),
                    "uriBaseId": "%SRCROOT%"
                },
                "region": {
                    "startLine": 1,
                    "startColumn": 1
                }
            });

            sarif_results.push(serde_json::json!({
                "ruleId": rule_id,
                "ruleIndex": i,
                "level": level,
                "message": {
                    "text": format!("{}: {}", rule_id, check.message)
                },
                "locations": [{
                    "physicalLocation": physical_location
                }],
                "properties": {
                    "endpoint_method": r.endpoint_method,
                    "endpoint_url": r.endpoint_url,
                    "status_code": r.status_code,
                    "response_time_ms": r.response_time_ms,
                    "severity": check.severity.as_str()
                }
            }));
        }
    }

    let tool_version = if version.is_empty() { "0.1.0" } else { version };

    let sarif = serde_json::json!({
        "$schema": "https://json.schemastore.org/sarif-2.1.0.json",
        "version": "2.1.0",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "rapiscm",
                    "fullName": "rapiscm — API Security Scanner",
                    "version": tool_version,
                    "informationUri": "https://github.com/grave0x/rapiscm",
                    "rules": build_rules_metadata(),
                    "organization": "grave0x",
                    "shortDescription": {
                        "text": "API security scanner for OpenAPI specs and live endpoints"
                    }
                }
            },
            "invocations": [{
                "executionSuccessful": true,
                "toolExecutionNotifications": []
            }],
            "results": sarif_results,
            "columnKind": "utf16CodeUnits",
            "properties": {
                "target": target_description,
                "total_endpoints": results.len(),
                "total_checks": results.iter().map(|r| r.checks.len()).sum::<usize>(),
                "failed_checks": sarif_results.len()
            }
        }]
    });

    serde_json::to_string_pretty(&sarif).unwrap_or_else(|_| "{}".to_string())
}

/// Build the `rules` array for the SARIF tool driver — one entry per check type.
fn build_rules_metadata() -> serde_json::Value {
    let rules = [
        (
            "CSP",
            "Content-Security-Policy header missing — prevents XSS via allowed sources",
            "The HTTP response does not include a Content-Security-Policy header, making the page vulnerable to cross-site scripting attacks.",
            "warning",
        ),
        (
            "HSTS",
            "Strict-Transport-Security header missing — no HTTPS enforcement",
            "The HTTP response does not include a Strict-Transport-Security header, allowing connections over unencrypted HTTP.",
            "error",
        ),
        (
            "X-Content-Type-Options",
            "X-Content-Type-Options header missing — MIME-sniffing allowed",
            "The HTTP response does not include X-Content-Type-Options: nosniff, allowing the browser to perform MIME type sniffing.",
            "warning",
        ),
        (
            "X-Frame-Options",
            "X-Frame-Options header missing — page can be framed (clickjacking risk)",
            "The HTTP response does not include an X-Frame-Options header, allowing the page to be embedded in frames.",
            "note",
        ),
        (
            "Cache-Control",
            "Cache-Control header missing — responses may be cached",
            "The HTTP response does not include Cache-Control directives, potentially allowing sensitive responses to be cached.",
            "note",
        ),
        (
            "CORS",
            "CORS misconfiguration — cross-origin access may be too permissive",
            "The endpoint returns permissive CORS headers, potentially allowing unauthorized cross-origin access.",
            "warning",
        ),
        (
            "Auth",
            "Endpoint lacks authentication enforcement",
            "The endpoint returned a successful response without requiring authentication, exposing it to unauthorized access.",
            "error",
        ),
        (
            "Schema",
            "Response body failed schema validation",
            "The response body does not match the expected schema defined in the API specification.",
            "warning",
        ),
        (
            "Response Status",
            "Unexpected HTTP response status code",
            "The endpoint returned an unexpected HTTP status code that may indicate an error or misconfiguration.",
            "warning",
        ),
        (
            "Trackers",
            "Tracking cookies detected",
            "The response includes tracking cookies that may indicate analytics or advertising integration.",
            "note",
        ),
    ];

    let rules: Vec<serde_json::Value> = rules
        .iter()
        .map(|(id, short, full, default_level)| {
            serde_json::json!({
                "id": id,
                "name": id,
                "shortDescription": { "text": short },
                "fullDescription": { "text": full },
                "defaultConfiguration": {
                    "level": default_level
                },
                "helpUri": format!("https://github.com/grave0x/rapiscm/blob/main/docs/checks.md#{}", id.to_lowercase()),
                "properties": {
                    "category": "security"
                }
            })
        })
        .collect();

    serde_json::Value::Array(rules)
}

/// Map a check severity string to a SARIF level and rule ID.
fn map_severity(check_name: &str, severity: &crate::types::Severity) -> (&'static str, String) {
    let rule_id = check_name.replace(' ', "-");
    let level = match severity {
        crate::types::Severity::Critical => "error",
        crate::types::Severity::Warn => "warning",
        crate::types::Severity::Info => "note",
    };
    (level, rule_id)
}
