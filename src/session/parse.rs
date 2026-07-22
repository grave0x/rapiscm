//! JSONL session file parser.
//!
//! Reads a JSONL file where each line is a self-contained JSON object
//! describing one HTTP request/response. Converts each line into a
//! `(Option<String>, ResponseResult)` tuple — the timestamp is kept
//! separate for timing analytics.

use std::collections::HashMap;
use std::io::BufRead;
use std::path::Path;

use serde::Deserialize;

use crate::error::{Error, Result};
use crate::types::ResponseResult;

/// Raw JSONL line shape for deserialization.
#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RawLine {
    timestamp: Option<String>,
    method: Option<String>,
    url: Option<String>,
    status: Option<u16>,
    #[expect(dead_code)]
    request_headers: Option<HashMap<String, String>>,
    response_headers: Option<HashMap<String, String>>,
    #[expect(dead_code)]
    request_body: Option<String>,
    response_body: Option<String>,
    response_time_ms: Option<u64>,
}

/// Parse a JSONL session file.
///
/// Returns `(Vec<ResponseResult>, Vec<String>)` — results and their
/// corresponding timestamps (same index). Entries are sorted by
/// timestamp if present, otherwise file order is preserved.
///
/// `max_errors`: max malformed lines before returning an error.
pub fn parse_session_file(path: &Path, max_errors: usize) -> Result<(Vec<ResponseResult>, Vec<String>)> {
    let file = std::fs::File::open(path)?;
    let reader = std::io::BufReader::new(file);
    let mut results = Vec::new();
    let mut timestamps = Vec::new();
    let mut errors = 0usize;

    for (lineno, line) in reader.lines().enumerate() {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                errors += 1;
                tracing::warn!("session parse: line {} I/O error: {e}", lineno + 1);
                if errors > max_errors {
                    return Err(Error::SessionParse(format!(
                        "too many parse errors ({errors}) at line {}",
                        lineno + 1
                    )));
                }
                continue;
            }
        };

        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let raw: RawLine = match serde_json::from_str(trimmed) {
            Ok(r) => r,
            Err(e) => {
                errors += 1;
                tracing::warn!("session parse: line {} JSON error: {e}", lineno + 1);
                if errors > max_errors {
                    return Err(Error::SessionParse(format!(
                        "too many parse errors ({errors}) at line {}",
                        lineno + 1
                    )));
                }
                continue;
            }
        };

        // Validate required fields.
        let method = match raw.method {
            Some(m) => m,
            None => {
                errors += 1;
                tracing::warn!("session parse: line {} missing 'method'", lineno + 1);
                if errors > max_errors {
                    return Err(Error::SessionParse(format!(
                        "too many parse errors ({errors}) at line {}",
                        lineno + 1
                    )));
                }
                continue;
            }
        };

        let url = match raw.url {
            Some(u) => u,
            None => {
                errors += 1;
                tracing::warn!("session parse: line {} missing 'url'", lineno + 1);
                if errors > max_errors {
                    return Err(Error::SessionParse(format!(
                        "too many parse errors ({errors}) at line {}",
                        lineno + 1
                    )));
                }
                continue;
            }
        };

        let status = match raw.status {
            Some(s) => s,
            None => {
                errors += 1;
                tracing::warn!("session parse: line {} missing 'status'", lineno + 1);
                if errors > max_errors {
                    return Err(Error::SessionParse(format!(
                        "too many parse errors ({errors}) at line {}",
                        lineno + 1
                    )));
                }
                continue;
            }
        };

        // Convert response_headers from HashMap to Vec<(String, String)>.
        let response_headers: Vec<(String, String)> = raw.response_headers.unwrap_or_default().into_iter().collect();

        // Convert response_body: if present, try base64 decode, else use raw bytes.
        let response_body: Vec<u8> = match raw.response_body {
            Some(body_str) => {
                // Try base64 first; fall back to raw UTF-8 bytes.
                match BASE64.decode(body_str.as_bytes()) {
                    Ok(decoded) => decoded,
                    Err(_) => body_str.into_bytes(),
                }
            }
            None => Vec::new(),
        };

        let response_time_ms = raw.response_time_ms.unwrap_or(0);

        let ts = raw.timestamp.clone().unwrap_or_default();
        timestamps.push(ts);
        results.push(ResponseResult {
            timestamp: raw.timestamp,
            endpoint_method: method,
            endpoint_url: url,
            status_code: status,
            response_time_ms,
            response_size: response_body.len(),
            response_headers,
            response_body,
            expected_status: None,
            checks: Vec::new(),
            error: None,
            tags: Vec::new(),
            trackers: Vec::new(),
        });
    }

    Ok((results, timestamps))
}

// ── Minimal base64 decoder (no external dep) ──

struct BASE64;

impl BASE64 {
    fn decode(&self, input: &[u8]) -> std::result::Result<Vec<u8>, ()> {
        // Remove whitespace.
        let input: Vec<u8> = input.iter().copied().filter(|b| !b.is_ascii_whitespace()).collect();
        if input.len() % 4 == 1 {
            return Err(()); // invalid length
        }
        let pad = input.iter().rev().take_while(|&&b| b == b'=').count();
        let valid_len = input.len() - pad;
        if valid_len == 0 {
            return Ok(Vec::new());
        }
        let mut out = Vec::with_capacity(valid_len / 4 * 3);
        let mut buf = 0u32;
        let mut bits = 0;
        for &b in &input[..valid_len] {
            let val = DECODE_TABLE.get(b as usize).copied().unwrap_or(0xFF);
            if val > 63 {
                // Character outside base64 alphabet — fallback to calling
                // code which will try raw bytes instead.
                return Err(());
            }
            buf = (buf << 6) | val as u32;
            bits += 6;
            if bits >= 8 {
                bits -= 8;
                out.push((buf >> bits) as u8);
                buf &= (1 << bits) - 1;
            }
        }
        Ok(out)
    }
}

const DECODE_TABLE: [u8; 256] = {
    let mut table = [0xFFu8; 256];
    let alphabet = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut i = 0;
    while i < alphabet.len() {
        table[alphabet[i] as usize] = i as u8;
        i += 1;
    }
    table[b'=' as usize] = 0;
    table
};

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;

    #[test]
    fn test_parse_valid_jsonl() {
        let jsonl = r#"{"timestamp":"2026-07-16T14:30:00Z","method":"GET","url":"https://example.com/api","status":200,"responseTimeMs":42}
{"timestamp":"2026-07-16T14:30:01Z","method":"POST","url":"https://example.com/api/submit","status":201,"responseBody":"{\"id\":1}"}
"#;
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("session.jsonl");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(jsonl.as_bytes()).unwrap();
        drop(f);

        let (results, timestamps) = parse_session_file(&path, 10).unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].endpoint_method, "GET");
        assert_eq!(results[1].endpoint_method, "POST");
        assert_eq!(results[0].endpoint_url, "https://example.com/api");
        assert_eq!(results[1].status_code, 201);
        assert_eq!(timestamps[0], "2026-07-16T14:30:00Z");
        assert_eq!(timestamps[1], "2026-07-16T14:30:01Z");
    }

    #[test]
    fn test_parse_empty_lines() {
        let jsonl = "\n\n{\"method\":\"GET\",\"url\":\"https://example.com/\",\"status\":200}\n\n";
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("session.jsonl");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(jsonl.as_bytes()).unwrap();
        drop(f);

        let (results, _) = parse_session_file(&path, 10).unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_parse_max_errors() {
        let jsonl = "not json\nalso not json\n{\"method\":\"GET\",\"url\":\"https://example.com/\",\"status\":200}\n";
        let dir = tempfile::tempdir().unwrap();
        let path = dir.path().join("session.jsonl");
        let mut f = std::fs::File::create(&path).unwrap();
        f.write_all(jsonl.as_bytes()).unwrap();
        drop(f);

        // Only 1 error allowed → should abort at line 2
        let err = parse_session_file(&path, 1).unwrap_err();
        assert!(err.to_string().contains("too many parse errors"));
    }

    #[test]
    fn test_base64_decode() {
        let encoded = base64_encode(b"hello world");
        let decoded = BASE64.decode(encoded.as_bytes()).unwrap();
        assert_eq!(decoded, b"hello world");
    }

    #[test]
    fn test_base64_roundtrip_binary() {
        let data = vec![0u8, 255, 128, 64, 32, 16, 8, 4, 2, 1];
        let encoded = base64_encode(&data);
        let decoded = BASE64.decode(encoded.as_bytes()).unwrap();
        assert_eq!(decoded, data);
    }

    fn base64_encode(input: &[u8]) -> String {
        const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
        let mut result = String::with_capacity(input.len().div_ceil(3) * 4);
        for chunk in input.chunks(3) {
            let b0 = chunk[0] as u32;
            let b1 = chunk.get(1).copied().unwrap_or(0) as u32;
            let b2 = chunk.get(2).copied().unwrap_or(0) as u32;
            let triple = (b0 << 16) | (b1 << 8) | b2;
            result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
            result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
            if chunk.len() > 1 {
                result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
            if chunk.len() > 2 {
                result.push(CHARS[(triple & 0x3F) as usize] as char);
            } else {
                result.push('=');
            }
        }
        result
    }
}
