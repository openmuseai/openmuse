#!/usr/bin/env python3
"""Sign an already staged Store app inside-out and create a signed Apple PKG.

Does not upload or mark a pipeline stage passed. Input builds still need frozen
source provenance and signed Release acceptance before submission.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
from pathlib import Path
import plistlib
import shutil
import stat
import subprocess
import tempfile

ROOT = Path(__file__).resolve().parents[2]
MACHO = {bytes.fromhex(s) for s in ['feedface', 'cefaedfe', 'feedfacf', 'cffaedfe',
                                   'cafebabe', 'bebafeca', 'cafebabf', 'bfbafeca']}


def execute(args):
    result = subprocess.run([str(a) for a in args], capture_output=True, text=True)
    if result.returncode:
        raise RuntimeError(result.stderr.strip() or result.stdout.strip())
    return result.stdout.strip()


def is_macho(path):
    if path.is_symlink() or not path.is_file():
        return False
    with path.open('rb') as stream:
        return stream.read(4) in MACHO


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--app', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--profile', type=Path, required=True)
    parser.add_argument('--identity', default='12C7FF3A2A764F0442A08B9FE77EC056B81841F6')
    parser.add_argument('--installer-identity', default='3rd Party Mac Developer Installer: Dongguan Chengxuan Electronic Science & Technology Co., Ltd (SDPXS943DW)')
    args = parser.parse_args()
    app_source, output = args.app.resolve(), args.output.resolve()
    if output.exists():
        raise SystemExit('Output exists; choose a new candidate directory.')
    info = plistlib.loads((app_source / 'Contents/Info.plist').read_bytes())
    if info.get('OpenMuseDistributionChannel') != 'app-store':
        raise SystemExit('Input is not marked as an App Store build.')
    profile = plistlib.loads(subprocess.check_output(['security', 'cms', '-D', '-i', str(args.profile)]))
    app_id = profile['Entitlements']['com.apple.application-identifier']
    team = profile['TeamIdentifier'][0]
    if app_id != f"{team}.{info['CFBundleIdentifier']}" or 'ProvisionedDevices' in profile:
        raise SystemExit('Profile is not a matching Store distribution profile.')
    expiry = profile['ExpirationDate'].replace(tzinfo=timezone.utc)
    if expiry <= datetime.now(timezone.utc):
        raise SystemExit('Profile has expired.')
    output.mkdir(parents=True)
    app = output / 'OpenMuse.app'
    execute(['ditto', app_source, app])
    shutil.copyfile(args.profile, app / 'Contents/embedded.provisionprofile')
    # Browser-downloaded provisioning profiles can carry quarantine metadata.
    # Apple rejects this attribute anywhere in a TestFlight/App Store payload.
    attribute_listing = execute(['xattr', '-lr', app])
    quarantine_removed = [line.split(': com.apple.quarantine:', 1)[0]
                          .removeprefix(str(app) + '/')
                          for line in attribute_listing.splitlines()
                          if ': com.apple.quarantine:' in line]
    execute(['xattr', '-r', '-d', 'com.apple.quarantine', app])
    # Installer installs payloads as root. Every distributed resource must be
    # readable by a normal user; local source files may have restrictive modes.
    permission_changes = []
    for path in [app, *app.rglob('*')]:
        if path.is_symlink():
            continue
        before = stat.S_IMODE(path.stat().st_mode)
        after = 0o755 if path.is_dir() or before & 0o111 else 0o644
        if before != after:
            path.chmod(after)
            permission_changes.append({'path': str(path.relative_to(app)),
                                       'before': oct(before), 'after': oct(after)})
    entitlements = plistlib.loads((ROOT / 'app/openmuse_host/macos/Runner/AppStore.entitlements').read_bytes())
    entitlements.update({'com.apple.application-identifier': app_id,
                         'com.apple.developer.team-identifier': team,
                         'keychain-access-groups': [app_id]})
    code_files = sorted((p for p in app.rglob('*') if is_macho(p)),
                        key=lambda p: len(p.parts), reverse=True)
    bundles = sorted((p for p in app.rglob('*') if p.is_dir() and not p.is_symlink()
                      and p.suffix in {'.framework', '.app', '.xpc', '.appex'}),
                     key=lambda p: len(p.parts), reverse=True)
    with tempfile.TemporaryDirectory() as temporary:
        temp = Path(temporary)
        root_entitlements = temp / 'app.plist'
        root_entitlements.write_bytes(plistlib.dumps(entitlements))
        inherited = temp / 'helper.plist'
        inherited.write_bytes(plistlib.dumps({'com.apple.security.app-sandbox': True,
                                             'com.apple.security.inherit': True}))
        for path in code_files:
            arguments = ['codesign', '--force', '--sign', args.identity, '--timestamp']
            # Libraries inherit their containing process. Standalone bundled
            # interpreters/helpers explicitly inherit the app's sandbox.
            if path.name == 'node':
                arguments += ['--entitlements', str(inherited)]
            execute([*arguments, path])
        for bundle in bundles:
            execute(['codesign', '--force', '--sign', args.identity, '--timestamp', bundle])
        execute(['codesign', '--force', '--sign', args.identity, '--timestamp',
                 '--entitlements', root_entitlements, app])
    for path in code_files:
        execute(['codesign', '--verify', '--strict', path])
    execute(['codesign', '--verify', '--deep', '--strict', app])
    signed = plistlib.loads(subprocess.run(['codesign', '-d', '--entitlements', ':-', str(app)],
                                          check=True, capture_output=True).stdout)
    if any(signed.get(k) != v for k, v in entitlements.items()):
        raise SystemExit('Final app entitlement mismatch.')
    unreadable = [str(path.relative_to(app)) for path in [app, *app.rglob('*')]
                  if not path.is_symlink()
                  and (not path.stat().st_mode & stat.S_IROTH
                       or path.is_dir() and not path.stat().st_mode & stat.S_IXOTH)]
    if unreadable:
        raise SystemExit('Distribution payload is unreadable by non-root users: '
                         + ', '.join(unreadable))
    if 'com.apple.quarantine:' in execute(['xattr', '-lr', app]):
        raise SystemExit('Quarantine metadata remains in the distribution payload.')
    pkg = output / f"OpenMuse-macos-store-{info['CFBundleShortVersionString']}-{info['CFBundleVersion']}.pkg"
    execute(['productbuild', '--component', app, '/Applications', '--sign', args.installer_identity, pkg])
    pkg_signature = execute(['pkgutil', '--check-signature', pkg])
    report = {'status': 'signed-awaiting-release-acceptance', 'distributionChannel': 'macos-store-us',
              'teamId': team, 'bundleId': info['CFBundleIdentifier'],
              'version': info['CFBundleShortVersionString'], 'build': info['CFBundleVersion'],
              'sandboxEnabled': True, 'signedCodeFiles': len(code_files),
              'permissionChanges': permission_changes,
              'unreadablePayloadFiles': unreadable,
              'quarantineAttributesRemoved': quarantine_removed,
              'quarantineAttributesRemaining': [],
              'sha256': hashlib.sha256(pkg.read_bytes()).hexdigest(),
              'profileUUID': profile['UUID'], 'pkgSignature': pkg_signature,
              'signedAt': datetime.now(timezone.utc).isoformat()}
    (output / 'signing-report.json').write_text(json.dumps(report, indent=2) + '\n')
    print(pkg)


if __name__ == '__main__':
    main()
