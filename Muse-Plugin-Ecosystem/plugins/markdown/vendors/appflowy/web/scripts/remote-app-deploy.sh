#!/bin/bash
set -euo pipefail

# Deploy build artifacts only (dist/) into mounted volume
DOMAIN="${DOMAIN:-app.openmuseai.com}"
APP_DIR="${APP_DIR:-/opt/appflowy-web}"
WEB_PORT="${WEB_PORT:-30014}"
CONTAINER_NAME="${CONTAINER_NAME:-appflowy-web-app}"
ARTIFACT_TAR="${ARTIFACT_TAR:-/tmp/appflowy-web-artifacts.tar.gz}"

if [ ! -f "${ARTIFACT_TAR}" ]; then
  echo "ERROR: ${ARTIFACT_TAR} not found."
  exit 1
fi

if [ ! -f "${APP_DIR}/.runtime-ready" ]; then
  echo "ERROR: Runtime not ready. Run ./deploy-runtime.sh first."
  exit 1
fi

echo "==> Deploying build artifacts..."
rm -rf "${APP_DIR}/dist"
mkdir -p "${APP_DIR}/dist"
tar -xzf "${ARTIFACT_TAR}" -C "${APP_DIR}"
rm -f "${ARTIFACT_TAR}"

if [ ! -f "${APP_DIR}/dist/index.html" ]; then
  echo "ERROR: dist/index.html not found after extraction."
  exit 1
fi

echo "==> Reloading runtime container..."
docker restart "${CONTAINER_NAME}"

echo "==> Waiting for AppFlowy Web to start..."
for i in $(seq 1 30); do
  if curl -sf "http://127.0.0.1:${WEB_PORT}/" > /dev/null 2>&1; then
    echo "AppFlowy Web is running!"
    break
  fi
  if [ "$i" -eq 30 ]; then
    echo "WARNING: AppFlowy Web may not be ready yet. Check logs: docker logs ${CONTAINER_NAME}"
  fi
  sleep 2
done

echo "==> App deployment complete!"
echo "    Site: https://${DOMAIN}"
docker ps --filter "name=${CONTAINER_NAME}"
