#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
export GRADLE_USER_HOME="$repo_root/target/mobile-gradle-cache"
serial="${OPENMUSE_ANDROID_SERIAL:-}"
if [[ -z "$serial" ]]; then
  devices="$(adb devices | awk 'NR > 1 && $2 == "device" { print $1 }')"
  if [[ "$(printf '%s\n' "$devices" | sed '/^$/d' | wc -l | tr -d ' ')" -ne 1 ]]; then
    echo "Set OPENMUSE_ANDROID_SERIAL; expected exactly one ready ADB device" >&2
    exit 1
  fi
  serial="$devices"
fi

# The Android app links the DOCX engine during every native build.
OPENMUSE_DOCX_PLATFORMS=android "$repo_root/scripts/build_office_docx_mobile_artifacts.sh"
OPENMUSE_PAIRED_PLATFORMS=android "$repo_root/scripts/build_paired_relay_mobile_artifacts.sh"
(
  cd "$repo_root/app/openmuse_mobile"
  flutter test integration_test/device_keystore_test.dart -d "$serial"
)
