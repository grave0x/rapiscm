# Installation

## Prerequisites

- **Rust toolchain** (stable ≥ 1.80): [rustup.rs](https://rustup.rs)
- **System dependencies**: `openssl`, `pkg-config`, `ca-certificates`
- **Optional — browser discovery**: Chromium or Firefox + `geckodriver`

## From source (recommended)

```sh
git clone https://github.com/your-org/rapiscm
cd rapiscm
cargo build --release
./install.sh                 # copies to ~/.local/bin/rapiscm
```

Or manually:

```sh
cargo build --release
cp target/release/rapiscm ~/.local/bin/
```

Verify:

```sh
rapiscm --version
```

## With browser features

```sh
./install.sh --browser
# or
cargo build --release --features browser
cp target/release/rapiscm ~/.local/bin/
```

## With cargo install

```sh
cargo install --git https://github.com/your-org/rapiscm
```

## Post-install

Add `~/.local/bin` to `PATH` if not already:

```sh
export PATH="$PATH:$HOME/.local/bin"
# add to ~/.bashrc or ~/.zshrc
```

### API keys (optional)

Create `~/.config/rapiscm/config.toml`:

```toml
[api_keys]
google_api_key = "..."
google_cx = "..."
shodan_api_key = "..."
```

Required only for `rapiscm corp` domain discovery sources (Google CSE, Shodan).

## Verify installation

```sh
rapiscm scan https://httpbin.org
```
