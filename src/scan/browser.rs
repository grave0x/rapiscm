//! Browser-based endpoint discovery (headless Chrome/Firefox).

/// Browser-based endpoint discovery.
///
/// Only compiled when `browser` feature is enabled.
/// Supports Chrome (CDP) and Firefox (WebDriver).
/// Interactively clicks links and submits forms to discover API endpoints
/// that are only reachable through JS-driven navigation.
use futures_util::StreamExt;
use tracing::{info, warn};

/// Which browser engine to use.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BrowserKind {
    Chrome,
    Firefox,
}

/// Discover API endpoints by interactively loading a page in a browser.
///
/// 1. Load the target URL
/// 2. Wait for JS to execute
/// 3. Collect all same-origin links from the rendered page
/// 4. Click each link to trigger more API calls
/// 5. Return all extracted URLs
pub async fn discover(
    url: &reqwest::Url,
    kind: BrowserKind,
    headed: bool,
    proxy: Option<&str>,
) -> anyhow::Result<Vec<reqwest::Url>> {
    match kind {
        BrowserKind::Chrome => discover_chrome(url, headed, proxy).await,
        BrowserKind::Firefox => discover_firefox(url, headed, proxy).await,
    }
}

/// Evaluate JS in the page and deserialize the result.
async fn eval_js<T: serde::de::DeserializeOwned>(page: &chromiumoxide::Page, js: &str) -> anyhow::Result<T> {
    let result = page.evaluate(js).await?;
    Ok(result.into_value()?)
}

/// Get rendered HTML from a page.
async fn page_html(page: &chromiumoxide::Page) -> anyhow::Result<String> {
    eval_js(page, "document.documentElement.outerHTML").await
}

// ── Chrome ──────────────────────────────────────────────────────────────

async fn discover_chrome(url: &reqwest::Url, headed: bool, proxy: Option<&str>) -> anyhow::Result<Vec<reqwest::Url>> {
    use chromiumoxide::{Browser, BrowserConfig};

    let mut cfg = BrowserConfig::builder();
    if headed {
        cfg = cfg.with_head();
    }
    if let Some(p) = proxy {
        cfg = cfg.arg(format!("--proxy-server={p}"));
    }
    let (browser, mut handler) = Browser::launch(cfg.build().map_err(|e| anyhow::anyhow!("{e}"))?).await?;

    tokio::spawn(async move { while let Some(_) = handler.next().await {} });

    let page = browser.new_page("about:blank").await?;
    page.goto(url.as_str()).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let mut all_urls: Vec<reqwest::Url> = Vec::new();
    let base = url.clone();

    // Collect links from initial page.
    if let Ok(html) = page_html(&page).await {
        all_urls.extend(crate::extract::html::extract_html(&html, &base));
    }

    // Interactively click same-origin links to discover more endpoints.
    let links = find_links(&page, &base).await;
    for link in &links {
        info!("browser clicking: {link}");
        if let Ok(new_page) = browser.new_page(link.as_str()).await {
            tokio::time::sleep(std::time::Duration::from_secs(2)).await;
            if let Ok(html2) = page_html(&new_page).await {
                all_urls.extend(crate::extract::html::extract_html(&html2, &base));
            }
        }
    }

    // Collect form action URLs.
    collect_form_urls(&page, &base, &mut all_urls).await;

    drop(page);
    drop(browser);

    all_urls.sort();
    all_urls.dedup();
    info!("browser discovered {} unique URLs", all_urls.len());
    Ok(all_urls)
}

/// Take a screenshot of a page and save to file.
#[cfg(feature = "browser")]
pub async fn screenshot(
    url: &reqwest::Url,
    out_path: &std::path::Path,
    headed: bool,
    proxy: Option<&str>,
) -> anyhow::Result<()> {
    use chromiumoxide::{Browser, BrowserConfig};

    let mut cfg = BrowserConfig::builder();
    if headed {
        cfg = cfg.with_head();
    }
    if let Some(p) = proxy {
        cfg = cfg.arg(format!("--proxy-server={p}"));
    }
    let (browser, mut handler) = Browser::launch(cfg.build().map_err(|e| anyhow::anyhow!("{e}"))?).await?;
    tokio::spawn(async move { while let Some(_) = handler.next().await {} });

    let page = browser.new_page("about:blank").await?;
    page.goto(url.as_str()).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    page.screenshot(chromiumoxide::ScreenshotParams::new(
        out_path.to_str().unwrap_or("screenshot.png"),
        true,  // full page
        None,  // clip
        false, // omit background
    ))
    .await?;

    drop(page);
    drop(browser);
    Ok(())
}

/// Evaluate JS in browser and extract URLs from the result.
///
/// This runs arbitrary JS in the page context and collects any URL strings
/// returned as an array. Useful for extracting dynamically-registered API routes.
#[cfg(feature = "browser")]
pub async fn eval_and_extract(
    url: &reqwest::Url,
    eval_js: &str,
    kind: BrowserKind,
    headed: bool,
    proxy: Option<&str>,
) -> anyhow::Result<Vec<reqwest::Url>> {
    match kind {
        BrowserKind::Chrome => eval_chrome(url, eval_js, headed, proxy).await,
        BrowserKind::Firefox => eval_firefox(url, eval_js, headed, proxy).await,
    }
}

#[cfg(feature = "browser")]
async fn eval_chrome(
    url: &reqwest::Url,
    eval_js: &str,
    headed: bool,
    proxy: Option<&str>,
) -> anyhow::Result<Vec<reqwest::Url>> {
    use chromiumoxide::{Browser, BrowserConfig};

    let mut cfg = BrowserConfig::builder();
    if headed {
        cfg = cfg.with_head();
    }
    if let Some(p) = proxy {
        cfg = cfg.arg(format!("--proxy-server={p}"));
    }
    let (browser, mut handler) = Browser::launch(cfg.build().map_err(|e| anyhow::anyhow!("{e}"))?).await?;
    tokio::spawn(async move { while let Some(_) = handler.next().await {} });

    let page = browser.new_page("about:blank").await?;
    page.goto(url.as_str()).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let result: serde_json::Value = page.evaluate(eval_js).await?.into_value()?;
    drop(page);
    drop(browser);

    let mut urls = Vec::new();
    match result {
        serde_json::Value::Array(arr) => {
            for v in arr {
                if let Some(s) = v.as_str() {
                    if let Ok(u) = reqwest::Url::parse(s) {
                        urls.push(u);
                    } else if s.starts_with('/') {
                        if let Ok(u) = url.join(s) {
                            urls.push(u);
                        }
                    }
                }
            }
        }
        serde_json::Value::String(s) => {
            if let Ok(u) = reqwest::Url::parse(&s) {
                urls.push(u);
            } else if s.starts_with('/') {
                if let Ok(u) = url.join(&s) {
                    urls.push(u);
                }
            }
        }
        _ => {
            // Try extracting URLs from the JSON string representation
            let text = result.to_string();
            let base = url.clone();
            urls.extend(crate::extract::js::extract_js(&text, &base));
        }
    }

    info!("browser eval returned {} URLs", urls.len());
    Ok(urls)
}

#[cfg(feature = "browser")]
async fn eval_firefox(
    url: &reqwest::Url,
    eval_js: &str,
    headed: bool,
    _proxy: Option<&str>,
) -> anyhow::Result<Vec<reqwest::Url>> {
    use fantoccini::ClientBuilder;

    let client = ClientBuilder::native().connect("http://localhost:4444").await?;

    if !headed {
        warn!("firefox headless requires geckodriver with --connect-existing");
    }

    client.goto(url.as_str()).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let result = client.eval(eval_js).await?;
    client.close().await?;

    let mut urls = Vec::new();
    let text = result.to_string();
    let base = url.clone();
    urls.extend(crate::extract::js::extract_js(&text, &base));
    Ok(urls)
}

/// Find same-origin links from the current page via JS.
async fn find_links(page: &chromiumoxide::Page, base: &reqwest::Url) -> Vec<reqwest::Url> {
    let js = r#"
        Array.from(document.querySelectorAll('a[href]'))
            .map(a => a.href)
            .filter(h => h.startsWith(location.origin))
    "#;
    let urls: Vec<String> = eval_js(page, js).await.unwrap_or_default();
    urls.iter()
        .filter_map(|s| reqwest::Url::parse(s).ok())
        .filter(|u| {
            let p = u.path().to_lowercase();
            !p.ends_with(".css")
                && !p.ends_with(".js")
                && !p.ends_with(".png")
                && !p.ends_with(".svg")
                && !p.ends_with(".ico")
                && !p.ends_with(".woff")
                && !p.ends_with(".woff2")
        })
        .collect()
}

/// Collect form action URLs.
async fn collect_form_urls(page: &chromiumoxide::Page, base: &reqwest::Url, out: &mut Vec<reqwest::Url>) {
    let js = r#"
        Array.from(document.forms).map(f => ({
            action: f.action,
            method: f.method
        }))
    "#;
    let forms: Vec<serde_json::Value> = eval_js(page, js).await.unwrap_or_default();
    for f in &forms {
        let action = match f["action"].as_str() {
            Some(a) => a,
            None => continue,
        };
        if let Ok(url) = base.join(action) {
            if crate::parser::url::same_origin(&url, base) {
                out.push(url);
            }
        }
    }
}

// ── Firefox ─────────────────────────────────────────────────────────────

async fn discover_firefox(url: &reqwest::Url, headed: bool, _proxy: Option<&str>) -> anyhow::Result<Vec<reqwest::Url>> {
    use fantoccini::ClientBuilder;

    let client = ClientBuilder::native().connect("http://localhost:4444").await?;

    if !headed {
        warn!("firefox headless requires geckodriver with --connect-existing");
    }

    client.goto(url.as_str()).await?;
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    let mut all_urls: Vec<reqwest::Url> = Vec::new();
    let base = url.clone();

    let html = client.source().await?;
    all_urls.extend(crate::extract::html::extract_html(&html, &base));

    // Click all same-origin links.
    let links = find_firefox_links(&client, &base).await;
    for link in &links {
        info!("firefox clicking: {link}");
        if let Err(e) = client.goto(link.as_str()).await {
            warn!("firefox nav failed: {e}");
            continue;
        }
        tokio::time::sleep(std::time::Duration::from_secs(2)).await;
        if let Ok(html2) = client.source().await {
            all_urls.extend(crate::extract::html::extract_html(&html2, &base));
        }
    }

    client.close().await?;

    all_urls.sort();
    all_urls.dedup();
    info!("firefox discovered {} unique URLs", all_urls.len());
    Ok(all_urls)
}

async fn find_firefox_links(client: &fantoccini::Client, base: &reqwest::Url) -> Vec<reqwest::Url> {
    use fantoccini::Locator;

    let elements = client.find_all(Locator::Css("a[href]")).await.unwrap_or_default();
    let mut urls = Vec::new();
    for el in elements {
        if let Ok(Some(href)) = el.attr("href").await {
            if let Ok(url) = base.join(&href) {
                if crate::parser::url::same_origin(&url, base) {
                    urls.push(url);
                }
            }
        }
    }
    urls
}
