//! Diff two saved tasks — compare result sets and classify changes.

use crate::types::{Check, Severity as Sev};
use serde::{Deserialize, Serialize};

use super::TaskStorage;

/// Per-endpoint diff classification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiffKind {
    Identical,
    StatusChanged {
        old: u16,
        new: u16,
    },
    TimeChanged {
        old_ms: u64,
        new_ms: u64,
    },
    NewCheck {
        check: Check,
    },
    RemovedCheck {
        check: Check,
    },
    CheckStatusChanged {
        check_name: String,
        old_passed: bool,
        new_passed: bool,
    },
    BodySizeChanged {
        old: usize,
        new: usize,
    },
    ErrorStateChanged {
        old: Option<String>,
        new: Option<String>,
    },
}

/// Result of comparing two tasks.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskDiff {
    pub old_id: u64,
    pub new_id: u64,
    pub old_name: String,
    pub new_name: String,
    pub total_old: usize,
    pub total_new: usize,
    pub changed_count: usize,
    pub added_count: usize,
    pub removed_count: usize,
    pub changes: Vec<EndpointDiff>,
}

/// Diff for a single endpoint.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EndpointDiff {
    pub method: String,
    pub url: String,
    pub kind: DiffKind,
}

/// Compare two saved tasks by their IDs.
pub fn diff_tasks(storage: &TaskStorage, old_id: u64, new_id: u64) -> Result<TaskDiff, String> {
    let old_meta = storage.load_meta(old_id)?;
    let new_meta = storage.load_meta(new_id)?;
    let old_results = storage.load_results(old_id)?;
    let new_results = storage.load_results(new_id)?;

    let mut changes = Vec::new();
    let mut changed_count = 0;
    let mut added_count = 0;
    let mut removed_count = 0;

    // Compare common endpoints by URL+method.
    let max_len = old_results.len().max(new_results.len());
    for i in 0..max_len {
        match (old_results.get(i), new_results.get(i)) {
            (Some(old), Some(new)) => {
                let kind = compare_endpoints(old, new);
                if kind != DiffKind::Identical {
                    changed_count += 1;
                    changes.push(EndpointDiff {
                        method: new.endpoint_method.clone(),
                        url: new.endpoint_url.clone(),
                        kind,
                    });
                }
            }
            (Some(old), None) => {
                removed_count += 1;
                changes.push(EndpointDiff {
                    method: old.endpoint_method.clone(),
                    url: old.endpoint_url.clone(),
                    kind: DiffKind::ErrorStateChanged {
                        old: old.error.clone(),
                        new: None,
                    },
                });
            }
            (None, Some(new)) => {
                added_count += 1;
                changes.push(EndpointDiff {
                    method: new.endpoint_method.clone(),
                    url: new.endpoint_url.clone(),
                    kind: DiffKind::ErrorStateChanged {
                        old: None,
                        new: new.error.clone(),
                    },
                });
            }
            (None, None) => {}
        }
    }

    Ok(TaskDiff {
        old_id,
        new_id,
        old_name: old_meta.task_name,
        new_name: new_meta.task_name,
        total_old: old_results.len(),
        total_new: new_results.len(),
        changed_count,
        added_count,
        removed_count,
        changes,
    })
}

fn compare_endpoints(
    old: &crate::types::ResponseResult,
    new: &crate::types::ResponseResult,
) -> DiffKind {
    if old.status_code != new.status_code {
        return DiffKind::StatusChanged {
            old: old.status_code,
            new: new.status_code,
        };
    }
    if old.error != new.error {
        return DiffKind::ErrorStateChanged {
            old: old.error.clone(),
            new: new.error.clone(),
        };
    }
    if old.response_time_ms != new.response_time_ms {
        return DiffKind::TimeChanged {
            old_ms: old.response_time_ms,
            new_ms: new.response_time_ms,
        };
    }
    if old.response_size != new.response_size {
        return DiffKind::BodySizeChanged {
            old: old.response_size,
            new: new.response_size,
        };
    }
    // Compare checks.
    for oc in &old.checks {
        let found = new.checks.iter().find(|nc| nc.name == oc.name);
        match found {
            None => return DiffKind::RemovedCheck { check: oc.clone() },
            Some(nc) if nc.passed != oc.passed => {
                return DiffKind::CheckStatusChanged {
                    check_name: oc.name.clone(),
                    old_passed: oc.passed,
                    new_passed: nc.passed,
                };
            }
            _ => {}
        }
    }
    for nc in &new.checks {
        if !old.checks.iter().any(|oc| oc.name == nc.name) {
            return DiffKind::NewCheck { check: nc.clone() };
        }
    }
    DiffKind::Identical
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{ResponseResult, Severity};

    fn mk_result(url: &str, status: u16, time: u64) -> ResponseResult {
        ResponseResult {
            endpoint_method: "GET".into(),
            endpoint_url: url.into(),
            status_code: status,
            response_time_ms: time,
            response_size: 100,
            response_headers: vec![],
            response_body: vec![],
            expected_status: Some(200),
            timestamp: None,
            checks: vec![],
            error: None,
            tags: vec![],
            trackers: vec![],
        }
    }

    #[test]
    fn test_compare_status_changed() {
        let old = mk_result("/api", 200, 50);
        let new = mk_result("/api", 500, 50);
        assert_eq!(
            compare_endpoints(&old, &new),
            DiffKind::StatusChanged { old: 200, new: 500 }
        );
    }

    #[test]
    fn test_compare_identical() {
        let old = mk_result("/api", 200, 50);
        let new = mk_result("/api", 200, 50);
        assert_eq!(compare_endpoints(&old, &new), DiffKind::Identical);
    }

    #[test]
    fn test_compare_time_changed() {
        let old = mk_result("/api", 200, 50);
        let new = mk_result("/api", 200, 150);
        assert_eq!(
            compare_endpoints(&old, &new),
            DiffKind::TimeChanged {
                old_ms: 50,
                new_ms: 150
            }
        );
    }

    #[test]
    fn test_new_check() {
        let old = mk_result("/api", 200, 50);
        let mut new = mk_result("/api", 200, 50);
        new.checks.push(Check {
            name: "csp".into(),
            passed: true,
            severity: Severity::Info,
            message: "CSP present".into(),
        });
        assert!(matches!(
            compare_endpoints(&old, &new),
            DiffKind::NewCheck { .. }
        ));
    }

    #[test]
    fn test_removed_check() {
        let mut old = mk_result("/api", 200, 50);
        old.checks.push(Check {
            name: "csp".into(),
            passed: true,
            severity: Severity::Info,
            message: "CSP present".into(),
        });
        let new = mk_result("/api", 200, 50);
        assert!(matches!(
            compare_endpoints(&old, &new),
            DiffKind::RemovedCheck { .. }
        ));
    }

    #[test]
    fn test_check_status_changed() {
        let mut old = mk_result("/api", 200, 50);
        old.checks.push(Check {
            name: "csp".into(),
            passed: true,
            severity: Severity::Info,
            message: "CSP present".into(),
        });
        let mut new = mk_result("/api", 200, 50);
        new.checks.push(Check {
            name: "csp".into(),
            passed: false,
            severity: Severity::Info,
            message: "CSP missing".into(),
        });
        assert_eq!(
            compare_endpoints(&old, &new),
            DiffKind::CheckStatusChanged {
                check_name: "csp".into(),
                old_passed: true,
                new_passed: false
            }
        );
    }

    #[test]
    fn test_body_size_changed() {
        let old = mk_result("/api", 200, 50);
        let mut new = mk_result("/api", 200, 50);
        new.response_size = 500;
        assert_eq!(
            compare_endpoints(&old, &new),
            DiffKind::BodySizeChanged { old: 100, new: 500 }
        );
    }

    #[test]
    fn test_error_state_changed() {
        let old = mk_result("/api", 200, 50);
        let mut new = mk_result("/api", 200, 50);
        new.error = Some("timeout".into());
        assert!(matches!(
            compare_endpoints(&old, &new),
            DiffKind::ErrorStateChanged { .. }
        ));
    }
}
