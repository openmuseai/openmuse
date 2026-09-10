#!/bin/bash
# Host-side DSH instance for systemd-run. No Docker, no loopback-proxy:
# bind 127.0.0.1:$PORT; nginx /u/<hash>/ reaches this via the pool proxy.
set -euo pipefail

HARNESS="${HARNESS:-/opt/muse-dsh/runtime/dsh}"
PATCH="${PATCH:-/opt/muse-dsh/runtime/patch.yml}"
NODE="${MUSE_DSH_NODE_BIN:-/opt/muse-dsh/runtime/bin/node}"
ENV_FILE="${MUSE_DSH_INSTANCE_ENV_FILE:-/opt/muse-dsh/instance.env}"
INSTANCE_PORT="${PORT:-}"
INSTANCE_HOME="${DSH_HOME:-}"
if [[ -f "${ENV_FILE}" ]]; then
  set -a
  # shellcheck disable=SC1090
  source "${ENV_FILE}"
  set +a
fi
# Tenant identity comes from systemd-run, not the shared template.
if [[ -n "${INSTANCE_PORT}" ]]; then
  export PORT="${INSTANCE_PORT}"
fi
if [[ -n "${INSTANCE_HOME}" ]]; then
  export DSH_HOME="${INSTANCE_HOME}"
fi
: "${DSH_HOME:?DSH_HOME is required}"
: "${PORT:?PORT is required}"
: "${DEEPSEEK_API_KEY:?DEEPSEEK_API_KEY is required}"
if [[ -z "${MUSE_DOCUMENT_CLOUD_URL:-}" ]]; then
  echo "start-instance: MUSE_DOCUMENT_CLOUD_URL is unset; parent-bridge will not inject" >&2
fi

mkdir -p \
  "${DSH_HOME}/profiles/web/node_modules/@muse" \
  "${DSH_HOME}/profiles/node_modules/@muse"

if [[ -d "${HARNESS}/node_modules/@muse" ]]; then
  for dest in \
    "${DSH_HOME}/profiles/node_modules/@muse" \
    "${DSH_HOME}/profiles/web/node_modules/@muse"; do
    for pkg in "${HARNESS}/node_modules/@muse/"*; do
      [[ -e "$pkg" ]] || continue
      ln -sfn "$pkg" "${dest}/$(basename "$pkg")"
    done
  done
fi

if [[ -d "${HARNESS}/node_modules/dshmarket" ]]; then
  mkdir -p "${DSH_HOME}/profiles/node_modules" "${DSH_HOME}/profiles/web/node_modules"
  ln -sfn "${HARNESS}/node_modules/dshmarket" "${DSH_HOME}/profiles/node_modules/dshmarket"
  ln -sfn "${HARNESS}/node_modules/dshmarket" "${DSH_HOME}/profiles/web/node_modules/dshmarket"
fi

export MUSE_PLUGIN_DIAGNOSTICS="${MUSE_PLUGIN_DIAGNOSTICS:-1}"
export PORT
export DSH_LOOPBACK_PORT="${DSH_LOOPBACK_PORT:-$PORT}"
export MUSE_REQUIRE_HOST_AUTH="${MUSE_REQUIRE_HOST_AUTH:-1}"

TRUSTED_ARGS=()
if [[ -n "${DSH_TRUSTED_HOST:-}" ]]; then
  IFS=',' read -ra _dsh_hosts <<< "${DSH_TRUSTED_HOST}"
  for _h in "${_dsh_hosts[@]}"; do
    _h="${_h// /}"
    [[ -n "${_h}" ]] && TRUSTED_ARGS+=(--trusted-host "${_h}")
  done
fi

cd "${HARNESS}"
exec "${NODE}" --import tsx/esm apps/cli/src/bin.ts \
  --profile web \
  --patch "${PATCH}" \
  --host 127.0.0.1 \
  --port "${PORT}" \
  "${TRUSTED_ARGS[@]}"
