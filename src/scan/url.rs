use std::collections::HashSet;
use tracing::{info, warn};

use crate::check;
use crate::config::ScanConfig;
use crate::error::Result;
use crate::extract;
use crate::parser;
use crate::scan::runner::ScanRunner;
use crate::types::{Endpoint, ResponseResult, Target};

/// Build an HTTP client from config (proxy, timeout, TLS, redirects).
fn build_client(config: &ScanConfig) -> Result<reqwest::Client> {
    let mut builder = reqwest::Client::builder()
        .timeout(config.timeout)
        .danger_accept_invalid_certs(config.insecure)
        .redirect(reqwest::redirect::Policy::limited(10));
    if let Some(ref proxy_url) = config.proxy
        && let Ok(proxy) = reqwest::Proxy::all(proxy_url)
    {
        builder = builder.proxy(proxy);
    }
    Ok(builder.build()?)
}

/// Crawl pages BFS-style up to `depth`, collecting same-origin API endpoints.
async fn crawl(
    client: &reqwest::Client,
    start: &reqwest::Url,
    base: &reqwest::Url,
    max_depth: usize,
    discovered: &mut Vec<reqwest::Url>,
) {
    let mut visited: HashSet<String> = HashSet::new();
    // (depth, url) queue
    let mut queue: Vec<(usize, reqwest::Url)> = vec![(0, start.clone())];
    visited.insert(start.to_string());

    while let Some((cur_depth, url)) = queue.pop() {
        match client.get(url.as_str()).send().await {
            Ok(resp) => {
                let content_type = resp
                    .headers()
                    .get(reqwest::header::CONTENT_TYPE)
                    .and_then(|v| v.to_str().ok())
                    .unwrap_or("")
                    .to_string();
                let body = resp.bytes().await.unwrap_or_default();
                let links = extract::extract_from_response(&body, &content_type, base);
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
                        // Enqueue HTML pages for further discovery.
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

    let mut discovered: Vec<reqwest::Url> = Vec::new();
    let client = build_client(config)?;

    // Step 1: fetch base URL.
    match client.get(base_url.as_str()).send().await {
        Ok(resp) => {
            let content_type = resp
                .headers()
                .get(reqwest::header::CONTENT_TYPE)
                .and_then(|v| v.to_str().ok())
                .unwrap_or("")
                .to_string();
            let body = resp.bytes().await.unwrap_or_default();
            let links = extract::extract_from_response(&body, &content_type, &base_url);
            for link in links {
                if parser::url::is_api_endpoint(&link) && parser::url::same_origin(&link, &base_url)
                {
                    discovered.push(link);
                }
            }
            info!("discovered {} API links from HTTP fetch", discovered.len());
        }
        Err(e) => warn!("failed to fetch base URL: {e}"),
    }

    // Step 1b: optional recursive crawl.
    if config.crawl {
        crawl(&client, &base_url, &base_url, config.depth, &mut discovered).await;
        info!(
            "crawl complete: {} total API endpoints found",
            discovered.len()
        );
    }

    // Step 2: optional browser-based interactive discovery.
    #[cfg(feature = "browser")]
    {
        info!("running browser discovery...");
        match crate::scan::browser::discover(
            &base_url,
            config.browser_kind,
            config.headed,
            config.proxy.as_deref(),
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

    // Apply tag filtering.
    crate::tag::filter_endpoints(&mut endpoints, &config.filter_tag, &config.exclude_tag);

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

    // Run async checks (CORS probe, auth probe).
    check::run_async_checks(config, &mut results).await;

    // Tag results.
    for r in &mut results {
        r.tags = crate::tag::tag_response(r);
    }

    Ok(results)
}
