#!/usr/bin/env bash
# Thin wrapper: the packer is pack-macos-client.py (Windows-parity closure).
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "${SCRIPT_DIR}/pack-macos-client.py" "$@"
