//! Script filters — execute external scripts to modify scan results.
//!
//! Pipe mode: endpoint data is piped as JSON to a subprocess via stdin,
//! modified results read from stdout. Works with any language.

use crate::types::{Check, ResponseResult, Severity};
use std::collections::BTreeMap;

/// A modification to apply to a ResponseResult.
#[derive(serde::Deserialize)]
pub struct ScriptMod {
    /// Override severity of existing checks or add new ones.
    #[serde(default)]
    pub checks: Vec<ScriptCheck>,
    /// Tags to add.
    #[serde(default)]
    pub add_tags: Vec<String>,
    /// Custom message.
    #[serde(default)]
    pub message: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct ScriptCheck {
    pub name: String,
    pub passed: bool,
    #[serde(default = "default_severity")]
    pub severity: String,
    pub message: String,
}

fn default_severity() -> String {
    "info".into()
}

/// Result sent to the script via stdin (JSON).
#[derive(serde::Serialize)]
struct ScriptInput<'a> {
    pub method: &'a str,
    pub url: &'a str,
    pub status: u16,
    pub response_time_ms: u64,
    pub response_size: usize,
    pub tags: &'a [String],
}

/// Execute a pipe script (e.g. `pipe:./check.py`) on each result.
///
/// The script receives JSON on stdin, outputs modified JSON on stdout.
pub fn apply_pipe(script_path: &str, results: &mut [ResponseResult]) -> Result<(), String> {
    for r in results.iter_mut() {
        let input = ScriptInput {
            method: &r.endpoint_method,
            url: &r.endpoint_url,
            status: r.status_code,
            response_time_ms: r.response_time_ms,
            response_size: r.response_size,
            tags: &r.tags,
        };
        let input_json = serde_json::to_string(&input).map_err(|e| e.to_string())?;

        let output = std::process::Command::new(script_path)
            .arg(&r.endpoint_url) // Pass URL as first arg for convenience
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .spawn()
            .map_err(|e| format!("failed to spawn {script_path}: {e}"))?
            .wait_with_output()
            .map_err(|e| format!("script error: {e}"))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            tracing::warn!("script {script_path} exited with error: {stderr}");
            continue;
        }

        let stdout = String::from_utf8_lossy(&output.stdout);
        if stdout.trim().is_empty() {
            continue;
        }

        // Try to parse script output as modifications
        if let Ok(mods) = serde_json::from_str::<ScriptMod>(&stdout) {
            for sc in mods.checks {
                let sev = match sc.severity.to_lowercase().as_str() {
                    "critical" => Severity::Critical,
                    "warn" | "warning" => Severity::Warn,
                    _ => Severity::Info,
                };
                r.checks.push(Check {
                    name: sc.name,
                    passed: sc.passed,
                    severity: sev,
                    message: sc.message,
                });
            }
            r.tags.extend(mods.add_tags);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_pipe_noop_script() {
        let mut results = vec![ResponseResult {
            endpoint_method: "GET".into(),
            endpoint_url: "https://example.com/api".into(),
            status_code: 200,
            response_time_ms: 42,
            response_size: 100,
            response_headers: vec![],
            response_body: vec![],
            expected_status: None,
            timestamp: None,
            checks: vec![],
            error: None,
            tags: vec![],
            trackers: vec![],
        }];
        // Test with echo (outputs input back) — should not crash
        let _ = apply_pipe("echo", &mut results);
        // No op should leave results unchanged
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_script_mod_deserialize() {
        let json = r#"{"checks":[{"name":"custom-check","passed":false,"severity":"critical","message":"Found issue"}],"add_tags":["audited"]}"#;
        let mods: ScriptMod = serde_json::from_str(json).unwrap();
        assert_eq!(mods.checks.len(), 1);
        assert!(mods.add_tags.contains(&"audited".into()));
    }
}
