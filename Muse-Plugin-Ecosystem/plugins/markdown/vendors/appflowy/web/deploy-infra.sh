#!/bin/bash
# Setup AppFlowy Web environment (.env with API URLs)
# Usage: ./deploy-infra.sh
# Note: nginx routing is configured by AppFlowy-Cloud/deploy-infra.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$SCRIPT_DIR"

ENV_ARGS=()
if [[ -n "${DEPLOY_ENV:-}" ]]; then
  ENV_ARGS+=(--env "${DEPLOY_ENV}")
fi

echo "=========================================="
echo "  AppFlowy Web - Infra Setup"
echo "=========================================="
echo ""

python3 scripts/deploy.py "${ENV_ARGS[@]}" infra "$@"
