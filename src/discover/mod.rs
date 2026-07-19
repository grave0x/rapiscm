//! Domain discovery engine for company/organization reconnaissance.
//!
//! Orchestrates 6 OSINT sources to find domains associated with an org.
//!
//! ## Sources
//! - 1: crt.sh — Certificate Transparency logs (always on)
//! - 2: RDAP — Reverse WHOIS / org entity search (always on)
//! - 3: ASN — ASN lookup → IP ranges → reverse DNS (always on)
//! - 4: Google Search — dork-based domain discovery (API key needed)
//! - 5: GA-ID — Google Analytics ID pivot (API key needed, stub)
//! - 6: Shodan — org-name search (API key needed, working)

pub mod asn;
pub mod crtsh;
pub mod favicon;
pub mod gaid;
pub mod rdap;
pub mod search;

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

use crate::error::Result;
use crate::types::{ApiKeys, DiscoveredDomain};

/// Configuration for a discovery run.
#[derive(Debug, Clone)]
pub struct DiscoverConfig {
    /// Organization name to search for.
    pub org_name: String,
    /// API keys for gated sources.
    pub api_keys: ApiKeys,
}

/// Run all available discovery sources and merge results into deduplicated domains.
pub async fn run_discover(config: &DiscoverConfig) -> Result<Vec<DiscoveredDomain>> {
    let mut merged: HashMap<String, DiscoveredDomain> = HashMap::new();
    let org = &config.org_name;

    // ── Source 1: crt.sh (always on) ──
    tracing::info!("[discover] Querying crt.sh for \"{org}\"...");
    match crtsh::query_crtsh(org).await {
        Ok(entries) => {
            for entry in entries {
                let d = merged
                    .entry(entry.domain.clone())
                    .or_insert_with(|| empty_domain(entry.domain.clone()));
                d.sources.push("crtsh".into());
                for s in &entry.subjects {
                    if !d.cert_subjects.contains(s) {
                        d.cert_subjects.push(s.clone());
                    }
                }
            }
        }
        Err(e) => tracing::warn!("[discover] crt.sh query failed: {e}"),
    }

    // ── Source 2: RDAP (always on) ──
    tracing::info!("[discover] Querying RDAP for \"{org}\"...");
    match rdap::rdap_discover(org).await {
        Ok(rdap_domains) => {
            for domain in &rdap_domains {
                let d = merged
                    .entry(domain.clone())
                    .or_insert_with(|| empty_domain(domain.clone()));
                if !d.sources.contains(&"rdap".into()) {
                    d.sources.push("rdap".into());
                }
                d.org_name.get_or_insert_with(|| org.clone());
            }
        }
        Err(e) => tracing::warn!("[discover] RDAP query failed: {e}"),
    }

    // ── Source 3: ASN chain (always on) ──
    tracing::info!("[discover] Resolving ASN data for \"{org}\"...");
    match asn::asn_discover(org).await {
        Ok(asn_results) => {
            for ar in asn_results {
                let d = merged
                    .entry(ar.domain.clone())
                    .or_insert_with(|| empty_domain(ar.domain.clone()));
                if !d.sources.contains(&"asn".into()) {
                    d.sources.push("asn".into());
                }
                if d.asn.is_none() {
                    d.asn = Some(ar.asn);
                }
                if d.asn_org.is_none() {
                    d.asn_org = Some(ar.asn_org.clone());
                }
                for cidr in &ar.ip_ranges {
                    if !d.ip_ranges.contains(cidr) {
                        d.ip_ranges.push(cidr.clone());
                    }
                }
            }
        }
        Err(e) => tracing::warn!("[discover] ASN query failed: {e}"),
    }

    // ── Source 4: Google dork search (gated) ──
    if let (Some(api_key), Some(cx)) = (&config.api_keys.google_api_key, &config.api_keys.google_cx)
    {
        tracing::info!("[discover] Google search for \"{org}\"...");
        match search::google_search(org, api_key, cx).await {
            Ok(domains) => {
                for domain in domains {
                    let d = merged
                        .entry(domain.clone())
                        .or_insert_with(|| empty_domain(domain.clone()));
                    if !d.sources.contains(&"search".into()) {
                        d.sources.push("search".into());
                    }
                }
            }
            Err(e) => tracing::warn!("[discover] Google search failed: {e}"),
        }
    } else {
        tracing::info!("[discover] Skipping Google search (no API key)");
    }

    // ── Source 5: GA-ID pivot (gated, stub) ──
    if config.api_keys.google_api_key.is_some() {
        tracing::info!("[discover] GA-ID pivot for \"{org}\"...");
        match gaid::gaid_pivot(org).await {
            Ok(domains) => {
                for domain in domains {
                    let d = merged
                        .entry(domain.clone())
                        .or_insert_with(|| empty_domain(domain.clone()));
                    if !d.sources.contains(&"gaid".into()) {
                        d.sources.push("gaid".into());
                    }
                }
            }
            Err(e) => tracing::warn!("[discover] GA-ID pivot failed: {e}"),
        }
    } else {
        tracing::info!("[discover] Skipping GA-ID pivot (no API key)");
    }

    // ── Source 6: Shodan favicon (gated, stub) ──
    if let Some(shodan_key) = &config.api_keys.shodan_api_key {
        tracing::info!("[discover] Shodan favicon search for \"{org}\"...");
        match favicon::shodan_favicon(org, shodan_key).await {
            Ok(domains) => {
                for domain in domains {
                    let d = merged
                        .entry(domain.clone())
                        .or_insert_with(|| empty_domain(domain.clone()));
                    if !d.sources.contains(&"favicon".into()) {
                        d.sources.push("favicon".into());
                    }
                }
            }
            Err(e) => tracing::warn!("[discover] Shodan favicon search failed: {e}"),
        }
    } else {
        tracing::info!("[discover] Skipping Shodan favicon search (no API key)");
    }

    // Finalize: dedup sources per domain, sort
    let mut results: Vec<DiscoveredDomain> = merged.into_values().collect();
    for d in &mut results {
        d.sources.sort();
        d.sources.dedup();
    }
    results.sort_by(|a, b| a.domain.cmp(&b.domain));

    Ok(results)
}

/// Save discovery report to `reports/corps/<org>/`.
pub fn save_report(domains: &[DiscoveredDomain], org: &str) -> Result<()> {
    let dir: PathBuf = ["reports", "corps", org].iter().collect();
    fs::create_dir_all(&dir)?;

    let json =
        serde_json::to_string_pretty(domains).map_err(|e| crate::error::Error::DiscoveryParse {
            src: "serialize",
            detail: e.to_string(),
        })?;
    fs::write(dir.join("discovery.json"), &json)?;

    let lines: Vec<&str> = domains.iter().map(|d| d.domain.as_str()).collect();
    fs::write(dir.join("domains.txt"), lines.join("\n"))?;

    tracing::info!("Discovery report saved to {}", dir.display());
    Ok(())
}

fn empty_domain(domain: String) -> DiscoveredDomain {
    DiscoveredDomain {
        domain,
        sources: Vec::new(),
        cert_subjects: Vec::new(),
        asn: None,
        asn_org: None,
        ip_ranges: Vec::new(),
        org_name: None,
    }
}
