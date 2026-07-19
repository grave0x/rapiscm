# Changelog

## [Unreleased]

### Added
- Ghost mode (`--ghost`): UA rotation, request jitter, header randomization, proxy rotation
- JS bundle scanning (`--crawl js`): download and parse SPA bundles for API endpoints
- Browser JS eval (`--eval`): run custom JS in headless browser, extract URLs
- Capture subcommand (`rapiscm capture <url>`): save page HTML + JS API endpoints + screenshot
- Structured docs output (`-o doc`): llm-api-style markdown documentation
- `--crawl` now accepts `html`, `js`, `full` modes (was boolean)
- `--ua-rotate`, `--jitter`, `--proxy-rotate` flags for ghost mode
- `tasks rebuild` works from CLI (was stubbed)

### Fixed
- `--show-report` was no-op (GlobalArgs always set show_report: false)
- `--show-tags` flag existed but was never read by formatters
- `--filter`/`--exclude` expression syntax (tag:, status:, path:, method: prefix parsing + AND logic)
- Cookie security analysis (added SameSite, Secure, HttpOnly, expiry detection)
- Fuzz `--ac` auto-calibrate (was hardcoded 404/50, now probes real baseline)
- `filter.rs` → `check/trackers.rs` module rename (misleading name)
- Test config literals updated for new fields

## [0.1.0] — 2026-07-16

### Added
- Initial release — Rust API scanner
- Spec mode: parse OpenAPI 3.x specs, extract + scan endpoints
- URL mode: discover endpoints from HTML, JS, robots.txt, sitemaps, wordlists
- Fuzz mode: wordlist-based fuzzing with match/filter by status, size, regex
- Session replay: replay JSONL session files through check pipeline
- Task system: persist, compare, export (md/sarif/html), rebuild, resume scans
- Domain discovery: crt.sh, RDAP, ASN, favicon hash, Google dork, GA ID pivot
- Security checks: CSP, HSTS, X-CT-O, X-FO, Cache-Control, CORS, auth enforcement
- Tracker analytics: detect analytics, ads, fingerprinting, social media trackers
- Output formats: terminal table, JSON, markdown
- Browse discovery (feature-gated): headless Chrome/Firefox for SPA endpoint discovery
- Git integration: capture SHA, branch, message with saved tasks
- corporate domain discovery (`rapiscm corp <org>`)
