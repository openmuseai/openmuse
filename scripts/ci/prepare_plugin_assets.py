#!/usr/bin/env python3
"""Generate and verify assets from the Client's immutable plugin composition."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys
import zipfile

from verify_plugin_sources import verify

ROOT = Path(__file__).resolve().parents[2]


def digest(data):
    return hashlib.sha256(data).hexdigest()


def verify_runtime(path):
    with zipfile.ZipFile(path) as archive:
        names = archive.namelist()
        if len(names) != len(set(names)):
            raise ValueError('Duplicate runtime archive entry')
        for name in names:
            if name.startswith('/') or '..' in Path(name).parts:
                raise ValueError('Unsafe runtime archive entry')
        catalog = json.loads(archive.read('catalog.json'))
        if set(names) != set(catalog) | {'catalog.json'}:
            raise ValueError('Runtime catalog does not cover the archive')
        for required in ('LICENSE', 'THIRD_PARTY_NOTICES.md', 'upstream.json'):
            if required not in catalog:
                raise ValueError('Runtime attribution missing: ' + required)
        for folder in ('app/appearance/', 'app/stage/', 'app/guide/'):
            if not any(name.startswith(folder) for name in catalog):
                raise ValueError('Runtime output missing: ' + folder)
        for name, expected in catalog.items():
            if digest(archive.read(name)) != expected:
                raise ValueError('Runtime catalog digest mismatch: ' + name)


def collect_assets(client, ecosystem, runtime=False):
    markdown = ecosystem / 'plugins/markdown'
    specs = [
        (client / 'packages/muse_appflowy_realtime/assets/wasm', 'muse_appflowy_realtime'),
        (markdown / 'smart_document/flutter/assets/wasm', 'muse_smart_document'),
        (markdown / 'smart_document/dataset/flutter/assets/wasm', 'muse_dataset_core'),
    ]
    files = []
    for directory, name in specs:
        wasm = directory / (name + '_bg.wasm')
        glue = directory / (name + '.js')
        if wasm.read_bytes()[:8] != b'\0asm\x01\0\0\0':
            raise ValueError('Invalid WASM module: ' + str(wasm))
        if 'globalThis.' + name + '_wasm' not in glue.read_text():
            raise ValueError('WASM global loader missing: ' + str(glue))
        files.extend([wasm, glue])
    if runtime:
        archive = markdown / 'flutter_runtime/assets/runtime.zip'
        verify_runtime(archive)
        pin = json.loads((markdown / 'upstream.json').read_text())
        with zipfile.ZipFile(archive) as value:
            if json.loads(value.read('upstream.json')) != pin:
                raise ValueError('Runtime upstream identity mismatch')
        files.append(archive)
    return [{'file': p.relative_to(client.parent).as_posix(),
             'bytes': p.stat().st_size, 'sha256': digest(p.read_bytes())} for p in files]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--runtime', action='store_true')
    parser.add_argument('--verify-only', action='store_true')
    args = parser.parse_args()
    ecosystem = ROOT.parent / 'Muse-Plugin-Ecosystem'
    # The upstream realtime builder deliberately targets this canonical sibling.
    if ROOT.name != 'Muse-Client':
        raise ValueError('Build in the canonical Muse-Client sibling layout')
    composition = ROOT / 'config/release/mobile-1.0.3-sources.json'
    verify(composition, ecosystem)
    markdown = ecosystem / 'plugins/markdown'
    if not args.verify_only:
        for script in ('build_realtime_web_core.py', 'build_web_core.py', 'build_dataset_web_core.py'):
            subprocess.run([sys.executable, str(markdown / 'smart_document/scripts' / script)], check=True)
        if args.runtime:
            # This builder verifies the independently pinned, clean SiYuan Git checkout.
            subprocess.run([sys.executable, str(markdown / 'scripts/build.py'), 'web'], check=True)
    artifacts = collect_assets(ROOT, ecosystem, args.runtime)
    output = ROOT / 'dist/plugin-assets.json'
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps({'schema': 'openmuse.plugin-assets/1',
                                 'compositionSha256': digest(composition.read_bytes()),
                                 'runtime': args.runtime, 'artifacts': artifacts}, indent=2) + '\n')
    print('Verified generated assets:', len(artifacts))


if __name__ == '__main__':
    main()
