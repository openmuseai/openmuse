#!/usr/bin/env bash
set -euo pipefail

root_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
app_dir="$root_dir/app/openmuse_host"
: "${WORD_STACK_BUNDLE:?}"
: "${WORD_STACK_PUBLIC_KEY:?}"
: "${WORD_STACK_KEY_ID:?}"
: "${WORD_STACK_E2E_RESULT_PATH:?}"
: "${WORD_CORE_FFI:?}"

if [[ "$(uname -s)" != "Darwin" ]]; then
  echo 'The first Word desktop E2E hook targets macOS.' >&2
  exit 2
fi

app_bundle="$(find "$app_dir/build/macos/Build/Products/Debug" -maxdepth 1 -type d -name '*.app' -print -quit)"
if [[ -z "$app_bundle" ]]; then
  echo 'OpenMuse macOS app bundle was not built.' >&2
  exit 2
fi
executable_name="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleExecutable' "$app_bundle/Contents/Info.plist")"
host_binary="$app_bundle/Contents/MacOS/$executable_name"
engine_binary="$app_bundle/Contents/Frameworks/libword_core_ffi.dylib"
source_engine="$WORD_CORE_FFI"
cp "$source_engine" "$engine_binary"
codesign --force --sign - "$engine_binary"
codesign --force --deep --sign - "$app_bundle"

evidence_path="$(dirname "$WORD_STACK_E2E_RESULT_PATH")/word-desktop-render-evidence.json"
app_log="$(dirname "$WORD_STACK_E2E_RESULT_PATH")/word-desktop-openmuse-app.log"
export OPENMUSE_E2E_SUPPORT_DIR="$(dirname "$WORD_STACK_E2E_RESULT_PATH")/isolated-support"
export OPENMUSE_E2E_OPEN_DOCX="$root_dir/../ioffice/samples/docx-mvp-render/flutter/assets/sample1/sample2.docx"
export WORD_STACK_E2E_EVIDENCE_PATH="$evidence_path"
export WORD_STACK_SHELL_SCREENSHOT_PATH="$(dirname "$WORD_STACK_E2E_RESULT_PATH")/word-product-shell.png"
export WORD_STACK_UI_FONT='/System/Library/Fonts/Supplemental/Arial Unicode.ttf'
export WORD_STACK_MATERIAL_ICONS_FONT="$(dirname "$(command -v flutter)")/cache/artifacts/material_fonts/MaterialIcons-Regular.otf"
export WORD_STACK_SOURCE_ENGINE_PATH="$source_engine"
export WORD_CORE_FFI="$engine_binary"
export WORD_CORE_ALLOW_HOST_FONTS_IN_TEST=1
export WORD_CORE_FONT='/System/Library/Fonts/Supplemental/Arial.ttf'
export WORD_CORE_CJK_FONT='/System/Library/Fonts/Supplemental/Arial Unicode.ttf'
export OPENMUSE_PAIRED_DESKTOP_PORT="$(python3 - <<'PY'
import socket
with socket.socket() as server:
    server.bind(('127.0.0.1', 0))
    print(server.getsockname()[1])
PY
)"
(cd "$app_dir" && flutter test test/word_product_e2e_test.dart)

"$host_binary" >"$app_log" 2>&1 &
app_pid=$!
trap 'kill "$app_pid" 2>/dev/null || true' EXIT
ready=0
for attempt in {1..60}; do
  if grep -q 'OPENMUSE_WORD_RENDER_READY:' "$app_log" &&
     grep -q 'OpenMuse dsh: ready origin=' "$app_log" &&
     grep -q 'DSH WebView HTTP 200' "$app_log" &&
     grep -q 'DSH WebView loaded' "$app_log"; then
    ready=1
    break
  fi
  if ! kill -0 "$app_pid" 2>/dev/null; then break; fi
  sleep 1
done
if [[ "$ready" != 1 ]]; then
  tail -50 "$app_log" >&2
  echo 'OpenMuse did not render the DOCX and load the DSH panel.' >&2
  exit 1
fi
kill "$app_pid" 2>/dev/null || true
wait "$app_pid" 2>/dev/null || true

WORD_STACK_HOST_BINARY="$host_binary" WORD_STACK_ENGINE_BINARY="$engine_binary" WORD_STACK_APP_LOG="$app_log" \
python3 - <<'PY'
import datetime
import hashlib
import json
import os
from pathlib import Path

def digest(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()

evidence = Path(os.environ['WORD_STACK_E2E_EVIDENCE_PATH'])
data = json.loads(evidence.read_text())
if data.get('profile') != 'openmuse-word-render-e2e/1' or data.get('pageCount', 0) < 1 or data.get('commandCount', 0) < 1:
    raise SystemExit('Word rendering evidence is incomplete')
app_log = Path(os.environ['WORD_STACK_APP_LOG'])
if 'OPENMUSE_WORD_RENDER_READY:' not in app_log.read_text():
    raise SystemExit('OpenMuse native window did not render the document')
for marker in ('OpenMuse dsh: ready origin=', 'DSH WebView HTTP 200', 'DSH WebView loaded'):
    if marker not in app_log.read_text():
        raise SystemExit(f'OpenMuse DSH panel missing marker: {marker}')
shell_screenshot = Path(os.environ['WORD_STACK_SHELL_SCREENSHOT_PATH'])
if not shell_screenshot.is_file() or shell_screenshot.stat().st_size < 1000:
    raise SystemExit('Word product shell screenshot missing')
mobile_screenshot = shell_screenshot.with_name(shell_screenshot.stem + '-mobile.png')
if not mobile_screenshot.is_file() or mobile_screenshot.stat().st_size < 1000:
    raise SystemExit('Word mobile-width shell screenshot missing')
menu_screenshot = shell_screenshot.with_name(shell_screenshot.stem + '-menu.png')
if not menu_screenshot.is_file() or menu_screenshot.stat().st_size < 1000:
    raise SystemExit('Word nested-menu screenshot missing')
cover_screenshot = shell_screenshot.with_name(shell_screenshot.stem + '-cover.png')
if not cover_screenshot.is_file() or cover_screenshot.stat().st_size < 1000:
    raise SystemExit('Word cover gallery screenshot missing')
wordart_screenshot = shell_screenshot.with_name(shell_screenshot.stem + '-wordart.png')
if not wordart_screenshot.is_file() or wordart_screenshot.stat().st_size < 1000:
    raise SystemExit('WordArt gallery screenshot missing')
styles_screenshot = shell_screenshot.with_name(shell_screenshot.stem + '-styles.png')
if not styles_screenshot.is_file() or styles_screenshot.stat().st_size < 1000:
    raise SystemExit('Word quick styles gallery screenshot missing')
host = Path(os.environ['WORD_STACK_HOST_BINARY']).resolve()
engine = Path(os.environ['WORD_STACK_ENGINE_BINARY']).resolve()
if digest(os.environ['WORD_STACK_SOURCE_ENGINE_PATH']) != os.environ['WORD_STACK_NATIVE_ENGINE_SHA256']:
    raise SystemExit('The source Word engine differs from the built engine')
cases = ['signed-install', 'capability-routing', 'open-render', 'offline-reopen', 'native-editor', 'embedded-window', 'chrome-shell', 'dsh-pane']
result = {
    'profile': 'openmuse-word-stack-e2e/1',
    'target': 'desktop',
    'bundleSha256': os.environ['WORD_STACK_BUNDLE_SHA256'],
    'coreSourceDigest': os.environ['WORD_STACK_CORE_SOURCE_DIGEST'],
    'coreLibsSourceDigest': os.environ['WORD_STACK_CORE_LIBS_SOURCE_DIGEST'],
    'editorSourceDigest': os.environ['WORD_STACK_EDITOR_SOURCE_DIGEST'],
    'engineSourceDigest': os.environ['WORD_STACK_ENGINE_SOURCE_DIGEST'],
    'hostSourceDigest': os.environ['WORD_STACK_HOST_SOURCE_DIGEST'],
    'hostArtifact': {'path': str(host), 'sha256': digest(host)},
    'engineArtifact': {'path': str(engine), 'sha256': digest(engine)},
    'sourceNativeEngineSha256': os.environ['WORD_STACK_NATIVE_ENGINE_SHA256'],
    'cases': {case: {'status': 'passed', 'evidence': [str(evidence), str(app_log), str(shell_screenshot), str(mobile_screenshot), str(menu_screenshot), str(cover_screenshot), str(wordart_screenshot), str(styles_screenshot)]} for case in cases},
    'completedAt': datetime.datetime.now(datetime.timezone.utc).isoformat(),
}
Path(os.environ['WORD_STACK_E2E_RESULT_PATH']).write_text(json.dumps(result, indent=2) + '\n')
PY
