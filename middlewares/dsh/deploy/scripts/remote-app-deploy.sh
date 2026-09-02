#!/bin/bash
set -euo pipefail

APP_DIR="${APP_DIR:-/opt/muse-dsh}"
DSH_PORT="${DSH_PORT:-3080}"
IMAGE_TAR="${IMAGE_TAR:-/tmp/muse-dsh-image.tar.gz}"
IMAGE_NAME="${IMAGE_NAME:-muse-dsh:local}"

if [[ ! -f "${APP_DIR}/.runtime-ready" ]]; then
  echo "ERROR: Runtime not ready. Run deploy-runtime.sh first." >&2
  exit 1
fi
if [[ ! -f "${IMAGE_TAR}" ]]; then
  echo "ERROR: ${IMAGE_TAR} not found." >&2
  exit 1
fi

echo "==> Loading updated DSH image"
if [[ "${IMAGE_TAR}" == *.gz ]]; then
  gzip -dc "${IMAGE_TAR}" | docker load
else
  docker load -i "${IMAGE_TAR}"
fi

export MUSE_DSH_IMAGE="${IMAGE_NAME}"
export DSH_ENV_FILE="${APP_DIR}/.env"
export DSH_PORT
(
  cd "${APP_DIR}/runtime"
  docker compose --env-file "${APP_DIR}/.env" up -d
)

echo "==> Waiting for loopback DSH..."
for i in $(seq 1 40); do
  if curl -sf "http://127.0.0.1:${DSH_PORT}/" >/dev/null 2>&1; then
    echo "DSH is responding"
    break
  fi
  if [[ "$i" -eq 40 ]]; then
    echo "WARNING: DSH may not be ready. docker logs muse-dsh" >&2
  fi
  sleep 3
done
