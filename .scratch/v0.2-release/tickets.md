# Tickets: rapiscm v0.2.0 — Distribution & Feature Expansion

Ship distributable builds + 5 new capabilities: crate, binaries, Homebrew,
Docker, OpenAPI 3.1, SARIF compliance, IP mode, fuzz expansion, batch queue.

Source: `.scratch/v0.2-release/spec.md`

Work the **frontier**: any ticket whose blockers are all done.

---

## T0: Dead code audit

**What to build:** Audit all 17 `#[allow(dead_code)]` sites. Remove genuinely dead
paths. Add `#[cfg(feature = "browser")]` annotations where code is feature-gated
but missing the gate. This prefactor keeps the codebase clean before the release.

**Blocked by:** None — can start immediately.

- [ ] All 17 `#[allow(dead_code)]` sites reviewed
- [ ] Dead code removed or properly feature-gated
- [ ] `cargo clippy` and `cargo test` pass

---

## T1: crates.io publish + prebuilt binaries

**What to build:** Publish rapiscm to crates.io. Set up GitHub Actions to build
prebuilt Linux binaries (x86_64 + aarch64) and attach them to GitHub Releases.

**Blocked by:** T0

- [ ] `cargo publish` succeeds (crate name available, metadata complete)
- [ ] GitHub Actions workflow builds both architectures on tag push
- [ ] Prebuilt binaries attached to a GitHub Release
- [ ] `cargo install rapiscm` works from crates.io

---

## T2: Homebrew tap + Docker image

**What to build:** Create a Homebrew formula in a tap repo. Build a multi-arch
Docker image and push to GitHub Container Registry.

**Blocked by:** T1

- [ ] Homebrew formula installs the prebuilt binary
- [ ] `brew install <tap>/rapiscm` works on macOS
- [ ] Dockerfile builds for linux/amd64 and linux/arm64
- [ ] `docker run ghcr.io/<user>/rapiscm scan --help` works

---

## T3: OpenAPI 3.1 support

**What to build:** Extend the OpenAPI parser to handle 3.1 features: nullable as
first-class (type arrays), path items as `$ref` targets, webhooks. Existing 3.0
parsing must not regress.

**Blocked by:** T0

- [ ] OpenAPI 3.1 spec document parses without error
- [ ] Nullable schemas resolve correctly (e.g. `{"type": ["string", "null"]}`)
- [ ] Path item `$ref` resolution works
- [ ] Webhooks are extracted as scan targets
- [ ] Existing 3.0 tests still pass

---

## T4: SARIF 2.1 compliance

**What to build:** Complete the partial SARIF export. Validate output against the
SARIF SDK validator. Ensure GitHub Code Scanning can ingest the output.

**Blocked by:** T0

- [ ] SARIF output passes the SARIF validator
- [ ] Output includes `runs[].tool.driver` with name + version
- [ ] Output includes `runs[].results` with ruleId, level, message, locations
- [ ] GitHub Code Scanning accepts the SARIF upload
- [ ] `rapiscm spec spec.json -o sarif` produces valid output

---

## T5: rapiscm ip mode

**What to build:** New `rapiscm ip <target>` subcommand. TCP connect scan on
common API ports (80, 443, 8080, 8443, 3000, 5000, 8000, 9000). Service version
detection via banner grabbing. OS fingerprint via TCP stack analysis. Feature-gated
behind `--features ip`.

**Blocked by:** T0

- [ ] `rapiscm ip 192.168.1.1` scans common ports and reports open/closed
- [ ] Service detection identifies HTTP servers, databases, etc.
- [ ] OS fingerprint reports likely OS from TCP parameters
- [ ] Output in table and JSON formats
- [ ] Feature-gated: `cargo build` (without `--features ip`) excludes the module

---

## T6: Fuzz mode expansion

**What to build:** Refactor the fuzz runner to support `FuzzMode::Param`,
`FuzzMode::Method`, `FuzzMode::Header`, `FuzzMode::Body`, and `FuzzMode::Wordlist`.
Each mode generates payloads from the appropriate source. Wordlist mode reads
from `--wordlist <file>`.

**Blocked by:** T0

- [ ] `rapiscm fuzz <target> --mode param` fuzzes query parameters
- [ ] `rapiscm fuzz <target> --mode method` tries different HTTP methods
- [ ] `rapiscm fuzz <target> --mode header` injects into headers
- [ ] `rapiscm fuzz <target> --mode body` fuzzes request bodies
- [ ] `rapiscm fuzz <target> --wordlist paths.txt` uses custom wordlist
- [ ] Match/filter/auto-calibrate work across all modes

---

## T7: Batch task queue

**What to build:** `rapiscm tasks queue <spec>`, `rapiscm tasks run`, `rapiscm
tasks status`. A queue runner reads pending tasks from the existing task store
and executes them sequentially. Crash recovery re-reads and marks interrupted
tasks as failed.

**Blocked by:** T0

- [ ] `rapiscm tasks queue spec1.json spec2.json` adds to queue
- [ ] `rapiscm tasks run` executes queued tasks in FIFO order
- [ ] `rapiscm tasks status` shows pending/running/completed/failed counts
- [ ] Crash recovery: kill the process mid-scan, restart, task marked failed
- [ ] Queue persists across process restarts

---

## T8: Release — version bump, changelog, tag

**What to build:** Bump version to 0.2.0, update CHANGELOG.md with all changes,
tag the release, push to GitHub. Verify the full CI pipeline passes.

**Blocked by:** T1, T2, T3, T4, T5, T6, T7

- [ ] Version bumped in Cargo.toml
- [ ] CHANGELOG.md updated with v0.2.0 entries
- [ ] Git tag `v0.2.0` pushed
- [ ] GitHub Release created with binary artifacts
- [ ] Full CI matrix (check, clippy, fmt, test) passes
