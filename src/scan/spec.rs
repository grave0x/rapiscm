//! Spec-driven scan: parse spec, discover endpoints, send requests.

use tracing::info;

use crate::check;
use crate::config::ScanConfig;
use crate::error::Result;
use crate::parser;
use crate::scan::runner::ScanRunner;
use crate::types::{ResponseResult, Target};

/// Run a spec-mode scan: parse the spec, then fire requests.
pub async fn run_spec_scan(config: &ScanConfig) -> Result<Vec<ResponseResult>> {
    let path = match &config.target {
        Target::Spec(p) => p.clone(),
        Target::Url(_) => unreachable!("run_spec_scan called with URL target"),
    };

    let mut endpoints = parser::spec::parse_spec_file(&path)?;
    info!("parsed {} endpoints from spec", endpoints.len());

    // Tag endpoints.
    for ep in &mut endpoints {
        ep.tags = crate::tag::tag_endpoint(ep);
    }

    // Apply filters.
    crate::filter::filter_endpoints(&mut endpoints, config);

    // Apply auth header.
    if let Some(auth_header) = crate::types::auth_to_header(&config.auth) {
        for ep in &mut endpoints {
            ep.headers.push(auth_header.clone());
        }
    }

    // Apply global headers from config to every endpoint.
    for ep in &mut endpoints {
        for (k, v) in &config.headers {
            ep.headers.push((k.clone(), v.clone()));
        }
    }

    if endpoints.is_empty() {
        info!("no endpoints found in spec");
        return Ok(vec![]);
    }

    let runner = ScanRunner::new(config)?;
    let mut results = runner.run(endpoints).await;

    // Run checks on each result.
    for r in &mut results {
        check::run_checks(r);
    }

    // Detect trackers/analytics in responses.
    if config.trackers {
        for r in &mut results {
            let body = String::from_utf8_lossy(&r.response_body);
            r.trackers = crate::analytics::detect_trackers(&body, &r.response_headers);
        }
    }

    // Run async checks (CORS probe, auth probe).
    check::run_async_checks(config, &mut results).await;

    // Tag results.
    for r in &mut results {
        r.tags = crate::tag::tag_response(r);
    }

    // Apply status filters on results.
    crate::filter::filter_results(&mut results, config);

    info!("scan complete: {} results", results.len());
    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::ScanConfig;
    use crate::types::Target;

    #[test]
    fn test_spec_scan_no_such_file() {
        let config = ScanConfig {
            target: Target::Spec(std::path::PathBuf::from("/nonexistent/spec.yaml")),
            allow_cross_origin: false,
            method: None,
            headers: vec![],
            auth: None,
            rate_limit: 50,
            timeout: std::time::Duration::from_secs(10),
            concurrency: 5,
            output: crate::types::OutputFormat::Table,
            follow_redirects: false,
            insecure: false,
            paths: vec![],
            tags: vec![],
            filter_tag: vec![],
            exclude_tag: vec![],
            proxy: None,
            log_level: "info".into(),
            log_filter: vec![],
            log_format: "text".into(),
            crawl_mode: None,
            depth: 2,
            filter_path: vec![],
            exclude_path: vec![],
            filter_method: vec![],
            exclude_method: vec![],
            filter_status: vec![],
            exclude_status: vec![],
            filter: vec![],
            exclude: vec![],
            show_tags: false,
            trackers: true,
            tracker_report: false,
            corp: None,
            save: false,
            task_name: None,
            task_tags: vec![],
            no_bodies: false,
            raw: false,
            task_dir: None,
            git: false,
            deep_spec: false,
            ghost: false,
            jitter_pct: 0,
            ua_rotate: None,
            proxy_rotate: vec![],
            eval_js: None,
            script: None,
        };
        let rt = tokio::runtime::Runtime::new().unwrap();
        let result = rt.block_on(run_spec_scan(&config));
        assert!(result.is_err());
    }
}
