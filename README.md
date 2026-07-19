# rapiscm

Rust API scanner. Point it at an API spec or a URL to scan.

```sh
rapiscm scan https://api.example.com
rapiscm spec ./openapi.json
rapiscm url https://example.com --crawl js --ghost
rapiscm capture https://example.com --extract
```

## Quick start

```sh
cargo build --release
./install.sh                   # install to ~/.local/bin
cargo test                     # run tests
cargo clippy                   # lint
```

## Documentation

| Doc | Description |
|---|---|
| [Installation](docs/installation.md) | Install from source, with browser features, via cargo |
| [Build](docs/build.md) | Build modes, feature flags, CI, troubleshooting |
| [Usage](docs/usage.md) | Full CLI reference with examples |
| [Features](docs/features.md) | All features, checks, output formats, module map |
| [Architecture](docs/architecture.md) | Internal design, data flow, module deps |
| [Contributing](docs/contributing.md) | Coding standards, PR process, testing |
| [Roadmap](docs/roadmap.md) | Future plans and known gaps |
| [Changelog](CHANGELOG.md) | Release history |
| [ADRs](docs/decisions/) | Architecture Decision Records |

## Commands

| Command | Description |
|---|---|
| `rapiscm scan <target>` | Auto-detect: spec file or URL |
| `rapiscm spec <file>` | Scan from OpenAPI spec (JSON/YAML) |
| `rapiscm url <url>` | Scan a URL / API base URL |
| `rapiscm fuzz <url>` | Fuzz endpoints with a wordlist |
| `rapiscm corp <name>` | Discover domains for an organisation |
| `rapiscm session <file>` | Replay a recorded JSONL session |
| `rapiscm tasks` | Manage saved scan tasks |
| `rapiscm capture <url>` | Capture page as evidence (HTML + JS endpoints + screenshot) |

## Features

- **Spec mode** — parse OpenAPI 3.x, extract + scan endpoints
- **URL mode** — discover endpoints from HTML, JS, robots.txt, sitemaps, common API paths
- **JS bundle scanning** — `--crawl js` downloads & parses SPA bundles for API endpoints
- **Ghost mode** — `--ghost` stealth scanning with UA rotation, jitter, header randomization, proxy rotation
- **Browser JS eval** — `--eval <js>` runs custom JS in headless browser, extracts returned URLs
- **Fuzzing** — wordlist-based fuzzing with match/filter by status, size, regex
- **Security checks** — security headers, CORS, auth enforcement, schema validation, tracker/analytics detection, cookie auditing (Secure/HttpOnly/SameSite/expiry)
- **Output formats** — table (ANSI terminal), JSON, markdown, structured docs (`-o doc`)
- **Filter system** — include/exclude by path, tag, method, status, expression (`tag:rest AND status:2xx`)
- **Task system** — persist, compare, export (MD/SARIF/HTML), rebuild, and resume scans
- **Corporate discovery** — discover owned domains via crt.sh, RDAP, favicon hash, ASN, Google CSE, Shodan
- **Browser discovery** — headless Chrome/Firefox for JS-rendered SPAs (`--features browser`)
- **Session replay** — replay JSONL recordings with live probes and timing analytics
- **Auth modes** — bearer token, basic auth, custom header
- **Static report site** — `--report <name>` generates full HTML report site

## Conventions

- `cargo clippy` must pass before commit
- `cargo fmt` before diff
- Errors: `anyhow`/`thiserror`, propagate with context
- CLI: `clap` derive macros
- HTTP: `reqwest` with configurable timeouts
- Minimise external deps

## Requirements

- Rust stable (≥ 1.80)
- OpenSSL dev libraries (`libssl-dev` on Debian, `openssl-devel` on Fedora)
- Optional: Chromium or Firefox + geckodriver (for browser discovery)
