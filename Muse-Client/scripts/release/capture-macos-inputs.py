#!/usr/bin/env python3
"""Fingerprint actual external source/assets before a build and verify afterward.

Uses a small manifest instead of duplicating dependency trees. The Client itself
must be a clean isolated source checkout. Dirty sibling repositories remain
explicitly identified and their consumed package contents are pinned by hashes.
"""
import argparse
from datetime import datetime, timezone
import hashlib
import json
import os
from pathlib import Path
import subprocess
from urllib.parse import unquote, urlparse

EXCLUDED = {'.git', '.dart_tool', 'build', 'target', 'node_modules', '.idea', '.vscode', 'tmp', '.DS_Store'}


def digest(path):
    value = hashlib.sha256()
    with path.open('rb') as stream:
        for chunk in iter(lambda: stream.read(1024 * 1024), b''):
            value.update(chunk)
    return value.hexdigest()


def inventory(roots, extras):
    files = set(map(Path, extras))
    for root in map(Path, roots):
        for directory, subdirs, names in os.walk(root):
            subdirs[:] = sorted(d for d in subdirs if d not in EXCLUDED)
            files.update(Path(directory) / name for name in names if name not in EXCLUDED)
    return {str(path): digest(path) for path in sorted(files) if path.is_file()}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--client', type=Path)
    parser.add_argument('--report', type=Path)
    parser.add_argument('--verify', type=Path)
    args = parser.parse_args()
    if args.verify:
        record = json.loads(args.verify.read_text())
        actual = inventory(record['roots'], record['extras'])
        if actual != record['files']:
            raise SystemExit('External build inputs changed; invalidate and rebuild the candidate.')
        client = Path(record['client'])
        revision = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=client, text=True).strip()
        dirty = subprocess.check_output(['git', 'status', '--porcelain'], cwd=client, text=True).strip()
        if revision != record['clientCommit'] or dirty:
            raise SystemExit('Client source changed; invalidate and rebuild the candidate.')
        print(f'Build input freeze verified: {len(actual)} external files; clean Client {revision}')
        return
    if not args.client or not args.report:
        parser.error('--client and --report are required to capture a freeze')
    client = args.client.resolve()
    if subprocess.check_output(['git', 'status', '--porcelain'], cwd=client, text=True).strip():
        raise SystemExit('Client source must be clean before capture.')
    config = client / 'app/openmuse_host/.dart_tool/package_config.json'
    roots = []
    for package in json.loads(config.read_text())['packages']:
        uri = package['rootUri']
        path = Path(unquote(urlparse(uri).path)) if uri.startswith('file:') else (config.parent / uri).resolve()
        if path.is_relative_to(client.parent / 'ioffice') or path.is_relative_to(client.parent / 'Muse-Plugin-Ecosystem'):
            roots.append(path)
    minimal = []
    for path in sorted(set(roots), key=lambda p: len(p.parts)):
        if not any(path.is_relative_to(parent) for parent in minimal):
            minimal.append(path)
    ppt = client.parent / 'Muse-Plugin-Ecosystem/plugins/genoffice-pptx'
    version = json.loads((ppt / 'package.json').read_text())['version']
    extras = [ppt / 'package.json', ppt / 'dist' / f'genoffice-pptx-{version}.mcpb']
    record = {'capturedAt': datetime.now(timezone.utc).isoformat(), 'client': str(client),
              'clientCommit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=client, text=True).strip(),
              'roots': list(map(str, minimal)), 'extras': list(map(str, extras)),
              'excludedGeneratedDirectories': sorted(EXCLUDED),
              'files': inventory(minimal, extras),
              'retentionNote': 'Client Git snapshot retained locally; external content hash freeze is verified before and after the build, not a claim that sibling Git worktrees are clean.'}
    args.report.parent.mkdir(parents=True, exist_ok=True)
    args.report.write_text(json.dumps(record, indent=2) + '\n')
    print(f'Captured {len(record["files"])} external inputs before build: {args.report}')


if __name__ == '__main__':
    main()
