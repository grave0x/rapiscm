//! Shared utility functions.

use std::time::{SystemTime, UNIX_EPOCH};

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
    fn test_days_to_date() {
        // Unix epoch = 1970-01-01
        assert_eq!(days_to_date(0), (1970, 1, 1));
        // 2025-01-01
        let days_2025 = (2025 - 1970) * 365 + 14; // approx
        let d = now_iso();
        // just check it doesn't panic
    }
}
