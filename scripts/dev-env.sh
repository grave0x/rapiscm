#!/usr/bin/env bash
set -euo pipefail
RAPISCM_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$RAPISCM_ROOT"

echo "=== rapiscm dev environment ==="
echo ""

# 1. Rust toolchain
if ! command -v cargo &>/dev/null; then
  echo "[1/4] installing Rust..."
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  . "$HOME/.cargo/env"
else
  echo "[1/4] Rust: $(rustc --version)"
fi

# 2. Install deps
echo "[2/4] running deps..."
"$RAPISCM_ROOT/scripts/deps" build

# 3. Build
echo "[3/4] building..."
"$RAPISCM_ROOT/scripts/build"

# 4. Git hooks (optional)
if [ -d .git ]; then
  echo "[4/4] setting up git hooks..."
  cat > .git/hooks/pre-commit <<'HOOK'
#!/usr/bin/env bash
set -euo pipefail
echo "pre-commit: running cargo check + clippy + fmt..."
cargo check || exit 1
cargo clippy -- -D warnings 2>/dev/null || cargo clippy || exit 1
cargo fmt --check || exit 1
HOOK
  chmod +x .git/hooks/pre-commit
  echo "  pre-commit hook installed"
else
  echo "[4/4] no .git directory, skipping hooks"
fi

echo ""
echo "=== dev environment ready ==="
echo "  build:   ./scripts/build"
echo "  test:    cargo test"
echo "  install: ./scripts/install [--browser]"
