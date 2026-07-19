# Build

## Quick start

```sh
cargo build --release
```

Binary at `target/release/rapiscm`.

## Build modes

| Command | Output |
|---|---|
| `cargo build` | Debug build at `target/debug/rapiscm` |
| `cargo build --release` | Optimised release at `target/release/rapiscm` |
| `./scripts/build debug` | Same as `cargo build` |
| `./scripts/build release` | Same as `cargo build --release` |
| `./scripts/build browser` | Release with `--features browser` |
| `./scripts/build check` | `cargo check` (fast) |

## Feature flags

| Feature | Deps | Purpose |
|---|---|---|
| `default = []` | — | Core scan engine, no browser |
| `browser` | `chromiumoxide`, `fantoccini` | Headless Chrome/Firefox endpoint discovery |

Enable browser features:

```sh
cargo build --release --features browser
```

## Release profile

`Cargo.toml` release profile optimises for size:

```toml
[profile.release]
strip = true
opt-level = "z"
codegen-units = 1
lto = true
```

## Documentation

```sh
cargo doc --no-deps --open
```

Generated docs at `target/doc/rapiscm/index.html` (also available at `doc/`).

## Cross-compilation

Cross-compile with [cross](https://github.com/cross-rs/cross) or standard `--target`:

```sh
# Linux x86_64 → aarch64 (requires aarch64 toolchain)
rustup target add aarch64-unknown-linux-gnu
cargo build --release --target aarch64-unknown-linux-gnu
```

## CI

GitHub Actions (`.github/workflows/ci.yml`) runs on push/PR to `master`/`main`:

1. `cargo check`
2. `cargo fmt --check`
3. `cargo clippy -- -D warnings`
4. `cargo test`
5. `cargo check --features browser`

## Troubleshooting

### `openssl` not found

```sh
# Debian/Ubuntu
sudo apt install pkg-config libssl-dev
# Fedora
sudo dnf install openssl-devel
# macOS
brew install openssl
```

### Build too slow

Use `cargo check` for fast compile checks.
