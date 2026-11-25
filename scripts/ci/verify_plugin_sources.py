#!/usr/bin/env python3
"""Verify the exact sibling plugin composition before any build executes it."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess


ROOT = Path(__file__).resolve().parents[2]


def git(root, *args):
    return subprocess.check_output(
        ['git', '-C', str(root), *args], text=True, stderr=subprocess.PIPE
    ).strip()


def verify(composition, ecosystem):
    expected = json.loads(Path(composition).read_text())['ecosystem']
    ecosystem = Path(ecosystem)
    if ecosystem.is_symlink():
        raise ValueError('Ecosystem source must not be a symlink')
    ecosystem = ecosystem.resolve(strict=True)
    if Path(git(ecosystem, 'rev-parse', '--show-toplevel')).resolve() != ecosystem:
        raise ValueError('Ecosystem must be its own Git checkout')
    if git(ecosystem, 'rev-parse', 'HEAD') != expected['commit']:
        raise ValueError('Ecosystem commit does not match the Client composition')
    lock_path = Path(expected['lock'])
    if lock_path.is_absolute() or '..' in lock_path.parts:
        raise ValueError('Invalid plugin lock path')
    lock_file = ecosystem / lock_path
    if any(p.is_symlink() for p in [lock_file, *lock_file.parents] if p != ecosystem):
        raise ValueError('Plugin lock must not resolve through a symlink')
    data = lock_file.read_bytes()
    if hashlib.sha256(data).hexdigest() != expected['lockSha256']:
        raise ValueError('Plugin source lock digest mismatch')
    plugins = json.loads(data)['plugins']
    if len(plugins) != 15 or len({p['path'] for p in plugins}) != 15:
        raise ValueError('The composition must contain the 15 distinct Ecosystem plugins')
    for plugin in plugins:
        name, commit = plugin['path'], plugin['commit']
        if not re.fullmatch(r'plugins/[A-Za-z0-9_-]+', name) or not re.fullmatch(r'[a-f0-9]{40}', commit):
            raise ValueError('Invalid plugin source identity')
        source = ecosystem / name
        if source.is_symlink() or not source.is_dir():
            raise ValueError('Missing or symlinked plugin source: ' + name)
        entry = git(ecosystem, 'ls-tree', 'HEAD', '--', name)
        if not entry.startswith('160000 commit ' + commit + '\t'):
            raise ValueError('Plugin gitlink does not match the source lock: ' + name)
        if Path(git(source, 'rev-parse', '--show-toplevel')).resolve() != source.resolve():
            raise ValueError('Plugin is not an initialized Git checkout: ' + name)
        if git(source, 'rev-parse', 'HEAD') != commit:
            raise ValueError('Plugin checkout commit mismatch: ' + name)
        if git(source, 'status', '--porcelain', '--untracked-files=no'):
            raise ValueError('Plugin tracked source is dirty: ' + name)
    if git(ecosystem, 'status', '--porcelain', '--untracked-files=no'):
        raise ValueError('Ecosystem tracked source is dirty')
    return {'ecosystemCommit': expected['commit'], 'verifiedPlugins': len(plugins)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--composition', type=Path, default=ROOT / 'config/release/mobile-1.0.3-sources.json')
    parser.add_argument('--ecosystem', type=Path, default=ROOT.parent / 'Muse-Plugin-Ecosystem')
    args = parser.parse_args()
    try:
        print(json.dumps(verify(args.composition, args.ecosystem)))
    except (ValueError, OSError, KeyError, subprocess.CalledProcessError) as error:
        parser.exit(1, 'Plugin source verification failed: ' + str(error) + '\n')


if __name__ == '__main__':
    main()
