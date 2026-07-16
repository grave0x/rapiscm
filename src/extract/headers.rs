/// Extract URLs from HTTP response headers.
use reqwest::Url;

pub fn extract_from_headers(headers: &[(String, String)], _base: &Url) -> Vec<Url> {
    let mut urls = Vec::new();

    // Headers commonly containing URLs
    let url_headers = [
        "Location",
        "Link",
        "Content-Location",
        "Referer",
        "Origin",
        "Access-Control-Allow-Origin",
    ];

    for (name, value) in headers {
        let lower = name.to_lowercase();
        if !url_headers.iter().any(|h| h.to_lowercase() == lower) {
            continue;
        }

        // Link header format: <https://example.com>; rel="next"
        if let Some(url_str) = value.strip_prefix('<').and_then(|v| v.split('>').next()) {
            if let Ok(url) = Url::parse(url_str) {
                urls.push(url);
            }
            continue;
        }

        // Direct URL in header
        if let Ok(url) = Url::parse(value)
            && (url.scheme() == "http" || url.scheme() == "https")
        {
            urls.push(url);
        }
    }

    urls
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_location_header() {
        let base = Url::parse("https://example.com").unwrap();
        let headers = vec![("Location".into(), "https://example.com/redirect".into())];
        let urls = extract_from_headers(&headers, &base);
        assert_eq!(urls.len(), 1);
        assert!(urls[0].as_str().contains("/redirect"));
    }

    #[test]
    fn test_link_header() {
        let base = Url::parse("https://example.com").unwrap();
        let headers = vec![(
            "Link".into(),
            "<https://api.example.com/next>; rel=\"next\"".into(),
        )];
        let urls = extract_from_headers(&headers, &base);
        assert_eq!(urls.len(), 1);
    }
}
