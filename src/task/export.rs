//! Export saved tasks to Markdown, SARIF, or HTML.

use std::fs;

use super::{TaskId, TaskMeta, TaskStorage};
use crate::types::{ResponseResult, Severity};

/// Export formats.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ExportFormat {
    Markdown,
    Sarif,
    Html,
}

/// Export a task to a file.
pub fn export(
    storage: &TaskStorage,
    id: TaskId,
    format: ExportFormat,
    output: &std::path::Path,
) -> Result<(), String> {
    let meta = storage.load_meta(id)?;
    let results = storage.load_results(id)?;

    let body = match format {
        ExportFormat::Markdown => to_markdown(&meta, &results),
        ExportFormat::Sarif => to_sarif(&meta, &results),
        ExportFormat::Html => to_html(&meta, &results),
    };

    fs::write(output, body).map_err(|e| format!("write export: {e}"))
}

/// Markdown report.
fn to_markdown(meta: &TaskMeta, results: &[ResponseResult]) -> String {
    let mut out = String::new();
    out.push_str(&format!("# Task {}: {}\n\n", meta.task_id, meta.task_name));
    out.push_str(&format!("- **Command:** {}\n", meta.command));
    out.push_str(&format!("- **Target:** {}\n", meta.target));
    out.push_str(&format!("- **Date:** {}\n", meta.created_at));
    out.push_str(&format!("- **Duration:** {:.1}s\n", meta.duration_seconds));
    out.push_str(&format!(
        "- **Endpoints:** {} total, {} ok, {} failed, {} errors\n",
        meta.result_summary.total,
        meta.result_summary.successful,
        meta.result_summary.failed,
        meta.result_summary.errors
    ));
    if let Some(g) = &meta.git {
        out.push_str(&format!("- **Git:** {} ({})\n", g.sha, g.branch));
    }
    out.push_str("\n## Results\n\n");
    for (i, r) in results.iter().enumerate() {
        let status_icon = if r.status_code == 0 {
            "❌"
        } else if r.status_code >= 500 {
            "⚠️"
        } else {
            "✅"
        };
        out.push_str(&format!(
            "{} **{}** `{} {}` — {}ms\n",
            status_icon, i, r.endpoint_method, r.endpoint_url, r.response_time_ms
        ));
        if !r.checks.is_empty() {
            for c in &r.checks {
                let icon = if c.passed { "✅" } else { "❌" };
                out.push_str(&format!("  {} {}: {}\n", icon, c.name, c.message));
            }
        }
        if let Some(e) = &r.error {
            out.push_str(&format!("  💥 Error: {e}\n"));
        }
        out.push('\n');
    }
    out
}

/// SARIF 2.1 output (minimal, one result per failing check).
fn to_sarif(meta: &TaskMeta, results: &[ResponseResult]) -> String {
    let mut runs = serde_json::json!({
        "version": "2.1.0",
        "$schema": "https://raw.githubusercontent.com/oasis-tcs/sarif-spec/master/Schemata/sarif-schema-2.1.0.json",
        "runs": [{
            "tool": {
                "driver": {
                    "name": "rapiscm",
                    "version": meta.cli_version,
                    "informationUri": "https://github.com/rapiscm"
                }
            },
            "results": [],
            "invocations": [{
                "executionSuccessful": meta.exit_code == 0,
                "startTimeUtc": meta.created_at,
            }]
        }]
    });

    let results_arr = runs["runs"][0]["results"].as_array_mut().unwrap();
    for r in results {
        for c in &r.checks {
            if !c.passed {
                let level = match c.severity {
                    Severity::Critical => "error",
                    Severity::Warn => "warning",
                    Severity::Info => "note",
                };
                results_arr.push(serde_json::json!({
                    "message": {
                        "text": format!("{}: {}", c.name, c.message)
                    },
                    "level": level,
                    "locations": [{
                        "physicalLocation": {
                            "artifactLocation": {
                                "uri": r.endpoint_url
                            }
                        }
                    }],
                    "properties": {
                        "method": r.endpoint_method,
                        "statusCode": r.status_code,
                    }
                }));
            }
        }
    }

    serde_json::to_string_pretty(&runs).unwrap_or_else(|_| "{}".into())
}

/// Standalone HTML report (inline CSS, no external deps).
fn to_html(meta: &TaskMeta, results: &[ResponseResult]) -> String {
    let mut rows = String::new();
    for r in results {
        let cls = if r.status_code == 0 {
            "error"
        } else if r.status_code >= 500 {
            "fail"
        } else {
            "pass"
        };
        let checks_html: String = r
            .checks
            .iter()
            .map(|c| {
                let icon = if c.passed { "✅" } else { "❌" };
                let sev = match c.severity {
                    Severity::Critical => "critical",
                    Severity::Warn => "warn",
                    Severity::Info => "info",
                };
                format!(
                    "<span class=\"check {} {}\">{icon} {}</span>",
                    if c.passed { "pass" } else { "fail" },
                    sev,
                    c.message
                )
            })
            .collect::<Vec<_>>()
            .join(" ");
        let err_html = r
            .error
            .as_ref()
            .map(|e| format!("<div class=\"error-msg\">{e}</div>"))
            .unwrap_or_default();
        rows.push_str(&format!(
            "<tr class=\"{}\"><td>{}</td><td>{}</td><td>{}</td><td>{}ms</td><td>{}{}</td></tr>",
            cls,
            r.endpoint_method,
            r.endpoint_url,
            r.status_code,
            r.response_time_ms,
            checks_html,
            err_html
        ));
    }

    format!(
        r#"<!DOCTYPE html>
<html lang="en">
<head><meta charset="UTF-8"><title>Task {}: {}</title>
<style>
  body {{ font-family: system-ui, sans-serif; margin: 2rem; background: #fafafa; }}
  h1 {{ color: #333; }}
  table {{ border-collapse: collapse; width: 100%; }}
  th, td {{ padding: 0.5rem; text-align: left; border-bottom: 1px solid #ddd; }}
  tr.pass {{ background: #f0fff4; }} tr.fail {{ background: #fff5f5; }} tr.error {{ background: #fffbea; }}
  .check {{ font-size: 0.85em; margin-right: 0.5em; padding: 2px 6px; border-radius: 3px; }}
  .check.fail {{ background: #fee; }} .check.pass {{ background: #efe; }}
  .critical {{ border-left: 3px solid #e53e3e; }}
  .warn {{ border-left: 3px solid #dd6b20; }}
  .info {{ border-left: 3px solid #3182ce; }}
  .error-msg {{ color: #e53e3e; font-size: 0.9em; }}
</style></head><body>
<h1>Task {}: {}</h1>
<p>Command: {} &mdash; Target: {} &mdash; {} endpoints in {:.1}s</p>
<table><thead><tr><th>Method</th><th>URL</th><th>Status</th><th>Time</th><th>Checks</th></tr></thead><tbody>{}</tbody></table>
</body></html>"#,
        meta.task_id,
        meta.task_name,
        meta.task_id,
        meta.task_name,
        meta.command,
        meta.target,
        meta.result_summary.total,
        meta.duration_seconds,
        rows
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Check, Severity};

    fn dummy_result() -> ResponseResult {
        ResponseResult {
            endpoint_method: "GET".into(),
            endpoint_url: "http://example.com".into(),
            status_code: 200,
            response_time_ms: 50,
            response_size: 100,
            response_headers: vec![],
            response_body: vec![],
            expected_status: Some(200),
            timestamp: None,
            checks: vec![Check {
                name: "csp".into(),
                passed: false,
                severity: Severity::Critical,
                message: "Missing CSP header".into(),
            }],
            error: None,
            tags: vec![],
            trackers: vec![],
        }
    }

    fn dummy_meta() -> TaskMeta {
        TaskMeta {
            task_id: 1,
            task_name: "test".into(),
            task_tags: vec![],
            cli_version: "0.1.0".into(),
            created_at: "now".into(),
            duration_seconds: 1.0,
            command: "spec".into(),
            target: "test.yaml".into(),
            config: serde_json::json!({}),
            git: None,
            endpoint_count: 1,
            result_summary: super::super::ResultSummary {
                total: 1,
                successful: 1,
                failed: 0,
                errors: 0,
                checks_passed: 0,
                checks_failed: 1,
                checks_warn: 0,
                p50_ms: 50,
                p90_ms: 50,
                p99_ms: 50,
            },
            storage: super::super::StorageInfo {
                has_bodies: true,
                has_raw: false,
                results_size_bytes: 100,
            },
            exit_code: 0,
        }
    }

    #[test]
    fn test_markdown_export() {
        let md = to_markdown(&dummy_meta(), &[dummy_result()]);
        assert!(md.contains("Task 1"));
        assert!(md.contains("Missing CSP header"));
    }

    #[test]
    fn test_sarif_export() {
        let sarif = to_sarif(&dummy_meta(), &[dummy_result()]);
        assert!(sarif.contains("rapiscm"));
        assert!(sarif.contains("Missing CSP header"));
    }

    #[test]
    fn test_html_export() {
        let html = to_html(&dummy_meta(), &[dummy_result()]);
        assert!(html.contains("<!DOCTYPE html"));
        assert!(html.contains("Missing CSP header"));
    }

    #[test]
    fn test_export_writes_file() {
        use std::fs;
        let dir = std::env::temp_dir().join("rapiscm_test_export_file");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let storage = super::super::TaskStorage::new(Some(dir.clone()));
        let meta = dummy_meta();
        let results = vec![dummy_result()];
        storage.save(&meta, &results, false, false).unwrap();
        let out_path = dir.join("report.md");
        export(&storage, 1, ExportFormat::Markdown, &out_path).unwrap();
        let content = fs::read_to_string(&out_path).unwrap();
        assert!(content.contains("Task 1"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_sarif_no_results() {
        let meta = dummy_meta();
        let sarif = to_sarif(&meta, &[]);
        assert!(sarif.contains("rapiscm"));
        assert!(sarif.contains("\"results\""));
    }

    #[test]
    fn test_sarif_all_pass() {
        let mut meta = dummy_meta();
        meta.result_summary.checks_failed = 0;
        let mut r = dummy_result();
        r.checks = vec![Check {
            name: "csp".into(),
            passed: true,
            severity: Severity::Info,
            message: "CSP present".into(),
        }];
        let sarif = to_sarif(&meta, &[r]);
        // All checks passed, so results array should be empty
        assert!(sarif.contains("\"results\": []") || sarif.contains("\"results\":  []"));
    }
}
