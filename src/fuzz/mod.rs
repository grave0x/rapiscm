//! Fuzzing engine: wordlist-driven endpoint discovery.

pub mod matcher;
pub mod runner;
pub mod wordlist;

use std::path::PathBuf;

use crate::config::ScanConfig;
use crate::fuzz::matcher::{Baseline, MatchConfig, parse_range_list};
use crate::fuzz::runner::FuzzRunner;

/// Fuzzer-specific options passed from CLI.
pub struct FuzzOpts {
    pub wordlist: Option<String>,
    pub mc: Option<String>,
    pub fc: Option<String>,
    pub ms: Option<String>,
    pub fs: Option<String>,
    pub mr: Option<String>,
    pub fr: Option<String>,
    pub ac: bool,
    pub mode: String,
    #[allow(dead_code)]
    pub wordlist_mode: String,
    pub keyword: String,
    pub request: Option<PathBuf>,
}

/// Run a fuzz scan: load wordlist, build matcher, run fuzzer, print results.
pub async fn run_fuzz_scan(
    base_url: &reqwest::Url,
    opts: &FuzzOpts,
    scan_config: &ScanConfig,
) -> anyhow::Result<()> {
    let words = load_words(&opts.wordlist);
    let matcher = MatchConfig {
        match_status: opts
            .mc
            .as_ref()
            .map(|s| parse_range_list(s))
            .unwrap_or_default(),
        filter_status: opts
            .fc
            .as_ref()
            .map(|s| parse_range_list(s))
            .unwrap_or_default(),
        match_size: opts
            .ms
            .as_ref()
            .map(|s| parse_range_list(s))
            .unwrap_or_default(),
        filter_size: opts
            .fs
            .as_ref()
            .map(|s| parse_range_list(s))
            .unwrap_or_default(),
        match_regex: opts.mr.clone(),
        filter_regex: opts.fr.clone(),
        baseline: if opts.ac {
            Some(Baseline {
                status: 404,
                size: 50,
            })
        } else {
            None
        },
    };
    let runner = FuzzRunner::new(scan_config)?;
    let keyword = if opts.keyword.is_empty() {
        "FUZZ"
    } else {
        &opts.keyword
    };

    let results = match opts.mode.as_str() {
        "param" => runner.fuzz_params(base_url, &words, &matcher).await,
        "method" => runner.fuzz_methods(base_url, &words, &matcher).await,
        "header" => {
            runner
                .fuzz_headers(base_url, &words, &matcher, keyword)
                .await
        }
        "body" => {
            runner
                .fuzz_bodies(base_url, &words, &matcher, keyword, opts.request.as_deref())
                .await
        }
        _ => runner.fuzz_paths(base_url, &words, &matcher).await,
    };

    if !results.is_empty() {
        println!(
            "{}",
            crate::report::format_results(&results, crate::types::OutputFormat::Table)
        );
    } else {
        println!("No matches found.");
    }
    Ok(())
}

fn load_words(wordlist_path: &Option<String>) -> Vec<String> {
    if let Some(path) = wordlist_path {
        match std::fs::read_to_string(path) {
            Ok(c) => c
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .collect(),
            Err(e) => {
                tracing::warn!("wordlist read failed: {e}, using built-in");
                builtin_words()
            }
        }
    } else {
        builtin_words()
    }
}

fn builtin_words() -> Vec<String> {
    wordlist::api_paths()
        .iter()
        .map(|s| s.to_string())
        .collect()
}
