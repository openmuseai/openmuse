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
  APK="$(muse_dist_dir)/android/dsh-office-android-debug.apk"
  if [[ ! -f "$APK" ]]; then
    APK="$(muse_dist_dir)/android/dsh-office-android-release.apk"
  fi
fi
if [[ ! -f "$APK" ]]; then
  echo "APK not found: $APK (run frontend/client/scripts/build-android-client.sh first)" >&2
  exit 1
fi
if ! command -v adb >/dev/null 2>&1; then
  echo "adb not on PATH" >&2
  exit 1
fi

echo "==> adb install -r $APK"
adb install -r "$APK"
