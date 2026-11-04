#!/usr/bin/env bash
# Build the browser product ABI from the same pinned PDFium source as native.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SOURCE="${KSP_PDFIUM_SOURCE:-$ROOT/3rdparty/src/pdfium}"
EMSDK_ROOT="${KSP_EMSDK_ROOT:-$ROOT/3rdparty/src/emsdk}"
GN="${KSP_GN:-$ROOT/3rdparty/src/gn/out/gn}"
NINJA="${KSP_NINJA:-ninja}"
OUT="$ROOT/build/web"
PRODUCT_PATCH="$ROOT/3rdparty/patches/pdfium-product.patch"
WEB_PATCH="$ROOT/3rdparty/patches/pdfium-web.patch"
BASE_REVISION="784a524ddaa26d86c0f499625b97902095c26dfe"
EMSDK_REVISION="3b3963d86a5c97adaf6eaa845cdfe0b811c2f960"

if [[ ! -d "$SOURCE/.git" || ! -x "$GN" || ! -f "$EMSDK_ROOT/emsdk_env.sh" ]]; then
  echo "Pinned PDFium, GN and Emsdk checkouts are required" >&2
  exit 2
fi
if [[ "$(git -C "$SOURCE" rev-parse HEAD)" != "$BASE_REVISION" ||
      "$(git -C "$EMSDK_ROOT" rev-parse HEAD)" != "$EMSDK_REVISION" ]]; then
  echo "PDFium or Emsdk source revision differs from the pinned Web build" >&2
  exit 2
fi

if ! git -C "$SOURCE" apply --reverse --check "$WEB_PATCH"; then
  if ! git -C "$SOURCE" apply --reverse --check "$PRODUCT_PATCH"; then
    echo "Apply the canonical product patch before the Web patch" >&2
    exit 2
  fi
  git -C "$SOURCE" apply "$WEB_PATCH"
fi

if [[ ! -e "$SOURCE/third_party/emsdk" ]]; then
  ln -s "$EMSDK_ROOT" "$SOURCE/third_party/emsdk"
elif [[ "$(realpath "$SOURCE/third_party/emsdk")" != "$(realpath "$EMSDK_ROOT")" ]]; then
  echo "PDFium third_party/emsdk points at a different toolchain" >&2
  exit 2
fi

source "$EMSDK_ROOT/emsdk_env.sh" >/dev/null 2>&1
if ! "$EMSDK_ROOT/upstream/emscripten/emcc" --version | grep ' 4.0.15 ' >/dev/null; then
  echo "Emscripten 4.0.15 is required" >&2
  exit 2
fi

case "$(uname -s):$(uname -m)" in
  Darwin:arm64) HOST_OS=mac; HOST_CPU=arm64 ;;
  Darwin:x86_64) HOST_OS=mac; HOST_CPU=x64 ;;
  Linux:x86_64) HOST_OS=linux; HOST_CPU=x64 ;;
  Linux:aarch64) HOST_OS=linux; HOST_CPU=arm64 ;;
  *) echo "Unsupported Web build host" >&2; exit 2 ;;
esac

ARGS="target_os=\"$HOST_OS\" target_cpu=\"$HOST_CPU\" is_debug=false is_component_build=false pdf_enable_v8=false pdf_enable_xfa=false pdf_is_standalone=true pdf_use_partition_alloc=false use_remoteexec=false use_siso=false clang_use_chrome_plugins=false treat_warnings_as_errors=false"
(
  cd "$SOURCE"
  "$GN" gen out/ksp-web-product --args="$ARGS"
  "$NINJA" -C out/ksp-web-product -j "${KSP_WEB_JOBS:-6}" ksp_web_pdfium
)

PDFIUM_ARCHIVE="$SOURCE/out/ksp-web-product/wasm/obj/libpdfium.a"
if [[ ! -f "$PDFIUM_ARCHIVE" ]] ||
   ! "$EMSDK_ROOT/upstream/bin/llvm-nm" "$PDFIUM_ARCHIVE" 2>/dev/null |
     grep ' KSPTextObj_SetGlyphPositions$' >/dev/null; then
  echo "Web PDFium archive is missing the rebuild editing extension" >&2
  exit 2
fi

mkdir -p "$OUT"
EXPORTS="$(tr -d '\n' < "$ROOT/scripts/web_exports.json")"
"$EMSDK_ROOT/upstream/emscripten/em++" \
  -std=c++26 -O2 -fexceptions -sSUPPORT_LONGJMP=emscripten \
  -I"$SOURCE" -I"$SOURCE/out/ksp-web-product/wasm/gen" \
  -I"$SOURCE/public" -I"$SOURCE/third_party/freetype/include" \
  -I"$SOURCE/third_party/freetype/src/include" \
  "$ROOT/src/ksp/product/WebPlatform.cpp" \
  "$ROOT/src/ksp/product/ProductBridge.cpp" \
  "$ROOT/src/ksp/product/PageOperations.cpp" \
  "$ROOT/src/ksp/product/ProductSession.cpp" \
  "$ROOT/src/ksp/product/ProductEditBridge.cpp" \
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
  "$PDFIUM_ARCHIVE" \
  -sALLOW_MEMORY_GROWTH=1 -sMODULARIZE=1 -sEXPORT_ES6=1 \
  -sENVIRONMENT=web,node -sFILESYSTEM=0 \
  "-sEXPORTED_FUNCTIONS=$EXPORTS" \
  '-sEXPORTED_RUNTIME_METHODS=["HEAPU8"]' \
  -o "$OUT/ksp_product.mjs"

node "$ROOT/harness/web_product_smoke.mjs" \
  "$OUT/ksp_product.mjs" "$ROOT/harness/assets/editable-two-page.pdf"
cp "$OUT/ksp_product.mjs" "$ROOT/../pdf-flutter-app/web/ksp_product.mjs"
cp "$OUT/ksp_product.wasm" "$ROOT/../pdf-flutter-app/web/ksp_product.wasm"
node "$ROOT/harness/web_bridge_smoke.mjs" \
  "$ROOT/../pdf-flutter-app/web/ksp_product_bridge.mjs" \
  "$ROOT/harness/assets/editable-two-page.pdf" \
  "$ROOT/harness/assets/page-order.pdf"
echo "Web product ABI ready: $OUT/ksp_product.mjs and $OUT/ksp_product.wasm"
