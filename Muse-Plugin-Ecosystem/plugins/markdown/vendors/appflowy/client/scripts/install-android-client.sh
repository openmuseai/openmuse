#!/usr/bin/env bash
# Install the last built Muse Android APK via adb.
#
# Usage:
#   frontend/client/scripts/install-android-client.sh [path-to.apk]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

ROOT="$(muse_root)"
APK="${1:-}"
if [[ -z "$APK" ]]; then
  DIST_ANDROID="$(muse_dist_dir)/android"
  CATALOG="$(ls -t "${DIST_ANDROID}/${BRAND_ARTIFACT_PREFIX}"-*-android-arm64.apk 2>/dev/null | head -n 1 || true)"
  if [[ -n "$CATALOG" && -f "$CATALOG" ]]; then
    APK="$CATALOG"
  elif [[ -f "${DIST_ANDROID}/${BRAND_ARTIFACT_PREFIX}-android-release.apk" ]]; then
    APK="${DIST_ANDROID}/${BRAND_ARTIFACT_PREFIX}-android-release.apk"
  else
    APK="${DIST_ANDROID}/${BRAND_ARTIFACT_PREFIX}-android-debug.apk"
  fi
fi
if [[ ! -f "$APK" ]]; then
  echo "APK not found: $APK (run frontend/client/scripts/pack-android-client.py first)" >&2
  exit 1
fi
if ! command -v adb >/dev/null 2>&1; then
  echo "adb not on PATH" >&2
  exit 1
fi

echo "==> adb install -r $APK"
adb install -r "$APK"
