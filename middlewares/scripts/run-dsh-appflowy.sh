#!/usr/bin/env bash
# Start the patched DSH web sidecar for the AppFlowy desktop panel.
#
# Development: uses the source tree + pnpm (or nvm-resolved node).
# Packed .app: MUSE_BUNDLE_ROOT points at Contents/Resources/muse and this
# script execs the bundled Node (no user pnpm required).
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

ROOT="$(muse_root)"
export MUSE_ROOT="$ROOT"
HOST="${DSH_WEB_HOST:-127.0.0.1}"
PORT="${DSH_WEB_PORT:-3080}"

load_env_file() {
  local file="$1"
  if [[ -f "$file" ]]; then
    set -a
    # shellcheck disable=SC1090
    source "$file"
    set +a
  fi
}

load_api_key_files() {
  if [[ -n "${DEEPSEEK_API_KEY:-}" ]]; then
    return 0
  fi
  load_env_file "$1"
}

if [[ -n "${MUSE_BUNDLE_ROOT:-}" && -x "${MUSE_BUNDLE_ROOT}/node/bin/node" ]]; then
  BUNDLE="$MUSE_BUNDLE_ROOT"
  NODE="$BUNDLE/node/bin/node"
  HARNESS="$BUNDLE/dsh"
  PATCH="$BUNDLE/patch.yml"
  DSH_HOME="${DSH_HOME:-$HOME/Library/Application Support/AppFlowy/Muse/dsh}"
  export DSH_HOME
  load_api_key_files "$HOME/Library/Application Support/AppFlowy/Muse/credentials.env"
  if [[ -z "${DEEPSEEK_API_KEY:-}" ]]; then
    echo "DEEPSEEK_API_KEY is required (panel form or credentials.env)" >&2
    exit 1
  fi
  mkdir -p "$DSH_HOME/profiles/web/node_modules/@muse" \
           "$DSH_HOME/profiles/node_modules/@muse"
  # Link the copies that already sit under dsh/node_modules/@muse so ESM
  # realpath resolution can see sibling @muse/* and @deepseek-ai/*.
  if [[ -d "$HARNESS/node_modules/@muse" ]]; then
    for dest in \
      "$DSH_HOME/profiles/node_modules/@muse" \
      "$DSH_HOME/profiles/web/node_modules/@muse"; do
      for pkg in "$HARNESS/node_modules/@muse"/*; do
        [[ -e "$pkg" ]] || continue
        ln -sfn "$pkg" "$dest/$(basename "$pkg")"
      done
    done
  fi
  muse_link_dshmarket "$HARNESS" "$DSH_HOME/profiles/node_modules"
  muse_link_dshmarket "$HARNESS" "$DSH_HOME/profiles/web/node_modules"
  export MUSE_PLUGIN_DIAGNOSTICS="${MUSE_PLUGIN_DIAGNOSTICS:-1}"
  cd "$HARNESS"
  exec "$NODE" --import tsx/esm apps/cli/src/bin.ts \
    --profile web --patch "$PATCH" --host "$HOST" --port "$PORT"
fi

muse_ensure_node || exit 127
HARNESS="$(muse_harness_dir)"
PATCH="$(muse_dsh_patch)"
DSH_HOME="${DSH_HOME:-$HOME/.dsh}"

load_api_key_files "$ROOT/.env.dsh.local"
load_api_key_files "$HOME/Library/Application Support/AppFlowy/Muse/credentials.env"

if [[ -z "${DEEPSEEK_API_KEY:-}" ]]; then
  echo "DEEPSEEK_API_KEY is required (env, $ROOT/.env.dsh.local, or credentials.env)" >&2
  exit 1
fi

export MUSE_PLUGIN_DIAGNOSTICS="${MUSE_PLUGIN_DIAGNOSTICS:-1}"

muse_link_dsh_packages "$HARNESS/node_modules/@muse"
muse_link_dsh_packages "$DSH_HOME/profiles/node_modules/@muse"
muse_link_dsh_packages "$DSH_HOME/profiles/web/node_modules/@muse"
muse_stage_dshmarket "$HARNESS"
muse_link_dshmarket "$HARNESS" "$DSH_HOME/profiles/node_modules"
muse_link_dshmarket "$HARNESS" "$DSH_HOME/profiles/web/node_modules"
muse_stage_dsh_model_capabilities "$HARNESS"
muse_link_dsh_model_capabilities "$HARNESS" "$DSH_HOME/profiles/node_modules"
muse_link_dsh_model_capabilities "$HARNESS" "$DSH_HOME/profiles/web/node_modules"

cd "$HARNESS"
exec pnpm dsh --profile web --patch "$PATCH" --host "$HOST" --port "$PORT"
