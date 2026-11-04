#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SDK="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
NDK="${ANDROID_NDK_HOME:-$SDK/ndk/27.0.12077973}"
case "$(uname -s)" in
  Darwin) HOST_TAG=darwin-x86_64 ;;
  Linux) HOST_TAG=linux-x86_64 ;;
  *) echo "Unsupported Android build host" >&2; exit 2 ;;
esac
TC="$NDK/toolchains/llvm/prebuilt/$HOST_TAG/bin"
ADB=("${ANDROID_ADB:-$SDK/platform-tools/adb}")
if [[ -n "${ANDROID_DEVICE_ID:-}" ]]; then
  ADB+=(-s "$ANDROID_DEVICE_ID")
fi
WORK="$(mktemp -d "${TMPDIR:-/tmp}/ksp-android-edit.XXXXXX")"
REMOTE="/data/local/tmp/ksp-edit-smoke"
trap 'rm -rf "$WORK"' EXIT

if [[ "$("${ADB[@]}" shell getprop ro.product.cpu.abi)" != arm64-v8a ]]; then
  echo "An arm64-v8a Android device is required" >&2
  exit 2
fi

bash "$ROOT/scripts/build_freetext_android.sh"
"$TC/aarch64-linux-android26-clang++" -std=c++17 -O2 \
  "$ROOT/harness/android_edit_smoke.cpp" \
  -L"$ROOT/3rdparty/prebuilt/android-arm64/lib" \
  -L"$ROOT/build" -lkspfreetext -lpdfium -static-libstdc++ \
  -o "$WORK/android_edit_smoke"

"${ADB[@]}" shell mkdir -p "$REMOTE"
"${ADB[@]}" push "$WORK/android_edit_smoke" "$ROOT/build/libkspfreetext.so" \
  "$ROOT/3rdparty/prebuilt/android-arm64/lib/libpdfium.so" \
  "$ROOT/harness/assets/editable-two-page.pdf" \
  "$ROOT/harness/assets/product-links.pdf" "$REMOTE/"
"${ADB[@]}" shell "LD_LIBRARY_PATH=$REMOTE $REMOTE/android_edit_smoke $REMOTE/editable-two-page.pdf $REMOTE/edited.pdf $REMOTE/product-links.pdf"
"${ADB[@]}" pull "$REMOTE/edited.pdf" "$WORK/edited.pdf"
pdfinfo "$WORK/edited.pdf" | grep -q '^Pages: *2$'
echo "PRODUCT_EDIT_PDF_ANDROID PASS"
