#!/usr/bin/env python3
"""Package only a real Linux bundle, derive system dependencies, and record provenance."""
import argparse
import hashlib
import json
import os
from pathlib import Path
import shutil
import subprocess
import tarfile
import tempfile
from datetime import datetime, timezone


def run(*args, **kwargs):
    return subprocess.check_output(args, text=True, **kwargs).strip()


def digest(path):
    with path.open('rb') as source:
        return hashlib.file_digest(source, 'sha256').hexdigest()


def is_elf(path):
    if not path.is_file() or path.is_symlink():
        return False
    with path.open('rb') as source:
        return source.read(4) == b'\x7fELF'


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--bundle', required=True, type=Path)
    parser.add_argument('--dist', required=True, type=Path)
    parser.add_argument('--arch', required=True, choices=['amd64', 'arm64'])
    parser.add_argument('--profile', required=True, choices=['debug', 'release'])
    parser.add_argument('--grammars', required=True, choices=['0', '1'])
    parser.add_argument('--region', choices=['cn', 'us'], default='cn')
    args = parser.parse_args()
    app = Path(__file__).resolve().parent.parent
    root = app.parent.parent
    bundle, dist = args.bundle.resolve(), args.dist.resolve()
    required = ['openmuse_linux', 'lib/libflutter_linux_gtk.so', 'data/icudtl.dat',
                'openmuse/dsh/node/bin/node',
                'openmuse/dsh/node_modules/@deepseek-ai/dsh/lib/bin.js',
                'data/flutter_assets/packages/openmuse_helix_plugin/assets/engines/helix/hx']
    for name in required:
        if not (bundle / name).is_file():
            raise RuntimeError(f'Bundle is incomplete: {name}')
    expected = 'aarch64' if args.arch == 'arm64' else 'x86-64'
    for name in [required[0], required[3], required[5]]:
        kind = run('file', str(bundle / name))
        if 'ELF 64-bit' not in kind or expected not in kind:
            raise RuntimeError(f'Wrong binary architecture: {kind}')
    for path in bundle.rglob('*'):
        if path.is_symlink() and not path.resolve().is_relative_to(bundle):
            raise RuntimeError(f'Bundle contains an escaping symlink: {path}')
        if is_elf(path):
            linked = subprocess.run(['ldd', str(path)], capture_output=True, text=True,
                                    env={**os.environ, 'LD_LIBRARY_PATH': str(bundle / 'lib')})
            if 'not found' in linked.stdout + linked.stderr:
                raise RuntimeError(f'Missing system dependency: {path}\n{linked.stdout}')
    version_line = next(line for line in (app / 'pubspec.yaml').read_text().splitlines() if line.startswith('version:'))
    version = version_line.split()[1].split('+')[0]
    name = f'OpenMuse-linux-{args.arch}-{args.region}-{version}'
    dist.mkdir(parents=True, exist_ok=True)
    info = {
        'product': 'OpenMuse Linux', 'version': version, 'architecture': args.arch,
        'region': args.region,
        'profile': args.profile, 'grammars': args.grammars == '1',
        'builtAt': datetime.now(timezone.utc).isoformat(),
        'commit': run('git', '-C', str(root), 'rev-parse', 'HEAD'),
        'workingTreeDirty': bool(run('git', '-C', str(root), 'status', '--porcelain')),
        'flutter': json.loads(run('flutter', '--version', '--machine')),
        'rust': run('rustc', '+1.90.0', '--version'),
        'glibc': run('getconf', 'GNU_LIBC_VERSION'),
        'webkit': run('pkg-config', '--modversion', 'webkit2gtk-4.1'),
        'pubLockSha256': digest(app / 'pubspec.lock'),
        'dshLockSha256': digest(root / 'third_party/dsh/package-lock.json'),
        'sourceFiles': {str(p.relative_to(root)): digest(p) for folder in
                        [app / 'lib', app / 'linux/runner', app / 'tool', app / 'packaging']
                        for p in sorted(folder.rglob('*')) if p.is_file() and '__pycache__' not in p.parts},
    }
    (bundle / 'build-info.json').write_text(json.dumps(info, ensure_ascii=False, indent=2) + '\n')
    with tempfile.TemporaryDirectory(prefix='openmuse-deb-') as temp:
        staging = Path(temp)
        target = staging / 'opt/openmuse'
        shutil.copytree(bundle, target, symlinks=True)
        applications = staging / 'usr/share/applications'
        applications.mkdir(parents=True)
        shutil.copy2(app / 'packaging/io.openmuse.linux.desktop', applications)
        icons = staging / 'usr/share/icons/hicolor/256x256/apps'
        icons.mkdir(parents=True)
        shutil.copy2(app / 'packaging/openmuse.png', icons / 'openmuse.png')
        binaries = staging / 'usr/bin'
        binaries.mkdir(parents=True)
        (binaries / 'openmuse').symlink_to('/opt/openmuse/openmuse_linux')
        control = staging / 'DEBIAN'
        control.mkdir()
        # dpkg-shlibdeps needs a source package identity even for a binary-only package.
        debian = staging / 'debian'
        debian.mkdir()
        (debian / 'control').write_text('Source: openmuse\nSection: devel\nPriority: optional\nMaintainer: OpenMuse <opensource@openmuseai.com>\n\nPackage: openmuse\nArchitecture: any\nDescription: OpenMuse desktop workbench\n')
        elf = [p for p in target.rglob('*') if is_elf(p)]
        library_paths = sorted({str(p.parent) for p in elf})
        depends = run('dpkg-shlibdeps', '--ignore-missing-info', '-O',
                      *(f'-l{directory}' for directory in library_paths),
                      *(f'-e{p}' for p in elf), cwd=staging)
        depends = next(line.removeprefix('shlibs:Depends=') for line in depends.splitlines() if line.startswith('shlibs:Depends='))
        size = sum(p.stat().st_size for p in target.rglob('*') if p.is_file()) // 1024
        (control / 'control').write_text(f'Package: openmuse\nVersion: {version}\nArchitecture: {args.arch}\nMaintainer: OpenMuse <opensource@openmuseai.com>\nSection: devel\nPriority: optional\nInstalled-Size: {size}\nDepends: {depends}, fonts-noto-cjk\nDescription: OpenMuse Linux AI workbench\n Local projects, Helix editor, terminal and an authenticated DSH sidecar.\n')
        shutil.rmtree(debian)
        subprocess.run(['dpkg-deb', '-Zzstd', '-z6', '--build', '--root-owner-group', str(staging), str(dist / f'{name}.deb')], check=True)
    with tarfile.open(dist / f'{name}.tar.gz', 'w:gz') as archive:
        archive.add(bundle, arcname='OpenMuse')
    (dist / 'build-info.json').write_text(json.dumps(info, ensure_ascii=False, indent=2) + '\n')
    artifacts = [dist / f'{name}.deb', dist / f'{name}.tar.gz', dist / 'build-info.json']
    (dist / 'SHA256SUMS').write_text(''.join(f'{digest(path)}  {path.name}\n' for path in artifacts))
    print('\n'.join(str(path) for path in artifacts))


if __name__ == '__main__':
    main()
