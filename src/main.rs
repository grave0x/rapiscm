//! CLI entry point. Dispatches to subcommand handlers.

mod analytics;
mod check;
mod cli;
mod config;
mod deepspec;
mod discover;
mod error;
mod extract;
mod filter;
mod fuzz;
mod ghost;
mod parser;
mod report;
mod scan;
mod script;
mod session;
mod tag;
mod task;
mod types;
mod util;

use crate::types::Endpoint;
use clap::Parser;
use tracing_subscriber::EnvFilter;

fn init_logging(level: &str, filters: &[String], format: &str) {
    let mut filter_builder = EnvFilter::builder();
    for f in filters {
        filter_builder = filter_builder.with_default_directive(f.parse().expect("invalid log filter directive"));
    }
    let filter = filter_builder
        .with_default_directive(level.parse().unwrap_or(tracing::Level::INFO.into()))
        .from_env_lossy();
    let _ = match format {
        "json" => tracing_subscriber::fmt().json().with_env_filter(filter).try_init(),
        _ => tracing_subscriber::fmt()
            .without_time()
            .with_env_filter(filter)
            .try_init(),
    };
}

/// SSRF guard: reject private/internal/reserved IP addresses and localhost.
fn is_safe_url(url_str: &str) -> bool {
    let parsed = match reqwest::Url::parse(url_str) {
        Ok(u) => u,
        Err(_) => return false,
    };
    let host = match parsed.host_str() {
        Some(h) => h,
        None => return false,
    };
    // Reject private, loopback, link-local, and reserved addresses
    if host == "localhost"
        || host == "127.0.0.1"
        || host == "::1"
        || host == "0.0.0.0"
        || host.starts_with("169.254.")
        || host.starts_with("10.")
        || host.starts_with("192.168.")
        || host.starts_with("172.16.")
        || host.starts_with("127.")
        || host.starts_with("[::1]")
    {
        return false;
    }
    true
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
        cli::Command::Capture { global, .. } => global,
        cli::Command::Ip { global, .. } => global,
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
            output: config::parse_output(&global.output).map_err(|e| anyhow::anyhow!("output format: {e}"))?,
        };
        session::run_session(&cfg).await.map_err(|e| anyhow::anyhow!("{e}"))?;
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
        let json = serde_json::to_string_pretty(&domains).map_err(|e| anyhow::anyhow!("serialize: {e}"))?;
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
                    println!("{:<6} {:<30} {:<20} {:<8} Target", "ID", "Name", "Created", "");
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
                    diff.old_id, diff.new_id, diff.changed_count, diff.added_count, diff.removed_count
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
            TasksAction::Rebuild { id, all } => {
                let meta = storage.load_meta(*id).map_err(|e| anyhow::anyhow!("{e}"))?;
                let results = storage.load_results(*id).map_err(|e| anyhow::anyhow!("{e}"))?;

                // Rebuild by re-scanning endpoints
                let target_url = if meta.target.starts_with("url:") {
                    reqwest::Url::parse(&meta.target[4..])
                        .unwrap_or_else(|_| reqwest::Url::parse("https://example.com").unwrap())
                } else {
                    reqwest::Url::parse("https://example.com").unwrap()
                };

                let rebuild_target = types::Target::Url(target_url.clone());
                let rebuild_config = config::ScanConfig::from_cli_global(g, rebuild_target)?;
                let runner = scan::runner::ScanRunner::new(&rebuild_config)?;

                // Collect endpoints to re-scan
                let target_endpoints: Vec<Endpoint> = if *all {
                    // Re-scan everything
                    results
                        .iter()
                        .map(|r| {
                            let mut headers = Vec::new();
                            if let Some(ref h) = crate::types::auth_to_header(&rebuild_config.auth) {
                                headers.push(h.clone());
                            }
                            Endpoint {
                                method: reqwest::Method::from_bytes(r.endpoint_method.as_bytes())
                                    .unwrap_or(reqwest::Method::GET),
                                url: reqwest::Url::parse(&r.endpoint_url).unwrap_or_else(|_| target_url.clone()),
                                headers,
                                body: None,
                                expected_status: None,
                                tags: vec![],
                            }
                        })
                        .collect()
                } else {
                    // Only re-scan failed endpoints
                    results
                        .iter()
                        .filter(|r| r.status_code == 0 || r.status_code >= 400)
                        .map(|r| {
                            let mut headers = Vec::new();
                            if let Some(ref h) = crate::types::auth_to_header(&rebuild_config.auth) {
                                headers.push(h.clone());
                            }
                            Endpoint {
                                method: reqwest::Method::from_bytes(r.endpoint_method.as_bytes())
                                    .unwrap_or(reqwest::Method::GET),
                                url: reqwest::Url::parse(&r.endpoint_url).unwrap_or_else(|_| target_url.clone()),
                                headers,
                                body: None,
                                expected_status: None,
                                tags: vec![],
                            }
                        })
                        .collect()
                };

                if target_endpoints.is_empty() {
                    println!("No endpoints to re-scan (task {id} has no failures).");
                    return Ok(());
                }

                tracing::info!("Rebuilding task {id}: re-scanning {} endpoints", target_endpoints.len());
                let target_endpoints_len = target_endpoints.len();
                let new_results = runner.run(target_endpoints).await;

                // Merge: replace old results with new ones (matched by URL+method)
                let mut merged: Vec<types::ResponseResult> = results;
                for new_r in new_results {
                    let found = merged.iter_mut().find(|old| {
                        old.endpoint_url == new_r.endpoint_url && old.endpoint_method == new_r.endpoint_method
                    });
                    if let Some(old) = found {
                        *old = new_r;
                    } else {
                        merged.push(new_r);
                    }
                }

                // Run checks on merged results
                for r in &mut merged {
                    crate::check::run_checks(r);
                }

                let updated_meta = task::TaskMeta {
                    result_summary: task::summarize(&merged),
                    endpoint_count: merged.len(),
                    duration_seconds: 0.0,
                    ..meta
                };
                storage
                    .save(&updated_meta, &merged, !rebuild_config.no_bodies, rebuild_config.raw)
                    .map_err(|e| anyhow::anyhow!("{e}"))?;

                println!(
                    "Rebuilt task {id}: {} endpoints re-scanned, task updated",
                    target_endpoints_len
                );

                // Print results
                let output = report::format_results(&merged, rebuild_config.output);
                println!("{output}");
                return Ok(());
            }
            TasksAction::Queue { targets, list } => {
                let mut all_targets = targets.clone();
                if let Some(list_path) = list {
                    let content = std::fs::read_to_string(list_path)
                        .map_err(|e| anyhow::anyhow!("failed to read list file: {e}"))?;
                    for line in content.lines() {
                        let trimmed = line.trim();
                        if !trimmed.is_empty() && !trimmed.starts_with('#') {
                            all_targets.push(trimmed.to_string());
                        }
                    }
                }
                if all_targets.is_empty() {
                    println!("No targets to queue.");
                    return Ok(());
                }
                let queue_path = storage.queue_path();
                let existing = task::load(&queue_path).len();
                for (idx, target) in all_targets.iter().enumerate() {
                    let item = task::QueueItem {
                        queue_id: format!("q-{:04}", existing + idx),
                        command: if target.ends_with(".json") || target.ends_with(".yaml") || target.ends_with(".yml") {
                            "spec".into()
                        } else {
                            "url".into()
                        },
                        target: target.clone(),
                        config_snapshot: serde_json::json!({}),
                        status: task::QueueItemStatus::Pending,
                        created_at: util::now_iso(),
                        started_at: None,
                        completed_at: None,
                        task_id: None,
                        retries: 0,
                        error: None,
                    };
                    task::enqueue(&queue_path, item).map_err(e)?;
                }
                println!("Queued {} target(s) for scanning.", all_targets.len());
                return Ok(());
            }
            TasksAction::Run { parallel: _concurrency } => {
                let queue_path = storage.queue_path();
                let recovered = task::recover_crashed(&queue_path).map_err(e)?;
                if recovered > 0 {
                    println!("Recovered {recovered} crashed queue item(s).");
                }
                let mut processed = 0;
                while let Some(item) = task::dequeue(&queue_path) {
                    println!("Processing: {} ({})", item.target, item.command);
                    // SSRF guard: reject private/internal IPs before fetching
                    let url = format!(
                        "https://{}",
                        item.target.trim_start_matches("https://").trim_start_matches("http://")
                    );
                    if !is_safe_url(&url) {
                        let err = format!("refused unsafe URL (private/internal): {url}");
                        task::complete(&queue_path, &item.queue_id, None, Some(err.clone())).map_err(e)?;
                        println!("  ✗ {err}");
                        processed += 1;
                        continue;
                    }
                    let scan_result: Result<Vec<crate::types::ResponseResult>, String> = (async {
                        let client = reqwest::Client::builder()
                            .timeout(std::time::Duration::from_secs(30))
                            .build()
                            .map_err(|e| e.to_string())?;
                        let resp = client.get(&url).send().await.map_err(|e| e.to_string())?;
                        let status = resp.status().as_u16();
                        let headers: Vec<(String, String)> = resp
                            .headers()
                            .iter()
                            .map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string()))
                            .collect();
                        let body = resp.bytes().await.unwrap_or_default().to_vec();
                        let elapsed = 0u64;
                        Ok(vec![crate::types::ResponseResult {
                            endpoint_method: "GET".into(),
                            endpoint_url: url.clone(),
                            status_code: status,
                            response_time_ms: elapsed,
                            response_size: body.len(),
                            response_headers: headers,
                            response_body: body,
                            expected_status: None,
                            timestamp: Some(util::now_iso()),
                            checks: vec![],
                            error: None,
                            tags: vec![],
                            trackers: vec![],
                        }])
                    })
                    .await;
                    match scan_result {
                        Ok(results) => {
                            let summary = task::summarize(&results);
                            let task_id = task::index::next_id(&storage.index_path());
                            let meta = task::TaskMeta {
                                task_id,
                                task_name: format!("queue-{}", item.target.replace([':', '/'], "-")),
                                task_tags: vec!["queued".into()],
                                cli_version: env!("CARGO_PKG_VERSION").into(),
                                created_at: util::now_iso(),
                                duration_seconds: 0.0,
                                command: item.command.clone(),
                                target: item.target.clone(),
                                config: serde_json::json!({}),
                                git: None,
                                endpoint_count: results.len(),
                                result_summary: summary,
                                storage: task::StorageInfo {
                                    has_bodies: true,
                                    has_raw: false,
                                    results_size_bytes: 0,
                                },
                                exit_code: 0,
                            };
                            storage.save(&meta, &results, false, false).map_err(e)?;
                            task::complete(&queue_path, &item.queue_id, Some(task_id), None).map_err(e)?;
                            println!("  ✓ Completed as task {task_id}");
                        }
                        Err(err) => {
                            task::complete(&queue_path, &item.queue_id, None, Some(err.clone())).map_err(e)?;
                            println!("  ✗ Failed: {err}");
                        }
                    }
                    processed += 1;
                }
                println!("Processed {processed} queue item(s).");
                return Ok(());
            }
            TasksAction::Status => {
                let queue_path = storage.queue_path();
                let items = task::load(&queue_path);
                let pending = items
                    .iter()
                    .filter(|i| i.status == task::QueueItemStatus::Pending)
                    .count();
                let running = items
                    .iter()
                    .filter(|i| i.status == task::QueueItemStatus::Running)
                    .count();
                let completed = items
                    .iter()
                    .filter(|i| i.status == task::QueueItemStatus::Completed)
                    .count();
                let failed = items
                    .iter()
                    .filter(|i| i.status == task::QueueItemStatus::Failed)
                    .count();
                println!("Queue Status:");
                println!("  Pending:   {pending}");
                println!("  Running:   {running}");
                println!("  Completed: {completed}");
                println!("  Failed:    {failed}");
                println!("  Total:     {}", items.len());
                if !items.is_empty() {
                    println!("\nItems:");
                    for item in &items {
                        let status_str = match item.status {
                            task::QueueItemStatus::Pending => "pending",
                            task::QueueItemStatus::Running => "running",
                            task::QueueItemStatus::Completed => "completed",
                            task::QueueItemStatus::Failed => "failed",
                        };
                        println!("  [{status_str:>9}] {} — {}", item.target, item.command);
                    }
                }
                return Ok(());
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
            let found = merged
                .iter_mut()
                .find(|old| old.endpoint_url == new_r.endpoint_url && old.endpoint_method == new_r.endpoint_method);
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
            .save(&updated_meta, &merged, !resume_config.no_bodies, resume_config.raw)
            .map_err(|e| anyhow::anyhow!("Failed to save resumed task: {e}"))?;
        task::resume::clear_checkpoint(&storage, *task_id);

        let output = report::format_results(&merged, resume_config.output);
        println!("{output}");
        return Ok(());
    }

    // ── Capture subcommand: save page as evidence ──
    if let cli::Command::Capture {
        url,
        capture_dir: output,
        screenshot: _,
        html,
        extract,
        ..
    } = &cli.command
    {
        let base_url = reqwest::Url::parse(url)
            .or_else(|_| reqwest::Url::parse(&format!("https://{url}")))
            .map_err(|e| anyhow::anyhow!("invalid capture URL: {e}"))?;

        std::fs::create_dir_all(output).map_err(|e| anyhow::anyhow!("failed to create output dir: {e}"))?;

        // Fetch and save the page
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()?;

        let resp = client.get(base_url.as_str()).send().await?;
        let body = resp.bytes().await.unwrap_or_default();

        if *html {
            let html_path = output.join("index.html");
            std::fs::write(&html_path, &body).map_err(|e| anyhow::anyhow!("failed to save HTML: {e}"))?;
            tracing::info!("Saved HTML ({} bytes) to {:?}", body.len(), html_path);
        }

        // Extract API endpoints from JS bundles if requested
        if *extract {
            let text = String::from_utf8_lossy(&body);
            match crate::parser::js_bundle::scan_bundles(&client, &text, &base_url, false).await {
                Ok(js_eps) => {
                    let urls = crate::parser::js_bundle::to_scan_urls(&js_eps, &base_url);
                    let api_path = output.join("api_endpoints.txt");
                    let content: String = urls.iter().map(|u| u.to_string()).collect::<Vec<_>>().join("\n");
                    std::fs::write(&api_path, &content)
                        .map_err(|e| anyhow::anyhow!("failed to save endpoints: {e}"))?;
                    tracing::info!("Found {} API endpoints, saved to {:?}", urls.len(), api_path);
                }
                Err(e) => tracing::warn!("JS bundle scan failed: {e}"),
            }
        }

        // Screenshot with browser if requested
        #[cfg(feature = "browser")]
        if *screenshot {
            use futures_util::StreamExt;
            let shot_path = output.join("screenshot.png");
            match crate::scan::browser::screenshot(&base_url, &shot_path, g.headed, g.proxy.as_deref()).await {
                Ok(_) => tracing::info!("Screenshot saved to {:?}", shot_path),
                Err(e) => tracing::warn!("Screenshot failed: {e}"),
            }
        }

        tracing::info!("Capture complete. Results in {:?}", output);
        return Ok(());
    }

    // ── Ip subcommand: TCP port scan ──
    #[cfg(feature = "ip")]
    if let cli::Command::Ip {
        host,
        ports,
        timeout_ms,
        os,
        ..
    } = &cli.command
    {
        let port_list = match ports.as_str() {
            "default" => scan::ip::DEFAULT_PORTS.to_vec(),
            "extended" => scan::ip::EXTENDED_PORTS.to_vec(),
            custom => custom.split(',').filter_map(|s| s.trim().parse::<u16>().ok()).collect(),
        };

        let timeout = std::time::Duration::from_millis(*timeout_ms);
        let result = scan::ip::run_ip_scan(host, &port_list, timeout);

        match g.output.as_str() {
            "json" => {
                let json = serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string());
                println!("{json}");
            }
            _ => {
                println!("{}", result.summary());
                for p in &result.ports {
                    if p.open {
                        let svc = p.service.as_deref().map(|s| format!(" ({s})")).unwrap_or_default();
                        let banner = p.banner.as_deref().map(|b| format!(" — {b}")).unwrap_or_default();
                        let lat = p.latency_ms.map(|l| format!("  {l:.1}ms")).unwrap_or_default();
                        println!("  {:<6} open{svc}{banner}{lat}", p.port);
                    }
                }
                if *os {
                    if let Some(ref os_info) = result.os {
                        let guess = os_info.guessed_os.as_deref().unwrap_or("unknown");
                        println!("  OS: {guess} (TTL={})", os_info.ttl);
                    }
                }
            }
        }

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
            mode,
            wordlist_mode,
            keyword,
            request,
            ..
        } => {
            let base_url = reqwest::Url::parse(target)
                .or_else(|_| reqwest::Url::parse(&format!("https://{target}")))
                .map_err(|e| anyhow::anyhow!("invalid fuzz target: {e}"))?;
            let scan_config = config::ScanConfig::from_cli_global(g, types::Target::Url(base_url.clone()))?;
            let opts = fuzz::FuzzOpts {
                wordlist: wordlist.clone(),
                mc: mc.clone(),
                fc: fc.clone(),
                ms: ms.clone(),
                fs: fs.clone(),
                mr: mr.clone(),
                fr: fr.clone(),
                ac: *ac,
                mode: mode.clone(),
                wordlist_mode: wordlist_mode.clone(),
                keyword: keyword.clone(),
                request: request.clone(),
            };
            fuzz::run_fuzz_scan(&base_url, &opts, &scan_config).await?;
        }
        cli::Command::Corp { .. } => {
            unreachable!("corp mode handled above")
        }
        cli::Command::Session { .. } => {
            unreachable!("session mode handled above")
        }
        cli::Command::Ip { .. } => {
            unreachable!("ip mode handled above")
        }
        cmd => {
            let config = config::ScanConfig::from_cli(cli::Cli { command: cmd.clone() })?;
            let mut results: Vec<crate::types::ResponseResult> = match &config.target {
                types::Target::Spec(_) => scan::spec::run_spec_scan(&config).await?,
                types::Target::Url(_) => scan::url::run_url_scan(&config).await?,
            };
            let output_format = config.output;
            // --script: apply pipe script filter to results
            if let Some(ref script_cmd) = g.script {
                if let Some(stripped) = script_cmd.strip_prefix("pipe:") {
                    if let Err(e) = script::apply_pipe(stripped, &mut results) {
                        tracing::warn!("script filter failed: {e}");
                    }
                } else {
                    tracing::warn!("unknown script scheme '{script_cmd}'. Use pipe:./script.py");
                }
            }
            let output = report::format_results(&results, output_format);
            println!("{output}");

            // --deep-spec: produce technical breakdown YAML
            if config.deep_spec {
                let spec = deepspec::analyze(&results);
                match deepspec::to_yaml(&spec) {
                    Ok(yaml) => println!("\n--- Deep Spec ---\n{yaml}"),
                    Err(e) => tracing::warn!("Failed to generate deep spec: {e}"),
                }
            }

            // --report: generate static HTML reports
            if let Some(ref report_name) = g.report {
                match report::site::generate(&results, report_name) {
                    Ok(path) => tracing::info!("Reports saved to {}", path.display()),
                    Err(e) => tracing::warn!("Failed to generate reports: {e}"),
                }
            }

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
                // Auto-name: {target-host}-{timestamp} or custom --task-name
                let auto_name = match &config.target {
                    types::Target::Url(u) => {
                        let host = u.host_str().unwrap_or("unknown");
                        format!("{host}-{}", &now[..19])
                    }
                    types::Target::Spec(p) => {
                        let stem = p.file_stem().and_then(|s| s.to_str()).unwrap_or("spec");
                        format!("{stem}-{}", &now[..19])
                    }
                };
                let meta = task::TaskMeta {
                    task_id: id,
                    task_name: config.task_name.unwrap_or(auto_name),
                    task_tags: config.task_tags.clone(),
                    cli_version: env!("CARGO_PKG_VERSION").to_string(),
                    created_at: now,
                    duration_seconds: 0.0,
                    command: cmd_str.to_string(),
                    target: config.target.to_string(),
                    config: serde_json::json!({}),
                    git: if config.git { util::capture_git_info() } else { None },
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
