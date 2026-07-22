# rapiscm — Task Checklist (v0.2.0)

Derived from `.scratch/v0.2-release/tickets.md`

## T0: Dead code audit
- [x] All 10 `#[allow(dead_code)]` sites reviewed
- [x] 1 genuinely dead: `task::rebuild::rebuild()` (97 lines) — CLI does rebuild inline, function never called
- [x] 9 legitimate: test-only utilities (`get_entry`, `disk_usage`, `write_checkpoint`, `fmt_duration`), future features (`wordlist_mode`, `task_id`), internal types (`JsApiEndpoint`), struct-level noise (`ScanConfig`, `message`)
- [x] Zero `unimplemented!`, `unreachable!`, or `todo!` macros in codebase
- [x] `cargo clippy` and `cargo test` pass
- [ ] *Note:* `task::rebuild::rebuild()` should be wired into the CLI handler or removed

## T1: crates.io publish + prebuilt binaries
- [ ] `cargo publish` succeeds
- [ ] GitHub Actions builds both architectures on tag
- [ ] Prebuilt binaries attached to GitHub Release
- [ ] `cargo install rapiscm` works

## T2: Homebrew tap + Docker image
- [ ] Homebrew formula installs prebuilt binary
- [ ] `brew install <tap>/rapiscm` works on macOS
- [ ] Multi-arch Dockerfile builds linux/amd64 + linux/arm64
- [ ] `docker run ghcr.io/.../rapiscm scan --help` works

## T3: OpenAPI 3.1 support
- [x] 3.1 spec document parses without error (conditional on license format)
- [x] Nullable schemas handled (openapiv3 v2 tolerates type arrays)
- [x] Path item `$ref` not yet supported (openapiv3 v2 limitation, noted)
- [x] Webhooks extracted as scan targets (secondary raw parse)
- [x] Webhook URL format: `/.webhooks/{name}/{method}`
- [x] Webhooks tagged with `webhook:{operationId}` + `openapi31`
- [x] Existing 3.0 tests still pass

## T4: SARIF 2.1 compliance
- [x] SARIF output produces valid JSON with `$schema`, `version`, `runs`
- [x] `tool.driver` with name "rapiscm", version, informationUri
- [x] `results` with ruleId, level, message, locations per failed check
- [x] `rules` metadata array with descriptions for known checks
- [x] `invocations[].executionSuccessful` set
- [x] CLI accepts `-o sarif`
- [x] `cargo test` passes (all existing + new tests)
- [x] GitHub Code Scanning compatible format
- [ ] *Note:* SARIF validator tooling not run (no SARIF SDK installed)

## T5: rapiscm ip mode
- [x] `rapiscm ip <target>` scans common ports (default, extended, custom)
- [x] Service detection identifies HTTP, SSH, databases, etc. from port numbers
- [x] OS fingerprint reports likely OS from TTL (Linux/Unix, Windows, Solaris)
- [x] Table + JSON output formats
- [x] Feature-gated: `cargo build` (no `--features ip`) excludes module
- [x] 5 unit tests in ip module
- [x] `cargo test --features ip` — 442 pass, 0 fail

## T6: Fuzz mode expansion
- [x] `--mode param` fuzzes query parameters with keyword URL construction
- [x] `--mode method` tries HTTP methods on keyword-replaced URLs
- [x] `--mode header` injects fuzzed headers on keyword URLs
- [x] `--mode body` POSTs word-based bodies to keyword URLs
- [x] `--wordlist paths.txt` uses custom wordlist (already worked)
- [x] Match/filter/auto-calibrate work across all modes
- [x] Fixed wordlist.rs typo (tuple → string)
- [x] Keyword replacement or path appending for all URL construction
- [x] `cargo test` passes

## T7: Batch task queue
- [x] `tasks queue` adds targets to queue (spec files, URLs, file-based lists)
- [x] `tasks run` processes queue items sequentially
- [x] `tasks status` shows pending/running/completed/failed counts
- [x] Crash recovery: `recover_crashed()` marks interrupted items as failed
- [x] Queue persists as JSON across process restarts
- [x] CLI subcommands wired in main.rs dispatch
- [x] `QueueItem` with full metadata (status, timestamps, retries, errors)

## T8: Release
- [x] Version bumped to 0.2.0
- [x] CHANGELOG.md updated with v0.2.0 entries
- [x] Git commit ready (ab69a21)
- [ ] Git tag `v0.2.0` (manual, post-review)
- [ ] `cargo publish` (manual, post-tag)
- [ ] GitHub Release with binary artifacts (manual, post-tag)
