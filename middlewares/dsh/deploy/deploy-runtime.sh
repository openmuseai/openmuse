#!/bin/bash
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

ENV_ARGS=()
if [[ -n "${DEPLOY_ENV:-}" ]]; then
  ENV_ARGS+=(--env "${DEPLOY_ENV}")
fi

echo "=========================================="
echo "  Muse Remote DSH - Runtime Setup"
echo "=========================================="

python3 scripts/deploy.py "${ENV_ARGS[@]}" runtime "$@"
