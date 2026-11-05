#!/bin/bash
# Deploy build artifacts only (local pnpm build → upload dist)
# Usage: DEPLOY_ENV=production ./deploy.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

ENV_ARGS=()
if [[ -n "${DEPLOY_ENV:-}" ]]; then
  ENV_ARGS+=(--env "${DEPLOY_ENV}")
fi

echo "=========================================="
echo "  AppFlowy Web - Artifact Deploy"
echo "=========================================="
echo ""

python3 scripts/deploy.py "${ENV_ARGS[@]}" app "$@"
