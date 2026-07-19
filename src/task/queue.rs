//! Queue file management — `queued.json` in the tasks root.
//!
//! Supports batch / async scan queuing and crash recovery.
#![expect(dead_code)]

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use super::TaskId;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum QueueItemStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueueItem {
    pub queue_id: String,
    pub command: String,
    pub target: String,
    pub config_snapshot: serde_json::Value,
    pub status: QueueItemStatus,
    pub created_at: String,
    pub started_at: Option<String>,
    pub completed_at: Option<String>,
    pub task_id: Option<TaskId>,
    pub retries: u32,
    pub error: Option<String>,
}

/// Load the queue from file.
pub fn load(path: &Path) -> Vec<QueueItem> {
    if !path.exists() {
        return Vec::new();
    }
    let data = match fs::read_to_string(path) {
        Ok(d) => d,
        Err(_) => return Vec::new(),
    };
    serde_json::from_str(&data).unwrap_or_default()
}

/// Save the queue (overwrite).
pub fn save(path: &Path, items: &[QueueItem]) -> Result<(), String> {
    let json = serde_json::to_string_pretty(items).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())?;
    Ok(())
}

/// Add an item to the queue.
pub fn enqueue(path: &Path, item: QueueItem) -> Result<(), String> {
    let mut items = load(path);
    items.push(item);
    save(path, &items)
}

/// Find and mark the first `Pending` item as `Running`. Returns it.
pub fn dequeue(path: &Path) -> Option<QueueItem> {
    let mut items = load(path);
    let idx = items
        .iter()
        .position(|i| i.status == QueueItemStatus::Pending)?;
    items[idx].status = QueueItemStatus::Running;
    items[idx].started_at = Some(crate::util::now_iso());
    let item = items[idx].clone();
    save(path, &items).ok()?;
    Some(item)
}

/// Mark an item as completed or failed.
pub fn complete(
    path: &Path,
    queue_id: &str,
    task_id: Option<TaskId>,
    error: Option<String>,
) -> Result<(), String> {
    let mut items = load(path);
    if let Some(item) = items.iter_mut().find(|i| i.queue_id == queue_id) {
        item.status = if error.is_some() {
            QueueItemStatus::Failed
        } else {
            QueueItemStatus::Completed
        };
        item.completed_at = Some(crate::util::now_iso());
        item.task_id = task_id;
        item.error = error;
    }
    save(path, &items)
}

/// On startup: mark all `Running` items back to `Pending` (crash recovery).
pub fn recover_crashed(path: &Path) -> Result<usize, String> {
    let mut items = load(path);
    let mut recovered = 0;
    for item in &mut items {
        if item.status == QueueItemStatus::Running {
            item.status = QueueItemStatus::Pending;
            item.started_at = None;
            recovered += 1;
        }
    }
    save(path, &items)?;
    Ok(recovered)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_item(id: &str) -> QueueItem {
        QueueItem {
            queue_id: id.into(),
            command: "spec".into(),
            target: "test.yaml".into(),
            config_snapshot: serde_json::json!({}),
            status: QueueItemStatus::Pending,
            created_at: "now".into(),
            started_at: None,
            completed_at: None,
            task_id: None,
            retries: 0,
            error: None,
        }
    }

    #[test]
    fn test_enqueue_dequeue() {
        let dir = std::env::temp_dir().join("rapiscm_test_queue");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("queued.json");

        enqueue(&path, sample_item("a")).unwrap();
        enqueue(&path, sample_item("b")).unwrap();

        let item = dequeue(&path).unwrap();
        assert_eq!(item.queue_id, "a");
        assert_eq!(item.status, QueueItemStatus::Running);

        complete(&path, "a", Some(1), None).unwrap();
        let items = load(&path);
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].status, QueueItemStatus::Completed);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_crash_recovery() {
        let dir = std::env::temp_dir().join("rapiscm_test_crash");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("queued.json");

        let mut item = sample_item("x");
        item.status = QueueItemStatus::Running;
        save(&path, &[item]).unwrap();

        let n = recover_crashed(&path).unwrap();
        assert_eq!(n, 1);

        let items = load(&path);
        assert_eq!(items[0].status, QueueItemStatus::Pending);

        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_dequeue_empty() {
        let dir = std::env::temp_dir().join("rapiscm_test_deq_empty");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("queued.json");
        assert!(dequeue(&path).is_none());
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_complete_failure() {
        let dir = std::env::temp_dir().join("rapiscm_test_comp_fail");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("queued.json");
        enqueue(&path, sample_item("x")).unwrap();
        let item = dequeue(&path).unwrap();
        complete(&path, &item.queue_id, None, Some("oops".into())).unwrap();
        let items = load(&path);
        assert_eq!(items[0].status, QueueItemStatus::Failed);
        assert_eq!(items[0].error.as_deref(), Some("oops"));
        let _ = fs::remove_dir_all(&dir);
    }

    #[test]
    fn test_recover_crashed_empty() {
        let dir = std::env::temp_dir().join("rapiscm_test_rec_empty");
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("queued.json");
        let n = recover_crashed(&path).unwrap();
        assert_eq!(n, 0);
        let _ = fs::remove_dir_all(&dir);
    }
}
