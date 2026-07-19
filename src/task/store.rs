//! Task persistence — save / load / list / delete / prune.

use std::fs;
use std::path::Path;

use crate::types::ResponseResult;

use super::index;
use super::{IndexEntry, TaskId, TaskMeta, TaskStorage};

impl TaskStorage {
    /// Build the full `TaskMeta` and save everything:
    ///   - `task.json` — metadata
    ///   - `results.json` — results without bodies (unless `no_bodies`)
    ///   - `results-nb.json` — always without bodies
    ///   - `raw/` directory (one file per endpoint, if `raw`)
    ///   - prepend entry to `index.json`
    pub fn save(
        &self,
        meta: &TaskMeta,
        results: &[ResponseResult],
        no_bodies: bool,
        raw: bool,
    ) -> Result<TaskId, String> {
        // Ensure base dir exists.
        fs::create_dir_all(&self.base_dir).map_err(|e| e.to_string())?;

        // Create task directory (safe to call multiple times).
        let tdir = self.task_dir(meta.task_id);
        fs::create_dir_all(&tdir).map_err(|e| e.to_string())?;

        // Write task.json (metadata).
        let meta_json = serde_json::to_string_pretty(meta).map_err(|e| e.to_string())?;
        fs::write(tdir.join("task.json"), &meta_json).map_err(|e| e.to_string())?;

        // Write results-nb.json (no bodies — always save this for quick loads).
        let no_body_results: Vec<ResponseResult> = results
            .iter()
            .map(|r| ResponseResult {
                response_body: Vec::new(),
                trackers: Vec::new(),
                ..r.clone()
            })
            .collect();
        let nb_json = serde_json::to_string_pretty(&no_body_results).map_err(|e| e.to_string())?;
        fs::write(tdir.join("results-nb.json"), &nb_json).map_err(|e| e.to_string())?;

        // Write results.json (with bodies unless no_bodies).
        if no_bodies {
            fs::write(tdir.join("results.json"), &nb_json).map_err(|e| e.to_string())?;
        } else {
            let body_results: Vec<ResponseResult> = results
                .iter()
                .map(|r| ResponseResult {
                    trackers: Vec::new(),
                    ..r.clone()
                })
                .collect();
            let b_json = serde_json::to_string_pretty(&body_results).map_err(|e| e.to_string())?;
            fs::write(tdir.join("results.json"), &b_json).map_err(|e| e.to_string())?;
        }

        // Write raw/ directory (one file per endpoint).
        if raw {
            let raw_dir = tdir.join("raw");
            fs::create_dir_all(&raw_dir).map_err(|e| e.to_string())?;
            for (i, r) in results.iter().enumerate() {
                let method = r.endpoint_method.to_lowercase();
                let safe_url = r
                    .endpoint_url
                    .replace(['/', '?', '&'], "_")
                    .replace("://", "_");
                let fname = format!("{i:05}_{method}_{safe_url}.json");
                let entry = serde_json::json!({
                    "method": r.endpoint_method,
                    "url": r.endpoint_url,
                    "status_code": r.status_code,
                    "response_time_ms": r.response_time_ms,
                    "response_size": r.response_size,
                    "response_headers": r.response_headers,
                    "response_body_base64": base64::encode(&r.response_body),
                    "checks": r.checks,
                    "error": r.error,
                    "tags": r.tags,
                });
                fs::write(
                    raw_dir.join(&fname),
                    serde_json::to_string_pretty(&entry).map_err(|e| e.to_string())?,
                )
                .map_err(|e| e.to_string())?;
            }
        }

        // Update index.
        let entry = IndexEntry {
            task_id: meta.task_id,
            task_name: meta.task_name.clone(),
            command: meta.command.clone(),
            target: meta.target.clone(),
            created_at: meta.created_at.clone(),
            duration_seconds: meta.duration_seconds,
            endpoint_count: meta.endpoint_count,
            checks_failed: meta.result_summary.checks_failed,
            exit_code: meta.exit_code,
            task_tags: meta.task_tags.clone(),
            git_sha: meta.git.as_ref().map(|g| g.sha.clone()),
        };
        index::push_entry(&self.index_path(), entry)?;

        Ok(meta.task_id)
    }

    /// Load task metadata from a task directory.
    pub fn load_meta(&self, id: TaskId) -> Result<TaskMeta, String> {
        let path = self.task_dir(id).join("task.json");
        let data = fs::read_to_string(&path).map_err(|e| format!("task {id} not found: {e}"))?;
        serde_json::from_str(&data).map_err(|e| format!("task {id} meta parse: {e}"))
    }

    /// Load results. Tries `results.json` first, falls back to `results-nb.json`.
    pub fn load_results(&self, id: TaskId) -> Result<Vec<ResponseResult>, String> {
        let dir = self.task_dir(id);
        let path = dir.join("results.json");
        let nb_path = dir.join("results-nb.json");
        if path.exists() {
            let data = fs::read_to_string(&path).map_err(|e| e.to_string())?;
            serde_json::from_str(&data).map_err(|e| format!("results parse: {e}"))
        } else if nb_path.exists() {
            let data = fs::read_to_string(&nb_path).map_err(|e| e.to_string())?;
            serde_json::from_str(&data).map_err(|e| format!("results parse: {e}"))
        } else {
            Err(format!("no results file for task {id}"))
        }
    }

    /// List all tasks (delegates to index).
    pub fn list(&self) -> Vec<IndexEntry> {
        index::load_index(&self.index_path())
    }

    /// Get a single index entry by id.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn get_entry(&self, id: TaskId) -> Option<IndexEntry> {
        let entries = index::load_index(&self.index_path());
        entries.into_iter().find(|e| e.task_id == id)
    }

    /// Delete a task directory and its index entry.
    pub fn delete(&self, id: TaskId) -> Result<(), String> {
        let dir = self.task_dir(id);
        if dir.exists() {
            fs::remove_dir_all(&dir).map_err(|e| format!("delete task {id}: {e}"))?;
        }
        index::remove_entry(&self.index_path(), id)
    }

    /// Prune old tasks, keeping at most `keep` newest entries.
    pub fn prune(&self, keep: usize) -> Result<usize, String> {
        let entries = index::load_index(&self.index_path());
        if entries.len() <= keep {
            return Ok(0);
        }
        let mut removed = 0;
        for entry in &entries[keep..] {
            let dir = self.task_dir(entry.task_id);
            if dir.exists() {
                fs::remove_dir_all(&dir).map_err(|e| e.to_string())?;
            }
            index::remove_entry(&self.index_path(), entry.task_id)?;
            removed += 1;
        }
        Ok(removed)
    }

    /// Total disk usage of the tasks directory.
    #[cfg_attr(not(test), expect(dead_code))]
    pub fn disk_usage(&self) -> u64 {
        fn dir_size(path: &Path) -> u64 {
            let Ok(rd) = fs::read_dir(path) else {
                return 0;
            };
            let mut total = 0u64;
            for entry in rd.flatten() {
                let meta = match entry.metadata() {
                    Ok(m) => m,
                    Err(_) => continue,
                };
                if meta.is_dir() {
                    total += dir_size(&entry.path());
                } else {
                    total += meta.len();
                }
            }
            total
        }
        dir_size(&self.base_dir)
    }
}

/// Minimal base64 module (no external dep).
mod base64 {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    pub fn encode(input: &[u8]) -> String {
        let mut result = String::with_capacity(input.len().div_ceil(3) * 4);
        for chunk in input.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
            let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
            let triple = (b0 << 16) | (b1 << 8) | b2;
            result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
            result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
            if chunk.len() > 1 {
                result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
            if chunk.len() > 2 {
                result.push(CHARS[(triple & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::{GitInfo, ResultSummary, StorageInfo};
    use crate::types::{Check, Severity};

    fn sample_meta(id: TaskId) -> TaskMeta {
        TaskMeta {
            task_id: id,
            task_name: "test-scan".into(),
            task_tags: vec![],
            cli_version: "0.1.0".into(),
            created_at: "2025-01-01T00:00:00Z".into(),
            duration_seconds: 1.5,
            command: "spec".into(),
            target: "test.yaml".into(),
            config: serde_json::json!({}),
            git: Some(GitInfo {
                sha: "abc123".into(),
                branch: "main".into(),
                message: "test".into(),
                dirty: false,
            }),
            endpoint_count: 2,
            result_summary: ResultSummary {
                total: 2,
                successful: 1,
                failed: 1,
                errors: 0,
                checks_passed: 2,
                checks_failed: 1,
                checks_warn: 0,
                p50_ms: 100,
                p90_ms: 200,
                p99_ms: 300,
            },
            storage: StorageInfo {
                has_bodies: true,
                has_raw: false,
                results_size_bytes: 100,
            },
            exit_code: 0,
        }
    }

    fn sample_results() -> Vec<ResponseResult> {
        vec![
            ResponseResult {
                endpoint_method: "GET".into(),
                endpoint_url: "http://example.com/api".into(),
                status_code: 200,
                response_time_ms: 100,
                response_size: 512,
                response_headers: vec![("content-type".into(), "application/json".into())],
                response_body: b"{\"ok\":true}".to_vec(),
                expected_status: Some(200),
                timestamp: Some("now".into()),
                checks: vec![Check {
                    name: "csp".into(),
                    passed: true,
                    severity: Severity::Info,
                    message: "CSP present".into(),
                }],
                error: None,
                tags: vec!["api".into()],
                trackers: vec![],
            },
            ResponseResult {
                endpoint_method: "POST".into(),
                endpoint_url: "http://example.com/api".into(),
                status_code: 500,
                response_time_ms: 200,
                response_size: 64,
                response_headers: vec![],
                response_body: vec![],
                expected_status: Some(200),
                timestamp: Some("now".into()),
                checks: vec![],
                error: Some("timeout".into()),
                tags: vec![],
                trackers: vec![],
            },
        ]
    }

    #[test]
    fn test_save_and_load() {
        let dir = std::env::temp_dir().join("rapiscm_test_store");
        let _ = fs::remove_dir_all(&dir);
        let storage = TaskStorage::new(Some(dir.clone()));
        let meta = sample_meta(1);
        let results = sample_results();
        let id = storage.save(&meta, &results, false, false).unwrap();
        assert_eq!(id, 1);

        let loaded_meta = storage.load_meta(id).unwrap();
        assert_eq!(loaded_meta.task_name, "test-scan");
        assert_eq!(loaded_meta.task_id, 1);

        let loaded_results = storage.load_results(id).unwrap();
        assert_eq!(loaded_results.len(), 2);
        assert_eq!(loaded_results[0].endpoint_method, "GET");
        // trackers should be empty on load
        assert!(loaded_results[0].trackers.is_empty());

        let entries = storage.list();
        assert!(!entries.is_empty());

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_prune() {
        let dir = std::env::temp_dir().join("rapiscm_test_prune");
        let _ = fs::remove_dir_all(&dir);
        let storage = TaskStorage::new(Some(dir.clone()));
        for i in 1..=5 {
            let meta = sample_meta(i);
            let results = sample_results();
            storage.save(&meta, &results, false, false).unwrap();
        }
        assert_eq!(storage.list().len(), 5);
        let removed = storage.prune(2).unwrap();
        assert_eq!(removed, 3);
        assert_eq!(storage.list().len(), 2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_delete() {
        let dir = std::env::temp_dir().join("rapiscm_test_delete");
        let _ = fs::remove_dir_all(&dir);
        let storage = TaskStorage::new(Some(dir.clone()));
        let meta = sample_meta(1);
        let results = sample_results();
        storage.save(&meta, &results, false, false).unwrap();
        assert!(storage.task_dir(1).exists());
        storage.delete(1).unwrap();
        assert!(!storage.task_dir(1).exists());
        assert!(storage.list().is_empty());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_get_entry() {
        let dir = std::env::temp_dir().join("rapiscm_test_get_entry");
        let _ = fs::remove_dir_all(&dir);
        let storage = TaskStorage::new(Some(dir.clone()));
        let meta = sample_meta(42);
        let results = sample_results();
        storage.save(&meta, &results, false, false).unwrap();
        let entry = storage.get_entry(42).unwrap();
        assert_eq!(entry.task_id, 42);
        assert_eq!(entry.task_name, "test-scan");
        // non-existent returns None
        assert!(storage.get_entry(999).is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_meta_nonexistent() {
        let dir = std::env::temp_dir().join("rapiscm_test_meta_nonexist");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let storage = TaskStorage::new(Some(dir.clone()));
        let err = storage.load_meta(999).unwrap_err();
        assert!(err.contains("999") || err.contains("not found"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_disk_usage() {
        let dir = std::env::temp_dir().join("rapiscm_test_disk_usage");
        let _ = fs::remove_dir_all(&dir);
        let storage = TaskStorage::new(Some(dir.clone()));
        let meta = sample_meta(1);
        let results = sample_results();
        storage.save(&meta, &results, false, false).unwrap();
        let bytes = storage.disk_usage();
        assert!(bytes > 0);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_prune_keep_all() {
        let dir = std::env::temp_dir().join("rapiscm_test_prune_keep");
        let _ = fs::remove_dir_all(&dir);
        let storage = TaskStorage::new(Some(dir.clone()));
        for i in 1..=3 {
            storage
                .save(&sample_meta(i), &sample_results(), false, false)
                .unwrap();
        }
        let removed = storage.prune(5).unwrap();
        assert_eq!(removed, 0);
        assert_eq!(storage.list().len(), 3);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_results_fallback() {
        let dir = std::env::temp_dir().join("rapiscm_test_fallback");
        let _ = fs::remove_dir_all(&dir);
        let storage = TaskStorage::new(Some(dir.clone()));
        let meta = sample_meta(1);
        let results = sample_results();
        storage.save(&meta, &results, false, false).unwrap();
        // Delete results.json but leave results-nb.json
        let results_path = storage.task_dir(1).join("results.json");
        assert!(results_path.exists());
        fs::remove_file(&results_path).unwrap();
        let loaded = storage.load_results(1).unwrap();
        assert_eq!(loaded.len(), 2);
        let _ = fs::remove_dir_all(&dir);
    }
}
