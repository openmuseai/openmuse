#!/bin/bash
# Setup runtime container once (nginx + volume mount)
# Usage: ./deploy-runtime.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

ENV_ARGS=()
if [[ -n "${DEPLOY_ENV:-}" ]]; then
  ENV_ARGS+=(--env "${DEPLOY_ENV}")
fi

echo "=========================================="
echo "  AppFlowy Web - Runtime Setup"
echo "=========================================="
echo ""

python3 scripts/deploy.py "${ENV_ARGS[@]}" runtime "$@"
