#!/usr/bin/env python3
"""Run an already-built acceptance APK without touching the store application."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time

ROOT = Path(__file__).resolve().parents[2]
PACKAGE = 'com.openmuseai.mobile.acceptance'
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--device', required=True)
parser.add_argument('--apk', type=Path, required=True)
parser.add_argument('--evidence', type=Path, required=True)
parser.add_argument('--credentials', type=Path)
parser.add_argument('--genoffice', action='store_true')
parser.add_argument('--capture-stages', action='store_true')
parser.add_argument('--skip-install', action='store_true', help='Reuse only an installed base APK whose SHA-256 exactly matches --apk.')
parser.add_argument('--resources',type=Path,help='Private resource-lab connection input (acceptance package only).')
args = parser.parse_args()
args.evidence.mkdir(parents=True, exist_ok=True)
sdk = Path(os.environ.get('ANDROID_HOME', str(Path.home() / 'Library/Android/sdk')))
analyzer = sdk / 'cmdline-tools/latest/bin/apkanalyzer'
package = subprocess.check_output([str(analyzer), 'manifest', 'application-id', str(args.apk)], text=True).strip()
if package != PACKAGE:
    raise SystemExit('Refusing to install a test APK over the store application.')

def adb(*parts, **kwargs):
    return subprocess.check_output(['adb', '-s', args.device, *parts], **kwargs)

# adb install fails on signature/version mismatch; it never uninstalls old data.
if args.skip_install:
    paths = adb('shell', 'pm', 'path', PACKAGE, text=True).splitlines()
    bases = [line.removeprefix('package:') for line in paths if line.endswith('/base.apk')]
    if len(bases) != 1 or not re.fullmatch(r'/data/app/[A-Za-z0-9_./=+~-]+/base\.apk', bases[0]):
        raise SystemExit('Cannot identify the installed isolated acceptance APK.')
    installed = adb('shell', 'sha256sum', bases[0], text=True).split()[0]
    if installed != hashlib.sha256(args.apk.read_bytes()).hexdigest():
        raise SystemExit('Installed acceptance APK differs from --apk; refusing to run stale tests.')
    print('Reusing byte-identical installed isolated acceptance APK.', flush=True)
else:
    print('Installing isolated acceptance APK (upgrade only).', flush=True)
    subprocess.run(['adb', '-s', args.device, 'install', '-r', str(args.apk)], check=True,
                   stdout=subprocess.PIPE, stderr=subprocess.STDOUT, timeout=120)
adb('shell', 'am', 'force-stop', PACKAGE)
adb('shell', 'run-as', PACKAGE, 'mkdir', '-p', 'files')
if args.resources:
    adb('shell','run-as',PACKAGE,'tee','files/acceptance-resources.json',input=args.resources.read_bytes())
if args.credentials or args.genoffice:
    stage = [sys.executable, str(ROOT / 'scripts/release/stage-mobile-acceptance-inputs.py'),
             '--platform', 'android', '--device', args.device]
    if args.credentials:
        stage += ['--credentials', str(args.credentials)]
    if args.genoffice:
        stage += ['--genoffice']
    subprocess.run(stage, check=True)
adb('shell', 'run-as', PACKAGE, 'rm', '-f', 'files/acceptance-results.json')
if args.capture_stages:
    adb('shell', 'run-as', PACKAGE, 'sh', '-c', '"rm -f files/acceptance-stage files/acceptance-release-*; touch files/acceptance-capture-stages"')
else:
    adb('shell', 'run-as', PACKAGE, 'rm', '-f', 'files/acceptance-capture-stages')
adb('shell', 'am', 'start', '-n', PACKAGE + '/com.openmuseai.mobile.MainActivity',
    '--ez', 'enable-checked-mode', 'true', '--ez', 'enable-impeller', 'false')
# A previously expanded notification shade leaves the app without clipboard
# focus even though Flutter can render. Restore the test Activity's foreground.
adb('shell','cmd','statusbar','collapse')
print('Waiting for the native test binding to finish the suite.', flush=True)
deadline = time.monotonic() + 360
captured = set()
while time.monotonic() < deadline:
    if args.capture_stages:
        stage_result = subprocess.run(['adb','-s',args.device,'shell','run-as',PACKAGE,'cat','files/acceptance-stage'],capture_output=True,text=True,timeout=10)
        stage = stage_result.stdout.strip()
        if stage_result.returncode == 0 and re.fullmatch(r'[a-z0-9-]{1,64}',stage) and stage not in captured:
            png = adb('exec-out','screencap','-p')
            (args.evidence / (stage + '.png')).write_bytes(png)
            adb('shell','run-as',PACKAGE,'touch','files/acceptance-release-'+stage)
            captured.add(stage)
            print('Captured native stage:',stage,flush=True)
    response = subprocess.run(['adb', '-s', args.device, 'shell', 'run-as', PACKAGE,
        'cat', 'files/acceptance-results.json'], capture_output=True, text=True, timeout=10)
    if response.returncode == 0 and response.stdout.strip():
        result = json.loads(response.stdout)
        if not isinstance(result.get('passed'), bool) or not result.get('tests'):
            raise SystemExit('Invalid native test result.')
        (args.evidence / 'result.json').write_text(json.dumps(result, indent=2) + '\n')
        print('Native suite passed:', result['passed'], flush=True)
        for name, status in result['tests'].items():
            print(name + ': ' + status, flush=True)
        if result['passed']:
            # A completed integration binding stops scheduling ordinary UI
            # frames. Never leave that frozen QA surface for manual review.
            adb('shell', 'am', 'force-stop', PACKAGE)
        raise SystemExit(0 if result['passed'] else 1)
    time.sleep(1)
raise SystemExit('The native test binding did not finish within six minutes.')
