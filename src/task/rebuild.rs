//! Rebuild a scan from a saved task.
//!
//! Loads the stored results, re-runs only endpoints that had errors
//! (`status_code == 0`), and merges fresh results back.

use crate::error::Error;
use crate::scan::runner::ScanRunner;
use crate::task::TaskStorage;
use crate::types::ResponseResult;

use super::TaskId;

/// Rebuild a task:
/// 1. Load stored meta + results.
/// 2. Re-scan failed (status_code == 0) endpoints.
/// 3. Merge fresh results into the stored set.
/// 4. Save as a new task (new ID).
pub async fn rebuild(
    storage: &TaskStorage,
    orig_id: TaskId,
    runner: &ScanRunner,
    rerun_all: bool,
) -> Result<TaskId, Error> {
    let meta = storage.load_meta(orig_id).map_err(Error::Task)?;
    let old_results = storage.load_results(orig_id).map_err(Error::Task)?;

    // Determine which endpoints to retry.
    let retry_indices: Vec<usize> = old_results
        .iter()
        .enumerate()
        .filter(|(_, r)| rerun_all || r.status_code == 0)
        .map(|(i, _)| i)
        .collect();

    if retry_indices.is_empty() {
        return Err(Error::Task("no endpoints to retry".into()));
    }

    // Rebuild endpoints from stored results.
    let endpoints: Vec<crate::types::Endpoint> = old_results
        .iter()
        .map(|r| crate::types::Endpoint {
            method: r.endpoint_method.parse().unwrap_or(reqwest::Method::GET),
            url: r.endpoint_url.parse().expect("invalid stored URL"),
            headers: r.response_headers.clone(),
            body: None,
            expected_status: r.expected_status,
            tags: r.tags.clone(),
        })
        .collect();

    // Scan only the retry endpoints.
    let retry_endpoints: Vec<crate::types::Endpoint> = retry_indices
        .iter()
        .map(|&i| endpoints[i].clone())
        .collect();

    let new_results = runner.run(retry_endpoints).await;

    // Merge: keep old results where unchanged, replace retried ones.
    let mut merged: Vec<ResponseResult> = old_results;
    for (j, &idx) in retry_indices.iter().enumerate() {
        if j < new_results.len() {
            merged[idx] = new_results[j].clone();
        }
    }

    // Save as new task.
    let new_id = crate::task::index::next_id(&storage.index_path());
    let summary = crate::task::summarize(&merged);
    let new_meta = crate::task::TaskMeta {
        task_id: new_id,
        task_name: format!("{} (rebuild of {})", meta.task_name, orig_id),
        task_tags: meta.task_tags.clone(),
        cli_version: meta.cli_version.clone(),
        created_at: crate::util::now_iso(),
        duration_seconds: 0.0, // unknown
        command: format!("rebuild:{}", meta.command),
        target: meta.target.clone(),
        config: meta.config.clone(),
        git: meta.git.clone(),
        endpoint_count: merged.len(),
        result_summary: summary,
        storage: crate::task::StorageInfo {
            has_bodies: true,
            has_raw: false,
            results_size_bytes: 0,
        },
        exit_code: 0,
    };
    storage
        .save(&new_meta, &merged, false, false)
        .map_err(Error::Task)?;

    Ok(new_id)
}
