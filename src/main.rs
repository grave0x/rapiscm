mod check;
mod cli;
mod config;
mod error;
mod extract;
mod fuzz;
mod parser;
mod report;
mod scan;
mod tag;
mod types;

use clap::Parser;
use tracing_subscriber::EnvFilter;

fn init_logging(level: &str, filters: &[String], format: &str) {
    let mut filter_builder = EnvFilter::builder();
    for f in filters {
        filter_builder =
            filter_builder.with_default_directive(f.parse().expect("invalid log filter directive"));
    }
    let filter = filter_builder
        .with_default_directive(level.parse().unwrap_or(tracing::Level::INFO.into()))
        .from_env_lossy();
    let _ = match format {
        "json" => tracing_subscriber::fmt()
            .json()
            .with_env_filter(filter)
            .try_init(),
        _ => tracing_subscriber::fmt()
            .without_time()
            .with_env_filter(filter)
            .try_init(),
    };
}

fn get_global(cli: &cli::Cli) -> &cli::GlobalArgs {
    match &cli.command {
        cli::Command::Spec { global, .. } => global,
        cli::Command::Url { global, .. } => global,
        cli::Command::Scan { global, .. } => global,
        cli::Command::Fuzz { global, .. } => global,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let g = get_global(&cli);
    init_logging(&g.log_level, &g.log_filter, &g.log_format);

    match &cli.command {
        cli::Command::Fuzz {
            target,
            wordlist,
            mc,
            fc,
            ms,
            fs,
            mr,
            fr,
            ac,
            ..
        } => {
            let base_url = reqwest::Url::parse(target)
                .or_else(|_| reqwest::Url::parse(&format!("https://{target}")))
                .map_err(|e| anyhow::anyhow!("invalid fuzz target: {e}"))?;

            let words: Vec<String> = if let Some(path) = wordlist {
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
            };

            let matcher = fuzz::matcher::MatchConfig {
                match_status: mc
                    .as_ref()
                    .map(|s| fuzz::matcher::parse_range_list(s))
                    .unwrap_or_default(),
                filter_status: fc
                    .as_ref()
                    .map(|s| fuzz::matcher::parse_range_list(s))
                    .unwrap_or_default(),
                match_size: ms
                    .as_ref()
                    .map(|s| fuzz::matcher::parse_range_list(s))
                    .unwrap_or_default(),
                filter_size: fs
                    .as_ref()
                    .map(|s| fuzz::matcher::parse_range_list(s))
                    .unwrap_or_default(),
                match_regex: mr.clone(),
                filter_regex: fr.clone(),
                baseline: if *ac {
                    Some(fuzz::matcher::Baseline {
                        status: 404,
                        size: 50,
                    })
                } else {
                    None
                },
            };

            let scan_config =
                config::ScanConfig::from_cli_global(g, types::Target::Url(base_url.clone()))?;

            let runner = fuzz::runner::FuzzRunner::new(&scan_config)?;
            let results = runner.fuzz_paths(&base_url, &words, &matcher).await;
            if !results.is_empty() {
                println!(
                    "{}",
                    report::format_results(&results, types::OutputFormat::Table)
                );
            } else {
                println!("No matches found.");
            }
        }
        cmd => {
            let config = config::ScanConfig::from_cli(cli::Cli {
                command: cmd.clone(),
            })?;
            let results = match &config.target {
                types::Target::Spec(_) => scan::spec::run_spec_scan(&config).await?,
                types::Target::Url(_) => scan::url::run_url_scan(&config).await?,
            };
            let output = report::format_results(&results, config.output);
            println!("{output}");
        }
    }

    Ok(())
}

fn builtin_words() -> Vec<String> {
    fuzz::wordlist::api_paths()
        .iter()
        .map(|s| s.to_string())
        .collect()
}
