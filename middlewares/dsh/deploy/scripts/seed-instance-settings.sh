#!/usr/bin/env bash
# Copy gateway default settings.yaml into a tenant DSH_HOME once, then fill
# any keys still missing from the defaults (never overwrite tenant values).
set -euo pipefail

DSH_HOME_DIR="${1:?DSH_HOME required}"
DEFAULTS="${2:?defaults settings.yaml required}"
DEST="${DSH_HOME_DIR}/settings.yaml"
MERGE="$(dirname "${BASH_SOURCE[0]}")/merge-default-settings.py"

if [[ ! -f "${DEFAULTS}" ]]; then
  echo "seed-instance-settings: skip (no defaults at ${DEFAULTS})"
  exit 0
fi
mkdir -p "${DSH_HOME_DIR}"
if [[ ! -e "${DEST}" ]]; then
  install -m 600 "${DEFAULTS}" "${DEST}"
  echo "seed-instance-settings: wrote ${DEST}"
  exit 0
fi
if [[ -f "${MERGE}" ]]; then
  python3 "${MERGE}" "${DEST}" "${DEFAULTS}"
else
  echo "seed-instance-settings: keep existing ${DEST}"
fi
