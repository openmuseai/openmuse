#!/usr/bin/env bash
# build_freetext_android.sh - cross-compile the rebuilt FreeText layer for
# android-arm64 using the NDK + bblanchon android PDFium prebuilt.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PREBUILT="$ROOT/3rdparty/prebuilt/android-arm64"
SDK="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
NDK="${ANDROID_NDK_HOME:-$SDK/ndk/27.0.12077973}"
case "$(uname -s)" in
  Darwin) HOST_TAG=darwin-x86_64 ;;
  Linux) HOST_TAG=linux-x86_64 ;;
  *) echo "Unsupported Android build host" >&2; exit 2 ;;
esac
TC="$NDK/toolchains/llvm/prebuilt/$HOST_TAG"
CXX="$TC/bin/aarch64-linux-android26-clang++"
EXPECTED_PDFIUM_SHA256="00eb0c6af4bef9468b6883f0c0def00ba64c24dce4debe96870528ed633be6d7"
ACTUAL_PDFIUM_SHA256="$(shasum -a 256 "$PREBUILT/lib/libpdfium.so" | awk '{print $1}')"
if [[ "$ACTUAL_PDFIUM_SHA256" != "$EXPECTED_PDFIUM_SHA256" ]]; then
  echo "Wrong Android product PDFium binary: $ACTUAL_PDFIUM_SHA256" >&2
  exit 2
fi
if ! "$TC/bin/llvm-nm" -D "$PREBUILT/lib/libpdfium.so" | grep ' KSPTextObj_SetGlyphPositions$' > /dev/null; then
  echo "Android PDFium is missing rebuild ExtensionAPI symbols" >&2
  exit 2
fi
OUT="$ROOT/build"
mkdir -p "$OUT"

echo "==[ cross-compile android-arm64 ]=="
"$CXX" -std=c++17 -O2 -fPIC -shared \
  -I"$PREBUILT/include" \
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
  -L"$PREBUILT/lib" -lpdfium -llog \
  -static-libstdc++ -Wl,-soname,libkspfreetext.so \
  -o "$OUT/libkspfreetext.so"

echo "==[ verify ]=="
file "$OUT/libkspfreetext.so"
JAVA_EXPORTS="$("$TC/bin/llvm-nm" -D --defined-only "$OUT/libkspfreetext.so" | grep -c ' Java_' || true)"
ONLOAD_EXPORTS="$("$TC/bin/llvm-nm" -D --defined-only "$OUT/libkspfreetext.so" | grep -c ' JNI_OnLoad$' || true)"
echo "java_jni_exports=$JAVA_EXPORTS jni_onload_exports=$ONLOAD_EXPORTS"
APP_JNI="$ROOT/../pdf-standard-app/app/src/main/jniLibs/arm64-v8a"
if [[ -d "$ROOT/../pdf-standard-app/app/src/main" ]]; then
  mkdir -p "$APP_JNI"
  cp "$OUT/libkspfreetext.so" "$APP_JNI/libkspfreetext.so"
  echo "STAGED -> $APP_JNI/libkspfreetext.so"
fi
echo "BUILD OK -> $OUT/libkspfreetext.so"
