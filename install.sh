#!/usr/bin/env bash
set -euo pipefail
RAPISCM_ROOT="$(cd "$(dirname "$0")" && pwd)"
exec "${RAPISCM_ROOT}/scripts/install" "$@"
