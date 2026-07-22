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
- [ ] `rapiscm ip <target>` scans common ports (80, 443, 8080, etc.)
- [ ] Service detection identifies HTTP, databases, etc.
- [ ] OS fingerprint reports likely OS
- [ ] Table + JSON output formats
- [ ] Feature-gated: `cargo build` (no `--features ip`) excludes module

## T6: Fuzz mode expansion
- [ ] `--mode param` fuzzes query parameters
- [ ] `--mode method` tries different HTTP methods
- [ ] `--mode header` injects into headers
- [ ] `--mode body` fuzzes request bodies
- [ ] `--wordlist paths.txt` uses custom wordlist
- [ ] Match/filter/auto-calibrate work across all modes

## T7: Batch task queue
- [ ] `tasks queue` adds to queue
- [ ] `tasks run` executes in FIFO order
- [ ] `tasks status` shows counts by state
- [ ] Crash recovery: interrupted tasks marked failed
- [ ] Queue persists across restarts

## T8: Release
- [ ] Version bumped to 0.2.0
- [ ] CHANGELOG.md updated
- [ ] Git tag `v0.2.0` pushed
- [ ] Full CI matrix passes
