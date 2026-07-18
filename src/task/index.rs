//! Index file management — `index.json` in the tasks root directory.
//!
//! The index is the source of truth for `TaskId` assignment (monotonically
//! increasing) and the lightweight list displayed by `tasks list`.

use std::fs;
use std::path::Path;

use super::{IndexEntry, TaskId};

/// Load the index entries, sorted by `task_id` descending (newest first).
pub fn load_index(path: &Path) -> Vec<IndexEntry> {
    if !path.exists() {
        return Vec::new();
    }
    let data = match fs::read_to_string(path) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };
    let mut entries: Vec<IndexEntry> = match serde_json::from_str(&data) {
        Ok(e) => e,
        Err(_) => return Vec::new(),
    };
    entries.sort_by_key(|b| std::cmp::Reverse(b.task_id));
    entries
}

/// Save the index (overwrite).
pub fn save_index(path: &Path, entries: &[IndexEntry]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(entries).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Return the next unused `TaskId` (max + 1).
pub fn next_id(path: &Path) -> TaskId {
    let entries = load_index(path);
    entries.iter().map(|e| e.task_id).max().unwrap_or(0) + 1
}

/// Push a new entry into the index (prepend).
pub fn push_entry(path: &Path, entry: IndexEntry) -> Result<(), String> {
    let mut entries = load_index(path);
    entries.insert(0, entry);
    save_index(path, &entries)
}

/// Remove an entry from the index by task_id.
pub fn remove_entry(path: &Path, id: TaskId) -> Result<(), String> {
    let entries = load_index(path);
    let filtered: Vec<_> = entries.into_iter().filter(|e| e.task_id != id).collect();
    save_index(path, &filtered)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_next_id_empty() {
        let dir = std::env::temp_dir().join("rapiscm_test_idx_empty");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("index.json");
        assert_eq!(next_id(&path), 1);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_push_and_next_id() {
        let dir = std::env::temp_dir().join("rapiscm_test_idx_push");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("index.json");
        push_entry(
            &path,
            IndexEntry {
                task_id: 1,
                task_name: "t1".into(),
                command: "spec".into(),
                target: "test".into(),
                created_at: "now".into(),
                duration_seconds: 1.0,
                endpoint_count: 3,
                checks_failed: 0,
                exit_code: 0,
                task_tags: vec![],
                git_sha: None,
            },
        )
        .unwrap();
        assert_eq!(next_id(&path), 2);
        push_entry(
            &path,
            IndexEntry {
                task_id: 2,
                task_name: "t2".into(),
                command: "url".into(),
                target: "test2".into(),
                created_at: "now".into(),
                duration_seconds: 2.0,
                endpoint_count: 5,
                checks_failed: 1,
                exit_code: 0,
                task_tags: vec![],
                git_sha: None,
            },
        )
        .unwrap();
        let entries = load_index(&path);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].task_id, 2);
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_remove_entry() {
        let dir = std::env::temp_dir().join("rapiscm_test_idx_remove");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("index.json");
        for id in 1..=3 {
            push_entry(
                &path,
                IndexEntry {
                    task_id: id,
                    task_name: format!("t{id}"),
                    command: "spec".into(),
                    target: "t".into(),
                    created_at: "now".into(),
                    duration_seconds: 1.0,
                    endpoint_count: 1,
                    checks_failed: 0,
                    exit_code: 0,
                    task_tags: vec![],
                    git_sha: None,
                },
            )
            .unwrap();
        }
        remove_entry(&path, 2).unwrap();
        let entries = load_index(&path);
        assert_eq!(entries.len(), 2);
        assert!(!entries.iter().any(|e| e.task_id == 2));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_load_index_corrupt() {
        let dir = std::env::temp_dir().join("rapiscm_test_idx_corrupt");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("index.json");
        fs::write(&path, "not valid json").unwrap();
        let entries = load_index(&path);
        assert!(entries.is_empty());
        let _ = fs::remove_dir_all(&dir);
    }
}
