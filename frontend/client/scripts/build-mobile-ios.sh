#!/usr/bin/env bash
# Shared Flutter iOS step for cargo-make and the Muse iOS wrapper.
# Local by default. Optional: MUSE_MOBILE_CONFIG_FILE for Cloud dart-defines.
#
# Usage:
#   frontend/client/scripts/build-mobile-ios.sh [debug|release]
# Env:
#   MUSE_IOS_SIMULATOR=1   build for the iOS Simulator
#   MUSE_IOS_CODESIGN=1    allow Xcode automatic signing (default: --no-codesign)
#   MUSE_MOBILE_CONFIG_FILE  optional public Cloud + DSH JSON
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"
muse_export_toolchain
MODE="${1:-debug}"
case "$MODE" in debug|release) ;; *) echo 'Expected debug or release' >&2; exit 64 ;; esac
cd "$(muse_flutter_dir)"
flutter pub get
(cd ios && pod install --silent)

SIM="${MUSE_IOS_SIMULATOR:-0}"
CODESIGN="${MUSE_IOS_CODESIGN:-0}"
COMMON=()
if [[ -n "${MUSE_MOBILE_CONFIG_FILE:-}" && -f "${MUSE_MOBILE_CONFIG_FILE}" ]]; then
  RESOLVED_PROFILE="$(mktemp -t muse-mobile-profile.XXXXXXXX)"
  trap 'rm -f "$RESOLVED_PROFILE"' EXIT
  dart "$(muse_flutter_dir)/tool/prepare_mobile_config.dart" "$MUSE_MOBILE_CONFIG_FILE" > "$RESOLVED_PROFILE"
  COMMON+=(--dart-define-from-file="$RESOLVED_PROFILE")
fi
if [[ "$CODESIGN" != "1" ]]; then
  COMMON+=(--no-codesign)
fi

if [[ "$SIM" == "1" ]]; then
  if [[ "$MODE" != debug ]]; then
    echo "Simulator builds are debug only" >&2
    exit 64
  fi
  flutter build ios --simulator --debug "${COMMON[@]}"
  exit 0
fi

if [[ "$MODE" == release && "$CODESIGN" == "1" ]]; then
  flutter build ipa --release --obfuscate --split-debug-info=build/ios-symbols "${COMMON[@]}"
  exit 0
fi

flutter build ios "--$MODE" "${COMMON[@]}"
