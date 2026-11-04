#!/usr/bin/env python3
"""Prepare a bounded source archive for review before AGPL distribution.

An archive alone is not a claim of complete Corresponding Source: reviewers
must check native origins, build instructions, notices and public availability.
No credentials, Git history, user documents or signing material are included.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import subprocess
import zipfile

GENERATED = {'.git', '.dart_tool', 'node_modules', 'target', 'out', 'dist',
             '.build', '.cache', '__pycache__', 'Pods', '.symlinks', '.gradle',
             'ephemeral', 'tmp', '~', 'depot_tools', 'emsdk'}
GENERATED |= {'evidence', 'rewrite-review'}
BINARY = {'.dylib', '.so', '.a', '.o', '.dll', '.exe', '.apk', '.ipa', '.pkg',
          '.aar', '.xcframework'}
SECRETS = re.compile(rb'-----BEGIN (?:RSA |EC |OPENSSH )?PRIVATE KEY-----|'
                     rb'gh[pousr]_[A-Za-z0-9]{30,}|github_pat_[A-Za-z0-9_]{30,}|'
                     rb'AKIA[0-9A-Z]{16}')


def git_files(root):
    result = subprocess.run(['git', '-C', str(root), 'rev-parse', '--show-toplevel'],
                            text=True, capture_output=True)
    if result.returncode or Path(result.stdout.strip()).resolve() != root.resolve():
        return None
    result = subprocess.check_output(['git', '-C', str(root), 'ls-files', '-z',
                                      '--cached', '--others', '--exclude-standard'])
    return [root / os.fsdecode(name) for name in result.split(b'\0') if name]


def sources(root):
    tracked = git_files(root)
    if tracked is not None:
        yield from tracked
        return
    for directory, dirs, names in os.walk(root):
        # PDFium/SiYuan keep source build rules in tracked build/ directories.
        # A nested Git inventory preserves those rather than pruning by name.
        current = Path(directory)
        if current != root and (current / '.git').exists():
            yield from sources(current)
            dirs[:] = []
            continue
        dirs[:] = sorted(d for d in dirs if d not in GENERATED and d != 'build'
                         and not (current / d).is_symlink())
        yield from (current / name for name in sorted(names))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--client', type=Path, required=True)
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--archive', action='store_true')
    parser.add_argument('--reviewed-fixtures', type=Path)
    parser.add_argument('--build-instructions', type=Path)
    args = parser.parse_args()
    reviewed = json.loads(args.reviewed_fixtures.read_text()) if args.reviewed_fixtures else {}
    client = args.client.resolve()
    parent = client.parent
    ecosystem = parent / 'Muse-Plugin-Ecosystem'
    office = parent / 'ioffice'
    roots = [(client, 'Muse-Client')]
    for name in ['plugins/markdown', 'plugins/genoffice-pptx',
                 'plugins/appflowy-editor/flutter_editor',
                 'plugins/appflowy-editor/vendor/appflowy_editor', 'packages/plugin-sdk']:
        roots.append((ecosystem / name, 'Muse-Plugin-Ecosystem/' + name))
    for name in ['libs', 'samples/libs', 'samples/docx-mvp-render/native',
                 'vendors/gridcraft', 'koffice/pdf/rebuild/pdf-flutter-app',
                 'koffice/pdf/rebuild/pdf-rebuild']:
        roots.append((office / name, 'ioffice/' + name))
    inventory, rejected, omitted = {}, [], []
    for root, prefix in roots:
        if not root.is_dir():
            raise SystemExit(f'Missing source root: {root}')
        for path in sources(root):
            if not path.is_file() or path.is_symlink():
                continue
            relative = path.relative_to(root)
            name = prefix + '/' + relative.as_posix()
            pinned_vendor = relative.parts[:2] == ('vendors', 'siyuan')
            if not pinned_vendor and (any(p in GENERATED for p in relative.parts) or path.suffix in BINARY):
                continue
            if relative.parts[0] in {'docs', 'evidence'} or path.name == '.DS_Store':
                continue
            if path.stat().st_size > 100 * 1024**2:
                omitted.append(name)
                continue
            data = path.read_bytes()
            digest = hashlib.sha256(data).hexdigest()
            approved_fixture = reviewed.get(name, {}).get('sha256') == digest
            env_file = path.name.startswith('.env') and path.name not in {'.env.example', '.env.sample'}
            if (SECRETS.search(data) or env_file) and not approved_fixture:
                rejected.append(name)
                continue
            inventory[name] = {'path': str(path), 'sha256': digest,
                               'bytes': len(data)}
    report = {'status': 'prepared-for-review', 'clientCommit': subprocess.check_output(
        ['git', 'rev-parse', 'HEAD'], cwd=client, text=True).strip(),
        'fileCount': len(inventory), 'uncompressedBytes': sum(v['bytes'] for v in inventory.values()),
        'rejectedPotentialSecrets': sorted(rejected), 'oversizedFilesRequireReview': sorted(omitted),
        'files': inventory,
        'openChecks': ['Review native-library source and license coverage.',
                       'Validate build instructions against included tree.',
                       'Publish at no charge and verify an unauthenticated download.',
                       'Confirm App Store terms do not add restrictions to AGPL rights.']}
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.with_suffix('.inventory.json').write_text(json.dumps(report, indent=2) + '\n')
    print(f"Source inventory: {len(inventory)} files, {report['uncompressedBytes'] / 1024**2:.1f} MiB; "
          f"{len(rejected)} potential secrets, {len(omitted)} oversized files", flush=True)
    if not args.archive:
        return
    if not args.build_instructions or not args.build_instructions.is_file():
        raise SystemExit('--build-instructions must name the reviewed rebuild guide.')
    if rejected or omitted:
        raise SystemExit('Review rejected/oversized files before archiving; values were not printed.')
    if shutil.disk_usage(args.output.parent).free < 10 * 1024**3:
        raise SystemExit('Source archive needs at least 10 GiB free before writing.')
    partial = args.output.with_suffix('.partial.zip')
    try:
        with zipfile.ZipFile(partial, 'w', zipfile.ZIP_DEFLATED, compresslevel=4) as archive:
            for name, entry in sorted(inventory.items()):
                path = Path(entry['path'])
                if hashlib.sha256(path.read_bytes()).hexdigest() != entry['sha256']:
                    raise ValueError(f'Source changed after inventory: {name}')
                archive.write(path, name)
                if partial.stat().st_size > 1536 * 1024**2 or shutil.disk_usage(partial.parent).free < 8 * 1024**3:
                    raise ValueError('Source archive stopped at size/free-space safety limit.')
            portable = {k: {x: y for x, y in v.items() if x != 'path'} for k, v in inventory.items()}
            archive.writestr('SOURCE-SHA256.json', json.dumps(portable, indent=2))
            archive.write(args.build_instructions, 'BUILD-MACOS.md')
        with zipfile.ZipFile(partial) as archive:
            if archive.testzip():
                raise ValueError('Source ZIP integrity verification failed.')
        partial.replace(args.output)
        print(f'Source archive verified: {args.output}', flush=True)
    finally:
        partial.unlink(missing_ok=True)


if __name__ == '__main__':
    main()
