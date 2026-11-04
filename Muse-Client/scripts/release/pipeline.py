#!/usr/bin/env python3
"""Plan release stages, check evidence, and create verified local archives.

This tool never uploads, submits, accepts agreements, or publishes an app.
Stage receipts are human-reviewed records; a green file check is not legal review.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path, PurePosixPath
import re
import sys
import tempfile
import zipfile
from datetime import datetime, timezone

ROOT = Path(__file__).resolve().parents[2]
DEFINITION = ROOT / 'config/release/pipeline.json'
STATES = {'pending', 'in_progress', 'partial', 'passed', 'failed', 'not_applicable'}
LANES = {'play', 'ios-us', 'android-cn', 'ios-cn', 'harmony-cn',
         'macos-store-us', 'macos-direct-us'}
SECRET_NAME = re.compile(r'(^\.env(?:\.|$)|(?:password|credential|secret|token|id_rsa|id_ed25519)|\.(?:p12|pfx|jks|keystore|key)$)', re.I)
SECRET_BYTES = re.compile(rb'-----BEGIN (?:[A-Z0-9 ]+ )?PRIVATE KEY-----\r?\n[A-Za-z0-9+/]{20,}|(?:AKIA|ASIA)[A-Z0-9]{16}|\bxkeysib-[A-Za-z0-9_-]{20,}')


def read_json(path):
    return json.loads(Path(path).read_text())


def write_json(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n')


def sha256(path):
    h = hashlib.sha256()
    with Path(path).open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''):
            h.update(chunk)
    return h.hexdigest()


def local_path(value):
    """Only repository-relative regular files; no external paths or symlinks."""
    p = PurePosixPath(value)
    if p.is_absolute() or '..' in p.parts or '\\' in value or not p.parts:
        raise ValueError(f'Unsafe repository path: {value}')
    path = ROOT.joinpath(*p.parts)
    if any(parent.is_symlink() for parent in [path, *path.parents] if parent != ROOT):
        raise ValueError(f'Symlink not allowed: {value}')
    path.resolve().relative_to(ROOT.resolve())
    return path


def archive_name(value):
    p = PurePosixPath(value)
    if not value or p.is_absolute() or '..' in p.parts or '\\' in value or str(p) != value:
        raise ValueError(f'Unsafe archive name: {value}')
    if any(SECRET_NAME.search(part) for part in p.parts):
        raise ValueError(f'Private credential path not allowed: {value}')
    return value


def lint_definition():
    definition = read_json(DEFINITION)
    seen = set()
    for stage in definition['stages']:
        if stage['id'] in seen or not set(stage['needs']).issubset(seen):
            raise ValueError('Stage IDs must be unique and dependencies ordered')
        seen.add(stage['id'])
        if not local_path(stage['document']).is_file():
            raise ValueError(f'Missing stage document: {stage["document"]}')
    if len(seen) < 8:
        raise ValueError('The release lifecycle must use separate stages')
    return definition


def initialize(run, run_id, version, lanes):
    if run.exists():
        raise ValueError('Run directory already exists; refusing to overwrite history')
    if not re.fullmatch(r'[a-zA-Z0-9][a-zA-Z0-9._-]{0,95}', run_id):
        raise ValueError('Invalid release ID')
    if not re.fullmatch(r'\d+\.\d+\.\d+', version) or not lanes or not set(lanes) <= LANES:
        raise ValueError('Invalid version or release lanes')
    definition = lint_definition()
    run.mkdir(parents=True)
    write_json(run / 'release.json', {
        'schemaVersion': 1, 'id': run_id, 'version': version,
        'mode': 'planned', 'lanes': lanes, 'createdAt': datetime.now(timezone.utc).isoformat(),
        'sourceFrozen': False, 'sourceRevisions': {}, 'artifacts': [],
        'stores': {lane: {'status': 'not_submitted', 'build': None,
                          'releaseMode': 'undecided'} for lane in lanes},
    })
    for stage in definition['stages']:
        write_json(run / 'stages' / (stage['id'] + '.json'), {
            'id': stage['id'], 'status': 'pending', 'owner': '',
            'summary': '', 'evidence': [], 'exceptions': [],
        })
    write_json(run / 'archive-inputs.json', {'files': []})


def load_run(run):
    definition = lint_definition()
    release = read_json(run / 'release.json')
    if release.get('schemaVersion') != 1 or release.get('mode') not in {'planned', 'historical'}:
        raise ValueError('Unknown release schema or mode')
    if not release.get('lanes') or not set(release['lanes']) <= LANES:
        raise ValueError('Unknown release lanes')
    receipts = {}
    for stage in definition['stages']:
        receipt = read_json(run / 'stages' / (stage['id'] + '.json'))
        if receipt.get('id') != stage['id'] or receipt.get('status') not in STATES:
            raise ValueError(f'Invalid stage receipt: {stage["id"]}')
        if receipt['status'] == 'passed' and (not receipt.get('owner') or not receipt.get('evidence')):
            raise ValueError(f'Passed stage needs an owner and evidence: {stage["id"]}')
        receipts[stage['id']] = receipt
    return definition, release, receipts


def check_run(run, gate=None, metadata_only=False):
    definition, release, receipts = load_run(run)
    for receipt in receipts.values():
        for value in receipt.get('evidence', []):
            if not local_path(value).is_file():
                raise ValueError(f'Missing evidence: {value}')
    for item in release.get('artifacts', []):
        if not re.fullmatch(r'[a-f0-9]{64}', item['sha256']):
            raise ValueError('Artifact needs an exact SHA-256')
        path = local_path(item['path'])
        if not metadata_only and (not path.is_file() or sha256(path) != item['sha256']):
            raise ValueError(f'Missing or changed artifact: {item["path"]}')
    if gate:
        by_id = {s['id']: s for s in definition['stages']}
        if gate not in by_id:
            raise ValueError('Unknown gate')
        required = set()

        def visit(stage_id):
            required.add(stage_id)
            for dep in by_id[stage_id]['needs']:
                if dep not in required:
                    visit(dep)

        visit(gate)
        required.remove(gate)  # Check readiness to start; never pre-approve its result.
        blocked = [s for s in required if receipts[s]['status'] != 'passed']
        if release['mode'] == 'historical' or not release.get('sourceFrozen'):
            blocked.append('frozen source (historical records cannot authorize a new release)')
        if not release.get('sourceRevisions'):
            blocked.append('source revisions')
        if metadata_only:
            blocked.append('final artifacts not checked in metadata-only mode')
        if gate >= '08-upload':
            artifacts = release.get('artifacts', [])
            if not set(release['lanes']) <= {a.get('lane') for a in artifacts}:
                blocked.append('final artifact for each lane')
            for item in artifacts:
                record_path = item.get('buildRecord')
                if not record_path:
                    blocked.append('original build record')
                    continue
                record = read_json(local_path(record_path))
                if (record.get('status') != 'store-candidate' or record.get('dirtySource') is not False
                        or record.get('testsPassed') is not True or record.get('sha256') != item['sha256']):
                    blocked.append('store-candidate provenance: ' + item['path'])
                lane = item.get('lane', '')
                if lane.startswith('macos-'):
                    if record.get('distributionChannel') != lane or not record.get('teamId'):
                        blocked.append('macOS channel/signing provenance: ' + item['path'])
                    if lane == 'macos-store-us' and record.get('sandboxEnabled') is not True:
                        blocked.append('Mac App Store sandbox verification: ' + item['path'])
                    if lane == 'macos-direct-us':
                        notarization = record.get('notarization', {})
                        if (record.get('signingIdentityType') != 'Developer ID Application'
                                or record.get('hardenedRuntime') is not True
                                or notarization.get('status') != 'Accepted'
                                or not notarization.get('submissionId')
                                or notarization.get('stapled') is not True):
                            blocked.append('Developer ID/notarization verification: ' + item['path'])
        if blocked:
            raise ValueError('Gate blocked: ' + ', '.join(sorted(blocked)))
    print(f'Record structure/evidence: PASS ({release["id"]}; {release["mode"]})')
    print('Artifact hashes: ' + ('NOT checked (metadata-only)' if metadata_only else 'PASS'))
    if not gate:
        print('This is a record check; unfinished stages are not approved.')
    else:
        print(f'Prerequisites verified for {gate}; its outcome still needs a receipt.')
    return definition, release, receipts


def archive_run(run, output):
    run = run.resolve()
    run.relative_to(ROOT.resolve())
    _, release, _ = check_run(run)
    if output.exists():
        raise ValueError('Archive already exists; choose a new snapshot name')
    records = read_json(run / 'archive-inputs.json')['files']
    if not records:
        raise ValueError('Archive input allowlist is empty')
    inputs = []
    used = {'MANIFEST.json'}
    for record in records:
        source = local_path(record['source'])
        name = archive_name(record['target'])
        archive_name(record['source'])
        if name in used or not source.is_file() or record.get('reviewedForSecrets') is not True:
            raise ValueError(f'Duplicate, absent or unreviewed archive input: {name}')
        if sha256(source) != record.get('sha256'):
            raise ValueError(f'Archive input changed after review: {name}')
        # Text is checked for common credential formats; allowlist review is
        # still required, especially for screenshots and packaged binaries.
        with source.open('rb') as stream:
            tail = b''
            for chunk in iter(lambda: stream.read(1024 * 1024), b''):
                if SECRET_BYTES.search(tail + chunk):
                    raise ValueError(f'Private key or API credential in archive input: {name}')
                tail = chunk[-256:]
        used.add(name)
        inputs.append((source, name, record))
    # Run definitions/receipts are always self-contained in the snapshot.
    for source in sorted(run.rglob('*.json')):
        if source.is_symlink():
            raise ValueError('Symlink in run directory')
        name = archive_name('run/' + source.relative_to(run).as_posix())
        if name in used:
            raise ValueError(f'Duplicate run entry: {name}')
        if SECRET_BYTES.search(source.read_bytes()):
            raise ValueError('Private credential in run record')
        used.add(name)
        inputs.append((source, name, {'source': source.relative_to(ROOT).as_posix(),
                                     'category': 'run-record', 'sha256': sha256(source)}))
    manifest = {'schemaVersion': 1, 'releaseId': release['id'],
                'snapshotAt': datetime.now(timezone.utc).isoformat(),
                'classification': 'internal-release-archive', 'files': []}
    output.parent.mkdir(parents=True, exist_ok=True)
    fd, temporary = tempfile.mkstemp(dir=output.parent, suffix='.zip')
    os.close(fd)
    try:
        with zipfile.ZipFile(temporary, 'w', zipfile.ZIP_DEFLATED, compresslevel=6) as z:
            for source, name, record in inputs:
                # Hash the bytes that were actually archived; fail on concurrent edits.
                entry = zipfile.ZipInfo(name)
                entry.compress_type = (zipfile.ZIP_STORED if source.suffix.lower() in
                    {'.zip', '.ipa', '.aab', '.apk', '.png', '.jpg', '.sydoc', '.pptx'}
                    else zipfile.ZIP_DEFLATED)
                entry.external_attr = 0o600 << 16
                with source.open('rb') as src, z.open(entry, 'w', force_zip64=True) as dst:
                    h = hashlib.sha256()
                    size = 0
                    for chunk in iter(lambda: src.read(1024 * 1024), b''):
                        h.update(chunk)
                        size += len(chunk)
                        dst.write(chunk)
                if h.hexdigest() != record['sha256']:
                    raise ValueError(f'Input changed while archiving: {name}')
                manifest['files'].append({'path': name, 'source': record['source'],
                    'category': record.get('category', 'material'), 'bytes': size,
                    'sha256': h.hexdigest()})
            z.writestr('MANIFEST.json', json.dumps(manifest, ensure_ascii=False, indent=2) + '\n')
        verify_archive(Path(temporary))
        os.link(temporary, output)  # Atomic, refuses an existing snapshot.
    finally:
        Path(temporary).unlink(missing_ok=True)
    digest = sha256(output)
    sidecar = output.with_suffix(output.suffix + '.sha256')
    sidecar.write_text(digest + '  ' + output.name + '\n')
    sidecar.chmod(0o600)
    print(f'Archived {len(manifest["files"])} files: {output}\nSHA-256: {digest}')


def verify_archive(path):
    with zipfile.ZipFile(path) as z:
        names = z.namelist()
        if len(names) != len(set(names)):
            raise ValueError('Duplicate ZIP entries')
        for name in names:
            archive_name(name)
        manifest = json.loads(z.read('MANIFEST.json'))
        expected = {f['path'] for f in manifest['files']}
        if len(expected) != len(manifest['files']) or set(names) != expected | {'MANIFEST.json'}:
            raise ValueError('Archive inventory mismatch')
        for item in manifest['files']:
            h = hashlib.sha256()
            size = 0
            with z.open(item['path']) as f:
                for chunk in iter(lambda: f.read(1024 * 1024), b''):
                    h.update(chunk)
                    size += len(chunk)
            if h.hexdigest() != item['sha256'] or size != item['bytes']:
                raise ValueError(f'Archive hash mismatch: {item["path"]}')
    print(f'Archive contents verified: {len(expected)} files')


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    sub = parser.add_subparsers(dest='command', required=True)
    sub.add_parser('lint')
    new = sub.add_parser('new')
    new.add_argument('--run', type=Path, required=True)
    new.add_argument('--id', required=True)
    new.add_argument('--version', required=True)
    new.add_argument('--lanes', nargs='+', required=True, choices=sorted(LANES))
    for command in ['plan', 'check', 'archive']:
        p = sub.add_parser(command)
        p.add_argument('--run', type=Path, required=True)
        if command == 'check':
            p.add_argument('--gate')
            p.add_argument('--metadata-only', action='store_true')
        if command == 'archive':
            p.add_argument('--output', type=Path, required=True)
    verify = sub.add_parser('verify-archive')
    verify.add_argument('archive', type=Path)
    args = parser.parse_args()
    try:
        if args.command == 'lint':
            print(f'Pipeline definition: PASS ({len(lint_definition()["stages"])} stages)')
        elif args.command == 'new':
            initialize(args.run, args.id, args.version, args.lanes)
        elif args.command == 'plan':
            definition, release, receipts = load_run(args.run)
            print(f'{release["id"]} / {release["version"]} / {release["mode"]}')
            for stage in definition['stages']:
                print(f'{stage["id"]:24} {receipts[stage["id"]]["status"]:14} {stage["title"]}')
        elif args.command == 'check':
            check_run(args.run, args.gate, args.metadata_only)
        elif args.command == 'archive':
            archive_run(args.run, args.output)
        else:
            verify_archive(args.archive)
    except (ValueError, OSError, KeyError, zipfile.BadZipFile) as error:
        parser.exit(1, f'Release pipeline: {error}\n')


if __name__ == '__main__':
    main()
