#!/usr/bin/env bash
set -euo pipefail

APP_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REBUILD_ROOT="$(cd "$APP_ROOT/../pdf-rebuild" && pwd)"
ASSET_DIR="$APP_ROOT/assets/native"

"$REBUILD_ROOT/scripts/build_freetext.sh"
VIEWER="$REBUILD_ROOT/3rdparty/prebuilt/lib/libpdfium_viewer.dylib"
if [[ ! -f "$VIEWER" ]]; then
  tar -xzf "$REBUILD_ROOT/3rdparty/prebuilt/pdfium-mac-arm64-viewer.tgz" \
    -C "$REBUILD_ROOT/3rdparty/prebuilt" lib/libpdfium_viewer.dylib
fi
if [[ "$(shasum -a 256 "$VIEWER" | awk '{print $1}')" != \
      8e48ad539ec119619ee86e683a91aba438d6c29874424f1c2d95111054a9c01e ]]; then
  echo "Wrong macOS reader PDFium binary" >&2
  exit 2
fi
mkdir -p "$ASSET_DIR"
cp "$REBUILD_ROOT/build/libkspfreetext.dylib" "$ASSET_DIR/"
cp "$REBUILD_ROOT/3rdparty/prebuilt/lib/libpdfium.dylib" "$ASSET_DIR/"
cp "$VIEWER" "$ASSET_DIR/"

echo "macOS native editor assets prepared in $ASSET_DIR"
