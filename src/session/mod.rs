//! Session replay mode — parse a recorded JSONL session and run the
//! check pipeline without live HTTP requests.

pub mod parse;
pub mod timing;

use std::path::PathBuf;

use crate::check;
use crate::error::Result;
use crate::report;
use crate::types::OutputFormat;

/// Session replay configuration.
#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub file: PathBuf,
    pub timing: bool,
    pub max_parse_errors: usize,
    pub skip_cors: bool,
    pub skip_auth: bool,
    pub output: OutputFormat,
}

/// Run a session replay against the given config.
///
/// Flow: parse JSONL → run sync checks → run async checks (unless
/// skipped) → compute timing analytics (if --timing) → print report.
pub async fn run_session(config: &SessionConfig) -> Result<()> {
    tracing::info!("parsing session file: {}", config.file.display());
    let (mut results, timestamps) =
        parse::parse_session_file(&config.file, config.max_parse_errors)?;

    tracing::info!("parsed {} requests, running checks...", results.len());

    // ── Sync checks (security headers, schema, trackers) ──
    for r in &mut results {
        check::run_checks(r);
    }

    // ── Async checks (CORS, auth) ──
    // Use a minimal config stub — just enough for CORS/auth probes.
    let stub_config = crate::config::ScanConfig {
        target: crate::types::Target::Url(
            results
                .first()
                .and_then(|r| reqwest::Url::parse(&r.endpoint_url).ok())
                .unwrap_or_else(|| reqwest::Url::parse("https://localhost").unwrap()),
        ),
        method: None,
        headers: Vec::new(),
        auth: None,
        rate_limit: 50,
        timeout: std::time::Duration::from_secs(30),
        concurrency: 10,
        output: config.output,
        follow_redirects: false,
        insecure: false,
        paths: Vec::new(),
        tags: Vec::new(),
        filter_tag: Vec::new(),
        exclude_tag: Vec::new(),
        proxy: None,
        log_level: String::new(),
        log_filter: Vec::new(),
        log_format: String::new(),
        #[cfg(feature = "browser")]
        browser_kind: crate::scan::browser::BrowserKind::Chrome,
        #[cfg(feature = "browser")]
        headed: false,
        crawl_mode: None,
        allow_cross_origin: false,
        depth: 2,
        ghost: false,
        jitter_pct: 0,
        ua_rotate: None,
        proxy_rotate: vec![],
        eval_js: None,
        script: None,
        filter_path: Vec::new(),
        exclude_path: Vec::new(),
        filter_method: Vec::new(),
        exclude_method: Vec::new(),
        filter_status: Vec::new(),
        exclude_status: Vec::new(),
        filter: Vec::new(),
        exclude: Vec::new(),
        show_tags: false,
        trackers: true,
        tracker_report: false,
        deep_spec: false,
        corp: None,
        save: false,
        task_name: None,
        task_tags: Vec::new(),
        no_bodies: true,
        raw: false,
        task_dir: None,
        git: false,
    };

    if !config.skip_cors || !config.skip_auth {
        check::run_async_checks(&stub_config, &mut results).await;
    }

    // ── Timing analytics ──
    if config.timing {
        let ta = timing::compute_timing_analytics(&timestamps, &results);
        let timing_output = timing::format_timing_analytics(&ta);

        // ── Report ──
        let main_output = report::format_results(&results, config.output);
        println!("{main_output}");
        print!("{timing_output}");
    } else {
        let output = report::format_results(&results, config.output);
        println!("{output}");
    }

    tracing::info!("session replay complete ({} requests)", results.len());
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_config_debug() {
        let cfg = SessionConfig {
            file: PathBuf::from("test.jsonl"),
            timing: false,
            max_parse_errors: 10,
            skip_cors: false,
            skip_auth: false,
            output: OutputFormat::Table,
        };
        assert!(format!("{cfg:?}").contains("test.jsonl"));
    }
}
