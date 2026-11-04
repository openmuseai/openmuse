#!/usr/bin/env bash
# Rebuild the fixed PDFium fork from the local gclient checkout.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SOURCE="${KSP_PDFIUM_SOURCE:-$ROOT/3rdparty/src/pdfium}"
GN="${KSP_GN:-$ROOT/3rdparty/src/gn/out/gn}"
NINJA="${KSP_NINJA:-ninja}"
BASE_REVISION="784a524ddaa26d86c0f499625b97902095c26dfe"
TARGET="${1:-}"

if [[ ! -d "$SOURCE/.git" || ! -x "$GN" ]]; then
  echo "PDFium gclient source checkout and GN binary are required; see 3rdparty/prebuilt/PRODUCT_BINARIES.md" >&2
  exit 2
fi
if [[ "$(git -C "$SOURCE" rev-parse HEAD)" != "$BASE_REVISION" ]]; then
  echo "PDFium source revision differs from the pinned product base" >&2
  exit 2
fi
if ! git -C "$SOURCE" apply --reverse --check \
    "$ROOT/3rdparty/patches/pdfium-product.patch"; then
  echo "Missing or changed PDFium product patch: pdfium-product.patch" >&2
  exit 2
fi

COMMON='is_debug=false is_component_build=false pdf_enable_v8=false pdf_enable_xfa=false pdf_is_standalone=true pdf_use_partition_alloc=false use_remoteexec=false use_siso=false clang_use_chrome_plugins=false treat_warnings_as_errors=false target_cpu="arm64"'
case "$TARGET" in
  macos-arm64)
    OUT=ksp-mac
    ARGS="$COMMON target_os=\"mac\" use_sysroot=false use_system_xcode=true"
    LIB=libpdfium.dylib
    ;;
  macos-viewer-arm64)
    OUT=ksp-mac-viewer
    ARGS="$COMMON target_os=\"mac\" use_sysroot=false use_system_xcode=true ksp_pdfium_viewer=true"
    LIB=libpdfium_viewer.dylib
    ;;
  android-arm64)
    OUT=ksp-android-arm64
    ARGS="$COMMON target_os=\"android\" default_min_sdk_version=23"
    LIB=libpdfium.so
    ;;
  android-viewer-arm64)
    OUT=ksp-android-viewer-arm64
    ARGS="$COMMON target_os=\"android\" default_min_sdk_version=23 ksp_pdfium_viewer=true"
    LIB=libpdfium_viewer.so
    ;;
  ios-simulator-arm64)
    OUT=ksp-ios-simulator
    ARGS="$COMMON target_os=\"ios\" target_environment=\"simulator\" ios_deployment_target=\"13.0\" use_system_xcode=true use_blink=true"
    LIB=libpdfium.dylib
    ;;
  *)
    echo "Usage: $0 {macos-arm64|macos-viewer-arm64|android-arm64|android-viewer-arm64|ios-simulator-arm64}" >&2
    exit 2
    ;;
esac

(
  cd "$SOURCE"
  "$GN" gen "out/$OUT" --args="$ARGS"
  "$NINJA" -C "out/$OUT" pdfium
)
if [[ ! -f "$SOURCE/out/$OUT/$LIB" ]]; then
  echo "PDFium build completed without $LIB" >&2
  exit 2
fi
if [[ "$TARGET" == android-arm64 || "$TARGET" == android-viewer-arm64 ]]; then
  case "$(uname -s)" in
    Darwin) HOST_TAG=darwin-x86_64 ;;
    Linux) HOST_TAG=linux-x86_64 ;;
    *) echo "Unsupported Android build host" >&2; exit 2 ;;
  esac
  SDK="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
  NDK="${ANDROID_NDK_HOME:-$SDK/ndk/27.0.12077973}"
  NM="${KSP_ANDROID_NM:-$NDK/toolchains/llvm/prebuilt/$HOST_TAG/bin/llvm-nm}"
  "$NM" -D "$SOURCE/out/$OUT/$LIB" | grep ' KSPTextObj_SetGlyphPositions$' > /dev/null
else
  nm -gU "$SOURCE/out/$OUT/$LIB" | grep '_KSPTextObj_SetGlyphPositions$' > /dev/null
fi
shasum -a 256 "$SOURCE/out/$OUT/$LIB"
