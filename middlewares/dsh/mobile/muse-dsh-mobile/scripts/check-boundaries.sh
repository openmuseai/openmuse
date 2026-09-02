#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if rg -n "package:appflowy|package:appflowy_|muse_appflowy_facets|flowy-|package:speech_to_text|package:image_picker|package:file_picker|package:share_plus|package:permission_handler" "$ROOT/lib"; then
  echo "muse_dsh_mobile must not import AppFlowy or host native plugins" >&2
  exit 1
fi
echo "muse_dsh_mobile boundaries ok"
