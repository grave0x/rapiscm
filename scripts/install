#!/usr/bin/env bash
set -euo pipefail
RAPISCM_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$RAPISCM_ROOT"

BIN_DIR="${HOME}/.local/bin"
INSTALL_BROWSER=false
BUILD_MODE="release"

while [[ $# -gt 0 ]]; do
  case "$1" in
    --browser) INSTALL_BROWSER=true ;;
    --debug) BUILD_MODE="debug" ;;
    --help|-h)
      echo "Usage: $0 [--browser] [--debug]"
      echo "  --browser    Include browser deps (Chrome + geckodriver)"
      echo "  --debug      Install debug build instead of release"
      exit 0
      ;;
    *) echo "unknown: $1"; exit 1 ;;
  esac
  shift
done

if $INSTALL_BROWSER; then
  ./scripts/build browser
else
  ./scripts/build "$BUILD_MODE"
fi

mkdir -p "$BIN_DIR"
if [[ "$BUILD_MODE" == "debug" ]]; then
  cp target/debug/rapiscm "$BIN_DIR/"
else
  cp target/release/rapiscm "$BIN_DIR/"
fi
echo "installed to $BIN_DIR/rapiscm"

if $INSTALL_BROWSER; then
  echo "--- browser deps ---"
  "$RAPISCM_ROOT/scripts/deps" browser
fi

if [[ ":$PATH:" != *":$BIN_DIR:"* ]]; then
  echo "WARNING: $BIN_DIR not in PATH. Add to ~/.bashrc:"
  echo "  export PATH=\"\$PATH:$BIN_DIR\""
fi
