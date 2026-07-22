//! URL extraction from JavaScript source strings.

/// Extract URLs from JavaScript by matching common API call patterns
/// and string literals containing URL-like content.
use reqwest::Url;

fn is_valid_url(url: &Url) -> bool {
    matches!(url.scheme(), "http" | "https" | "ws" | "wss")
}

pub fn extract_js(js: &str, base: &Url) -> Vec<Url> {
    let mut urls = Vec::new();

    find_str_args(js, "fetch", &mut urls, base);
    find_str_args(js, "axios.", &mut urls, base);
    find_str_args(js, ".get(", &mut urls, base);
    find_str_args(js, ".post(", &mut urls, base);
    find_str_args(js, ".put(", &mut urls, base);
    find_str_args(js, ".delete(", &mut urls, base);
    find_str_args(js, ".patch(", &mut urls, base);
    find_str_args(js, "$.ajax", &mut urls, base);
    find_str_args(js, "$.get(", &mut urls, base);
    find_str_args(js, "$.post(", &mut urls, base);
    find_str_args(js, "WebSocket(", &mut urls, base);
    find_str_after(js, "location.href", &mut urls, base);
    find_str_after(js, "url:", &mut urls, base);

    // Any https?:// or wss?:// string literals
    for prefix in &["https://", "http://", "wss://", "ws://"] {
        for (i, _) in js.match_indices(prefix) {
            let end = js[i..]
                .find(|c: char| {
                    c.is_whitespace() || c == '"' || c == '\'' || c == '`' || c == ')' || c == '}' || c == ']'
                })
                .map(|e| i + e)
                .unwrap_or(js.len());
            if let Ok(url) = Url::parse(&js[i..end])
                && is_valid_url(&url)
            {
                urls.push(url);
            }
        }
    }

    urls
}

fn find_str_args(js: &str, prefix: &str, out: &mut Vec<Url>, base: &Url) {
    for quote in ['"', '\''] {
        let pattern = format!("{prefix}({quote}");
        for (i, _) in js.match_indices(&pattern) {
            let start = i + pattern.len();
            if let Some(end) = js[start..].find(quote) {
                let candidate = &js[start..start + end];
                if let Ok(url) = Url::parse(candidate) {
                    if is_valid_url(&url) {
                        out.push(url);
                    }
                } else if candidate.starts_with('/')
                    && let Ok(url) = base.join(candidate)
                {
                    out.push(url);
                }
            }
        }
    }
}

fn find_str_after(js: &str, marker: &str, out: &mut Vec<Url>, base: &Url) {
    for quote in ['"', '\'', '`'] {
        for pattern in &[format!("{}{}", marker, quote), format!("{} = {}", marker, quote)] {
            for (i, _) in js.match_indices(pattern) {
                let start = i + pattern.len();
                if let Some(end) = js[start..].find(quote) {
                    let candidate = &js[start..start + end];
                    if let Ok(url) = Url::parse(candidate) {
                        if is_valid_url(&url) {
                            out.push(url);
                        }
                    } else if candidate.starts_with('/')
                        && let Ok(url) = base.join(candidate)
                    {
                        out.push(url);
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fetch_url() {
        let base = Url::parse("https://example.com").unwrap();
        let js = "fetch('/api/users').then(r => r.json())";
        let urls = extract_js(js, &base);
        assert!(urls.iter().any(|u| u.as_str().contains("/api/users")));
    }

    #[test]
    fn test_axios_url() {
        let base = Url::parse("https://example.com").unwrap();
        let js = "axios.get('https://api.example.com/v2/users')";
        let urls = extract_js(js, &base);
        assert!(urls.iter().any(|u| u.as_str().contains("api.example.com")));
    }

    #[test]
    fn test_websocket_url() {
        let base = Url::parse("https://example.com").unwrap();
        let js = "new WebSocket('wss://ws.example.com/chat')";
        let urls = extract_js(js, &base);
        assert!(urls.iter().any(|u| u.as_str().contains("ws.example.com")));
    }

    #[test]
    fn test_http_string_literal() {
        let base = Url::parse("https://example.com").unwrap();
        let js = r#"const URL = "https://api.example.com/v3";"#;
        let urls = extract_js(js, &base);
        assert!(urls.iter().any(|u| u.as_str().contains("api.example.com/v3")));
    }
}
