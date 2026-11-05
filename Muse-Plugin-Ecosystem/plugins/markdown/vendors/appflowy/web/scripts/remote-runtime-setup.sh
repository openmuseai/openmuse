#!/bin/bash
set -euo pipefail

# First-time runtime setup: nginx container with volume-mounted dist (no app source on server)
DOMAIN="${DOMAIN:-app.openmuseai.com}"
APP_DIR="${APP_DIR:-/opt/appflowy-web}"
WEB_PORT="${WEB_PORT:-30014}"
CONTAINER_NAME="${CONTAINER_NAME:-appflowy-web-app}"
RUNTIME_IMAGE="${RUNTIME_IMAGE:-nginx:1.25-alpine}"

mkdir -p "${APP_DIR}/dist" "${APP_DIR}/runtime"

if [ ! -f "${APP_DIR}/runtime/nginx.conf" ]; then
  echo "ERROR: ${APP_DIR}/runtime/nginx.conf not found. Upload runtime config first."
  exit 1
fi

if ! docker image inspect "${RUNTIME_IMAGE}" > /dev/null 2>&1; then
  echo "==> Pulling runtime image..."
  docker pull docker.m.daocloud.io/library/nginx:1.25-alpine || docker pull nginx:1.25-alpine
  docker tag docker.m.daocloud.io/library/nginx:1.25-alpine "${RUNTIME_IMAGE}" 2>/dev/null \
    || docker tag nginx:1.25-alpine "${RUNTIME_IMAGE}" 2>/dev/null || true
fi

echo "==> Starting runtime container..."
docker rm -f "${CONTAINER_NAME}" 2>/dev/null || true
docker run -d \
  --name "${CONTAINER_NAME}" \
  -p 127.0.0.1:${WEB_PORT}:80 \
  -v "${APP_DIR}/dist:/usr/share/nginx/html:ro" \
  -v "${APP_DIR}/runtime/nginx.conf:/etc/nginx/nginx.conf:ro" \
  --restart always \
  "${RUNTIME_IMAGE}"

touch "${APP_DIR}/.runtime-ready"
echo "==> Runtime setup complete!"
echo "    Container: ${CONTAINER_NAME}"
echo "    Port:      ${WEB_PORT}"
echo "    Next:      run ./deploy.sh to upload build artifacts"
