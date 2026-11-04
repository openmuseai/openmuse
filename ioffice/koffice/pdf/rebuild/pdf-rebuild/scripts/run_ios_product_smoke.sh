#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
DEVICE="${IOS_SIMULATOR_UDID:-booted}"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/ksp-ios-edit.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT
SDK="$(xcrun --sdk iphonesimulator --show-sdk-path)"

tar -xzf "$ROOT/3rdparty/prebuilt/pdfium-ios-simulator-arm64-ksp.tgz" \
  -C "$WORK" libpdfium.dylib libksp_product.dylib
xcrun clang++ -target arm64-apple-ios14.0-simulator -isysroot "$SDK" \
  -std=c++17 -femulated-tls -O2 \
  "$ROOT/harness/android_edit_smoke.cpp" \
  -L"$WORK" -lksp_product -Wl,-rpath,@executable_path \
  -o "$WORK/product_edit_smoke"
xcrun simctl spawn "$DEVICE" "$WORK/product_edit_smoke" \
  "$ROOT/harness/assets/editable-two-page.pdf" "$WORK/edited.pdf"
pdfinfo "$WORK/edited.pdf" | grep -q '^Pages: *2$'
echo "PRODUCT_EDIT_IOS_SIMULATOR PASS"
