#!/usr/bin/env python3
"""Stage reviewed fixtures and optional credentials in a development app only."""
import argparse
import hashlib
from pathlib import Path
import shlex
import subprocess

ROOT = Path(__file__).resolve().parents[2]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('--platform', choices=['android', 'ios'], required=True)
parser.add_argument('--device', required=True)
parser.add_argument('--credentials', type=Path)
parser.add_argument('--genoffice', action='store_true')
parser.add_argument('--slides', action='store_true', help='Stage reviewed bundled OpenMuse Slides and the shipped introduction')
args = parser.parse_args()
files = {}
if args.credentials:
    if args.credentials.stat().st_mode & 0o077:
        raise SystemExit('Credentials must be owner-readable only (chmod 600).')
    files['acceptance-credentials.json'] = args.credentials
if args.genoffice:
    files['acceptance-inputs/genoffice.mcpb'] = ROOT.parent / 'Muse-Plugin-Ecosystem/plugins/genoffice-pptx/dist/genoffice-pptx-0.8.0.mcpb'
    files['acceptance-inputs/sample.pptx'] = ROOT / 'app/openmuse_mobile/test_assets/genoffice_standard_business.pptx'
if args.slides:
    files['acceptance-inputs/genoffice.mcpb'] = ROOT / 'app/openmuse_mobile/assets/launch/slides.mcpb'
    files['acceptance-inputs/sample.pptx'] = ROOT / 'packages/openmuse_resource_center_host/assets/welcome/OpenMuse-Introduction.pptx'
for destination, source in files.items():
    if args.platform == 'android':
        package = 'com.openmuseai.mobile.acceptance'
        target = 'files/' + destination
        script = 'mkdir -p ' + shlex.quote(str(Path(target).parent)) + '; umask 077; cat > ' + shlex.quote(target)
        command = 'run-as ' + package + ' sh -c ' + shlex.quote(script)
        subprocess.run(['adb', '-s', args.device, 'shell', command], input=source.read_bytes(), check=True, capture_output=True)
    else:
        subprocess.run(['xcrun', 'devicectl', 'device', 'copy', 'to', '--device', args.device,
                        '--source', str(source), '--destination', 'Library/Application Support/' + destination,
                        '--domain-type', 'appDataContainer', '--domain-identifier', 'com.openmuseai.mobile', '--quiet'], check=True, capture_output=True)
    if destination.endswith('.mcpb'):
        print('Reviewed GenOffice SHA-256: ' + hashlib.sha256(source.read_bytes()).hexdigest())
print('Acceptance inputs staged in the development app private directory.')
