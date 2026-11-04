"""Stage a remote MCP service using the reviewed MCPB's exact UI bytes.

Never deploy dist/web after a subsequent rebuild; bundler output can differ.
"""
import argparse
import hashlib
import json
import shutil
import zipfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser(description=__doc__)
parser.add_argument('bundle', type=Path)
parser.add_argument('--out', type=Path, default=ROOT / 'dist/release-service')
args = parser.parse_args()
with zipfile.ZipFile(args.bundle) as archive:
    manifest = json.loads(archive.read('manifest.json'))
    ui = archive.read('web/index.html')
    if ui != archive.read('server/web/index.html'):
        raise ValueError('Bundle UI copies differ')
    if manifest['version'] != json.loads((ROOT / 'package.json').read_text())['version']:
        raise ValueError('Service source and reviewed bundle versions differ')
target = args.out / 'dist'
(target / 'web').mkdir(parents=True, exist_ok=True)
shutil.copyfile(ROOT / 'dist/http.mjs', target / 'http.mjs')
(target / 'web/index.html').write_bytes(ui)
record = {'version': manifest['version'], 'packageSha256': hashlib.sha256(args.bundle.read_bytes()).hexdigest(), 'uiSha256': hashlib.sha256(ui).hexdigest()}
(args.out / 'release.json').write_text(json.dumps(record, indent=2) + '\n')
print(json.dumps(record, indent=2))
