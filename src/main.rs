mod analytics;
mod check;
mod cli;
mod config;
mod discover;
mod error;
mod extract;
mod filter;
mod fuzz;
mod parser;
mod report;
mod scan;
mod session;
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
        cli::Command::Corp { global, .. } => global,
        cli::Command::Fuzz { global, .. } => global,
        cli::Command::Session { global, .. } => global,
    }
}

/// Extract the target string from a command (for auto-detect org from URL).
fn get_target_str(cmd: &cli::Command) -> Option<&str> {
    match cmd {
        cli::Command::Spec { file, .. } => file.to_str(),
        cli::Command::Url { url, .. } => Some(url.as_str()),
        cli::Command::Scan { target, .. } => Some(target.as_str()),
        _ => None,
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = cli::Cli::parse();
    let g = get_global(&cli);
    init_logging(&g.log_level, &g.log_filter, &g.log_format);

    // ── Session subcommand: replay mode ──
    if let cli::Command::Session {
        file,
        timing,
        max_parse_errors,
        skip_cors,
        skip_auth,
        global,
    } = &cli.command
    {
        let cfg = session::SessionConfig {
            file: file.clone(),
            timing: *timing,
            max_parse_errors: *max_parse_errors,
            skip_cors: *skip_cors,
            skip_auth: *skip_auth,
            output: config::parse_output(&global.output)
                .map_err(|e| anyhow::anyhow!("output format: {e}"))?,
        };
        session::run_session(&cfg)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        return Ok(());
    }

    // ── Corp subcommand: discovery-only mode ──
    if let cli::Command::Corp { name, .. } = &cli.command {
        let keys = config::load_config();
        let disc_config = discover::DiscoverConfig {
            org_name: name.clone(),
            api_keys: keys,
        };
        let domains = discover::run_discover(&disc_config).await?;
        let json = serde_json::to_string_pretty(&domains)
            .map_err(|e| anyhow::anyhow!("serialize: {e}"))?;
        println!("{json}");
        discover::save_report(&domains, name)?;
        tracing::info!("Found {} domains for {name}", domains.len());
        return Ok(());
    }

    // ── --corp flag: run discovery before scan, save report ──
    if let Some(org_val) = &g.corp {
        let keys = config::load_config();
        let org_name = if org_val.is_empty() {
            // Auto-detect org name from target URL
            let target_str = get_target_str(&cli.command).unwrap_or("unknown");
            discover::crtsh::auto_detect_org(target_str).await?
        } else {
            org_val.clone()
        };
        let disc_config = discover::DiscoverConfig {
            org_name: org_name.clone(),
            api_keys: keys,
        };
        let domains = discover::run_discover(&disc_config).await?;
        discover::save_report(&domains, &org_name)?;
        tracing::info!("Discovered {} domains for {org_name}", domains.len());
    }

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
            let scan_config =
                config::ScanConfig::from_cli_global(g, types::Target::Url(base_url.clone()))?;
            let opts = fuzz::FuzzOpts {
                wordlist: wordlist.clone(),
                mc: mc.clone(),
                fc: fc.clone(),
                ms: ms.clone(),
                fs: fs.clone(),
                mr: mr.clone(),
                fr: fr.clone(),
                ac: *ac,
            };
            fuzz::run_fuzz_scan(&base_url, &opts, &scan_config).await?;
        }
        cli::Command::Corp { .. } => {
            unreachable!("corp mode handled above")
        }
        cli::Command::Session { .. } => {
            unreachable!("session mode handled above")
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
