#!/usr/bin/env python3
"""Prepare a development-only AOT app without changing the App Store archive."""
import argparse
import hashlib
from pathlib import Path
import plistlib
import subprocess

ROOT = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--app', type=Path, required=True)
parser.add_argument('--output', type=Path, required=True)
parser.add_argument('--profile', type=Path, required=True)
parser.add_argument('--identity', required=True)
parser.add_argument('--device-udid', required=True)
parser.add_argument('--display-name', choices=['OpenMuse QA'], help='Label an integration-test entry point distinctly; never use it for manual/store screenshots.')
args = parser.parse_args()
output = args.output.resolve()
if not output.is_relative_to(ROOT / 'target/mobile-acceptance') or output.exists():
    raise SystemExit('Use a new output inside target/mobile-acceptance.')
info = plistlib.loads((args.app / 'Info.plist').read_bytes())
profile = plistlib.loads(subprocess.check_output([
    'security', 'cms', '-D', '-i', str(args.profile),
]))
entitlements = profile['Entitlements']
team = profile['TeamIdentifier'][0]
assert entitlements['application-identifier'] == team + '.' + info['CFBundleIdentifier']
assert entitlements.get('get-task-allow') is True
assert args.device_udid in profile.get('ProvisionedDevices', [])
assert args.identity.lower() in [
    hashlib.sha1(cert).hexdigest() for cert in profile['DeveloperCertificates']
]
output.parent.mkdir(parents=True, exist_ok=True)
subprocess.run(['ditto', str(args.app), str(output)], check=True)
if args.display_name:
    info['CFBundleDisplayName'] = args.display_name
    (output / 'Info.plist').write_bytes(plistlib.dumps(info))
(output / 'embedded.mobileprovision').write_bytes(args.profile.read_bytes())
entitlement_file = output.parent / 'development-entitlements.plist'
entitlement_file.write_bytes(plistlib.dumps(entitlements))
# Xcode debug builds also contain Runner.debug.dylib next to the executable.
# Sign nested Mach-O images before signing the enclosing app.
for library in output.rglob('*.dylib'):
    subprocess.run(['codesign', '--force', '--sign', args.identity,
                    '--timestamp=none', str(library)], check=True, capture_output=True)
for framework in (output / 'Frameworks').glob('*.framework'):
    subprocess.run(['codesign', '--force', '--sign', args.identity,
                    '--timestamp=none', str(framework)], check=True, capture_output=True)
subprocess.run(['codesign', '--force', '--sign', args.identity, '--timestamp=none',
                '--entitlements', str(entitlement_file), '--generate-entitlement-der',
                str(output)], check=True, capture_output=True)
subprocess.run(['codesign', '--verify', '--deep', '--strict', str(output)], check=True)
for verifier in ['verify-ios-bundle.py', 'verify-mobile-closure.py']:
    subprocess.run(['python3', str(ROOT / 'scripts/release' / verifier), str(output)], check=True)
print('Prepared development-only device app:', output)
