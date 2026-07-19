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
mod task;
mod types;
mod util;

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
        cli::Command::Tasks { global, .. } => global,
    }
}

/// Extract the target string from a command (for auto-detect org from URL).
fn get_target_str(cmd: &cli::Command) -> Option<&str> {
    match cmd {
        cli::Command::Spec { file, .. } => file.to_str(),
        cli::Command::Url { url, .. } => Some(url.as_str()),
        cli::Command::Scan { target, .. } => Some(target.as_str()),
        cli::Command::Tasks { .. } => None,
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

    // ── Tasks subcommand ──
    if let cli::Command::Tasks { action, .. } = &cli.command {
        use cli::TasksAction;

        fn e(s: String) -> anyhow::Error {
            anyhow::anyhow!("task error: {s}")
        }

        let storage = task::TaskStorage::new(None);
        match action {
            TasksAction::List => {
                let tasks = storage.list();
                if tasks.is_empty() {
                    println!("No tasks found.");
                } else {
                    println!(
                        "{:<6} {:<30} {:<20} {:<8} Target",
                        "ID", "Name", "Created", ""
                    );
                    println!("{}", "-".repeat(90));
                    for t in &tasks {
                        println!(
                            "{:<6} {:<30} {:<20} {:<8} {}",
                            t.task_id, t.task_name, t.created_at, "", t.target
                        );
                    }
                }
                return Ok(());
            }
            TasksAction::Show { id } => {
                let meta = storage.load_meta(*id).map_err(e)?;
                println!("Task {}", meta.task_id);
                println!("  Name:       {}", meta.task_name);
                println!("  Tags:       {}", meta.task_tags.join(", "));
                println!("  Created:    {}", meta.created_at);
                println!("  Target:     {}", meta.target);
                println!("  Command:    {}", meta.command);
                println!("  Endpoints:  {}", meta.endpoint_count);
                println!("  Exit code:  {}", meta.exit_code);
                let s = &meta.result_summary;
                println!("  Total:      {}", s.total);
                println!("  Successful: {}", s.successful);
                println!("  Failed:     {}", s.failed);
                println!("  Errors:     {}", s.errors);
                println!(
                    "  Checks:     {}+{}P {}W",
                    s.checks_passed, s.checks_failed, s.checks_warn
                );
                println!("  p50:        {}ms", s.p50_ms);
                return Ok(());
            }
            TasksAction::Delete { id } => {
                storage.delete(*id).map_err(e)?;
                println!("Deleted task {id}");
                return Ok(());
            }
            TasksAction::Prune { keep } => {
                let count = storage.prune(*keep).map_err(e)?;
                println!("Pruned {count} tasks, keeping {keep}");
                return Ok(());
            }
            TasksAction::Export { id, format, output } => {
                let fmt = match format.as_str() {
                    "md" | "markdown" => task::export::ExportFormat::Markdown,
                    "sarif" => task::export::ExportFormat::Sarif,
                    "html" => task::export::ExportFormat::Html,
                    _ => {
                        return Err(anyhow::anyhow!(
                            "unknown export format '{format}'. Use md, sarif, or html"
                        ));
                    }
                };
                let out_path = output; // output is PathBuf, already defined
                task::export::export(&storage, *id, fmt, out_path).map_err(e)?;
                println!("Exported task {id} to {out_path:?}");
                return Ok(());
            }
            TasksAction::Diff { old_id, new_id } => {
                let diff = task::diff_tasks(&storage, *old_id, *new_id).map_err(e)?;
                println!(
                    "Task {} vs {}: {} changed, {} added, {} removed",
                    diff.old_id,
                    diff.new_id,
                    diff.changed_count,
                    diff.added_count,
                    diff.removed_count
                );
                for ed in &diff.changes {
                    let label = match &ed.kind {
                        task::DiffKind::Identical => continue,
                        task::DiffKind::StatusChanged { old, new } => format!("status {old}→{new}"),
                        task::DiffKind::TimeChanged { old_ms, new_ms } => {
                            format!("time {old_ms}ms→{new_ms}ms")
                        }
                        task::DiffKind::NewCheck { check } => format!("+check {:?}", check),
                        task::DiffKind::RemovedCheck { check } => format!("-check {:?}", check),
                        task::DiffKind::CheckStatusChanged {
                            check_name,
                            old_passed,
                            new_passed,
                        } => {
                            format!(
                                "check {check_name} {}→{}",
                                if *old_passed { "PASS" } else { "FAIL" },
                                if *new_passed { "PASS" } else { "FAIL" }
                            )
                        }
                        task::DiffKind::BodySizeChanged { old, new } => {
                            format!("body {old}→{new} bytes")
                        }
                        task::DiffKind::ErrorStateChanged { .. } => "error state".into(),
                    };
                    println!("  {} {} — {}", ed.method, ed.url, label);
                }
                return Ok(());
            }
            TasksAction::Rebuild { .. } => {
                return Err(anyhow::anyhow!(
                    "rebuild not available from CLI; use programmatic API"
                ));
            }
        }
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

    // ── --resume <ID>: re-scan failed/incomplete endpoints from a saved task ──
    if let Some(task_id) = &g.resume {
        let storage = task::TaskStorage::new(g.task_dir.clone());
        let checkpoint = task::resume::load_checkpoint(&storage, *task_id)
            .map_err(|e| anyhow::anyhow!("Failed to load task {task_id} checkpoint: {e}"))?;
        let state = match checkpoint {
            Some(s) => s,
            None => {
                tracing::info!("Task {task_id} has no incomplete endpoints — scan complete");
                return Ok(());
            }
        };

        tracing::info!(
            "Resuming task {task_id}: {} remaining, {} already done",
            state.remaining.len(),
            state.skipped
        );

        let resume_config = config::ScanConfig::from_cli_global(
            g,
            types::Target::Url(reqwest::Url::parse("https://resume.local").expect("static URL")),
        )?;
        let runner = scan::runner::ScanRunner::new(&resume_config)?;
        let new_results = runner.run(state.remaining).await;

        // Merge: replace old failed results with new ones (matched by URL+method)
        let mut merged: Vec<types::ResponseResult> = state.existing_results;
        for new_r in new_results {
            let found = merged.iter_mut().find(|old| {
                old.endpoint_url == new_r.endpoint_url
                    && old.endpoint_method == new_r.endpoint_method
            });
            if let Some(old) = found {
                *old = new_r;
            } else {
                merged.push(new_r);
            }
        }

        // Re-save the task with merged results
        let meta = storage
            .load_meta(*task_id)
            .map_err(|e| anyhow::anyhow!("Failed to load task {task_id} meta: {e}"))?;
        let updated_meta = task::TaskMeta {
            result_summary: task::summarize(&merged),
            endpoint_count: merged.len(),
            duration_seconds: 0.0,
            ..meta
        };
        storage
            .save(
                &updated_meta,
                &merged,
                !resume_config.no_bodies,
                resume_config.raw,
            )
            .map_err(|e| anyhow::anyhow!("Failed to save resumed task: {e}"))?;
        task::resume::clear_checkpoint(&storage, *task_id);

        let output = report::format_results(&merged, resume_config.output);
        println!("{output}");
        return Ok(());
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

            // --save: persist scan results as a task
            if config.save {
                let storage = task::TaskStorage::new(config.task_dir.clone());
                let id = task::index::next_id(&storage.index_path());
                let now = util::now_iso();
                let cmd_str = match &config.target {
                    types::Target::Spec(_) => "spec",
                    types::Target::Url(_) => "url",
                };
                let summary = task::summarize(&results);
                let meta = task::TaskMeta {
                    task_id: id,
                    task_name: config
                        .task_name
                        .unwrap_or_else(|| format!("scan-{}", &now[..19])),
                    task_tags: config.task_tags.clone(),
                    cli_version: env!("CARGO_PKG_VERSION").to_string(),
                    created_at: now,
                    duration_seconds: 0.0,
                    command: cmd_str.to_string(),
                    target: config.target.to_string(),
                    config: serde_json::json!({}),
                    git: if config.git {
                        util::capture_git_info()
                    } else {
                        None
                    },
                    endpoint_count: results.len(),
                    result_summary: summary,
                    storage: task::StorageInfo {
                        has_bodies: !config.no_bodies,
                        has_raw: config.raw,
                        results_size_bytes: 0,
                    },
                    exit_code: 0,
                };
                match storage.save(&meta, &results, config.no_bodies, config.raw) {
                    Ok(_) => tracing::info!("Saved scan as task {id}"),
                    Err(e) => tracing::warn!("Failed to save task: {e}"),
                }
            }
        }
    }

    Ok(())
}
