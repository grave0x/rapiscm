//! ASN → IP ranges → Reverse DNS discovery chain.
//!
//! Finds ASNs matching an organization name, gets their announced IP prefixes,
//! then performs reverse DNS lookups to discover domains.
//!
//! Uses the BGPView API (free, no auth) for ASN data:
//!   GET https://api.bgpview.io/asn/{asn}/prefixes

use crate::error::{Error, Result};
use serde::Deserialize;
use std::net::Ipv4Addr;

#[derive(Debug, Clone)]
pub struct AsnResult {
    pub domain: String,
    pub asn: u32,
    pub asn_org: String,
    pub ip_ranges: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct BgpViewSearch {
    #[serde(default)]
    data: BgpViewSearchData,
}

#[derive(Debug, Deserialize, Default)]
struct BgpViewSearchData {
    #[serde(default)]
    asns: Vec<BgpViewAsn>,
}

#[derive(Debug, Deserialize)]
struct BgpViewAsn {
    #[serde(default)]
    asn: u32,
    #[serde(default)]
    name: String,
    #[serde(default)]
    description: String,
}

#[derive(Debug, Deserialize, Default)]
struct BgpViewPrefixes {
    #[serde(default)]
    data: BgpViewPrefixData,
}

#[derive(Debug, Deserialize, Default)]
struct BgpViewPrefixData {
    #[serde(default)]
    ipv4_prefixes: Vec<BgpViewPrefix>,
    #[serde(default)]
    ipv6_prefixes: Vec<BgpViewPrefix>,
}

#[derive(Debug, Deserialize)]
struct BgpViewPrefix {
    #[serde(default)]
    prefix: String,
    #[serde(default)]
    #[expect(dead_code)]
    description: String,
}

/// Search for ASNs matching the org name and return domains found via
/// reverse DNS of their IP ranges.
pub async fn asn_discover(org: &str) -> Result<Vec<AsnResult>> {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .user_agent("rapiscm/0.1")
        .build()
        .map_err(|e| Error::DiscoveryHttp {
            src: "asn",
            detail: e.to_string(),
        })?;

    // Search for matching ASNs
    let search_url = format!(
        "https://api.bgpview.io/search?query_term={}",
        urlencoding(org)
    );

    let resp = client
        .get(&search_url)
        .send()
        .await
        .map_err(|e| Error::DiscoveryHttp {
            src: "asn",
            detail: format!("search: {e}"),
        })?;

    if !resp.status().is_success() {
        return Ok(Vec::new());
    }

    let search: BgpViewSearch = resp.json().await.map_err(|e| Error::DiscoveryParse {
        src: "asn",
        detail: format!("search JSON: {e}"),
    })?;

    let org_lower = org.to_lowercase();
    let mut results: Vec<AsnResult> = Vec::new();

    for asn_info in &search.data.asns {
        let name_lower = asn_info.name.to_lowercase();
        let desc_lower = asn_info.description.to_lowercase();

        // Only consider ASNs whose name or description matches the org
        if !name_lower.contains(&org_lower) && !desc_lower.contains(&org_lower) {
            continue;
        }

        // Fetch prefixes for this ASN
        let prefixes_url = format!("https://api.bgpview.io/asn/{}/prefixes", asn_info.asn);
        let p_resp = client.get(&prefixes_url).send().await;

        let prefixes: BgpViewPrefixes = match p_resp {
            Ok(r) if r.status().is_success() => r.json().await.unwrap_or_default(),
            _ => continue,
        };

        let mut cidrs: Vec<String> = Vec::new();
        cidrs.extend(prefixes.data.ipv4_prefixes.iter().map(|p| p.prefix.clone()));
        cidrs.extend(prefixes.data.ipv6_prefixes.iter().map(|p| p.prefix.clone()));

        // Try reverse DNS on a sample of IPs from each prefix (max 5 per prefix)
        for cidr in &cidrs {
            if let Some(domains) = sample_rdns(&client, cidr).await {
                for domain in domains {
                    results.push(AsnResult {
                        domain,
                        asn: asn_info.asn,
                        asn_org: asn_info.name.clone(),
                        ip_ranges: cidrs.clone(),
                    });
                }
            }
        }
    }

    results.sort_by(|a, b| a.domain.cmp(&b.domain));
    results.dedup_by(|a, b| a.domain == b.domain);

    Ok(results)
}

/// Try reverse DNS on up to 3 IPs from a CIDR range.
/// Returns Some(domains) if any PTR records found.
async fn sample_rdns(client: &reqwest::Client, cidr: &str) -> Option<Vec<String>> {
    let ips = sample_ips(cidr, 3)?;
    let mut domains: Vec<String> = Vec::new();

    for ip in &ips {
        // Use dns.google REST API for reverse DNS
        let ptr_name = ip.split('.').rev().collect::<Vec<_>>().join(".") + ".in-addr.arpa";
        let url = format!(
            "https://dns.google/resolve?name={}&type=PTR",
            urlencoding(&ptr_name)
        );

        if let Ok(resp) = client.get(&url).send().await
            && let Ok(body) = resp.json::<serde_json::Value>().await
            && let Some(answer) = body.get("Answer").and_then(|a| a.as_array())
        {
            for ans in answer {
                if let Some(data) = ans.get("data").and_then(|d| d.as_str()) {
                    let domain = data.trim_end_matches('.').to_lowercase();
                    if !domains.contains(&domain) {
                        domains.push(domain);
                    }
                }
            }
        }
    }

    if domains.is_empty() {
        None
    } else {
        Some(domains)
    }
}

/// Sample up to `n` IP addresses from a CIDR range (just the first few).
fn sample_ips(cidr: &str, n: usize) -> Option<Vec<String>> {
    let (ip_str, bits_str) = cidr.split_once('/')?;
    let bits: u8 = bits_str.parse().ok()?;
    let ip: Ipv4Addr = ip_str.parse().ok()?;
    let ip_u32 = u32::from(ip);

    // Number of usable addresses
    let host_bits = 32u32.saturating_sub(bits as u32);
    let count = (1u32 << host_bits).saturating_sub(2); // exclude network & broadcast
    if count == 0 {
        return None;
    }

    let sample_count = std::cmp::min(n as u32, count);
    let mut result = Vec::with_capacity(sample_count as usize);
    for i in 1..=sample_count {
        let addr = Ipv4Addr::from(ip_u32 + i);
        result.push(addr.to_string());
    }
    Some(result)
}

fn urlencoding(s: &str) -> String {
    s.replace('%', "%25")
        .replace(' ', "%20")
        .replace('&', "%26")
        .replace('=', "%3D")
        .replace('+', "%2B")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_ips_small_range() {
        let ips = sample_ips("192.168.1.0/30", 3).unwrap();
        // /30 = 2 usable: 192.168.1.1, 192.168.1.2
        assert_eq!(ips.len(), 2);
        assert_eq!(ips[0], "192.168.1.1");
        assert_eq!(ips[1], "192.168.1.2");
    }

    #[test]
    fn test_sample_ips_large_range() {
        let ips = sample_ips("10.0.0.0/24", 3).unwrap();
        assert_eq!(ips.len(), 3);
        assert_eq!(ips[0], "10.0.0.1");
        assert_eq!(ips[1], "10.0.0.2");
    }

    #[test]
    fn test_sample_ips_invalid() {
        assert!(sample_ips("not-a-cidr", 3).is_none());
    }
}
