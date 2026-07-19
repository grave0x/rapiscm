# Contributing

## Quick start

```sh
git clone https://github.com/your-org/rapiscm
cd rapiscm
cargo build --release
cargo test
cargo clippy
cargo fmt
```

## Before every commit

1. **`cargo clippy`** — must pass with `-D warnings` (no warnings allowed)
2. **`cargo fmt`** — must be run before diff
3. **`cargo test`** — all 30+ unit tests must pass

## CI expectations

GitHub Actions (`.github/workflows/ci.yml`) enforces:

- `cargo check` compiles
- `cargo fmt --check` passes
- `cargo clippy -- -D warnings` passes
- `cargo test` passes
- `cargo check --features browser` compiles

## Code conventions

### Rust style

- Edition 2024
- Errors: `anyhow` / `thiserror`, propagate with `.context()` or `.map_err()`
- CLI: `clap` derive macros
- HTTP: `reqwest` with configurable timeouts
- Minimise external deps — prefer stdlib or manual impls (e.g. manual ANSI codes, std time, AtomicU64 IDs)

### Naming

- Types: PascalCase (`ScanConfig`, `ResponseResult`)
- Functions/modules: snake_case (`run_checks`, `save_report`)
- Constants/statics: SCREAMING_SNAKE_CASE

### Module structure

- `mod.rs` for module roots with public re-exports
- One concern per file, one file per module
- `lib.rs` for public crate API (re-exports only)
- `main.rs` for binary entry point (Tokio runtime, dispatch, CLI parsing)

### Testing

- Unit tests in `#[cfg(test)] mod tests` blocks within each source file
- Integration tests in `tests/` directory
- Test with real HTTP servers where possible (`wiremock` for mock servers, `tempfile` for temp dirs)
- Keep tests deterministic — no network-dependent tests in unit tests

## PR process

1. Create a feature branch from `main`
2. Make changes with atomic commits
3. Run `cargo clippy && cargo fmt && cargo test`
4. Push and open a PR
5. CI must pass
6. Request review

## Adding a new security check

1. Create a new file in `src/check/` (e.g. `mycheck.rs`)
2. Implement a public function returning `Vec<Check>`
3. Register it in `src/check/mod.rs`:
   - In `run_checks()` for synchronous checks
   - In `run_async_checks()` for async checks
4. Add tests in the same file
5. Run `cargo test && cargo clippy`

## Adding a new discovery source

1. Create a new file in `src/discover/` (e.g. `mysource.rs`)
2. Implement a public async function
3. Add source reference to `DiscoverConfig` if it needs config
4. Wire it into `discover::mod.rs`
5. Add tests

## Adding a new output format

1. Create a new file in `src/report/` (e.g. `html.rs`)
2. Implement formatter function
3. Add format variant to `OutputFormat` enum
4. Add dispatch in `report::mod.rs`
5. Add CLI parser in `config.rs`
6. Update clap arg in `cli.rs`

## Documentation

- Keep `AGENTS.md` up to date with project layout and conventions
- Keep `docs/*.md` accurate with CLI flags and features
- Update `README.md` when adding major features
- Rustdoc for public API in `lib.rs` and key modules

## Release process

1. Update version in `Cargo.toml`
2. Update `docs/roadmap.md` with completed milestones
3. Run full test suite
4. Tag release
5. Build binaries
6. Publish to crates.io (future)
