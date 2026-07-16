# rapiscm

Rust API scanner. Point at an API spec or a URL to scan.

## Status

Working. Spec-mode and URL-mode scan pipelines implemented. Output currently raw JSON lines; table/json/md formatters in later phases.

## Quick start

```sh
cargo build --release
./install.sh [--browser]   # install binary + optional browser tools
cargo test                 # 30 unit tests
cargo clippy               # lint (no warnings expected)
cargo fmt                  # format
```

## CLI

```
rapiscm spec <file>          # scan from OpenAPI spec (JSON/YAML)
rapiscm url <url>            # scan a URL / API base URL
rapiscm scan <target>        # auto-detect

Flags:  -H/--header, --auth, --rate-limit, --timeout, --concurrency,
        -o/--output (table|json|md), --follow-redirects, -k/--insecure,
        --paths, --tags, --proxy
  (browser feature): --browser (chrome|firefox), --headed
```

## Features

- `browser`: headless browser discovery via chromiumoxide + geckodriver. Enable with `--features browser`.

## Project layout

```
src/
  main.rs           # tokio::main, dispatch to spec/url
  cli.rs            # clap CLI definition (3 subcommands, 13+ flags)
  config.rs         # ScanConfig builder from Cli
  types.rs          # Target, Endpoint, ResponseResult, Check, etc.
  error.rs          # Error enum (thiserror)
  check/
    mod.rs          # run_checks() + run_async_checks()
    security.rs     # security header checks (CSP, HSTS, etc.)
    cors.rs         # CORS preflight probe
    auth.rs         # Auth-enforcement probe
    schema.rs       # Response status + body validation
  scan/
    runner.rs       # ScanRunner: concurrent HTTP + rate limiting
    spec.rs         # spec-mode: parse spec → scan
    url.rs          # url-mode: discover endpoints → scan
    browser.rs      # (feature-gated) Chrome/Firefox interactive discovery
  parser/
    spec.rs         # openapiv3 → Vec<Endpoint>
    url.rs          # HTML link extraction, wordlist, API endpoint heuristics
  report/
    mod.rs          # format_results() dispatcher
    table.rs        # Terminal (ANSI) + markdown table formatters
    json.rs         # JSON formatter
    summary.rs      # Summary statistics
install.sh          # installs binary + optional browser deps (--browser)
.github/workflows/ci.yml  # check, clippy, fmt, test
SPEC.md, PLAN.md    # design documents
```

## Conventions

- `cargo clippy` must pass before commit
- `cargo fmt` before diff
- Errors: `anyhow`/`thiserror`, propagate with context
- CLI: `clap` derive macros
- HTTP: `reqwest` with configurable timeouts
- Minimize external deps: manual ANSI codes, std time, AtomicU64 IDs
