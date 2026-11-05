#!/usr/bin/env bash
# Build the DSH Office iOS client (Flutter + rust-lib via cargo --target).
# Fails closed if Xcode / Flutter 3.27 / iOS Rust target is missing.
# Does not embed Node/DSH. Default artifact is an unsigned .app plus a sideload IPA.
#
# Usage:
#   frontend/client/scripts/build-ios-client.sh [--debug|--release] [--simulator]
#                                        [--skip-packages] [--skip-core] [--codesign]
#   --mobile-config <json> is optional (Cloud endpoints). Default is local, no account.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

MODE=debug
SIMULATOR=false
SKIP_PACKAGES=false
SKIP_CORE=false
CODESIGN=false
DSH_PUBLIC_URL="${MUSE_DSH_PUBLIC_URL:-${DSH_PUBLIC_URL:-}}"
MOBILE_CONFIG="${MUSE_MOBILE_CONFIG_FILE:-}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug) MODE=debug ;;
    --release) MODE=release ;;
    --simulator) SIMULATOR=true ;;
    --skip-packages) SKIP_PACKAGES=true ;;
    --skip-core) SKIP_CORE=true ;;
    --codesign) CODESIGN=true ;;
    --mobile-config)
      shift
      MOBILE_CONFIG="${1:?--mobile-config requires a JSON path}"
      ;;
    --dsh-public-url)
      shift
      if [[ $# -eq 0 ]]; then
        echo "--dsh-public-url requires an HTTPS URL" >&2
        exit 1
      fi
      DSH_PUBLIC_URL="$1"
      ;;
    -h|--help)
      sed -n '2,12p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
  shift
done

if [[ "$(uname -s)" != Darwin ]]; then
  echo "iOS builds require macOS and Xcode." >&2
  exit 1
fi

muse_export_toolchain
muse_require_flutter_327

if ! xcodebuild -version >/dev/null 2>&1; then
  echo "Xcode is required (xcodebuild not found). Install Xcode and run xcode-select --install." >&2
  exit 1
fi
if ! xcrun simctl list runtimes 2>/dev/null | grep -q 'iOS'; then
  echo "Xcode iOS platform/runtime is not installed (needed for device and simulator builds)." >&2
  echo "Install with: xcodebuild -downloadPlatform iOS" >&2
  echo "Then re-run this script." >&2
  exit 1
fi

export MUSE_DSH_PUBLIC_URL="$DSH_PUBLIC_URL"
if [[ "$SIMULATOR" == true ]]; then
  export MUSE_IOS_SIMULATOR=1
else
  export MUSE_IOS_SIMULATOR=0
fi
if [[ "$CODESIGN" == true ]]; then
  export MUSE_IOS_CODESIGN=1
else
  export MUSE_IOS_CODESIGN=0
fi

ROOT="$(muse_root)"
FRONTEND="$(muse_appflowy_frontend)"
FLUTTER_DIR="$(muse_flutter_dir)"
if [[ -n "$MOBILE_CONFIG" ]]; then
  export MUSE_MOBILE_CONFIG_FILE="$(cd "$(dirname "$MOBILE_CONFIG")" && pwd)/$(basename "$MOBILE_CONFIG")"
  dart "$FLUTTER_DIR/tool/prepare_mobile_config.dart" "$MUSE_MOBILE_CONFIG_FILE"
else
  unset MUSE_MOBILE_CONFIG_FILE
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "cargo is required on PATH" >&2
  exit 1
fi

if [[ "$SIMULATOR" == true ]]; then
  RUST_COMPILE_TARGET="${RUST_COMPILE_TARGET:-aarch64-apple-ios-sim}"
  PROFILE="development-ios-arm64-sim"
  MAKE_TASK="appflowy-ios-dev"
else
  RUST_COMPILE_TARGET="${RUST_COMPILE_TARGET:-aarch64-apple-ios}"
  PROFILE="development-ios-arm64"
  MAKE_TASK="appflowy-ios-dev"
  if [[ "$MODE" == release ]]; then
    PROFILE="production-ios-arm64"
    MAKE_TASK="appflowy-ios"
  fi
fi
export RUST_COMPILE_TARGET
if ! (cd "$FRONTEND/rust-lib" && rustup target list --installed) | grep -Fxq "$RUST_COMPILE_TARGET"; then
  echo "Rust target $RUST_COMPILE_TARGET is required: rustup target add $RUST_COMPILE_TARGET" >&2
  exit 1
fi

if [[ "$SKIP_PACKAGES" == false ]]; then
  "$(muse_script_build_packages)" --skip-tests
fi

if [[ "$SKIP_CORE" == false ]]; then
  echo "==> Building rust-lib for iOS ($PROFILE, cargo --target $RUST_COMPILE_TARGET)"
  (
    cd "$FRONTEND"
    cargo make --profile "$PROFILE" "$MAKE_TASK"
  )
else
  echo "==> Skipping cargo make; flutter build ios only"
  bash "$(muse_script_build_mobile_ios)" "$MODE"
fi

FFI="$FLUTTER_DIR/packages/appflowy_backend/ios/libdart_ffi.a"
if [[ ! -f "$FFI" ]]; then
  echo "expected static library missing: $FFI" >&2
  exit 1
fi

DEST_DIR="$(muse_dist_dir)/ios"
mkdir -p "$DEST_DIR"

pack_unsigned_ipa() {
  local app="$1"
  local ipa="$2"
  local staging
  staging="$(mktemp -d "${TMPDIR:-/tmp}/muse-ios-ipa.XXXXXX")"
  mkdir -p "$staging/Payload"
  ditto "$app" "$staging/Payload/$(basename "$app")"
  rm -f "$ipa"
  (cd "$staging" && zip -qry "$ipa" Payload)
  rm -rf "$staging"
}

if [[ "$SIMULATOR" == true ]]; then
  APP="$FLUTTER_DIR/build/ios/iphonesimulator/Runner.app"
  if [[ ! -d "$APP" ]]; then
    echo "expected simulator app missing: $APP" >&2
    exit 1
  fi
  DEST_APP="$DEST_DIR/${BRAND_ARTIFACT_PREFIX}-ios-simulator-debug.app"
  rm -rf "$DEST_APP"
  ditto "$APP" "$DEST_APP"
  echo "Built $DEST_APP (simulator; libdart_ffi.a present)"
  exit 0
fi

if [[ "$MODE" == release && "$CODESIGN" == true ]]; then
  IPA="$(find "$FLUTTER_DIR/build/ios/ipa" -maxdepth 1 -name '*.ipa' -print -quit 2>/dev/null || true)"
  if [[ -z "$IPA" || ! -f "$IPA" ]]; then
    echo "expected signed IPA missing under $FLUTTER_DIR/build/ios/ipa" >&2
    exit 1
  fi
  DEST="$DEST_DIR/${BRAND_ARTIFACT_PREFIX}-ios-release.ipa"
  cp "$IPA" "$DEST"
  echo "Built $DEST (device; signed IPA; libdart_ffi.a present)"
  exit 0
fi

APP="$FLUTTER_DIR/build/ios/iphoneos/Runner.app"
if [[ ! -d "$APP" ]]; then
  echo "expected device app missing: $APP" >&2
  exit 1
fi
DEST_APP="$DEST_DIR/${BRAND_ARTIFACT_PREFIX}-ios-${MODE}.app"
rm -rf "$DEST_APP"
ditto "$APP" "$DEST_APP"
DEST_IPA="$DEST_DIR/${BRAND_ARTIFACT_PREFIX}-ios-${MODE}.ipa"
pack_unsigned_ipa "$DEST_APP" "$DEST_IPA"
echo "Built $DEST_APP and $DEST_IPA (device arm64; unsigned sideload IPA; libdart_ffi.a present)"
