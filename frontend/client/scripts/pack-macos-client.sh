#!/usr/bin/env bash
# Build a distributable macOS DSH Office + DSH client.
#
# Usage:
#   frontend/client/scripts/pack-macos-client.sh
#   frontend/client/scripts/pack-macos-client.sh --debug
#   frontend/client/scripts/pack-macos-client.sh --debug --skip-app-build --skip-packages
#
# Output:
#   dist/macos/DSH Office.app
#   dist/macos/Muse-macos.zip
#   dist/macos/Muse.dmg          (if create-dmg is on PATH)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

DEBUG=false
SKIP_APP_BUILD=false
SKIP_PACKAGES=false
SKIP_TESTS=false
REUSE_RUNTIME=false
SKIP_ZIP=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --debug) DEBUG=true ;;
    --skip-app-build) SKIP_APP_BUILD=true ;;
    --skip-packages) SKIP_PACKAGES=true ;;
    --skip-tests) SKIP_TESTS=true ;;
    --reuse-runtime) REUSE_RUNTIME=true ;;
    --skip-zip) SKIP_ZIP=true ;;
    -h|--help)
      sed -n '2,16p' "$0"
      exit 0
      ;;
    *)
      echo "Unknown option: $1" >&2
      exit 1
      ;;
  esac
  shift
done

muse_export_toolchain
muse_require_flutter_327
ROOT="$(muse_root)"
FRONTEND="$(muse_appflowy_frontend)"
FLUTTER_DIR="$(muse_flutter_dir)"
OUT="$(muse_dist_dir)/macos"
mkdir -p "$OUT"

if [[ "$SKIP_PACKAGES" == false ]]; then
  if [[ "$SKIP_TESTS" == true ]]; then
    "$(muse_script_build_packages)" --skip-tests
  else
    "$(muse_script_build_packages)"
  fi
fi

if [[ "$DEBUG" == true ]]; then
  PROFILE="$(muse_macos_profile)"
  BUILD_FLAG="debug"
  PRODUCT_DIR="$FLUTTER_DIR/build/macos/Build/Products/Debug/DSH Office.app"
else
  case "$(uname -m)" in
    arm64) PROFILE="production-mac-arm64" ;;
    x86_64) PROFILE="production-mac-x86_64" ;;
    *)
      echo "unsupported macOS arch: $(uname -m)" >&2
      exit 1
      ;;
  esac
  BUILD_FLAG="release"
  PRODUCT_DIR="$FLUTTER_DIR/build/macos/Build/Products/Release/DSH Office.app"
fi

if [[ "$SKIP_APP_BUILD" == false ]]; then
  echo "==> Building DSH Office macOS ($PROFILE, $BUILD_FLAG)"
  if ! command -v protoc-gen-dart >/dev/null; then
    echo "protoc-gen-dart is not on PATH (expected in \$HOME/.pub-cache/bin)" >&2
    exit 1
  fi
  if [[ "$DEBUG" == true ]]; then
    CORE_TASK="appflowy-core-dev"
  else
    CORE_TASK="appflowy-core-release"
  fi
  (
    cd "$FRONTEND"
    cargo make --profile "$PROFILE" "$CORE_TASK"
  )
  (
    cd "$FLUTTER_DIR"
    flutter pub get
    flutter build macos --"$BUILD_FLAG"
  )
fi

if [[ ! -d "$PRODUCT_DIR" ]]; then
  echo "expected app missing: $PRODUCT_DIR" >&2
  echo "Build it first or omit --skip-app-build." >&2
  exit 1
fi

echo "==> Copying $PRODUCT_DIR → $OUT/DSH Office.app"
SAVED_RUNTIME=""
if [[ "$REUSE_RUNTIME" == true && -d "$OUT/DSH Office.app/Contents/Resources/muse/dsh" ]]; then
  SAVED_RUNTIME="$(mktemp -d "${TMPDIR:-/tmp}/muse-runtime.XXXXXX")"
  echo "==> Reusing staged DSH runtime"
  mv "$OUT/DSH Office.app/Contents/Resources/muse" "$SAVED_RUNTIME/muse"
fi
rm -rf "$OUT/DSH Office.app"
ditto "$PRODUCT_DIR" "$OUT/DSH Office.app"

MUSE_RES="$OUT/DSH Office.app/Contents/Resources/muse"
if [[ -n "$SAVED_RUNTIME" ]]; then
  rm -rf "$MUSE_RES"
  mv "$SAVED_RUNTIME/muse" "$MUSE_RES"
  rmdir "$SAVED_RUNTIME" 2>/dev/null || rm -rf "$SAVED_RUNTIME"
  echo "==> Refreshing Muse packages in reused runtime"
  rm -rf "$MUSE_RES/dsh/node_modules/@muse"
  mkdir -p "$MUSE_RES/packages" "$MUSE_RES/dsh/node_modules/@muse"
  muse_copy_dsh_packages "$MUSE_RES/packages"
  muse_copy_dsh_packages "$MUSE_RES/dsh/node_modules/@muse"
  muse_wire_muse_node_modules "$MUSE_RES/dsh"
  muse_stage_dshmarket "$MUSE_RES/dsh"
  cp "$(muse_dsh_patch)" "$MUSE_RES/patch.yml"
else
  "$(muse_script_stage_dsh)" "$MUSE_RES"
fi

echo "==> Ad-hoc codesign (keep Flutter entitlements; DSH/Node live in Resources)"
muse_codesign_app "$OUT/DSH Office.app"

NODE_BIN="$MUSE_RES/node/bin/node"
if [[ ! -x "$NODE_BIN" || ! -f "$MUSE_RES/dsh/apps/cli/src/bin.ts" || ! -f "$MUSE_RES/patch.yml" || ! -f "$MUSE_RES/dsh/node_modules/dshmarket/lib/index.js" ]]; then
  echo "packed runtime is incomplete under $MUSE_RES" >&2
  exit 1
fi
echo "Bundled $($NODE_BIN --version) and DSH CLI are present."

ZIP="$OUT/Muse-macos.zip"
if [[ "$SKIP_ZIP" == false ]]; then
  echo "==> Zipping $ZIP"
  rm -f "$ZIP"
  ditto -c -k --keepParent "$OUT/DSH Office.app" "$ZIP"
else
  echo "==> Skipping zip"
fi

if command -v create-dmg >/dev/null 2>&1; then
  echo "==> Creating DMG"
  rm -f "$OUT/Muse.dmg"
  create-dmg --overwrite --dmg-title "Muse" "$OUT/DSH Office.app" "$OUT"
  shopt -s nullglob
  for dmg in "$OUT"/*.dmg; do
    if [[ "$(basename "$dmg")" != "Muse.dmg" ]]; then
      mv "$dmg" "$OUT/Muse.dmg"
    fi
  done
  shopt -u nullglob
fi

echo
echo "Distributable client:"
echo "  $OUT/DSH Office.app"
echo "  $ZIP"
echo "Install: copy DSH Office.app to /Applications (or run it from dist/macos)."
echo "First launch: enter DEEPSEEK_API_KEY in the DeepSeek panel."
