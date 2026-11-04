#!/usr/bin/env bash
# build_freetext.sh - build the rebuilt FreeText layer + JNI dylib (macOS host)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PREBUILT="$ROOT/3rdparty/prebuilt"
if [[ ! -f "$PREBUILT/lib/libpdfium.dylib" ]]; then
  tar -xzf "$PREBUILT/pdfium-mac-arm64-ksp.tgz" -C "$PREBUILT" lib/libpdfium.dylib
fi
EXPECTED_PDFIUM_SHA256="c0ff3e1e104c262ffec28bf27a846d3cbaf72fd7025c3883aec3cacf180ad8b9"
ACTUAL_PDFIUM_SHA256="$(shasum -a 256 "$PREBUILT/lib/libpdfium.dylib" | awk '{print $1}')"
if [[ "$ACTUAL_PDFIUM_SHA256" != "$EXPECTED_PDFIUM_SHA256" ]]; then
  echo "Wrong macOS product PDFium binary: $ACTUAL_PDFIUM_SHA256" >&2
  exit 2
fi
if ! nm -gU "$PREBUILT/lib/libpdfium.dylib" | grep '_KSPTextObj_SetGlyphPositions$' > /dev/null; then
  echo "macOS PDFium is missing rebuild ExtensionAPI symbols" >&2
  exit 2
fi
JAVA_HOME="${JAVA_HOME:-$(/usr/libexec/java_home -v 17)}"
OUT="$ROOT/build"
mkdir -p "$OUT"

echo "==[ clang++ FreeText.cpp FreeTextJNI.cpp ]=="
clang++ -std=c++17 -O2 -Wall -Wextra \
  -I"$PREBUILT/include" \
  -I"$JAVA_HOME/include" -I"$JAVA_HOME/include/darwin" \
  -dynamiclib -install_name @rpath/libkspfreetext.dylib \
  "$ROOT/src/ksp/freetext/FreeText.cpp" \
  "$ROOT/src/ksp/freetext/FreeTextJNI.cpp" \
  "$ROOT/src/ksp/freetext/RenderJNI.cpp" \
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
  "$ROOT/src/ksp/product/ProductEditBridge.cpp" \
  "$ROOT/src/ksp/product/PageOperations.cpp" \
  "$ROOT/src/ksp/product/ProductBridge.cpp" \
  "$ROOT/src/ksp/jni/JniSupport.cpp" \
  "$ROOT/src/ksp/jni/DocumentJNI.cpp" \
  "$ROOT/src/ksp/jni/EditJNI.cpp" \
  "$ROOT/src/ksp/jni/FontJNI.cpp" \
  "$ROOT/src/ksp/jni/ObjectJNI.cpp" \
  "$ROOT/src/ksp/jni/SaveJNI.cpp" \
  "$ROOT/src/ksp/jni/OptimizeJNI.cpp" \
  "$ROOT/src/ksp/jni/TextPageJNI.cpp" \
  -L"$PREBUILT/lib" -lpdfium \
  -o "$OUT/libkspfreetext.dylib"

# Keep the bridge and PDFium relocatable when Flutter copies both dylibs into
# the application asset directory.
install_name_tool -change ./libpdfium.dylib \
  @loader_path/libpdfium.dylib "$OUT/libkspfreetext.dylib"

echo "==[ symbols ]=="
echo "java_jni_exports=$(nm -gU "$OUT/libkspfreetext.dylib" | grep -c ' Java_' || true)"
nm -gU "$OUT/libkspfreetext.dylib" | grep Java_cn_wps | sed -n '1,8p'
echo "BUILD OK -> $OUT/libkspfreetext.dylib"
