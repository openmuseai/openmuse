#!/usr/bin/env bash
# Local Docker (production-like) Remote DSH on 127.0.0.1:3080.
# Does not SSH. Does not bind 0.0.0.0:3080.
#
# Usage:
#   ./middlewares/dsh/deploy/dev-up.sh           # build if needed, compose up, wait
#   ./middlewares/dsh/deploy/dev-up.sh --rebuild # always rebuild image
#   ./middlewares/dsh/deploy/dev-up.sh --down
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../../.." && pwd)"
COMPOSE_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
COMPOSE_FILE="${COMPOSE_DIR}/docker-compose.yml"
TAG="${MUSE_DSH_IMAGE:-muse-dsh:local}"
DSH_PORT="${DSH_PORT:-3080}"
PLATFORM="${DEPLOY_PLATFORM:-}"
REBUILD=false
DOWN=false

while [[ $# -gt 0 ]]; do
  case "$1" in
    --rebuild) REBUILD=true ;;
    --down) DOWN=true ;;
    -h|--help)
      sed -n '2,12p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
  shift
done

if [[ "${DOWN}" == true ]]; then
  (cd "${COMPOSE_DIR}" && docker compose down)
  exit 0
fi

if ! docker info >/dev/null 2>&1; then
  echo "Docker daemon is not running. Start Docker Desktop and retry." >&2
  exit 1
fi

if [[ -z "${PLATFORM}" ]]; then
  case "$(uname -m)" in
    arm64|aarch64) PLATFORM=linux/arm64 ;;
    x86_64|amd64) PLATFORM=linux/amd64 ;;
    *) PLATFORM=linux/amd64 ;;
  esac
fi

ENV_FILE="${COMPOSE_DIR}/.env"
if [[ ! -f "${ENV_FILE}" ]]; then
  local_env=""
  if [[ -f "${ROOT}/.env.dsh.local" ]]; then
    local_env="${ROOT}/.env.dsh.local"
  elif [[ -f "${ROOT}/local/.env.dsh.local" ]]; then
    local_env="${ROOT}/local/.env.dsh.local"
  fi
  if [[ -n "${local_env}" ]]; then
    cp "${local_env}" "${ENV_FILE}"
    chmod 600 "${ENV_FILE}"
    echo "wrote ${ENV_FILE} from a local env file (not printed)"
  else
    echo "Missing ${ENV_FILE}." >&2
    echo "Create it with DEEPSEEK_API_KEY=... (do not commit)." >&2
    exit 1
  fi
fi
if grep -Eq '^DEEPSEEK_API_KEY=$' "${ENV_FILE}" || ! grep -Eq '^DEEPSEEK_API_KEY=.+' "${ENV_FILE}"; then
  echo "DEEPSEEK_API_KEY is empty in ${ENV_FILE}" >&2
  exit 1
fi
if ! grep -q '^MUSE_DOCUMENT_CLOUD_URL=' "${ENV_FILE}"; then
  printf '\nMUSE_DOCUMENT_CLOUD_URL=http://host.docker.internal:8000\n' >> "${ENV_FILE}"
fi

NEED_BUILD=false
if [[ "${REBUILD}" == true ]]; then
  NEED_BUILD=true
elif ! docker image inspect "${TAG}" >/dev/null 2>&1; then
  NEED_BUILD=true
fi
if [[ "${NEED_BUILD}" == true ]]; then
  echo "==> Building ${TAG} for ${PLATFORM}"
  "${ROOT}/middlewares/scripts/build-dsh-image.sh" --load --tag "${TAG}" --platform "${PLATFORM}"
fi

export MUSE_DSH_IMAGE="${TAG}"
export DSH_PORT
export DSH_ENV_FILE="${ENV_FILE}"
echo "==> Starting muse-dsh on 127.0.0.1:${DSH_PORT}"
(
  cd "${COMPOSE_DIR}"
  docker compose up -d
)

echo "==> Waiting for container muse-dsh + http://127.0.0.1:${DSH_PORT}/"
ok=0
for i in $(seq 1 40); do
  status="$(docker inspect -f '{{.State.Status}} {{.State.Restarting}}' muse-dsh 2>/dev/null || true)"
  ports="$(docker port muse-dsh 2>/dev/null || true)"
  if [[ "${status}" == "running false" ]] \
    && echo "${ports}" | grep -q "127.0.0.1:${DSH_PORT}" \
    && ! echo "${ports}" | grep -qE '0\.0\.0\.0:' \
    && curl -sf "http://127.0.0.1:${DSH_PORT}/" >/dev/null 2>&1; then
    ok=$((ok + 1))
    if [[ "${ok}" -ge 3 ]]; then
      echo "DSH is responding on 127.0.0.1:${DSH_PORT}"
      echo "${ports}"
      docker ps --filter "name=muse-dsh" --format '{{.Names}} {{.Status}} {{.Ports}}'
      exit 0
    fi
  else
    ok=0
  fi
  sleep 3
done
echo "DSH did not become ready. docker logs muse-dsh:" >&2
docker inspect -f 'status={{.State.Status}} restarting={{.State.Restarting}}' muse-dsh 2>/dev/null >&2 || true
docker port muse-dsh >&2 || true
docker logs muse-dsh 2>&1 | tail -80 >&2
exit 1
