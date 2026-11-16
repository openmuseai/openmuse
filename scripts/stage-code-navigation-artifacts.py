#!/usr/bin/env python3
"""Build the curated, digest-pinned offline plugin distribution from locked inputs.

No network, package hooks or private SDK paths. Output ships under the application
signature; index.json is the trust root, not a remote/unverified install request.
"""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import tarfile
import io
import gzip

ROOT = Path(__file__).resolve().parents[1]
parser = argparse.ArgumentParser()
parser.add_argument('--out', required=True)
parser.add_argument('--from-archives', action='store_true')
parser.add_argument('--write-archives', action='store_true')
parser.add_argument('--archive-root', default=str(ROOT / 'third_party/dsh/navigation-plugin-artifacts'))
args = parser.parse_args()
output = Path(args.out).resolve()
output.mkdir(parents=True, exist_ok=True)
archives = Path(args.archive_root)
if args.from_archives:
    lock = json.loads((archives / 'catalog.lock.json').read_text())
    for identity, entry in lock.items():
        assert identity.startswith('com.openmuse.code-navigation') and '/' not in identity
        archive = archives / entry['archive']
        assert hashlib.sha256(archive.read_bytes()).hexdigest() == entry['archiveSha256']
        target = output / identity
        if target.exists():
            shutil.rmtree(target)
        target.mkdir()
        with tarfile.open(archive) as bundle:
            members = bundle.getmembers()
            assert len(members) == len(entry['files'])
            for member in members:
                assert member.isfile() and member.name in entry['files']
                assert not Path(member.name).is_absolute() and '..' not in Path(member.name).parts
                data = bundle.extractfile(member).read()
                assert hashlib.sha256(data).hexdigest() == entry['files'][member.name]
                destination = target / member.name
                destination.parent.mkdir(parents=True, exist_ok=True)
                destination.write_bytes(data)
    (output / 'index.json').write_text(json.dumps({key: {'version': value['version'], 'files': value['files']} for key, value in lock.items()}, indent=2) + '\n')
    print(f'Restored {len(lock)} verified offline plugin artifacts')
    raise SystemExit(0)
service = ROOT / 'plugins/code-navigation-service'
ids = ['com.openmuse.code-navigation', 'com.openmuse.code-navigation.dart', 'com.openmuse.code-navigation.typescript']
index = {}
for identity in ids:
    target = output / identity
    if target.exists():
        shutil.rmtree(target)
    target.mkdir()
    if identity == ids[0]:
        shutil.copytree(service / 'lib', target / 'lib')
        for filename in ['package.json', 'README.zh-CN.md', 'THIRD_PARTY_NOTICES.txt']:
            shutil.copyfile(service / filename, target / filename)
        shutil.copytree(service / 'node_modules/vscode-jsonrpc', target / 'node_modules/vscode-jsonrpc')
        (target / 'plugin.json').write_text(json.dumps({'id': identity, 'version': json.loads((service / 'package.json').read_text())['version'], 'license': 'AGPL-3.0-only', 'kind': 'navigation-service'}))
    else:
        language = 'dart' if identity == ids[1] else 'typescript'
        shutil.copyfile(ROOT / f'plugins/code-navigation-{language}/plugin.json', target / 'plugin.json')
        if language == 'typescript':
            for package, version in [('typescript', '5.9.3'), ('typescript-language-server', '6.1.0')]:
                source = service / 'node_modules' / package
                assert json.loads((source / 'package.json').read_text())['version'] == version
                shutil.copytree(source, target / 'node_modules' / package)
    files = {}
    for file in sorted(target.rglob('*')):
        assert not file.is_symlink(), file
        if file.is_file():
            files[file.relative_to(target).as_posix()] = hashlib.sha256(file.read_bytes()).hexdigest()
    index[identity] = {'version': json.loads((target / 'plugin.json').read_text())['version'], 'files': files}
(output / 'index.json').write_text(json.dumps(index, indent=2) + '\n')
if args.write_archives:
    archives.mkdir(parents=True, exist_ok=True)
    lock = {}
    for identity, entry in index.items():
        name = f'{identity}-{entry["version"]}.tgz'
        with (archives / name).open('wb') as file:
            with gzip.GzipFile(fileobj=file, mode='wb', mtime=0, filename='') as compressed:
                with tarfile.open(fileobj=compressed, mode='w') as bundle:
                    for path in entry['files']:
                        data = (output / identity / path).read_bytes()
                        member = tarfile.TarInfo(path)
                        member.size, member.mode, member.mtime = len(data), 0o644, 0
                        bundle.addfile(member, io.BytesIO(data))
        lock[identity] = {**entry, 'archive': name, 'archiveSha256': hashlib.sha256((archives / name).read_bytes()).hexdigest()}
    (archives / 'catalog.lock.json').write_text(json.dumps(lock, indent=2) + '\n')
print(f'Staged {len(index)} plugins, {sum(len(x["files"]) for x in index.values())} verified files')
