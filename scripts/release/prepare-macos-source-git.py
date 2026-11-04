#!/usr/bin/env python3
"""Build a local, unpublished Git source snapshot without copying a worktree."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--inventory', type=Path, required=True)
    parser.add_argument('--build-instructions', type=Path, required=True)
    parser.add_argument('--repository', type=Path, required=True)
    args = parser.parse_args()
    record = json.loads(args.inventory.read_text())
    if record['rejectedPotentialSecrets'] or record['oversizedFilesRequireReview']:
        raise SystemExit('Source inventory has unresolved exclusions.')
    repo = args.repository.resolve()
    repo.mkdir(parents=True, exist_ok=True, mode=0o700)
    repo.chmod(0o700)
    if not (repo / 'HEAD').exists():
        subprocess.run(['git', 'init', '--bare', str(repo)], check=True, stdout=subprocess.DEVNULL)
    env = {**os.environ, 'GIT_DIR': str(repo), 'GIT_INDEX_FILE': str(repo / 'release.index'),
           'GIT_AUTHOR_NAME': 'OpenMuse Release', 'GIT_AUTHOR_EMAIL': 'developer@openmuseai.com',
           'GIT_COMMITTER_NAME': 'OpenMuse Release', 'GIT_COMMITTER_EMAIL': 'developer@openmuseai.com'}

    def git(*args, data=None):
        return subprocess.check_output(['git', *args], input=data, env=env)

    ref = 'refs/heads/release/macos-1.0.0-source'
    if subprocess.run(['git', 'show-ref', '--verify', '--quiet', ref], env=env).returncode == 0:
        raise SystemExit('Snapshot ref already exists; inspect before replacing it.')
    git('read-tree', '--empty')
    entries = sorted(record['files'].items())
    for offset in range(0, len(entries), 400):
        if shutil.disk_usage(repo).free < 8 * 1024**3:
            raise SystemExit('Stopped source Git object generation below 8 GiB free.')
        batch = entries[offset:offset + 400]
        for name, entry in batch:
            if hashlib.sha256(Path(entry['path']).read_bytes()).hexdigest() != entry['sha256']:
                raise SystemExit(f'Source changed: {name}')
        paths = ''.join(json.dumps(entry['path'], ensure_ascii=False) + '\n' for _, entry in batch)
        objects = git('hash-object', '-w', '--stdin-paths', data=paths.encode()).decode().splitlines()
        if len(objects) != len(batch):
            raise SystemExit('Unexpected Git object count.')
        index = bytearray()
        for (name, entry), oid in zip(batch, objects):
            mode = '100755' if Path(entry['path']).stat().st_mode & 0o111 else '100644'
            index.extend(f'{mode} {oid}\t{name}'.encode() + b'\0')
        git('update-index', '-z', '--index-info', data=bytes(index))
        if offset % 4000 == 0:
            print(f'Source Git inputs stored: {min(offset + len(batch), len(entries))}/{len(entries)}', flush=True)
    portable = {k: {x: y for x, y in v.items() if x != 'path'} for k, v in record['files'].items()}
    instructions = args.build_instructions.read_text().replace('source archive', 'source version').replace('source ZIP', 'source tree')
    instructions = re.sub(r'--build-number \d+', '--build-number 11', instructions)
    extra = {
        'BUILD-MACOS.md': instructions.encode(),
        'README.md': ('# OpenMuse Desktop 1.0.0 macOS source\n\n'
                      'This version provides the source used by the macOS release, '
                      'including local dependencies and build adaptations. '
                      'See [BUILD-MACOS.md](BUILD-MACOS.md) for layout, tools and rebuilding. '
                      'OpenMuse is AGPL-3.0-or-later; third-party licenses remain applicable.\n').encode(),
        'SOURCE-SHA256.json': (json.dumps(portable, indent=2) + '\n').encode(),
        'SOURCE-REVISION.json': (json.dumps({'clientBuildCommit': record['clientCommit'],
            'clientDirectory': 'Muse-Client', 'fileCount': len(entries),
            'sourcePublication': 'Git source branch; no separately uploaded source ZIP',
            'provenance': 'Client source files map to the clean isolated build commit. External source file hashes are pinned by SOURCE-SHA256.json. Signing credentials and generated build caches are excluded.'}, indent=2) + '\n').encode(),
    }
    for name, content in extra.items():
        oid = git('hash-object', '-w', '--stdin', data=content).decode().strip()
        git('update-index', '--add', '--cacheinfo', '100644', oid, name)
    tree = git('write-tree').decode().strip()
    commit = git('commit-tree', tree, data=b'Provide macOS 1.0.0 corresponding source and build instructions\n').decode().strip()
    git('update-ref', ref, commit)
    print(f'Prepared local source snapshot {commit}; not pushed.', flush=True)


if __name__ == '__main__':
    main()
