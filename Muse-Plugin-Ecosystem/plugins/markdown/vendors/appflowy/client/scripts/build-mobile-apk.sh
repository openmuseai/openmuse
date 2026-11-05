#!/usr/bin/env bash
# Shared final APK step for wrapper, cargo-make and CI. No .env mutation.
# Local by default (no dart-define cloud profile). Optional: MUSE_MOBILE_CONFIG_FILE.
set -euo pipefail
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"
muse_export_toolchain
MODE="${1:-debug}"
case "$MODE" in debug|release) ;; *) echo 'Expected debug or release' >&2; exit 64 ;; esac
cd "$(muse_flutter_dir)"
flutter pub get
RELEASE_ARGS=()
if [[ "$MODE" == release ]]; then
  RELEASE_ARGS=(--obfuscate --split-debug-info=build/mobile-symbols)
fi
DEFINE_ARGS=()
if [[ -n "${MUSE_MOBILE_CONFIG_FILE:-}" && -f "${MUSE_MOBILE_CONFIG_FILE}" ]]; then
  RESOLVED_PROFILE="$(mktemp -t muse-mobile-profile.XXXXXXXX)"
  trap 'rm -f "$RESOLVED_PROFILE"' EXIT
  dart "$(muse_flutter_dir)/tool/prepare_mobile_config.dart" "$MUSE_MOBILE_CONFIG_FILE" > "$RESOLVED_PROFILE"
  DEFINE_ARGS=(--dart-define-from-file="$RESOLVED_PROFILE")
fi
flutter build apk "--$MODE" --target-platform android-arm64 --split-per-abi \
  ${DEFINE_ARGS[@]+"${DEFINE_ARGS[@]}"} ${RELEASE_ARGS[@]+"${RELEASE_ARGS[@]}"}
