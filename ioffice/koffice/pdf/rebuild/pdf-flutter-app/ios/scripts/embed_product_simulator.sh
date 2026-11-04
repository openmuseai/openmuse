#!/bin/sh
set -eu

if [ "${PLATFORM_NAME:-}" != iphonesimulator ]; then
  exit 0
fi

SOURCE="$SRCROOT/Native"
DEST="$TARGET_BUILD_DIR/$FRAMEWORKS_FOLDER_PATH"
for name in libpdfium.dylib libpdfium_viewer.dylib libksp_product.dylib; do
  if [ ! -f "$SOURCE/$name" ]; then
    echo "Missing $SOURCE/$name; run tool/prepare_ios_native.sh" >&2
    exit 1
  fi
  mkdir -p "$DEST"
  cp "$SOURCE/$name" "$DEST/$name"
  codesign --force --sign - --timestamp=none "$DEST/$name"
done
