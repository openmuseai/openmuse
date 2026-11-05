#!/usr/bin/env bash
# Build the DSH Office Android client (Flutter + rust-lib via cargo-ndk).
# Fails closed if NDK / cargo-ndk / Flutter 3.27 is missing. Does not embed Node/DSH.
#
# Usage:
#   frontend/client/scripts/build-android-client.sh [--debug|--release] [--skip-packages] [--skip-core]
#   --mobile-config <json> is optional (Cloud endpoints). Default is local, no account.
#   --application-id io.appflowy.appflowy builds a data-preserving legacy update.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"
# shellcheck source=lib/android-cargokit.sh
source "${SCRIPT_DIR}/lib/android-cargokit.sh"

MODE=debug
SKIP_PACKAGES=false
SKIP_CORE=false
DSH_PUBLIC_URL="${MUSE_DSH_PUBLIC_URL:-${DSH_PUBLIC_URL:-}}"
MOBILE_CONFIG="${MUSE_MOBILE_CONFIG_FILE:-}"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug) MODE=debug ;;
    --release) MODE=release ;;
    --skip-packages) SKIP_PACKAGES=true ;;
    --skip-core) SKIP_CORE=true ;;
    --mobile-config)
      shift
      MOBILE_CONFIG="${1:?--mobile-config requires a JSON path}"
      ;;
    --application-id)
      shift
      export MUSE_ANDROID_APPLICATION_ID="${1:?--application-id requires a package id}"
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
      sed -n '2,10p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
  shift
done

muse_export_toolchain
muse_require_flutter_327

# Flutter may prefer Android Studio's JBR 21 over JAVA_HOME. Select JDK 17
# consistently, without changing the user's global Flutter settings.
if [[ -z "${JAVA_HOME:-}" && "$(uname -s)" == Darwin ]]; then
  JAVA_HOME="$(/usr/libexec/java_home -v 17 2>/dev/null || true)"
fi
if [[ -z "${JAVA_HOME:-}" || ! -x "${JAVA_HOME}/bin/java" ]] || \
   ! "${JAVA_HOME}/bin/java" -version 2>&1 | grep -q 'version "17\.'; then
  echo "JDK 17 is required for this Android build. Set JAVA_HOME to a JDK 17 installation." >&2
  exit 1
fi
export JAVA_HOME
printf -v MUSE_GRADLE_JAVA_OPT '%q' "-Dorg.gradle.java.home=${JAVA_HOME}"
export GRADLE_OPTS="${GRADLE_OPTS:-} ${MUSE_GRADLE_JAVA_OPT}"
export GRADLE_USER_HOME="${GRADLE_USER_HOME:-${HOME}/.gradle}"
if [[ "$GRADLE_USER_HOME" == '~/'* ]]; then
  export GRADLE_USER_HOME="${HOME}/${GRADLE_USER_HOME#\~/}"
fi
echo "JDK: $JAVA_HOME"

export MUSE_DSH_PUBLIC_URL="$DSH_PUBLIC_URL"

ROOT="$(muse_root)"
FRONTEND="$(muse_appflowy_frontend)"
FLUTTER_DIR="$(muse_flutter_dir)"
if [[ -n "$MOBILE_CONFIG" ]]; then
  export MUSE_MOBILE_CONFIG_FILE="$(cd "$(dirname "$MOBILE_CONFIG")" && pwd)/$(basename "$MOBILE_CONFIG")"
  dart "$FLUTTER_DIR/tool/prepare_mobile_config.dart" "$MUSE_MOBILE_CONFIG_FILE"
else
  unset MUSE_MOBILE_CONFIG_FILE
fi

if [[ -z "${ANDROID_NDK_HOME:-}" ]]; then
  if [[ -n "${ANDROID_HOME:-}" && -d "${ANDROID_HOME}/ndk" ]]; then
    # Prefer the version pinned in android/app/build.gradle when present.
    if [[ -d "${ANDROID_HOME}/ndk/24.0.8215888" ]]; then
      export ANDROID_NDK_HOME="${ANDROID_HOME}/ndk/24.0.8215888"
    else
      ANDROID_NDK_HOME="$(find "${ANDROID_HOME}/ndk" -mindepth 1 -maxdepth 1 -type d | sort | tail -n 1 || true)"
      export ANDROID_NDK_HOME
    fi
  fi
fi
if [[ -z "${ANDROID_NDK_HOME:-}" || ! -d "${ANDROID_NDK_HOME}" ]]; then
  echo "ANDROID_NDK_HOME is not set or not a directory. Android NDK is required (see frontend/client/frontend/appflowy_flutter/android/README.md)." >&2
  exit 1
fi
if ! command -v cargo-ndk >/dev/null 2>&1 && ! cargo ndk --help >/dev/null 2>&1; then
  echo "cargo-ndk is required: cargo install cargo-ndk" >&2
  exit 1
fi
export RUST_COMPILE_TARGET="${RUST_COMPILE_TARGET:-aarch64-linux-android}"
if ! (cd "$FRONTEND" && rustup target list --installed) | grep -Fxq "$RUST_COMPILE_TARGET"; then
  echo "Rust target $RUST_COMPILE_TARGET is required: rustup target add $RUST_COMPILE_TARGET" >&2
  exit 1
fi

PROFILE="development-android"
MAKE_TASK="appflowy-android-dev"
if [[ "$MODE" == release ]]; then
  PROFILE="production-android"
  MAKE_TASK="appflowy-android"
fi

if [[ "$SKIP_PACKAGES" == false ]]; then
  "$(muse_script_build_packages)" --skip-tests
fi

# irondash 0.5.5 CargoKit does not attach to the legacy Flutter Gradle plugin.
# Build that cdylib (and super_native if needed) into app jniLibs before APK.
muse_android_ensure_plugin_cdylibs

if [[ "$SKIP_CORE" == false ]]; then
  echo "==> Building rust-lib for Android ($PROFILE, cargo-ndk arm64-v8a)"
  (
    cd "$FRONTEND"
    cargo make --profile "$PROFILE" "$MAKE_TASK"
  )
else
  echo "==> Skipping cargo make; flutter build apk only"
fi

# rust-lib copy wipes plugin jni; restage then produce the APK that pack verifies.
muse_android_stage_pack_jni
bash "$(muse_script_build_mobile_apk)" "$MODE"

APK="$FLUTTER_DIR/build/app/outputs/flutter-apk/app-arm64-v8a-${MODE}.apk"
if [[ ! -f "$APK" ]]; then
  echo "expected APK missing after Android build" >&2
  exit 1
fi

# CargoKit can silently skip plugins when its Flutter integration is
# incompatible. Check these required native libraries as well as the core.
for native_lib in libdart_ffi.so libc++_shared.so libirondash_engine_context_native.so libsuper_native_extensions.so; do
  if ! unzip -Z1 "$APK" | awk -v required="lib/arm64-v8a/$native_lib" '$0 == required { found = 1 } END { exit !found }'; then
    echo "APK is missing lib/arm64-v8a/$native_lib" >&2
    exit 1
  fi
done

DEST_DIR="$(muse_dist_dir)/android"
mkdir -p "$DEST_DIR"
PACKAGE_SUFFIX="${MUSE_ANDROID_APPLICATION_ID:+-${MUSE_ANDROID_APPLICATION_ID}}"
DEST="$DEST_DIR/${BRAND_ARTIFACT_PREFIX}-android-${MODE}${PACKAGE_SUFFIX}.apk"
cp "$APK" "$DEST"
echo "Built $DEST (arm64-v8a; Rust FFI verified)"
