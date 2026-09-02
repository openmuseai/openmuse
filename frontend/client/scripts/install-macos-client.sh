#!/usr/bin/env bash
# Replace /Applications/DSH Office.app with the packed Muse client.
# The stock AppFlowy already in /Applications is a different app (no DSH runtime).
#
# Usage:
#   frontend/client/scripts/install-macos-client.sh
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

ROOT="$(muse_root)"
SRC="$(muse_dist_dir)/macos/DSH Office.app"
DEST="/Applications/DSH Office.app"

if [[ ! -d "$SRC/Contents/Resources/muse/dsh" ]]; then
  echo "packed client missing: $SRC" >&2
  echo "Run frontend/client/scripts/pack-macos-client.sh --debug first." >&2
  exit 1
fi

echo "==> Installing $SRC → $DEST"
if [[ -d "$DEST" ]]; then
  echo "Removing existing $DEST"
  rm -rf "$DEST"
fi
ditto "$SRC" "$DEST"
muse_codesign_app "$DEST"
echo "Installed. Open with: open $DEST"
echo "If Gatekeeper blocks it: right-click the app → Open."
