use tracing::{info, warn};

use crate::check;
use crate::config::ScanConfig;
use crate::error::Result;
use crate::extract;
use crate::parser;
use crate::scan::runner::ScanRunner;
use crate::types::{Endpoint, ResponseResult, Target};

/// Run a URL-mode scan: discover endpoints, then scan them.
pub async fn run_url_scan(config: &ScanConfig) -> Result<Vec<ResponseResult>> {
    let base_url = match &config.target {
        Target::Url(u) => u.clone(),
        Target::Spec(_) => unreachable!("run_url_scan called with Spec target"),
    };

    let mut discovered: Vec<reqwest::Url> = Vec::new();

    // Step 1: fetch base URL with proxy support.
    let mut client_builder = reqwest::Client::builder()
        .timeout(config.timeout)
        .danger_accept_invalid_certs(config.insecure)
        .redirect(reqwest::redirect::Policy::limited(10));
    if let Some(ref proxy_url) = config.proxy
        && let Ok(proxy) = reqwest::Proxy::all(proxy_url)
    {
        client_builder = client_builder.proxy(proxy);
    }
    let client = client_builder.build()?;

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
