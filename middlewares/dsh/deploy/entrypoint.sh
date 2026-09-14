#!/bin/bash
set -euo pipefail

: "${DSH_HOME:=/var/lib/muse-dsh}"
: "${PORT:=3080}"
: "${DSH_LOOPBACK_PORT:=13080}"
PATCH="${PATCH:-/muse/patch.yml}"
HARNESS="${HARNESS:-/muse/dsh}"

if [[ -z "${DEEPSEEK_API_KEY:-}" ]]; then
  echo "DEEPSEEK_API_KEY is required in the container environment (set via ${DSH_APP_DIR:-/opt/muse-dsh}/.env)." >&2
  exit 1
fi

mkdir -p \
  "${DSH_HOME}/profiles/web/node_modules/@muse" \
  "${DSH_HOME}/profiles/node_modules/@muse"

bash /muse/seed-instance-settings.sh \
  "${DSH_HOME}" \
  "${MUSE_DSH_DEFAULT_SETTINGS:-/muse/defaults/settings.yaml}"

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

if [[ -d "${HARNESS}/node_modules/dsh-model-capabilities" ]]; then
  mkdir -p "${DSH_HOME}/profiles/node_modules" "${DSH_HOME}/profiles/web/node_modules"
  ln -sfn "${HARNESS}/node_modules/dsh-model-capabilities" "${DSH_HOME}/profiles/node_modules/dsh-model-capabilities"
  ln -sfn "${HARNESS}/node_modules/dsh-model-capabilities" "${DSH_HOME}/profiles/web/node_modules/dsh-model-capabilities"
fi

if [[ -f /muse/wire-muse-node-modules.py && -d /opt/muse/packages ]]; then
  python3 /muse/wire-muse-node-modules.py "${HARNESS}" /opt/muse "$(command -v node)"
elif [[ -f /muse/wire-harness-aliases.py ]]; then
  python3 /muse/wire-harness-aliases.py "${HARNESS}"
else
  echo "missing Muse package wire scripts" >&2
  exit 1
fi

export MUSE_PLUGIN_DIAGNOSTICS="${MUSE_PLUGIN_DIAGNOSTICS:-1}"
export PORT
export DSH_LOOPBACK_PORT

TRUSTED_ARGS=()
if [[ -n "${DSH_TRUSTED_HOST:-}" ]]; then
  IFS=',' read -ra _dsh_hosts <<< "${DSH_TRUSTED_HOST}"
  for _h in "${_dsh_hosts[@]}"; do
    _h="${_h// /}"
    [[ -n "${_h}" ]] && TRUSTED_ARGS+=(--trusted-host "${_h}")
  done
fi

# Official CLI rejects --host 0.0.0.0. Proxy is reachable by docker-proxy;
# compose still publishes 127.0.0.1 on the host. nohup + disown so exec
# does not SIGHUP the proxy when the shell is replaced.
nohup node /muse/loopback-proxy.mjs >/tmp/loopback-proxy.log 2>&1 &
disown || true

cd "${HARNESS}"
exec node --import tsx/esm apps/cli/src/bin.ts \
  --profile web \
  --patch "${PATCH}" \
  --host 127.0.0.1 \
  --port "${DSH_LOOPBACK_PORT}" \
  "${TRUSTED_ARGS[@]}"
