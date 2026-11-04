"""Run resource UI acceptance with native suite results as the authority."""
import argparse
import json
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = 'com.openmuseai.mobile.acceptance'
CHECKS = {'resourceCenterCreator', 'deviceSourceNavigation', 'standaloneEditorWithoutApplicationShell', 'editorThemeAndIcons', 'nativeMobileDocumentStructure', 'nativeEditControlsAndAutosave', 'noInjectedEditorToolbar', 'automaticSaveAndCloseBarrier', 'recentReopenAndReadOnlyView', 'hostNavigationAndDocumentTitle', 'keyboardViewportAndCaret', 'nativeIconPicker', 'linkedDocumentHandoff', 'packageImageRendering', 'emptyDocumentTitleSuppressed'}
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--device', required=True)
parser.add_argument('--apk', type=Path)
parser.add_argument('--skip-install', action='store_true')
parser.add_argument('--visual-checkpoints', action='store_true')
parser.add_argument('--word-attachment', action='store_true')
args = parser.parse_args()
label = 'android-emulator' if args.device.startswith('emulator-') else 'android-device'
evidence = ROOT / 'docs/evidence/editor-parity'
destination = ROOT / 'dist/acceptance' / ('resource-ui-' + label)
destination.mkdir(parents=True, exist_ok=True)

def adb(*parts, timeout=30):
    return subprocess.check_output(['adb', '-s', args.device, *parts], stderr=subprocess.STDOUT, timeout=timeout)

if not args.skip_install:
    if args.apk is None:
        parser.error('--apk is required unless --skip-install is used')
    import os
    analyzer = Path(os.environ.get('ANDROID_HOME', str(Path.home() / 'Library/Android/sdk'))) / 'cmdline-tools/latest/bin/apkanalyzer'
    actual = subprocess.check_output([str(analyzer), 'manifest', 'application-id', str(args.apk)], text=True).strip()
    if actual != PACKAGE:
        raise SystemExit('Refusing to install a test over the ordinary application')
    print('Installing isolated acceptance APK (upgrade only).', flush=True)
    try:
        print(adb('install', '--no-streaming', '-r', str(args.apk), timeout=300).decode(), flush=True)
    except subprocess.CalledProcessError as error:
        raise SystemExit(error.output.decode(errors='replace')) from None
adb('shell', 'am', 'force-stop', PACKAGE)
adb('shell', 'run-as', PACKAGE, 'rm', '-f', 'files/acceptance-results.json', 'files/siyuan-resource-ui.json', 'files/resource-visual-ready.json')
adb('shell', 'run-as', PACKAGE, 'rm', '-f', 'files/resource-visual-release-icon-picker', 'files/resource-visual-release-focus-native', 'files/resource-visual-release-keyboard', 'files/resource-visual-release-image-preview', 'files/resource-visual-release-linked-document', 'files/resource-visual-checkpoints', 'files/resource-word-attachment', 'files/resource-visual-release-word-attachment')
if args.visual_checkpoints:
    adb('shell', 'run-as', PACKAGE, 'touch', 'files/resource-visual-checkpoints')
if args.word_attachment:
    adb('shell', 'run-as', PACKAGE, 'touch', 'files/resource-word-attachment')
adb('shell', 'am', 'start', '-n', PACKAGE + '/com.openmuseai.mobile.MainActivity', '--ez', 'enable-checked-mode', 'true', '--ez', 'enable-impeller', 'false')
adb('shell', 'cmd', 'statusbar', 'collapse')
print('Waiting for the native suite.', flush=True)
deadline = time.monotonic() + 480
while time.monotonic() < deadline:
    try:
        raw = adb('shell', 'run-as', PACKAGE, 'cat', 'files/acceptance-results.json', timeout=20)
    except (subprocess.CalledProcessError, subprocess.TimeoutExpired):
        time.sleep(1)
        continue
    result = json.loads(raw)
    (destination / 'result.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    if result.get('passed') is not True:
        raise SystemExit(json.dumps(result, ensure_ascii=False))
    ui = json.loads(adb('shell', 'run-as', PACKAGE, 'cat', 'files/siyuan-resource-ui.json'))
    if ui.get('passed') is not True or any(ui.get('checks', {}).get(c) is not True for c in CHECKS):
        raise SystemExit('Incomplete resource UI result')
    if args.word_attachment and ui.get('checks', {}).get('systemWordDocumentOpen') is not True:
        raise SystemExit('Incomplete system Word UI result')
    ui.update({'device': args.device, 'deviceKind': label})
    evidence.mkdir(parents=True, exist_ok=True)
    (evidence / (label + '.json')).write_text(json.dumps(ui, ensure_ascii=False, indent=2) + '\n')
    (evidence / (label + '-native-suite.json')).write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(json.dumps(ui, ensure_ascii=False, indent=2), flush=True)
    break
else:
    raise SystemExit('Native resource UI suite timed out')
