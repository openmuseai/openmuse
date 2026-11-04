#!/usr/bin/env bash
set -euo pipefail

APP_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REBUILD_ROOT="$(cd "$APP_ROOT/../pdf-rebuild" && pwd)"
JNI_DIR="$APP_ROOT/android/app/src/main/jniLibs/arm64-v8a"

bash "$REBUILD_ROOT/scripts/build_freetext_android.sh"
VIEWER="$REBUILD_ROOT/3rdparty/prebuilt/android-arm64/lib/libpdfium_viewer.so"
echo 'c454a50a4b4bdb87c5fe0f7dde7fa5b0b8eb8b3d3a325165a4e62134ca50eb20  '"$VIEWER" | shasum -a 256 -c -
mkdir -p "$JNI_DIR"
cp "$REBUILD_ROOT/build/libkspfreetext.so" "$JNI_DIR/"
cp "$REBUILD_ROOT/3rdparty/prebuilt/android-arm64/lib/libpdfium.so" "$JNI_DIR/"

echo "Android arm64 product kernel staged in $JNI_DIR"
