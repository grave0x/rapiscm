pub mod html;
pub mod js;
pub mod json;
pub mod sitemap;

/// Extract all URLs from an HTTP response body by dispatching to
/// the appropriate extractor based on content type.
pub fn extract_from_response(
    body: &[u8],
    content_type: &str,
    base_url: &reqwest::Url,
) -> Vec<reqwest::Url> {
    let ct = content_type.to_lowercase();
    if ct.contains("text/html") || ct.contains("application/html") {
        return html::extract_html(std::str::from_utf8(body).unwrap_or(""), base_url);
    }
    if ct.contains("application/json") || ct.contains("application/problem+json") {
        return json::extract_json(body, base_url);
    }
    if ct.contains("javascript") || ct.contains("ecmascript") || ct.contains("x-javascript") {
        return js::extract_js(std::str::from_utf8(body).unwrap_or(""), base_url);
    }
    if ct.contains("text/xml") || ct.contains("application/xml") || ct.contains("text/plain") {
        let text = std::str::from_utf8(body).unwrap_or("");
        let mut urls = sitemap::extract_robots_txt(text, base_url);
        urls.append(&mut js::extract_js(text, base_url));
        return urls;
    }
    vec![]
}
