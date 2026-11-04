"""Run the isolated plugin acceptance APK without a Dart VM log handshake."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import time

ROOT = Path(__file__).resolve().parents[1]
APP = ROOT / 'flutter_runtime/example'
PACKAGE = 'io.openmuse.siyuan.siyuan_plugin_acceptance'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--device', required=True)
    parser.add_argument('--skip-build', action='store_true')
    args = parser.parse_args()
    env = os.environ.copy()
    env['GRADLE_USER_HOME'] = os.path.expanduser(env.get('GRADLE_USER_HOME', '~/.gradle'))
    env['GRADLE_OPTS'] = env.get('GRADLE_OPTS', '') + ' -Dorg.gradle.daemon=false'
    if not args.skip_build:
        subprocess.run(['flutter', 'build', 'apk', '--debug', '--target-platform', 'android-arm64',
                        '--target', 'integration_test/client_acceptance_test.dart'], cwd=APP, env=env, check=True)
    apk = APP / 'build/app/outputs/flutter-apk/app-debug.apk'
    sdk = Path(env.get('ANDROID_HOME', str(Path.home() / 'Library/Android/sdk')))
    actual = subprocess.check_output([str(sdk / 'cmdline-tools/latest/bin/apkanalyzer'),
                                     'manifest', 'application-id', str(apk)], text=True).strip()
    if actual != PACKAGE:
        raise SystemExit('Refusing to replace a non-acceptance application')
    adb = ['adb', '-s', args.device]
    subprocess.run([*adb, 'install', '-r', str(apk)], check=True, timeout=240)
    subprocess.run([*adb, 'shell', 'am', 'force-stop', PACKAGE], check=True)
    subprocess.run([*adb, 'shell', 'run-as', PACKAGE, 'rm', '-f', 'files/siyuan-acceptance.json'], check=True)
    subprocess.run([*adb, 'shell', 'am', 'start', '-n', PACKAGE + '/.MainActivity',
                    '--ez', 'enable-checked-mode', 'true', '--ez', 'enable-impeller', 'false'], check=True)
    destination = ROOT / 'dist/acceptance'
    destination.mkdir(parents=True, exist_ok=True)
    deadline = time.monotonic() + 300
    while time.monotonic() < deadline:
        reply = subprocess.run([*adb, 'shell', 'run-as', PACKAGE, 'cat', 'files/siyuan-acceptance.json'],
                               capture_output=True, text=True, timeout=10)
        if reply.returncode == 0 and reply.stdout.strip():
            result = json.loads(reply.stdout)
            if not isinstance(result.get('passed'), bool) or not result.get('checks'):
                raise SystemExit('Invalid native acceptance result')
            (destination / 'android.json').write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
            with (destination / 'android.png').open('wb') as image:
                subprocess.run([*adb, 'exec-out', 'screencap', '-p'], stdout=image, check=True)
            print(json.dumps(result, ensure_ascii=False, indent=2), flush=True)
            raise SystemExit(0 if result['passed'] else 1)
        time.sleep(1)
    raise SystemExit('Native acceptance did not finish within five minutes')


if __name__ == '__main__':
    main()
