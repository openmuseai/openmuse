#!/bin/bash
set -euo pipefail

# Remote infrastructure setup for AppFlowy Web (.env only)
# Nginx routing is configured by AppFlowy-Cloud deploy-infra.sh
DOMAIN="${DOMAIN:-app.openmuseai.com}"
APP_DIR="${APP_DIR:-/opt/appflowy-web}"
WEB_PORT="${WEB_PORT:-30014}"

echo "==> Creating AppFlowy Web environment file..."
mkdir -p "${APP_DIR}"
cat > "${APP_DIR}/.env" << ENV_EOF
APPFLOWY_BASE_URL=https://${DOMAIN}
APPFLOWY_GOTRUE_BASE_URL=https://${DOMAIN}/gotrue
APPFLOWY_WS_BASE_URL=wss://${DOMAIN}/ws/v2
ENV_EOF

chmod 600 "${APP_DIR}/.env"

echo "==> Infrastructure setup complete!"
echo "    Domain:   https://${DOMAIN}"
echo "    App dir:  ${APP_DIR}"
echo "    Web port: ${WEB_PORT}"
echo ""
echo "Next step: run ./deploy-runtime.sh then ./deploy.sh"
