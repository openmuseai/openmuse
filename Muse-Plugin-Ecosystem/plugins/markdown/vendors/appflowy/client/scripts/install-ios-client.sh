#!/usr/bin/env bash
# Install the last built Muse iOS app onto the iOS Simulator.
# Boots a simulator if none is running. Unsigned device IPA is sideload-only.
#
# Usage:
#   frontend/client/scripts/install-ios-client.sh [path-to.app]
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

ROOT="$(muse_root)"
APP="${1:-}"
if [[ -z "$APP" ]]; then
  APP="$(muse_dist_dir)/ios/${BRAND_ARTIFACT_PREFIX}-ios-simulator-debug.app"
  if [[ ! -d "$APP" ]]; then
    APP="$(muse_dist_dir)/ios/${BRAND_ARTIFACT_PREFIX}-ios-debug.app"
  fi
fi
if [[ ! -d "$APP" ]]; then
  echo "iOS app not found: $APP (run frontend/client/scripts/build-ios-client.sh --debug --simulator first)" >&2
  exit 1
fi
if ! command -v xcrun >/dev/null 2>&1; then
  echo "xcrun not on PATH" >&2
  exit 1
fi

boot_simulator_if_needed() {
  if xcrun simctl list devices | grep -q '(Booted)'; then
    return 0
  fi
  local udid=""
  udid="$(xcrun simctl list devices available | sed -n 's/.*iPhone[^(]*(\([A-F0-9-]\{36\}\)).*/\1/p' | head -n 1)"
  if [[ -z "$udid" ]]; then
    udid="$(xcrun simctl list devices available | sed -n 's/.*(\([A-F0-9-]\{36\}\)).*/\1/p' | head -n 1)"
  fi
  if [[ -z "$udid" ]]; then
    echo "No iOS Simulator runtime/device is available. Open Xcode > Settings > Components and install iOS." >&2
    exit 1
  fi
  echo "==> booting simulator $udid"
  open -a Simulator
  xcrun simctl boot "$udid" >/dev/null 2>&1 || true
  xcrun simctl bootstatus "$udid" -b
}

boot_simulator_if_needed

echo "==> xcrun simctl install booted $APP"
xcrun simctl install booted "$APP"

BUNDLE_ID="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$APP/Info.plist" 2>/dev/null || true)"
if [[ -n "$BUNDLE_ID" ]]; then
  echo "==> xcrun simctl launch booted $BUNDLE_ID"
  xcrun simctl launch booted "$BUNDLE_ID"
fi
