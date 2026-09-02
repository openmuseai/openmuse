#!/bin/bash
set -euo pipefail

APP_DIR="${APP_DIR:-/opt/muse-dsh}"
DSH_PORT="${DSH_PORT:-3080}"
IMAGE_TAR="${IMAGE_TAR:-/tmp/muse-dsh-image.tar.gz}"
IMAGE_NAME="${IMAGE_NAME:-muse-dsh:local}"
COMPOSE_FILE="${APP_DIR}/runtime/docker-compose.yml"

if [[ ! -f "${APP_DIR}/.infra-ready" ]]; then
  echo "ERROR: Infra not ready. Run deploy-infra.sh first." >&2
  exit 1
fi
if [[ ! -f "${IMAGE_TAR}" ]]; then
  echo "ERROR: ${IMAGE_TAR} not found." >&2
  exit 1
fi
if [[ ! -f "${APP_DIR}/.env" ]]; then
  echo "ERROR: ${APP_DIR}/.env missing." >&2
  exit 1
fi
if grep -q '^DEEPSEEK_API_KEY=$' "${APP_DIR}/.env" || grep -q '^DEEPSEEK_API_KEY=\s*$' "${APP_DIR}/.env"; then
  echo "ERROR: DEEPSEEK_API_KEY is empty in ${APP_DIR}/.env" >&2
  exit 1
fi

echo "==> Loading DSH image"
if [[ "${IMAGE_TAR}" == *.gz ]]; then
  gzip -dc "${IMAGE_TAR}" | docker load
else
  docker load -i "${IMAGE_TAR}"
fi

mkdir -p "${APP_DIR}/runtime"
if [[ ! -f "${COMPOSE_FILE}" ]]; then
  echo "ERROR: ${COMPOSE_FILE} missing (upload docker-compose.yml with runtime)." >&2
  exit 1
fi

echo "==> Starting muse-dsh on 127.0.0.1:${DSH_PORT}"
export DSH_PORT IMAGE_NAME
export MUSE_DSH_IMAGE="${IMAGE_NAME}"
export DSH_ENV_FILE="${APP_DIR}/.env"
(
  cd "${APP_DIR}/runtime"
  docker compose --env-file "${APP_DIR}/.env" up -d
)

touch "${APP_DIR}/.runtime-ready"
echo "==> Waiting for loopback DSH..."
for i in $(seq 1 40); do
  if curl -sf "http://127.0.0.1:${DSH_PORT}/" >/dev/null 2>&1; then
    echo "DSH is responding on 127.0.0.1:${DSH_PORT}"
    break
  fi
  if [[ "$i" -eq 40 ]]; then
    echo "WARNING: DSH not responding yet. docker logs muse-dsh" >&2
  fi
  sleep 3
done

docker ps --filter "name=muse-dsh"
