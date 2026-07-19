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
