//! URL extraction from various response content types.

pub mod headers;
pub mod html;
pub mod js;
pub mod json;
pub mod sitemap;

/// Extract all URLs from an HTTP response — headers first, then body
/// (dispatched by content type).
///
/// Header extraction covers `Location`, `Content-Location`, and `Link` headers.
/// Body extraction is dispatched to the appropriate content-type handler
/// (HTML, JSON, JS, sitemap/robots.txt).
pub fn extract_from_response(
    body: &[u8],
    content_type: &str,
    base_url: &reqwest::Url,
    headers: &[(String, String)],
) -> Vec<reqwest::Url> {
    let mut urls = headers::extract_headers(headers, base_url);
    let ct = content_type.to_lowercase();
    if ct.contains("text/html") || ct.contains("application/html") {
        urls.extend(html::extract_html(
            std::str::from_utf8(body).unwrap_or(""),
            base_url,
        ));
    } else if ct.contains("application/json") || ct.contains("application/problem+json") {
        urls.extend(json::extract_json(body, base_url));
    } else if ct.contains("javascript") || ct.contains("ecmascript") || ct.contains("x-javascript")
    {
        urls.extend(js::extract_js(
            std::str::from_utf8(body).unwrap_or(""),
            base_url,
        ));
    } else if ct.contains("text/xml") || ct.contains("application/xml") || ct.contains("text/plain")
    {
        let text = std::str::from_utf8(body).unwrap_or("");
        urls.extend(sitemap::extract_robots_txt(text, base_url));
        urls.extend(js::extract_js(text, base_url));
    }
    urls
}
