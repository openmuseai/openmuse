#!/usr/bin/env bash
set -euo pipefail

APP_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
ARCHIVE="$APP_ROOT/../pdf-rebuild/3rdparty/prebuilt/pdfium-ios-simulator-arm64-ksp.tgz"
OUT="$APP_ROOT/ios/Native"
mkdir -p "$OUT"
tar -xzf "$ARCHIVE" -C "$OUT" libpdfium.dylib libksp_product.dylib
echo 'b9348c51fa81fc268fac9a45288af9092d0d2036d30f3f30140c65a374ce488f  '"$OUT/libpdfium.dylib" | shasum -a 256 -c -
echo '5f927c372798fb2023982d0eb7e9278f9619022a32efd2589bf96f3b0f2cae2c  '"$OUT/libksp_product.dylib" | shasum -a 256 -c -
cp "$OUT/libpdfium.dylib" "$OUT/libpdfium_viewer.dylib"
install_name_tool -id @rpath/libpdfium_viewer.dylib "$OUT/libpdfium_viewer.dylib"
echo "iOS simulator product libraries prepared in $OUT"
