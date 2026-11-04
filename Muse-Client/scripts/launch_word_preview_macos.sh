#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ecosystem_root="$repo_root/../Muse-Plugin-Ecosystem"
run_dir="${1:-}"
if [[ -z "$run_dir" ]]; then
  run_dir="$(python3 - "$ecosystem_root/.word-stack-runs" <<'PY'
import json
import sys
from pathlib import Path

root = Path(sys.argv[1])
for report in sorted(root.glob('*/report.json'), reverse=True):
    data = json.loads(report.read_text())
    options = data.get('options', {})
    if (data.get('status') == 'e2e-passed' and
            options.get('platform') == 'desktop' and
            options.get('scope') == 'render'):
        print(report.parent)
        break
else:
    raise SystemExit('No successful Desktop Word render E2E run found')
PY
)"
fi
run_dir="$(cd "$run_dir" && pwd)"
app="$repo_root/app/openmuse_host/build/macos/Build/Products/Debug/OpenMuse.app/Contents/MacOS/OpenMuse"
sample="$repo_root/../ioffice/samples/docx-mvp-render/flutter/assets/sample1/sample2.docx"
signature="$run_dir/isolated-support/OpenMuse/mcpb/io.openmuse_word/catalog.signature.json"
test -x "$app"
test -f "$sample"
test -f "$run_dir/test-public.pem"
test -f "$signature"
test -f "$repo_root/app/openmuse_host/build/macos/Build/Products/Debug/OpenMuse.app/Contents/Frameworks/libword_core_ffi.dylib"
key_id="$(python3 - "$signature" <<'PY'
import json
import sys
print(json.load(open(sys.argv[1]))['keyId'])
PY
)"

export OPENMUSE_E2E_SUPPORT_DIR="$run_dir/isolated-support"
export OPENMUSE_E2E_OPEN_DOCX="$sample"
export WORD_STACK_PUBLIC_KEY="$run_dir/test-public.pem"
export WORD_STACK_KEY_ID="$key_id"
export WORD_CORE_FFI="$repo_root/app/openmuse_host/build/macos/Build/Products/Debug/OpenMuse.app/Contents/Frameworks/libword_core_ffi.dylib"
export WORD_CORE_FONT='/System/Library/Fonts/Supplemental/Arial.ttf'
export WORD_CORE_CJK_FONT='/System/Library/Fonts/Supplemental/Arial Unicode.ttf'
export OPENMUSE_PAIRED_DESKTOP_PORT="$(python3 - <<'PY'
import socket
with socket.socket() as server:
    server.bind(('127.0.0.1', 0))
    print(server.getsockname()[1])
PY
)"
echo "Opening the signed Word preview in OpenMuse (run: $run_dir)"
echo 'The DOCX opens automatically; the DSH panel is on the right. Quit OpenMuse to end this session.'
exec "$app"
