#!/usr/bin/env bash
# Copy Node + DSH (dereferenced node_modules) + Muse packages into a
# Resources/muse directory that can be launched without the source tree.
#
# Usage:
#   middlewares/scripts/stage-dsh-runtime.sh <dest-muse-dir>
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
REPO_ROOT="$(cd "${SCRIPT_DIR}/../.." && pwd)"
# shellcheck source=lib/muse-macos.sh
source "${SCRIPT_DIR}/lib/muse-macos.sh"

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <dest-muse-dir>" >&2
  exit 1
fi

DEST="$(mkdir -p "$1" && cd "$1" && pwd)"
ROOT="$(muse_root)"
HARNESS="$(muse_harness_dir)"
PATCH="$(muse_dsh_patch)"

if [[ ! -d "$HARNESS/node_modules" ]]; then
  echo "DSH node_modules missing; run pnpm install in $HARNESS first." >&2
  exit 1
fi
if [[ ! -f "$PATCH" ]]; then
  echo "missing $PATCH" >&2
  exit 1
fi

echo "==> Staging bundled Node into $DEST/node"
muse_stage_node "$DEST/node" "$(muse_dist_dir)/cache"

echo "==> Staging DSH harness (this is the large copy)"
date
mkdir -p "$DEST/dsh"
rsync -aH --delete \
  --exclude '.git/' \
  --exclude 'website/' \
  --exclude 'python/' \
  --exclude 'coverage/' \
  --exclude '.turbo/' \
  --exclude '.DS_Store' \
  --exclude 'node_modules/.cache/' \
  "$HARNESS/" "$DEST/dsh/"

echo "==> Staging Muse packages"
# Replace source-tree @muse symlinks with real copies (do not rsync through them).
rm -rf "$DEST/dsh/node_modules/@muse"
mkdir -p "$DEST/packages" "$DEST/dsh/node_modules/@muse"
muse_copy_dsh_packages "$DEST/packages"
muse_copy_dsh_packages "$DEST/dsh/node_modules/@muse"
muse_wire_muse_node_modules "$DEST/dsh"

echo "==> Staging dshmarket (plugin market)"
muse_stage_dshmarket "$DEST/dsh"

echo "==> Staging Cordis patch"
cp "$PATCH" "$DEST/patch.yml"

cat > "$DEST/README.txt" <<'EOF'
Muse bundled DSH runtime.

Launched by AppFlowy as:
  node --import tsx/esm apps/cli/src/bin.ts --profile web --patch ../patch.yml
User data lives in ~/Library/Application Support/AppFlowy/Muse/
EOF

echo "Staged Muse runtime at $DEST"
