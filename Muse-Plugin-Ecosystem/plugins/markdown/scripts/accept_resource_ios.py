"""Run the real Mobile Host UI suite in an isolated iOS Simulator application."""
import argparse
import json
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
PACKAGE = 'com.openmuseai.mobile.acceptance'
CHECKS = {'resourceCenterCreator', 'deviceSourceNavigation', 'standaloneEditorWithoutApplicationShell', 'editorThemeAndIcons', 'nativeMobileDocumentStructure', 'nativeEditControlsAndAutosave', 'noInjectedEditorToolbar', 'automaticSaveAndCloseBarrier', 'recentReopenAndReadOnlyView', 'hostNavigationAndDocumentTitle', 'keyboardViewportAndCaret', 'nativeIconPicker', 'linkedDocumentHandoff', 'packageImageRendering', 'emptyDocumentTitleSuppressed'}
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--simulator', required=True)
parser.add_argument('--app', type=Path, required=True)
parser.add_argument('--visual-checkpoints', action='store_true')
args = parser.parse_args()
import plistlib
assert plistlib.loads((args.app / 'Info.plist').read_bytes())['CFBundleIdentifier'] == PACKAGE
subprocess.run(['xcrun', 'simctl', 'install', args.simulator, str(args.app)], check=True)
data = Path(subprocess.check_output(['xcrun', 'simctl', 'get_app_container', args.simulator, PACKAGE, 'data'], text=True).strip()) / 'Library/Application Support'
for name in ['acceptance-results.json', 'siyuan-resource-ui.json', 'resource-visual-ready.json']:
    (data / name).unlink(missing_ok=True)
for name in ['resource-visual-release-icon-picker', 'resource-visual-release-focus-native', 'resource-visual-release-keyboard', 'resource-visual-release-image-preview', 'resource-visual-release-linked-document', 'resource-visual-checkpoints']:
    (data / name).unlink(missing_ok=True)
if args.visual_checkpoints:
    (data / 'resource-visual-checkpoints').touch()
destination = ROOT / 'dist/acceptance/resource-ui-ios'
destination.mkdir(parents=True, exist_ok=True)
with (destination / 'console.log').open('w') as log:
    process = subprocess.Popen(['xcrun', 'simctl', 'launch', '--console', '--terminate-running-process', args.simulator, PACKAGE], stdout=log, stderr=subprocess.STDOUT)
    deadline = time.monotonic() + (480 if args.visual_checkpoints else 240)
    while time.monotonic() < deadline:
        report = data / 'acceptance-results.json'
        if report.exists():
            result = json.loads(report.read_text())
            (destination / 'result.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
            if result.get('passed') is not True:
                raise SystemExit(json.dumps(result, ensure_ascii=False))
            ui = json.loads((data / 'siyuan-resource-ui.json').read_text())
            if ui.get('passed') is not True or any(ui.get('checks', {}).get(c) is not True for c in CHECKS):
                raise SystemExit('Incomplete Host UI result')
            (ROOT / 'docs/evidence/editor-parity/ios.json').write_text(json.dumps(ui, ensure_ascii=False, indent=2) + '\n')
            (ROOT / 'docs/evidence/editor-parity/ios-native-suite.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
            print(json.dumps(ui, ensure_ascii=False, indent=2), flush=True)
            break
        if process.poll() is not None:
            raise SystemExit('Simulator launch failed; inspect ' + str(destination / 'console.log'))
        time.sleep(1)
    else:
        raise SystemExit('Native Host UI suite timed out')
