#!/usr/bin/env bash
set -euo pipefail
RAPISCM_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$RAPISCM_ROOT"

MODE="${1:-release}"

case "$MODE" in
  release)
    cargo build --release
    echo "built: target/release/rapiscm"
    ;;
  debug)
    cargo build
    echo "built: target/debug/rapiscm"
    ;;
  browser)
    cargo build --release --features browser
    echo "built (browser): target/release/rapiscm"
    ;;
  check)
    cargo check
    ;;
  *)
    echo "Usage: $0 [release|debug|browser|check]"
    exit 1
    ;;
esac
