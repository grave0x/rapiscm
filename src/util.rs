//! Shared utility functions.

use std::path::Path;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::task::GitInfo;

/// ISO-8601-like timestamp (UTC, second precision).
pub fn now_iso() -> String {
    let dur = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default();
    let secs = dur.as_secs();
    // Convert to yyyy-mm-ddThh:mm:ssZ
    let days = secs / 86400;
    let time_secs = secs % 86400;
    let hours = time_secs / 3600;
    let minutes = (time_secs % 3600) / 60;
    let seconds = time_secs % 60;

    // Compute year/month/day from days since epoch.
    let (year, month, day) = days_to_date(days as i64);
    format!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}Z",
        year, month, day, hours, minutes, seconds
    )
}

/// Convert days since Unix epoch to (year, month, day).
fn days_to_date(mut days: i64) -> (i64, u32, u32) {
    // Algorithm from Howard Hinnant
    days += 719468;
    let era = if days >= 0 { days } else { days - 146096 } / 146097;
    let doe = days - era * 146097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
    let y = yoe + era * 400;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = if m <= 2 { y + 1 } else { y };
    (y, m as u32, d as u32)
}

/// Capture git context at compile-time or from the working directory.
///
/// Reads `.git/HEAD` to determine branch and SHA. Falls back to
/// `git` CLI if `.git/HEAD` is a symlink or needs chasing.
/// Returns `None` if not in a git repo or git info can't be read.
pub fn capture_git_info() -> Option<GitInfo> {
    // Try from manifest dir first (cleaner for compiled binary), then cwd
    let git_dir = std::env::var("CARGO_MANIFEST_DIR")
        .ok()
        .map(|d| Path::new(&d).join(".git"))
        .filter(|p| p.exists())
        .unwrap_or_else(|| Path::new(".git").to_path_buf());

    let head_path = git_dir.join("HEAD");
    let head_raw = std::fs::read_to_string(&head_path).ok()?;
    let head = head_raw.trim();

    let (sha, branch) = if let Some(ref_path) = head.strip_prefix("ref: ") {
        let branch_name = ref_path.strip_prefix("refs/heads/").unwrap_or(ref_path);
        let ref_file = git_dir.join(ref_path);
        let sha_val = std::fs::read_to_string(&ref_file)
            .ok()
            .map(|s| s.trim().to_string())
            .or_else(|| {
                // Fallback: use git CLI
                Command::new("git")
                    .args(["rev-parse", "HEAD"])
                    .output()
                    .ok()
                    .and_then(|o| {
                        if o.status.success() {
                            String::from_utf8(o.stdout)
                                .ok()
                                .map(|s| s.trim().to_string())
                        } else {
                            None
                        }
                    })
            })?;
        (sha_val, branch_name.to_string())
    } else {
        // Detached HEAD — SHA directly in HEAD file
        (head.to_string(), "HEAD".to_string())
    };

    // Check dirty: `git status --porcelain`
    let dirty = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .ok()
        .map(|o| !o.stdout.is_empty())
        .unwrap_or(false);

    // Commit message
    let message = Command::new("git")
        .args(["log", "-1", "--format=%s"])
        .output()
        .ok()
        .and_then(|o| {
            if o.status.success() {
                String::from_utf8(o.stdout)
                    .ok()
                    .map(|s| s.trim().to_string())
            } else {
                None
            }
        })
        .unwrap_or_default();

    Some(GitInfo {
        sha,
        branch,
        message,
        dirty,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_now_iso_format() {
        let s = now_iso();
        assert!(s.len() == 20, "expected ISO format, got: {s}");
        assert!(s.ends_with('Z'));
    }

    #[test]
    fn test_capture_git_info() {
        // Should work in our own repo during tests
        let info = capture_git_info();
        assert!(info.is_some(), "expected git info in repo checkout");
        let info = info.unwrap();
        assert!(!info.sha.is_empty(), "SHA should be non-empty");
        assert!(!info.branch.is_empty(), "branch should be non-empty");
        assert!(!info.message.is_empty(), "message should be non-empty");
    }

    #[test]
    fn test_days_to_date() {
        // Unix epoch = 1970-01-01
        assert_eq!(days_to_date(0), (1970, 1, 1));
        // 2025-01-01
        let days_2025 = (2025 - 1970) * 365 + 14; // approx
        let d = now_iso();
        // just check it doesn't panic
    }
}
