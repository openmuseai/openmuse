#!/usr/bin/env bash
# Build (unless skipped) and run the Muse AppFlowy macOS debug client.
#
# Usage:
#   frontend/client/scripts/run-macos-appflowy.sh [--skip-packages] [--skip-core] [--skip-tests] [--no-build]
#
# Requires Flutter 3.27.x. Impeller is disabled because WKWebView platform
# views otherwise black out the desktop window. DSH is started by the in-app
# sidecar from middlewares/scripts/run-dsh-appflowy.sh when the DeepSeek panel opens.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

SKIP_PACKAGES=false
SKIP_CORE=false
SKIP_TESTS=false
NO_BUILD=false
BUILD_ARGS=()
while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-packages) SKIP_PACKAGES=true; BUILD_ARGS+=(--skip-packages) ;;
    --skip-core) SKIP_CORE=true; BUILD_ARGS+=(--skip-core) ;;
    --skip-tests) SKIP_TESTS=true; BUILD_ARGS+=(--skip-tests) ;;
    --no-build) NO_BUILD=true ;;
    -h|--help)
      sed -n '2,11p' "$0"
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
FLUTTER_DIR="$(muse_flutter_dir)"
APP="$FLUTTER_DIR/build/macos/Build/Products/Debug/DSH Office.app"

if [[ "$NO_BUILD" == false ]]; then
  "${SCRIPT_DIR}/build-macos-appflowy.sh" "${BUILD_ARGS[@]+"${BUILD_ARGS[@]}"}"
fi

if pgrep -f "$APP/Contents/MacOS/DSH Office" >/dev/null 2>&1; then
  echo "==> Stopping previous debug DSH Office"
  pkill -f "$APP/Contents/MacOS/DSH Office" || true
  sleep 1
fi

echo "==> flutter run -d macos --no-enable-impeller"
cd "$FLUTTER_DIR"
exec flutter run -d macos --no-enable-impeller
