/// Parse robots.txt and sitemap.xml for URLs.
use reqwest::Url;

/// Extract URLs from robots.txt content (sitemap directives + disallowed paths).
pub fn extract_robots_txt(text: &str, base: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    for line in text.lines() {
        let line = line.trim();
        if line.to_lowercase().starts_with("sitemap:") {
            let url_str = line[8..].trim();
            if let Ok(url) = Url::parse(url_str) {
                urls.push(url);
            }
        } else if line.to_lowercase().starts_with("disallow:") {
            let path = line[9..].trim();
            if !path.is_empty() && path != "/" {
                // Remove trailing wildcard
                let clean = path.trim_end_matches('*');
                if let Ok(url) = base.join(clean) {
                    urls.push(url);
                }
            }
        }
    }
    urls
}

/// Extract URLs from sitemap XML content.
pub fn extract_sitemap(text: &str, _base: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    // Simple loc tag extraction (no XML parser dependency)
    let marker = "<loc>";
    let end_marker = "</loc>";
    for (i, _) in text.match_indices(marker) {
        let start = i + marker.len();
        if let Some(end) = text[start..].find(end_marker) {
            let url_str = &text[start..start + end].trim();
            if let Ok(url) = Url::parse(url_str)
                && (url.scheme() == "http" || url.scheme() == "https")
            {
                urls.push(url);
            }
        }
    }
    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_robots_sitemap() {
        let base = Url::parse("https://example.com").unwrap();
        let robots = "User-agent: *\nSitemap: https://example.com/sitemap.xml\nDisallow: /admin";
        let urls = extract_robots_txt(robots, &base);
        assert!(urls.iter().any(|u| u.as_str().contains("sitemap.xml")));
        assert!(urls.iter().any(|u| u.as_str().contains("/admin")));
    }

    #[test]
    fn test_sitemap_locs() {
        let base = Url::parse("https://example.com").unwrap();
        let sitemap = r#"<?xml version="1.0"?><urlset><url><loc>https://example.com/page1</loc></url></urlset>"#;
        let urls = extract_sitemap(sitemap, &base);
        assert_eq!(urls.len(), 1);
    }
}
