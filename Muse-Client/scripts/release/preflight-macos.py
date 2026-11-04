#!/usr/bin/env python3
"""Read-only macOS preparation audit. Never signs, uploads, or approves a release.

Exit 1: observed blockers; exit 0: no observed blockers (manual checks still open).
The optional app is an existing inspection sample, not a new build provenance claim.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import plistlib
import shutil
import subprocess

ROOT = Path(__file__).resolve().parents[2]


def command(args, cwd=None):
    try:
        result = subprocess.run(args, cwd=cwd, capture_output=True, text=True, timeout=60)
        return {'exitCode': result.returncode, 'stdout': result.stdout.strip(),
                'stderr': result.stderr.strip()}
    except (OSError, subprocess.TimeoutExpired) as error:
        return {'exitCode': -1, 'stdout': '', 'stderr': str(error)}


def inspect(root, app=None):
    files = [
        'app/openmuse_host/macos/Runner/Release.entitlements',
        'app/openmuse_host/macos/Runner/Info.plist',
        'app/openmuse_host/macos/Runner/Configs/AppInfo.xcconfig',
        'app/openmuse_host/macos/Runner.xcodeproj/project.pbxproj',
        'app/openmuse_host/lib/main.dart',
        'app/openmuse_host/lib/src/host/mcp_market.dart',
        'packages/openmuse_app_update/lib/src/update_bootstrap.dart',
        'scripts/package_macos.sh',
    ]
    report = {'schemaVersion': 1, 'observedAt': datetime.now(timezone.utc).isoformat(),
              'kind': 'preparation-audit', 'readyForDistribution': False,
              'sourceFiles': {}, 'blockers': [], 'manualChecks': [
                  'Confirm US scope, price, age rating, copyright and store identity.',
                  'Freeze reviewed commits and cross-repository dependencies before building.',
                  'Audit SDK privacy manifests, required-reason APIs and actual data flows.',
                  'Verify consent, account deletion and AI sharing in the macOS product.',
                  'Run signed Release install/upgrade and Notes/Slides save/reopen acceptance.',
                  'Capture real Mac screenshots with build provenance; validate store processing.',
              ]}

    def block(lanes, code, detail):
        report['blockers'].append({'lanes': lanes, 'code': code, 'detail': detail})

    both = ['macos-store-us', 'macos-direct-us']
    free = shutil.disk_usage(root).free
    report['diskSpace'] = {'freeBytes': free, 'minimumBuildFreeBytes': 10 * 1024**3}
    if free < 10 * 1024**3:
        block(both, 'insufficient-disk-space',
              f'{free / 1024**3:.1f} GiB free; release builds require at least 10 GiB.')
    texts = {}
    for relative in files:
        path = root / relative
        if not path.is_file():
            block(both, 'missing-input', relative)
            continue
        data = path.read_bytes()
        report['sourceFiles'][relative] = hashlib.sha256(data).hexdigest()
        texts[relative] = data.decode()
    entitlements = root / files[0]
    if entitlements.is_file():
        try:
            values = plistlib.loads(entitlements.read_bytes())
            report['releaseEntitlements'] = values
            if values.get('com.apple.security.app-sandbox') is not True:
                block(['macos-store-us'], 'sandbox-disabled', files[0])
        except (ValueError, plistlib.InvalidFileException):
            block(both, 'invalid-entitlements', files[0])
    # These are review triggers, not proof that all paths are reachable.
    triggers = [
        (files[4], 'bootOpenMuseUpdates(', 'external-updater-review',
         'Store build must exclude custom update polling, UI and installer.'),
        (files[5], 'Future<Map<String, Object?>> install(', 'downloaded-code-review',
         'Review MCPB download/execution; define a Store-specific capability allowlist.'),
        (files[7], 'dsh_dest=', 'child-process-review',
         'Node/DSH/CLI/Helix helpers need sandbox, signing, JIT and exit-lifecycle review.'),
    ]
    for relative, marker, code, detail in triggers:
        if marker in texts.get(relative, ''):
            block(['macos-store-us'], code, detail + ' Source: ' + relative)

    report['xcode'] = command(['xcodebuild', '-version'])
    report['signingIdentities'] = command(['security', 'find-identity', '-v', '-p', 'codesigning'])
    identities = report['signingIdentities']
    if identities['exitCode'] or 'Developer ID Application:' not in identities['stdout']:
        block(['macos-direct-us'], 'developer-id-unavailable',
              'No usable Developer ID Application identity observed in this keychain context.')
    if identities['exitCode'] or not any(name in identities['stdout'] for name in
                                       ['Apple Distribution:', '3rd Party Mac Developer Application:']):
        block(['macos-store-us'], 'store-identity-unavailable',
              'No usable Store application identity observed; also verify the macOS profile.')
    report['repositories'] = {}
    for name in ['Muse-Client', 'Muse-Plugin-Ecosystem', 'Muse-Server', 'Muse-WebSite', 'ioffice']:
        repo = root if name == 'Muse-Client' else root.parent / name
        revision = command(['git', 'rev-parse', 'HEAD'], cwd=repo)
        status = command(['git', 'status', '--porcelain', '--untracked-files=normal'], cwd=repo)
        dirty = bool(status['stdout']) if status['exitCode'] == 0 else None
        report['repositories'][name] = {'commit': revision['stdout'], 'dirty': dirty,
                                        'inspectionOnly': True}
        if dirty is not False or revision['exitCode']:
            block(both, 'source-not-frozen', name + ': dirty or revision unavailable')
    if app:
        info = app / 'Contents/Info.plist'
        report['appSample'] = {'path': str(app), 'provenance': 'existing-unfrozen-sample'}
        sample = report['appSample']
        if not info.is_file():
            block(both, 'app-missing', str(info))
        else:
            values = plistlib.loads(info.read_bytes())
            sample['identity'] = {key: values.get(key) for key in
                                  ['CFBundleIdentifier', 'CFBundleShortVersionString',
                                   'CFBundleVersion', 'LSMinimumSystemVersion']}
            sample['signature'] = command(['codesign', '-dv', '--verbose=4', str(app)])
            signature = sample['signature']['stderr'] + sample['signature']['stdout']
            if sample['signature']['exitCode'] or 'Signature=adhoc' in signature:
                block(both, 'sample-not-distribution-signed', 'Existing app has no distribution signature.')
            if 'runtime)' not in signature:
                block(['macos-direct-us'], 'sample-runtime-not-hardened', 'Hardened Runtime not observed.')
            sample['entitlements'] = command(['codesign', '-d', '--entitlements', ':-', str(app)])
            try:
                signed = plistlib.loads(sample['entitlements']['stdout'].encode())
                if signed.get('com.apple.security.app-sandbox') is not True:
                    block(['macos-store-us'], 'sample-not-sandboxed', 'Signed sample sandbox is not enabled.')
            except (ValueError, plistlib.InvalidFileException):
                block(both, 'sample-entitlements-unreadable', 'Could not parse signed entitlements.')
    else:
        block(both, 'no-binary-inspected', 'Provide --app to inspect an existing app sample.')
    report['status'] = 'blocked' if report['blockers'] else 'manual-validation-required'
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--app', type=Path)
    parser.add_argument('--report', type=Path, required=True)
    args = parser.parse_args()
    result = inspect(ROOT, args.app.resolve() if args.app else None)
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(result, ensure_ascii=False, indent=2) + '\n')
    print(f"macOS preparation: {result['status']}; {len(result['blockers'])} blockers. {args.report}")
    raise SystemExit(1 if result['blockers'] else 0)


if __name__ == '__main__':
    main()
