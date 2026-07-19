//! Resume — load a saved task and re-run only failed endpoints.
//!
//! Unlike `rebuild`, `resume` is used during an active scan to pick up where
//! a previous run left off. The scan runner writes checkpoint files, and
//! `resume` reads them to determine which endpoints still need scanning.

use std::fs;

use crate::error::Error;
use crate::task::TaskStorage;
use crate::types::{Endpoint, ResponseResult};

use super::TaskId;

/// Resume checkpoint state.
#[derive(Debug, Clone)]
pub struct ResumeState {
    #[expect(dead_code)]
    pub task_id: TaskId,
    pub skipped: usize,
    pub remaining: Vec<Endpoint>,
    pub existing_results: Vec<ResponseResult>,
}

/// Load checkpoint for a task. Returns `None` if no checkpoint exists.
pub fn load_checkpoint(storage: &TaskStorage, id: TaskId) -> Result<Option<ResumeState>, Error> {
    let ck_path = storage.task_dir(id).join("checkpoint.json");
    if !ck_path.exists() {
        return Ok(None);
    }
    let data = fs::read_to_string(&ck_path).map_err(|e| Error::Task(e.to_string()))?;
    #[derive(serde::Deserialize)]
    struct Checkpoint {
        completed_indices: Vec<usize>,
    }
    let ck: Checkpoint = serde_json::from_str(&data).map_err(|e| Error::Task(e.to_string()))?;

    let old_results = storage.load_results(id).map_err(Error::Task)?;
    let completed: std::collections::HashSet<usize> = ck.completed_indices.into_iter().collect();

    let mut remaining = Vec::new();
    let mut skipped = 0;
    for (i, r) in old_results.iter().enumerate() {
        if completed.contains(&i) {
            skipped += 1;
        } else {
            remaining.push(Endpoint {
                method: r.endpoint_method.parse().unwrap_or(reqwest::Method::GET),
                url: r.endpoint_url.parse().expect("invalid stored URL"),
                headers: r.response_headers.clone(),
                body: None,
                expected_status: r.expected_status,
                tags: r.tags.clone(),
            });
        }
    }

    if remaining.is_empty() {
        return Ok(None);
    }

    Ok(Some(ResumeState {
        task_id: id,
        skipped,
        remaining,
        existing_results: old_results,
    }))
}

/// Write a checkpoint file recording which endpoint indices have been scanned.
#[expect(dead_code)]
pub fn write_checkpoint(
    storage: &TaskStorage,
    id: TaskId,
    completed_indices: &[usize],
) -> Result<(), Error> {
    let ck_path = storage.task_dir(id).join("checkpoint.json");
    let data = serde_json::json!({ "completed_indices": completed_indices });
    let json = serde_json::to_string_pretty(&data).map_err(|e| Error::Task(e.to_string()))?;
    fs::write(&ck_path, json).map_err(|e| Error::Task(e.to_string()))?;
    Ok(())
}

/// Clear a checkpoint (after successful completion).
pub fn clear_checkpoint(storage: &TaskStorage, id: TaskId) {
    let ck_path = storage.task_dir(id).join("checkpoint.json");
    let _ = fs::remove_file(&ck_path);
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::error::Error;
    use crate::task::TaskStorage;
    use crate::types::ResponseResult;

    #[test]
    fn test_no_checkpoint() {
        let dir = std::env::temp_dir().join("rapiscm_test_nock");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let storage = TaskStorage::new(Some(dir));
        let state = load_checkpoint(&storage, 1).unwrap();
        assert!(state.is_none());
        let _ = fs::remove_dir_all(&storage.base_dir);
    }

    #[test]
    fn test_write_and_load() {
        let dir = std::env::temp_dir().join("rapiscm_test_wl");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let storage = TaskStorage::new(Some(dir.clone()));

        // Save a task with some results first.
        use crate::task::{GitInfo, ResultSummary, StorageInfo, TaskMeta};
        use crate::types::Severity;
        let meta = TaskMeta {
            task_id: 1,
            task_name: "test".into(),
            task_tags: vec![],
            cli_version: "0.1.0".into(),
            created_at: "now".into(),
            duration_seconds: 1.0,
            command: "spec".into(),
            target: "t".into(),
            config: serde_json::json!({}),
            git: None,
            endpoint_count: 2,
            result_summary: ResultSummary::default(),
            storage: StorageInfo {
                has_bodies: true,
                has_raw: false,
                results_size_bytes: 0,
            },
            exit_code: 0,
        };
        let results = vec![
            ResponseResult {
                endpoint_method: "GET".into(),
                endpoint_url: "http://a.com".into(),
                status_code: 200,
                response_time_ms: 10,
                response_size: 10,
                response_headers: vec![],
                response_body: vec![],
                expected_status: None,
                timestamp: None,
                checks: vec![],
                error: None,
                tags: vec![],
                trackers: vec![],
            },
            ResponseResult {
                endpoint_method: "POST".into(),
                endpoint_url: "http://b.com".into(),
                status_code: 0,
                response_time_ms: 0,
                response_size: 0,
                response_headers: vec![],
                response_body: vec![],
                expected_status: None,
                timestamp: None,
                checks: vec![],
                error: Some("timeout".into()),
                tags: vec![],
                trackers: vec![],
            },
        ];
        storage.save(&meta, &results, false, false).unwrap();

        write_checkpoint(&storage, 1, &[0]).unwrap();
        let state = load_checkpoint(&storage, 1).unwrap().unwrap();
        assert_eq!(state.skipped, 1);
        assert_eq!(state.remaining.len(), 1);

        clear_checkpoint(&storage, 1);
        assert!(!storage.task_dir(1).join("checkpoint.json").exists());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_clear_nonexistent_checkpoint() {
        let dir = std::env::temp_dir().join("rapiscm_test_clear_nonexist");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let storage = TaskStorage::new(Some(dir));
        // Should not panic
        clear_checkpoint(&storage, 999);
        let _ = fs::remove_dir_all(&storage.base_dir);
    }

    #[test]
    fn test_corrupt_checkpoint() {
        let dir = std::env::temp_dir().join("rapiscm_test_corrupt_ck");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let storage = TaskStorage::new(Some(dir.clone()));

        // Save a task + results, then write corrupt checkpoint
        use crate::task::{GitInfo, ResultSummary, StorageInfo, TaskMeta};
        let meta = TaskMeta {
            task_id: 1,
            task_name: "test".into(),
            task_tags: vec![],
            cli_version: "0.1.0".into(),
            created_at: "now".into(),
            duration_seconds: 1.0,
            command: "spec".into(),
            target: "t".into(),
            config: serde_json::json!({}),
            git: None,
            endpoint_count: 1,
            result_summary: ResultSummary::default(),
            storage: StorageInfo {
                has_bodies: true,
                has_raw: false,
                results_size_bytes: 0,
            },
            exit_code: 0,
        };
        let results = vec![ResponseResult {
            endpoint_method: "GET".into(),
            endpoint_url: "http://a.com".into(),
            status_code: 200,
            response_time_ms: 10,
            response_size: 10,
            response_headers: vec![],
            response_body: vec![],
            expected_status: None,
            timestamp: None,
            checks: vec![],
            error: None,
            tags: vec![],
            trackers: vec![],
        }];
        storage.save(&meta, &results, false, false).unwrap();
        fs::write(storage.task_dir(1).join("checkpoint.json"), "not json").unwrap();

        let err = load_checkpoint(&storage, 1).unwrap_err();
        assert!(matches!(err, Error::Task(_)));
        let _ = fs::remove_dir_all(&dir);
    }
}
