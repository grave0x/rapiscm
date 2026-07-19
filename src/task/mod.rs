//! Phase 12 — task system: store, index, queue, rebuild, diff, export, resume.

pub mod diff;
pub mod export;
pub mod index;
pub mod queue;
pub mod rebuild;
pub mod resume;
pub mod store;

#[allow(unused_imports)]
pub use diff::{DiffKind, TaskDiff, diff_tasks};
#[allow(unused_imports)]
pub use export::ExportFormat;
pub use queue::*;

use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Monotonic task identifier (assigned by the index).
pub type TaskId = u64;

/// Git context at scan time, if available.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitInfo {
    pub sha: String,
    pub branch: String,
    pub message: String,
    pub dirty: bool,
}

/// Aggregated statistics from a scan result set.
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ResultSummary {
    pub total: usize,
    pub successful: usize,
    pub failed: usize,
    pub errors: usize,
    pub checks_passed: usize,
    pub checks_failed: usize,
    pub checks_warn: usize,
    pub p50_ms: u64,
    pub p90_ms: u64,
    pub p99_ms: u64,
}

/// Storage information for a task (bodies / raw-data flags, size).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StorageInfo {
    pub has_bodies: bool,
    pub has_raw: bool,
    pub results_size_bytes: u64,
}

/// Full metadata written to every task directory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMeta {
    pub task_id: TaskId,
    pub task_name: String,
    pub task_tags: Vec<String>,
    pub cli_version: String,
    pub created_at: String,
    pub duration_seconds: f64,
    pub command: String,
    pub target: String,
    pub config: serde_json::Value,
    pub git: Option<GitInfo>,
    pub endpoint_count: usize,
    pub result_summary: ResultSummary,
    pub storage: StorageInfo,
    pub exit_code: i32,
}

/// Lightweight entry stored in the index.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IndexEntry {
    pub task_id: TaskId,
    pub task_name: String,
    pub command: String,
    pub target: String,
    pub created_at: String,
    pub duration_seconds: f64,
    pub endpoint_count: usize,
    pub checks_failed: usize,
    pub exit_code: i32,
    pub task_tags: Vec<String>,
    pub git_sha: Option<String>,
}

/// Task storage handle.
#[derive(Debug, Clone)]
pub struct TaskStorage {
    pub base_dir: PathBuf,
}

impl TaskStorage {
    /// Locate the task directory, using `RAPISCM_TASKS_DIR` env var
    /// or `~/.local/share/rapiscm/tasks/`.
    pub fn default_dir() -> PathBuf {
        if let Ok(d) = std::env::var("RAPISCM_TASKS_DIR") {
            return PathBuf::from(d);
        }
        let home = std::env::var("HOME")
            .or_else(|_| std::env::var("USERPROFILE"))
            .unwrap_or_else(|_| ".".into());
        PathBuf::from(home).join(".local/share/rapiscm/tasks")
    }

    pub fn new(dir: Option<PathBuf>) -> Self {
        TaskStorage {
            base_dir: dir.unwrap_or(Self::default_dir()),
        }
    }

    /// Path to a single task directory.
    pub fn task_dir(&self, id: TaskId) -> PathBuf {
        self.base_dir.join(id.to_string())
    }

    /// Path to the index file.
    pub fn index_path(&self) -> PathBuf {
        self.base_dir.join("index.json")
    }

    /// Path to the queue file.
    pub fn queue_path(&self) -> PathBuf {
        self.base_dir.join("queued.json")
    }
}

/// Compute p50/p90/p99 from sorted durations.
pub fn compute_percentiles(mut times: Vec<u64>) -> (u64, u64, u64) {
    if times.is_empty() {
        return (0, 0, 0);
    }
    times.sort_unstable();
    let len = times.len();
    let idx =
        |pct: usize| -> usize { (len.saturating_sub(1) * pct / 100).min(len.saturating_sub(1)) };
    (times[idx(50)], times[idx(90)], times[idx(99)])
}

/// Compute a result summary from a slice of scan results.
pub fn summarize(results: &[crate::types::ResponseResult]) -> ResultSummary {
    let total = results.len();
    let successful = results
        .iter()
        .filter(|r| r.status_code > 0 && r.status_code < 500)
        .count();
    let failed = results.iter().filter(|r| r.status_code >= 500).count();
    let errors = results.iter().filter(|r| r.status_code == 0).count();
    let mut checks_passed = 0;
    let mut checks_failed = 0;
    let mut checks_warn = 0;
    for r in results {
        for c in &r.checks {
            match c.severity {
                crate::types::Severity::Critical => checks_failed += 1,
                crate::types::Severity::Warn => checks_warn += 1,
                crate::types::Severity::Info => checks_passed += 1,
            }
        }
    }
    let times: Vec<u64> = results.iter().map(|r| r.response_time_ms).collect();
    let (p50_ms, p90_ms, p99_ms) = compute_percentiles(times);
    ResultSummary {
        total,
        successful,
        failed,
        errors,
        checks_passed,
        checks_failed,
        checks_warn,
        p50_ms,
        p90_ms,
        p99_ms,
    }
}

/// Format a duration in seconds to human-friendly string.
#[cfg_attr(not(test), expect(dead_code))]
pub fn fmt_duration(secs: f64) -> String {
    if secs < 60.0 {
        format!("{secs:.1}s")
    } else if secs < 3600.0 {
        format!("{:.0}m {:.0}s", secs / 60.0, secs % 60.0)
    } else {
        format!(
            "{:.0}h {:.0}m {:.0}s",
            secs / 3600.0,
            (secs % 3600.0) / 60.0,
            secs % 60.0
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::ResponseResult;

    #[test]
    fn test_compute_percentiles_empty() {
        assert_eq!(compute_percentiles(vec![]), (0, 0, 0));
    }

    #[test]
    fn test_compute_percentiles_single() {
        assert_eq!(compute_percentiles(vec![42]), (42, 42, 42));
    }

    #[test]
    fn test_compute_percentiles_basic() {
        let times: Vec<u64> = (1..=100).collect();
        let (p50, p90, p99) = compute_percentiles(times);
        assert_eq!(p50, 50);
        assert_eq!(p90, 90);
        assert_eq!(p99, 99);
    }

    #[test]
    fn test_fmt_duration() {
        assert_eq!(fmt_duration(5.0), "5.0s");
        assert!(fmt_duration(90.0).contains("m"));
        assert!(fmt_duration(3661.0).contains("h"));
    }

    #[test]
    fn test_summarize_empty() {
        let s = summarize(&[]);
        assert_eq!(s.total, 0);
    }

    #[test]
    fn test_summarize_mixed() {
        use crate::types::{Check, Severity};
        let results = vec![
            ResponseResult {
                endpoint_method: "GET".into(),
                endpoint_url: "/ok".into(),
                status_code: 200,
                response_time_ms: 50,
                response_size: 100,
                response_headers: vec![],
                response_body: vec![],
                expected_status: Some(200),
                timestamp: None,
                checks: vec![],
                error: None,
                tags: vec![],
                trackers: vec![],
            },
            ResponseResult {
                endpoint_method: "POST".into(),
                endpoint_url: "/fail".into(),
                status_code: 500,
                response_time_ms: 200,
                response_size: 64,
                response_headers: vec![],
                response_body: vec![],
                expected_status: Some(200),
                timestamp: None,
                checks: vec![
                    Check {
                        name: "csp".into(),
                        passed: false,
                        severity: Severity::Critical,
                        message: "missing".into(),
                    },
                    Check {
                        name: "hsts".into(),
                        passed: true,
                        severity: Severity::Info,
                        message: "present".into(),
                    },
                ],
                error: Some("timeout".into()),
                tags: vec![],
                trackers: vec![],
            },
            ResponseResult {
                endpoint_method: "GET".into(),
                endpoint_url: "/err".into(),
                status_code: 0,
                response_time_ms: 0,
                response_size: 0,
                response_headers: vec![],
                response_body: vec![],
                expected_status: None,
                timestamp: None,
                checks: vec![Check {
                    name: "status".into(),
                    passed: false,
                    severity: Severity::Warn,
                    message: "no response".into(),
                }],
                error: Some("connection refused".into()),
                tags: vec![],
                trackers: vec![],
            },
        ];
        let s = summarize(&results);
        assert_eq!(s.total, 3);
        assert_eq!(s.successful, 1);
        assert_eq!(s.failed, 1);
        assert_eq!(s.errors, 1);
        assert_eq!(s.checks_passed, 1);
        assert_eq!(s.checks_failed, 1);
        assert_eq!(s.checks_warn, 1);
        assert_eq!(s.p50_ms, 50);
        assert_eq!(s.p90_ms, 50);
        assert_eq!(s.p99_ms, 50);
    }

    #[test]
    fn test_fmt_duration_edges() {
        assert_eq!(fmt_duration(0.0), "0.0s");
        assert_eq!(fmt_duration(59.9), "59.9s");
        assert_eq!(fmt_duration(60.0), "1m 0s");
        // 3599s → 59.983min, format rounds to 60m 59s due to {:.0}
        assert_eq!(fmt_duration(3540.0), "59m 0s");
        assert_eq!(fmt_duration(3600.0), "1h 0m 0s");
        assert_eq!(fmt_duration(3661.0), "1h 1m 1s");
        assert_eq!(fmt_duration(86400.0), "24h 0m 0s");
    }
}
