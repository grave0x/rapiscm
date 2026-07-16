/// Extract URLs from HTML content by scanning href, src, action attributes.
/// Originally in parser/url.rs, moved here for Phase 2 restructure.
use reqwest::Url;

pub fn extract_html(html: &str, base: &Url) -> Vec<Url> {
    let mut urls = Vec::new();
    for attr in &["href", "src", "action"] {
        scan_attr(html, attr, base, &mut urls);
    }
    // Also scan for <base href> to resolve relative URLs correctly
    urls
}

fn scan_attr(html: &str, attr: &str, base: &Url, out: &mut Vec<Url>) {
    for quote in ['"', '\''] {
        let pattern = format!("{attr}={quote}");
        let mut pos = 0;
        while let Some(start) = html[pos..].find(&pattern) {
            let value_begin = pos + start + pattern.len();
            if let Some(end) = html[value_begin..].find(quote) {
                let value = &html[value_begin..value_begin + end];
                if let Ok(url) = base.join(value) {
                    match url.scheme() {
                        "http" | "https" | "ws" | "wss" => out.push(url),
                        _ => {}
                    }
                }
                pos = value_begin + end + 1;
            } else {
                pos = value_begin + 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_simple() {
        let base = Url::parse("https://example.com").unwrap();
        let html = r#"<a href="/api/users">Users</a> <form action="/api/login">"#;
        let urls = extract_html(html, &base);
        assert_eq!(urls.len(), 2);
    }

    #[test]
    fn test_skips_mailto() {
        let base = Url::parse("https://example.com/page").unwrap();
        let html =
            "<a href=\"mailto:user@x.com\">Email</a> <a href=\"https://other.com\">Other</a>";
        let urls = extract_html(html, &base);
        assert_eq!(urls.len(), 1);
        assert!(urls[0].as_str().contains("other.com"));
    }
}
