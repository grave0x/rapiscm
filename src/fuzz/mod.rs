pub mod matcher;
pub mod runner;
pub mod wordlist;

use crate::config::ScanConfig;
use crate::error::Result;
use crate::fuzz::matcher::{Baseline, MatchConfig, parse_range_list};
use crate::fuzz::runner::FuzzRunner;
use crate::types::ResponseResult;

/// Run a fuzz scan: load wordlist, run fuzzer, return matched results.
pub async fn run_fuzz_scan(
    config: &ScanConfig,
    fuzz_args: &super::cli::Command,
) -> Result<Vec<ResponseResult>> {
    let base_url = match &config.target {
        crate::types::Target::Url(u) => u.clone(),
        _ => unreachable!(),
    };

    // Extract fuzz-specific args from the Command
    let (wordlist_path, mc, fc, ms, fs, mr, fr, ac) = match fuzz_args {
        super::cli::Command::Fuzz {
            wordlist,
            mc,
            fc,
            ms,
            fs,
            mr,
            fr,
            ac,
            ..
        } => (wordlist, mc, fc, ms, fs, mr, fr, *ac),
        _ => unreachable!(),
    };

    // Load wordlist
    let words: Vec<String> = if let Some(path) = wordlist_path {
        match std::fs::read_to_string(path) {
            Ok(content) => content
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty() && !l.starts_with('#'))
                .collect(),
            Err(e) => {
                tracing::warn!("failed to read wordlist: {e}, using built-in");
                wordlist::api_paths()
                    .iter()
                    .map(|s| s.to_string())
                    .collect()
            }
        }
    } else {
        wordlist::api_paths()
            .iter()
            .map(|s| s.to_string())
            .collect()
    };

    // Build matcher config
    let matcher = MatchConfig {
        match_status: mc.as_ref().map(|s| parse_range_list(s)).unwrap_or_default(),
        filter_status: fc.as_ref().map(|s| parse_range_list(s)).unwrap_or_default(),
        match_size: ms.as_ref().map(|s| parse_range_list(s)).unwrap_or_default(),
        filter_size: fs.as_ref().map(|s| parse_range_list(s)).unwrap_or_default(),
        match_regex: mr.clone(),
        filter_regex: fr.clone(),
        baseline: if ac {
            Some(Baseline {
                status: 404,
                size: 50,
            }) // placeholder, real auto-calibration probes first
        } else {
            None
        },
    };

    let runner = FuzzRunner::new(config)?;
    let results = runner.fuzz_paths(&base_url, &words, &matcher).await;
    tracing::info!("fuzz matched {} results", results.len());
    Ok(results)
}
