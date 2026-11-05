#!/usr/bin/env bash
# Rebuild the local AppFlowy macOS debug app with Muse Host + DSH packages.
#
# Usage:
#   frontend/client/scripts/build-macos-appflowy.sh [--skip-packages] [--skip-core] [--skip-tests]
#
# Steps:
#   1. Muse TS packages (unless --skip-packages)
#   2. AppFlowy dart-ffi / Rust Host (unless --skip-core)
#   3. engine assets + flutter pub get + flutter build macos (debug)
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

SKIP_PACKAGES=false
SKIP_CORE=false
SKIP_TESTS=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-packages) SKIP_PACKAGES=true ;;
    --skip-core) SKIP_CORE=true ;;
    --skip-tests) SKIP_TESTS=true ;;
    -h|--help)
      sed -n '2,12p' "$0"
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
PROFILE="$(muse_macos_profile)"
FRONTEND="$(muse_appflowy_frontend)"
FLUTTER_DIR="$(muse_flutter_dir)"

if [[ "$SKIP_PACKAGES" == false ]]; then
  if [[ "$SKIP_TESTS" == true ]]; then
    "$(muse_script_build_packages)" --skip-tests
  else
    "$(muse_script_build_packages)"
  fi
fi

if [[ "$SKIP_CORE" == false ]]; then
  echo "==> Rebuilding AppFlowy Rust Host / dart-ffi ($PROFILE)"
  if ! command -v protoc-gen-dart >/dev/null; then
    echo "protoc-gen-dart is not on PATH (expected in \$HOME/.pub-cache/bin)" >&2
    exit 1
  fi
  # collab-folder git rev 4dfccef has no Excel/Slides/Pdf; patch the checkout
  # then drop the cached rlib (cargo fingerprints git deps by rev, not dirty files).
  "${FRONTEND}/scripts/tool/patch_collab_word_layout.sh"
  (
    cd "${FRONTEND}/rust-lib"
    # cargo make builds with --target, which lives in target/<triple>/ not target/debug.
    cargo clean -p collab-folder --target "$(uname -m | sed 's/arm64/aarch64/;s/x86_64/x86_64/')-apple-darwin"
  )
  (
    cd "$FRONTEND"
    cargo make --profile "$PROFILE" appflowy-core-dev
  )
fi

echo "==> Flutter pub get + debug macOS build"
(
  cd "$FLUTTER_DIR"
  echo "==> Building embedded Helix and Open File Viewer engine assets"
  ./tool/build_macos_engine_assets.sh
  flutter pub get
  # Impeller is disabled in macos/Runner/MainFlutterWindow.swift (engine switch)
  # because FLTEnableImpeller in Info.plist is not applied on macOS. Direct
  # `open .app` would otherwise black-screen. `flutter run --no-enable-impeller`
  # remains the supported path for `run-macos-appflowy.sh`.
  flutter build macos --debug
)

APP="$FLUTTER_DIR/build/macos/Build/Products/Debug/${BRAND_MACOS_BUNDLE}"
if [[ ! -d "$APP" ]]; then
  echo "expected app missing: $APP" >&2
  exit 1
fi
echo "Built $APP"
