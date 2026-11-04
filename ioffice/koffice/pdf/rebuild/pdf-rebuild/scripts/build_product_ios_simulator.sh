#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SOURCE="${KSP_PDFIUM_SOURCE:-$ROOT/3rdparty/src/pdfium}"
OUT="$ROOT/build/ios-simulator"
SDK="$(xcrun --sdk iphonesimulator --show-sdk-path)"

bash "$ROOT/scripts/build_product_pdfium.sh" ios-simulator-arm64
mkdir -p "$OUT"
cp "$SOURCE/out/ksp-ios-simulator/libpdfium.dylib" "$OUT/libpdfium.dylib"

xcrun clang++ -target arm64-apple-ios13.0-simulator -isysroot "$SDK" \
  -std=c++17 -femulated-tls -O2 -fPIC -dynamiclib \
  -install_name @rpath/libksp_product.dylib \
  -I"$ROOT/3rdparty/prebuilt/include" \
  "$ROOT/src/ksp/runtime/Error.cpp" \
  "$ROOT/src/ksp/runtime/HandleRegistry.cpp" \
  "$ROOT/src/ksp/runtime/PdfRuntime.cpp" \
  "$ROOT/src/ksp/runtime/PdfiumLifecycle.cpp" \
  "$ROOT/src/ksp/runtime/EditRuntime.cpp" \
  "$ROOT/src/ksp/runtime/FontRuntime.cpp" \
  "$ROOT/src/ksp/runtime/UnicodeText.cpp" \
  "$ROOT/src/ksp/runtime/ObjectRuntime.cpp" \
  "$ROOT/src/ksp/runtime/SaveRuntime.cpp" \
  "$ROOT/src/ksp/runtime/OptimizeRuntime.cpp" \
  "$ROOT/src/ksp/bridge/MacEditBridge.cpp" \
  "$ROOT/src/ksp/product/ProductSession.cpp" \
  "$ROOT/src/ksp/product/PageOperations.cpp" \
  "$ROOT/src/ksp/product/ProductBridge.cpp" \
  "$ROOT/src/ksp/product/ProductEditBridge.cpp" \
  -L"$OUT" -lpdfium -o "$OUT/libksp_product.dylib"

install_name_tool -change ./libpdfium.dylib \
  @loader_path/libpdfium.dylib "$OUT/libksp_product.dylib"
nm -gU "$OUT/libpdfium.dylib" | grep '_KSPTextObj_SetGlyphPositions$' > /dev/null
nm -gU "$OUT/libksp_product.dylib" | grep '_ksp_product_edit_abi_version$' > /dev/null
echo "iOS simulator product core built in $OUT"
