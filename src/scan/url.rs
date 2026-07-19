//! URL-driven scan: single entry point with optional crawling.

use std::collections::HashSet;
use tracing::{info, warn};

use crate::check;
use crate::cli::CrawlMode;
use crate::config::ScanConfig;
use crate::error::Result;
use crate::extract;
use crate::ghost::{GhostConfig, GhostState};
use crate::parser;
use crate::scan::runner::ScanRunner;
use crate::types::{Endpoint, ResponseResult, Target};

/// Build an HTTP client from config (proxy, timeout, TLS, redirects).
fn build_client(
    config: &ScanConfig,
    mut ghost: Option<&mut GhostState>,
) -> Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder()
        .timeout(config.timeout)
        .danger_accept_invalid_certs(config.insecure)
        .redirect(reqwest::redirect::Policy::limited(10));

    // Apply ghost proxy if available and ghost mode is active
    if let Some(gs) = &mut ghost
        && let Some(proxy_url) = gs.next_proxy()
        && let Ok(proxy) = reqwest::Proxy::all(&proxy_url)
    {
        builder = builder.proxy(proxy);
        info!("ghost proxy: {proxy_url}");
    }
    // Fall back to config proxy
    if let Some(ref proxy_url) = config.proxy
        && let Ok(proxy) = reqwest::Proxy::all(proxy_url)
    {
        builder = builder.proxy(proxy);
    }

    // Configure default headers for ghost mode
    if let Some(gs) = &mut ghost
        && gs.config.enabled
        && let Some(ua) = gs.next_ua()
    {
        let val: &str = ua;
        builder = builder.user_agent(val);
    }

    Ok(builder.build()?)
}

/// Build a set of ghost-mode randomized headers.
fn ghost_headers(state: &mut GhostState) -> Vec<(String, String)> {
    let mut hdrs = Vec::new();
    if let Some(accept) = state.next_accept() {
        hdrs.push(("Accept".into(), accept.to_string()));
    }
    if let Some(lang) = state.next_lang() {
        hdrs.push(("Accept-Language".into(), lang.to_string()));
    }
    if let Some(enc) = state.next_encoding() {
        hdrs.push(("Accept-Encoding".into(), enc.to_string()));
    }
    hdrs
}

/// Crawl pages BFS-style up to `depth`, collecting same-origin API endpoints.
async fn crawl(
    client: &reqwest::Client,
    start: &reqwest::Url,
    base: &reqwest::Url,
    max_depth: usize,
    crawl_mode: CrawlMode,
    discovered: &mut Vec<reqwest::Url>,
    mut ghost: Option<&mut GhostState>,
) {
    let mut visited: HashSet<String> = HashSet::new();
    let mut queue: Vec<(usize, reqwest::Url)> = vec![(0, start.clone())];
    visited.insert(start.to_string());

    while let Some((cur_depth, url)) = queue.pop() {
        let mut req = client.get(url.as_str());
        // Apply ghost headers if active
        if let Some(gs) = &mut ghost {
            for (k, v) in ghost_headers(gs) {
                req = req.header(&k, &v);
            }
        }
        match req.send().await {
            Ok(resp) => {
                let headers: Vec<(String, String)> = resp
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect();
                let content_type = resp
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string();
                let body = resp.bytes().await.unwrap_or_default();

                // JS bundle scanning (for js or full crawl mode)
                if matches!(crawl_mode, CrawlMode::Js | CrawlMode::Full) {
                    let text = String::from_utf8_lossy(&body);
                    info!("scanning JS bundles from {}", url);
                    // allow_cross_origin is checked inside run_url_scan's scan_bundles calls
                    // We do not pass it here because crawl only processes same-origin JS bundles
                    match parser::js_bundle::scan_bundles(client, &text, base, false).await {
                        Ok(js_eps) => {
                            let js_urls = parser::js_bundle::to_scan_urls(&js_eps, base);
                            for u in js_urls {
                                if visited.insert(u.to_string()) {
                                    if parser::url::is_api_endpoint(&u) {
                                        discovered.push(u);
                                    } else {
                                        info!("  js discovered: {u}");
                                    }
                                }
                            }
                        }
                        Err(e) => warn!("js bundle scan failed: {e}"),
                    }
                }

                let links = extract::extract_from_response(&body, &content_type, base, &headers);
                for link in links {
                    if !parser::url::same_origin(&link, base) {
                        continue;
                    }
                    if !visited.insert(link.to_string()) {
                        continue;
                    }
                    if parser::url::is_api_endpoint(&link) {
                        discovered.push(link.clone());
                    } else if cur_depth < max_depth {
                        queue.push((cur_depth + 1, link));
                    }
                }
            }
            Err(e) => warn!("crawl failed for {url}: {e}"),
        }
    }
}

/// Run a URL-mode scan: discover endpoints, then scan them.
pub async fn run_url_scan(config: &ScanConfig) -> Result<Vec<ResponseResult>> {
    let base_url = match &config.target {
        Target::Url(u) => u.clone(),
        Target::Spec(_) => unreachable!("run_url_scan called with Spec target"),
    };

    // Initialize ghost state if active.
    let ghost_cfg = GhostConfig::new(
        config.ghost,
        config.jitter_pct,
        config.ua_rotate.clone(),
        config.proxy_rotate.clone(),
    );
    let mut ghost_state = GhostState::new(ghost_cfg);

    let client = build_client(config, Some(&mut ghost_state))?;
    let mut discovered: Vec<reqwest::Url> = Vec::new();

    // Step 1: fetch base URL.
    {
        let mut req = client.get(base_url.as_str());
        if ghost_state.config.is_active() {
            for (k, v) in ghost_headers(&mut ghost_state) {
                req = req.header(&k, &v);
            }
        }
        match req.send().await {
            Ok(resp) => {
                let resp_headers: Vec<(String, String)> = resp
                    .headers()
                    .iter()
                    .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                    .collect();
                let content_type = resp
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string();
                let body = resp.bytes().await.unwrap_or_default();

                // JS bundle scanning on base page
                if let Some(crawl_mode) = config.crawl_mode
                    && matches!(crawl_mode, CrawlMode::Js | CrawlMode::Full)
                {
                    let text = String::from_utf8_lossy(&body);
                    info!("scanning JS bundles from base page");
                    match parser::js_bundle::scan_bundles(&client, &text, &base_url, config.allow_cross_origin).await {
                        Ok(js_eps) => {
                            let js_urls = parser::js_bundle::to_scan_urls(&js_eps, &base_url);
                            for u in js_urls {
                                if parser::url::is_api_endpoint(&u) {
                                    discovered.push(u);
                                }
                            }
                        }
                        Err(e) => warn!("js bundle scan failed: {e}"),
                    }
                }

                let links =
                    extract::extract_from_response(&body, &content_type, &base_url, &resp_headers);
                for link in links {
                    if parser::url::is_api_endpoint(&link)
                        && parser::url::same_origin(&link, &base_url)
                    {
                        discovered.push(link);
                    }
                }
                info!("discovered {} API links from HTTP fetch", discovered.len());
            }
            Err(e) => warn!("failed to fetch base URL: {e}"),
        }
    }

    // Step 1b: optional recursive crawl.
    if let Some(crawl_mode) = config.crawl_mode {
        info!("crawling (mode: {crawl_mode:?}, depth: {})", config.depth);
        crawl(
            &client,
            &base_url,
            &base_url,
            config.depth,
            crawl_mode,
            &mut discovered,
            ghost_state.config.is_active().then_some(&mut ghost_state),
        )
        .await;
        info!(
            "crawl complete: {} total API endpoints found",
            discovered.len()
        );
    }

    // Step 2: optional browser-based interactive discovery.
    #[cfg(feature = "browser")]
    {
        info!("running browser discovery...");
        let proxy = ghost_state.next_proxy().or_else(|| config.proxy.clone());
        match crate::scan::browser::discover(
            &base_url,
            config.browser_kind,
            config.headed,
            proxy.as_deref(),
        )
        .await
        {
            Ok(browser_urls) => {
                let before = discovered.len();
                for u in browser_urls {
                    if parser::url::is_api_endpoint(&u) {
                        discovered.push(u);
                    }
                }
                info!("browser added {} new endpoints", discovered.len() - before);
            }
            Err(e) => warn!("browser discovery failed: {e}"),
        }
    }

    // Step 2b: browser JS eval if --eval flag is set.
    #[cfg(feature = "browser")]
    if let Some(ref eval_js) = config.eval_js {
        info!("running browser JS eval...");
        if let Ok(eval_urls) = crate::scan::browser::eval_and_extract(
            &base_url,
            eval_js,
            config.browser_kind,
            config.headed,
            config.proxy.as_deref(),
        )
        .await
        {
            let before = discovered.len();
            for u in eval_urls {
                if parser::url::is_api_endpoint(&u) {
                    discovered.push(u);
                }
            }
            info!(
                "browser eval added {} new endpoints",
                discovered.len() - before
            );
        }
    }

    // Step 3: probe common API paths from wordlist.
    for path in parser::url::api_wordlist() {
        if let Ok(url) = base_url.join(path) {
            discovered.push(url);
        }
    }

    // Step 4: deduplicate.
    discovered.sort_by(|a, b| a.as_str().cmp(b.as_str()));
    discovered.dedup();

    // Step 5: build endpoints with auth + headers.
    let auth_header = crate::types::auth_to_header(&config.auth);
    let mut endpoints: Vec<Endpoint> = discovered
        .into_iter()
        .filter(|url| {
            if config.paths.is_empty() {
                return true;
            }
            let p = url.path();
            config
                .paths
                .iter()
                .any(|pat| p.starts_with(pat) || p == pat)
        })
        .map(|url| {
            let mut headers = config.headers.clone();
            if let Some(ref h) = auth_header {
                headers.push(h.clone());
            }
            Endpoint {
                method: config
                    .method
                    .as_deref()
                    .and_then(|m| reqwest::Method::from_bytes(m.as_bytes()).ok())
                    .unwrap_or(reqwest::Method::GET),
                url,
                headers,
                body: None,
                expected_status: None,
                tags: vec![],
            }
        })
        .collect();

    // Tag endpoints.
    for ep in &mut endpoints {
        ep.tags = crate::tag::tag_endpoint(ep);
    }

    // Apply filters.
    crate::filter::filter_endpoints(&mut endpoints, config);

    info!("scanning {} endpoints from URL mode", endpoints.len());

    if endpoints.is_empty() {
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

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_client_default() {
        let config = ScanConfig {
            target: Target::Url(reqwest::Url::parse("https://example.com").unwrap()),
            method: None,
            headers: vec![],
            auth: None,
            rate_limit: 50,
            timeout: std::time::Duration::from_secs(30),
            concurrency: 10,
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
        let client = build_client(&config, None).unwrap();
        // Client builds without panic — verify by checking it's usable.
        let _ = client.get("https://example.com");
    }
}
