//! Static HTML report generator.
//!
//! Creates `reports/<name>/` with:
//!   - `documentation/index.html` — API sitemap / endpoint listing
//!   - `documentation/endpoints/<id>.html` — one page per endpoint
//!   - `security-audit.html` — aggregated security check results

use std::fmt::Write;
use std::fs;
use std::path::PathBuf;

use crate::types::{Check, ResponseResult, Severity};

/// Generate all reports for a scan into `reports/<name>/`.
///
/// Returns the path to the reports base directory.
pub fn generate(results: &[ResponseResult], name: &str) -> Result<PathBuf, String> {
    let base = PathBuf::from("reports").join(name);
    let docs_dir = base.join("documentation");
    let endpoints_dir = docs_dir.join("endpoints");

    fs::create_dir_all(&endpoints_dir).map_err(|e| format!("create endpoints dir: {e}"))?;

    // ── API documentation site ──
    let docs_index = build_docs_index(results, name);
    fs::write(docs_dir.join("index.html"), &docs_index)
        .map_err(|e| format!("write docs index: {e}"))?;

    for (i, r) in results.iter().enumerate() {
        let page = build_endpoint_page(r, i, results.len(), name);
        let fname = format!("endpoint_{i:05}.html");
        fs::write(endpoints_dir.join(&fname), &page).map_err(|e| format!("write {fname}: {e}"))?;
    }

    // ── Security audit report ──
    let audit = build_security_audit(results, name);
    fs::write(base.join("security-audit.html"), &audit)
        .map_err(|e| format!("write security audit: {e}"))?;

    tracing::info!("Reports generated in {}", base.display());
    Ok(base)
}

// ── HTML shells ──

fn page_html(title: &str, body: &str, name: &str) -> String {
    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="UTF-8">
<meta name="viewport" content="width=device-width, initial-scale=1.0">
<title>{title} — {name}</title>
<style>
  * {{ box-sizing: border-box; margin: 0; padding: 0; }}
  body {{ font-family: -apple-system,BlinkMacSystemFont,"Segoe UI",Roboto,Oxygen,Ubuntu,sans-serif; background: #f5f5f7; color: #1d1d1f; line-height: 1.6; }}
  .container {{ max-width: 1100px; margin: 0 auto; padding: 2rem 1rem; }}
  h1 {{ font-size: 1.8rem; font-weight: 700; margin-bottom: 0.25rem; }}
  h2 {{ font-size: 1.3rem; font-weight: 600; margin: 1.5rem 0 0.75rem; }}
  .subtitle {{ color: #6e6e73; margin-bottom: 1.5rem; }}
  .summary-cards {{ display: flex; gap: 1rem; margin-bottom: 2rem; flex-wrap: wrap; }}
  .card {{ background: #fff; border-radius: 12px; padding: 1rem 1.25rem; flex: 1; min-width: 140px; box-shadow: 0 1px 3px rgba(0,0,0,.08); }}
  .card .num {{ font-size: 1.6rem; font-weight: 700; display: block; }}
  .card .label {{ font-size: .8rem; color: #6e6e73; text-transform: uppercase; letter-spacing: .04em; }}
  table {{ width: 100%; border-collapse: collapse; background: #fff; border-radius: 12px; overflow: hidden; box-shadow: 0 1px 3px rgba(0,0,0,.08); }}
  th, td {{ padding: .65rem .85rem; text-align: left; border-bottom: 1px solid #e8e8ed; font-size: .875rem; }}
  th {{ background: #f5f5f7; font-weight: 600; font-size: .75rem; text-transform: uppercase; letter-spacing: .04em; color: #6e6e73; }}
  tr:hover {{ background: #fafafa; }}
  .method {{ display: inline-block; padding: .15rem .5rem; border-radius: 4px; font-size: .75rem; font-weight: 600; color: #fff; }}
  .get {{ background: #34c759; }}
  .post {{ background: #007aff; }}
  .put {{ background: #ff9500; }}
  .patch {{ background: #ff9500; }}
  .delete {{ background: #ff3b30; }}
  .head {{ background: #5856d6; }}
  .options {{ background: #5856d6; }}
  .status-good {{ color: #34c759; font-weight: 600; }}
  .status-bad {{ color: #ff3b30; font-weight: 600; }}
  .status-warn {{ color: #ff9500; font-weight: 600; }}
  .check-pass {{ color: #34c759; }}
  .check-fail {{ color: #ff3b30; }}
  .check-warn {{ color: #ff9500; }}
  .tag {{ display: inline-block; background: #e8e8ed; border-radius: 4px; padding: 0 .4rem; font-size: .75rem; color: #515154; margin-right: .25rem; }}
  .code {{ font-family: "SF Mono","Fira Code",monospace; font-size: .8rem; background: #f0f0f2; padding: .1rem .35rem; border-radius: 3px; word-break: break-all; }}
  .endpoint-detail {{ background: #fff; border-radius: 12px; padding: 1.5rem; margin-bottom: 1.5rem; box-shadow: 0 1px 3px rgba(0,0,0,.08); }}
  .detail-label {{ font-size: .75rem; font-weight: 600; text-transform: uppercase; letter-spacing: .04em; color: #6e6e73; margin-top: 1rem; margin-bottom: .25rem; }}
  .checks-grid {{ display: grid; grid-template-columns: repeat(auto-fill, minmax(200px, 1fr)); gap: .75rem; margin-top: .5rem; }}
  .check-card {{ background: #f5f5f7; border-radius: 8px; padding: .75rem; }}
  .check-card.pass {{ border-left: 3px solid #34c759; }}
  .check-card.fail {{ border-left: 3px solid #ff3b30; }}
  .check-card.warn {{ border-left: 3px solid #ff9500; }}
  .check-card .cname {{ font-weight: 600; font-size: .85rem; }}
  .check-card .cmsg {{ font-size: .8rem; color: #515154; }}
  nav {{ background: #fff; border-bottom: 1px solid #e8e8ed; padding: .75rem 1rem; }}
  nav a {{ color: #007aff; text-decoration: none; margin-right: 1rem; font-size: .875rem; }}
  nav a:hover {{ text-decoration: underline; }}
  .pagination {{ display: flex; justify-content: space-between; margin-top: 1rem; }}
  .pagination a {{ color: #007aff; text-decoration: none; font-size: .875rem; }}
  ul {{ list-style: none; }}
  li {{ margin-bottom: .35rem; }}
  .no-results {{ text-align: center; padding: 3rem; color: #6e6e73; }}
  .severity-critical {{ color: #ff3b30; font-weight: 600; }}
  .severity-warn {{ color: #ff9500; font-weight: 600; }}
  .severity-info {{ color: #6e6e73; }}
  pre {{ background: #1d1d1f; color: #f5f5f7; border-radius: 8px; padding: 1rem; overflow-x: auto; font-size: .8rem; }}
</style>
</head>
<body>
<nav><a href="../index.html">← Sitemap</a> <a href="../../security-audit.html">Security Audit</a></nav>
<div class="container">
{body}
</div>
</body>
</html>"#,
        title = title,
        name = name,
    )
}

// ── Documentation index (sitemap) ──

fn build_docs_index(results: &[ResponseResult], name: &str) -> String {
    let total = results.len();
    let successful = results
        .iter()
        .filter(|r| r.status_code > 0 && r.status_code < 500)
        .count();
    let failed = results.iter().filter(|r| r.status_code >= 500).count();
    let errors = results.iter().filter(|r| r.status_code == 0).count();
    let check_fails: usize = results
        .iter()
        .map(|r| r.checks.iter().filter(|c| !c.passed).count())
        .sum();

    let mut body = String::new();
    let _ = write!(
        body,
        r#"<h1>API Documentation</h1><p class="subtitle">{name}</p>"#
    );

    // Summary cards
    let _ = write!(
        body,
        r#"<div class="summary-cards">
<div class="card"><span class="num">{total}</span><span class="label">Endpoints</span></div>
<div class="card"><span class="num">{successful}</span><span class="label">Successful</span></div>
<div class="card"><span class="num">{failed}</span><span class="label">Failed</span></div>
<div class="card"><span class="num">{errors}</span><span class="label">Errors</span></div>
<div class="card"><span class="num">{check_fails}</span><span class="label">Check Failures</span></div>
</div>"#
    );

    // Endpoint table
    let _ = writeln!(
        body,
        r#"<table><thead><tr><th>Method</th><th>URL</th><th>Status</th><th>Time</th><th>Checks</th></tr></thead><tbody>"#
    );
    for (i, r) in results.iter().enumerate() {
        let method_class = r.endpoint_method.to_lowercase();
        let status_class = if r.status_code >= 500 {
            "status-bad"
        } else if r.status_code == 0 {
            "status-warn"
        } else {
            "status-good"
        };
        let status_display = if r.status_code == 0 {
            "ERR".to_string()
        } else {
            r.status_code.to_string()
        };
        let checks_str = check_summary(&r.checks);
        let time_str = if r.response_time_ms > 0 {
            format!("{}ms", r.response_time_ms)
        } else {
            "-".into()
        };
        let _ = writeln!(
            body,
            r#"<tr><td><span class="method {method_class}">{}</span></td><td><a class="code" href="endpoints/endpoint_{i:05}.html">{}</a></td><td><span class="{status_class}">{status_display}</span></td><td>{time_str}</td><td>{checks_str}</td></tr>"#,
            r.endpoint_method, r.endpoint_url,
        );
    }
    let _ = writeln!(body, "</tbody></table>");

    page_html("API Documentation", &body, name)
}

fn check_summary(checks: &[Check]) -> String {
    let passed = checks.iter().filter(|c| c.passed).count();
    let failed = checks.iter().filter(|c| !c.passed).count();
    if checks.is_empty() {
        "—".into()
    } else {
        format!(
            r#"<span class="check-pass">{passed}P</span> <span class="check-fail">{failed}F</span>"#
        )
    }
}

// ── Individual endpoint page ──

fn build_endpoint_page(r: &ResponseResult, idx: usize, total: usize, name: &str) -> String {
    let status_class = if r.status_code >= 500 {
        "status-bad"
    } else if r.status_code == 0 {
        "status-warn"
    } else {
        "status-good"
    };
    let status_display = if r.status_code == 0 {
        "Error".to_string()
    } else {
        r.status_code.to_string()
    };
    let method_class = r.endpoint_method.to_lowercase();
    let severity_str = |s: &Severity| match s {
        Severity::Critical => "fail",
        Severity::Warn => "warn",
        Severity::Info => "pass",
    };

    let mut body = String::new();
    let _ = write!(
        body,
        r#"<h1><span class="method {method_class}">{}</span> {}</h1><p class="subtitle">{name}</p>"#,
        r.endpoint_method, r.endpoint_url,
    );

    // Pagination
    let prev_link = if idx > 0 {
        format!(r#"<a href="endpoint_{:05}.html">← Previous</a>"#, idx - 1)
    } else {
        String::new()
    };
    let next_link = if idx + 1 < total {
        format!(r#"<a href="endpoint_{:05}.html">Next →</a>"#, idx + 1)
    } else {
        String::new()
    };
    let _ = writeln!(
        body,
        r#"<div class="pagination">{prev_link}<span>{} / {}</span>{next_link}</div>"#,
        idx + 1,
        total
    );

    // Details
    let _ = write!(body, r#"<div class="endpoint-detail">"#);
    let _ = writeln!(
        body,
        r#"<div class="detail-label">Status</div><p><span class="{status_class}">{status_display}</span></p>"#
    );
    let _ = writeln!(
        body,
        r#"<div class="detail-label">Response Time</div><p>{time}ms</p>"#,
        time = r.response_time_ms
    );
    let _ = writeln!(
        body,
        r#"<div class="detail-label">Response Size</div><p>{size} bytes</p>"#,
        size = r.response_size
    );

    if let Some(expected) = r.expected_status {
        let _ = writeln!(
            body,
            r#"<div class="detail-label">Expected Status</div><p>{expected}</p>"#
        );
    }
    if let Some(ref err) = r.error {
        let _ = writeln!(
            body,
            r#"<div class="detail-label">Error</div><p class="status-bad">{err}</p>"#
        );
    }

    // Tags
    if !r.tags.is_empty() {
        let _ = write!(body, r#"<div class="detail-label">Tags</div><p>"#);
        for t in &r.tags {
            let _ = write!(body, r#"<span class="tag">{t}</span> "#);
        }
        let _ = writeln!(body, "</p>");
    }

    // Headers
    if !r.response_headers.is_empty() {
        let _ = write!(
            body,
            r#"<div class="detail-label">Response Headers</div><pre>"#
        );
        for (k, v) in &r.response_headers {
            let _ = writeln!(body, "{}: {}", html_escape(k), html_escape(v));
        }
        let _ = writeln!(body, "</pre>");
    }

    // Body preview (first 2KB)
    if !r.response_body.is_empty() {
        let preview_len = r.response_body.len().min(2048);
        let preview = String::from_utf8_lossy(&r.response_body[..preview_len]);
        let truncated = if r.response_body.len() > 2048 {
            "..."
        } else {
            ""
        };
        let _ = write!(
            body,
            r#"<div class="detail-label">Body Preview ({size} bytes)</div><pre>{preview}{truncated}</pre>"#,
            size = r.response_body.len()
        );
    }

    // Trackers
    if !r.trackers.is_empty() {
        let _ = write!(
            body,
            r#"<div class="detail-label">Trackers Detected</div><ul>"#
        );
        for t in &r.trackers {
            let domains_str = t.domains.join(", ");
            let _ = writeln!(
                body,
                r#"<li><span class="code">{}</span> — <span class="tag">{}</span> <span class="code">{}</span></li>"#,
                html_escape(t.name),
                t.category.as_str(),
                html_escape(&domains_str),
            );
        }
        let _ = writeln!(body, "</ul>");
    }

    let _ = writeln!(body, "</div>");

    // Checks
    if !r.checks.is_empty() {
        let _ = write!(body, r#"<h2>Checks</h2><div class="checks-grid">"#);
        for c in &r.checks {
            let sev = severity_str(&c.severity);
            let _ = writeln!(
                body,
                r#"<div class="check-card {sev}"><div class="cname">{name}</div><div class="cmsg">{msg}</div></div>"#,
                name = html_escape(&c.name),
                msg = html_escape(&c.message),
            );
        }
        let _ = writeln!(body, "</div>");
    }

    page_html(
        &format!("{} {}", r.endpoint_method, r.endpoint_url),
        &body,
        name,
    )
}

// ── Security audit report ──

fn build_security_audit(results: &[ResponseResult], name: &str) -> String {
    let total_checks: usize = results.iter().map(|r| r.checks.len()).sum();
    let passed: usize = results
        .iter()
        .map(|r| r.checks.iter().filter(|c| c.passed).count())
        .sum();
    let failed: usize = results
        .iter()
        .map(|r| {
            r.checks
                .iter()
                .filter(|c| !c.passed && matches!(c.severity, Severity::Critical))
                .count()
        })
        .sum();
    let warned: usize = results
        .iter()
        .map(|r| {
            r.checks
                .iter()
                .filter(|c| !c.passed && matches!(c.severity, Severity::Warn))
                .count()
        })
        .sum();
    let endpoints_with_fails = results
        .iter()
        .filter(|r| r.checks.iter().any(|c| !c.passed))
        .count();
    let endpoints_with_errors = results
        .iter()
        .filter(|r| r.status_code >= 500 || r.status_code == 0)
        .count();

    let mut body = String::new();
    let _ = write!(
        body,
        r#"<h1>Security Audit Report</h1><p class="subtitle">{name}</p>"#
    );

    // Summary
    let _ = write!(
        body,
        r#"<div class="summary-cards">
<div class="card"><span class="num">{total_checks}</span><span class="label">Total Checks</span></div>
<div class="card"><span class="num">{passed}</span><span class="label">Passed</span></div>
<div class="card"><span class="num">{failed}</span><span class="label">Failed (Critical)</span></div>
<div class="card"><span class="num">{warned}</span><span class="label">Warnings</span></div>
<div class="card"><span class="num">{endpoints_with_fails}</span><span class="label">Endpoints with Failures</span></div>
<div class="card"><span class="num">{endpoints_with_errors}</span><span class="label">Endpoints with Errors</span></div>
</div>"#
    );

    // All checks table
    let _ = writeln!(
        body,
        r#"<h2>All Check Results</h2><table><thead><tr><th>Endpoint</th><th>Check</th><th>Status</th><th>Message</th></tr></thead><tbody>"#
    );
    for (i, r) in results.iter().enumerate() {
        for c in &r.checks {
            let status_label = if c.passed {
                "PASS"
            } else {
                match c.severity {
                    Severity::Critical => "FAIL",
                    Severity::Warn => "WARN",
                    Severity::Info => "INFO",
                }
            };
            let status_class = if c.passed {
                "check-pass"
            } else {
                match c.severity {
                    Severity::Critical => "check-fail",
                    Severity::Warn => "check-warn",
                    Severity::Info => "check-pass",
                }
            };
            let _ = writeln!(
                body,
                r#"<tr><td><a class="code" href="documentation/endpoints/endpoint_{i:05}.html">{url}</a></td><td>{cname}</td><td class="{status_class}">{status_label}</td><td>{msg}</td></tr>"#,
                url = r.endpoint_url,
                cname = html_escape(&c.name),
                msg = html_escape(&c.message),
            );
        }
    }
    let _ = writeln!(body, "</tbody></table>");

    // Endpoints with errors
    if endpoints_with_errors > 0 {
        let _ = writeln!(body, r#"<h2>Endpoints with Errors</h2><ul>"#);
        for r in results
            .iter()
            .filter(|r| r.status_code >= 500 || r.status_code == 0)
        {
            let _ = writeln!(
                body,
                r#"<li><span class="code">{}</span> — {} ({})</li>"#,
                r.endpoint_url,
                r.status_code,
                r.error.as_deref().unwrap_or("no response"),
            );
        }
        let _ = writeln!(body, "</ul>");
    }

    page_html("Security Audit", &body, name)
}

// ── Helpers ──

fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
