"""Assemble the Muse desktop closure: symlink-free production node_modules.

dsh-desktop-style npm-tarball route:
  1. pack.ts --family dsh/vendor -> per-package tarballs under <harness>/dist/npm-*
  2. a closure manifest pins every tarball of both families + @muse (file: dirs)
     + dshmarket (npm tarball) as dependencies
  3. `npm install --omit=dev` resolves the graph (registry covers the rest:
     ajv, @noble/hashes, canonicalize, ...) into <out>/node_modules with no
     junctions, which makes the tree zip-correct by construction.

Usage: python scripts/build-muse-closure.py --out <dir>
"""
from __future__ import annotations

import argparse
import json
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]  # openmuse root (scripts/client/frontend/openmuse)
HARNESS = ROOT / 'vendors' / 'deepseek-harness'
DSHMARKET_TARBALL = ROOT / 'frontend' / 'client' / 'dist' / 'cache' / 'dshmarket-1.31.1.tgz'

# Shared packing library: VENDORED_PLUGIN_DIRS lives there so the closure
# manifest, the packers and the packed-runtime assertions stay in sync.
sys.path.insert(0, str(ROOT / 'frontend' / 'client' / 'scripts' / 'lib'))
import muse_windows as mw  # noqa: E402

MUSE_PACKAGE_DIRS = (
    ('core/protocol/host-bridge', '@muse/host-bridge'),
    ('core/contract-resource', '@muse/contract-resource'),
    ('core/contract-presentation', '@muse/contract-presentation'),
    ('core/contract-engine-session', '@muse/contract-engine-session'),
    ('core/resource-host', '@muse/resource-host'),
    ('core/dsh-resource-presentation', '@muse/dsh-resource-presentation'),
    ('core/plugin-facets', '@muse/plugin-facets'),
    ('core/plugin-kit', '@muse/plugin-kit'),
    ('core/plugin-graph', '@muse/plugin-graph'),
    ('core/context-broker', '@muse/context-broker'),
    ('core/contract-document', '@muse/contract-document'),
    ('plugins/appflowy-view-reference', '@muse/plugin-appflowy-view-reference'),
    ('plugins/appflowy-view-rename', '@muse/plugin-appflowy-view-rename'),
    ('plugins/dsh-resource-presentation-host', '@muse/dsh-resource-presentation-host'),
    ('plugins/dsh-client-ui-resource-open', '@muse/dsh-client-ui-resource-open'),
    ('plugins/dsh-tool-resource-present', '@muse/dsh-tool-resource-present'),
    ('plugins/appflowy-markdown', '@muse/plugin-appflowy-markdown'),
    ('plugins/appflowy-workspace', '@muse/plugin-appflowy-workspace'),
    ('plugins/dsh-mobile-surface', '@muse/dsh-mobile-surface'),
    ('plugins/dsh-mobile-input', '@muse/dsh-mobile-input'),
    ('plugins/dsh-appflowy', '@muse/dsh-appflowy'),
)

# Full paths only: Python subprocess cannot spawn bare `.cmd` names.
PNPM = shutil.which('pnpm') or 'pnpm'
NPM = shutil.which('npm') or 'npm'

# Carriers of large native SDKs the Muse product does not use (preset rows are
# disabled by default; official minimal closures exclude them too). Removing
# them saves ~700 MB unpacked. See packaging-setup-exe-plan.md step 0.
EXCLUDED_PACKAGES = {
    '@deepseek-ai/dsh-subagent-codex',
    '@deepseek-ai/dsh-subagent-claude-code',
}


def run(args: list[str], cwd: Path, env: dict[str, str] | None = None) -> None:
    print('+', ' '.join(args), flush=True)
    # Resolve junction cwds: CreateProcess rejects a reparse-point path (WinError 267).
    subprocess.run(args, cwd=str(cwd.resolve()), env=env, check=True)


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--out', default=str(ROOT / 'tmp' / 'muse-closure'))
    parser.add_argument('--skip-pack', action='store_true', help='reuse existing npm-dsh/npm-vendor tarballs')
    args = parser.parse_args()

    out = Path(args.out)
    if not out.is_absolute():
        out = ROOT / out
    if args.skip_pack and (out / 'tarballs').is_dir() and any((out / 'tarballs').rglob('*.tgz')):
        # Reuse the packed tarballs; only a fresh install tree is rebuilt.
        for stale in ('node_modules', 'package-lock.json'):
            target = out / stale
            if target.exists() or target.is_symlink():
                shutil.rmtree(target, ignore_errors=True)
    else:
        if out.exists():
            shutil.rmtree(out)
        out.mkdir(parents=True)
    tarballs_dir = out / 'tarballs'
    tarballs_dir.mkdir(parents=True, exist_ok=True)
    for sub in ('dsh', 'vendor'):
        (tarballs_dir / sub).mkdir(parents=True, exist_ok=True)

    if not args.skip_pack:
        # pack.ts verifies the client build record against the official profile.
        record_path = HARNESS / '.dsh-build' / 'client-build-environment.json'
        if not record_path.is_file():
            print(
                'warning: missing client build record at '
                f'{record_path}; packing tarballs from the current tree '
                '(Windows official-profile gate is skipped on this checkout).',
                flush=True,
            )
        else:
            record = json.loads(record_path.read_text(encoding='utf-8'))
            record_env = record.get('environment') or {}
            if record_env.get('DSH_CLIENT_BUILD_PROFILE') not in (None, 'official'):
                print(
                    'client build record is not the official profile; rebuild with:',
                    'DSH_BUILD_CLIENT_PROFILE=official pnpm run build (in vendors/deepseek-harness)',
                    file=sys.stderr,
                )
                return 1
        # pack.ts spawns the bare name `pnpm` from Node, which cannot launch a
        # `.cmd` shim on Windows; prepend a `pnpm.exe` launcher dir when one is
        # available (see MUSE_PNPM_SHIM in the docs).
        pack_env = None
        shim = ROOT / 'tmp' / 'pnpm-shim'
        if shim.is_dir() and (shim / 'pnpm.exe').is_file():
            pack_env = dict(os.environ)
            pack_env['PATH'] = os.pathsep.join([str(shim), pack_env.get('PATH', '')])
        # 1. pack both release families (dsh + vendor); pack.ts CLEARS its
        # --out directory, so each family gets its own subdirectory.
        for family in ('dsh', 'vendor'):
            run(
                [PNPM, 'exec', 'tsx', 'scripts/release/pack.ts',
                 '--family', family, '--out', str(tarballs_dir / family)],
                cwd=HARNESS,
                env=pack_env,
            )
        # dshmarket: pack the latest published release into the closure tarball
        # dir (older releases peer against dsh-settings APIs the current harness
        # does not ship; the market is maintained in lockstep with the harness).
        run([NPM, 'pack', 'dshmarket@latest'], cwd=tarballs_dir)
    else:
        # --skip-pack still refreshes the market release, then reuses family tarballs.
        for stale in tarballs_dir.glob('dshmarket-*.tgz'):
            stale.unlink()
        run([NPM, 'pack', 'dshmarket@latest'], cwd=tarballs_dir)
    # If --skip-pack, reuse tarballs already under the closure, else copy
    # whatever the harness produced.
    if args.skip_pack:
        for sub in ('dsh', 'vendor'):
            sub_dir = tarballs_dir / sub
            if sub_dir.is_dir() and len(list(sub_dir.glob('*.tgz'))) > 0:
                continue
            family_dir = HARNESS / 'dist' / f'npm-{sub}'
            if family_dir.is_dir():
                sub_dir.mkdir(parents=True, exist_ok=True)
                for tgz in family_dir.glob('*.tgz'):
                    shutil.copy2(tgz, sub_dir / tgz.name)

    # @muse as packed tarballs: `file:` directory deps are LINKED by npm and
    # their dependencies never enter the closure (ajv, canonicalize, ...), while
    # `file:` tarball deps are installed WITH their dependencies. Pack them into
    # tarballs/muse so npm install brings the whole @muse graph into the closure.
    # Vendored bare-name plugins ride the same route: patch.yml mounts them by
    # name, so they must be closure dependencies, not just source-tree folders.
    muse_tarballs = tarballs_dir / 'muse'
    muse_tarballs.mkdir(parents=True, exist_ok=True)
    for rel, _name in (*MUSE_PACKAGE_DIRS, *mw.VENDORED_PLUGIN_DIRS):
        pkg_dir = ROOT / 'middlewares' / 'dsh' / Path(*rel.split('/'))
        vendored = (rel, _name) in mw.VENDORED_PLUGIN_DIRS
        if not (pkg_dir / 'dist').is_dir() and not vendored:
            print(f'tarball missing built dist for {_name} ({pkg_dir})', file=sys.stderr)
            return 1
        if not (pkg_dir / 'package.json').is_file():
            print(f'tarball missing vendored plugin {_name} ({pkg_dir})', file=sys.stderr)
            return 1
        # @muse manifests use pnpm `link:`/`workspace:` specs for sibling and
        # harness packages (npm rejects `link:`); those packages are provided
        # by the closure root, so drop the protocol specs and keep the registry
        # deps (ajv, canonicalize, ...) that npm must install into the closure.
        _pack_muse(pkg_dir, muse_tarballs)

    deps: dict[str, str] = {}
    import tarfile
    for tgz in sorted(tarballs_dir.rglob('*.tgz')):
        with tarfile.open(tgz, 'r:gz') as archive:
            member = next(
                m for m in archive.getmembers() if m.name.endswith('package/package.json')
            )
            manifest = json.loads(archive.extractfile(member).read())
        if manifest['name'] in EXCLUDED_PACKAGES:
            print(f'    excluded {manifest["name"]}', flush=True)
            continue
        deps[manifest['name']] = f'file:{tgz.relative_to(out).as_posix()}'

    manifest = {
        'name': 'dsh-office-closure',
        'private': True,
        'version': '0.0.1',
        'dependencies': dict(sorted(deps.items())),
    }
    # dshmarket's peerOptional range predates newer harness alphas; pin its
    # settings peer to the closure's version (mirrors repair_dshmarket_peers).
    harness_version = record_env.get('DSH_CLIENT_VERSION') if 'record_env' in dir() else None
    record_path = HARNESS / '.dsh-build' / 'client-build-environment.json'
    if harness_version is None and record_path.is_file():
        try:
            harness_version = (json.loads(record_path.read_text(encoding='utf-8'))
                              .get('environment') or {}).get('DSH_CLIENT_VERSION')
        except json.JSONDecodeError:
            harness_version = None
    if harness_version:
        manifest['overrides'] = {'dshmarket': {'@deepseek-ai/dsh-settings': harness_version}}
    (out / 'package.json').write_text(json.dumps(manifest, indent=2) + '\n', encoding='utf-8')

    # 3. install the closure. --legacy-peer-deps: @muse packages peer against older
    # harness alphas (rc.7) that the current closure cannot satisfy; the peers
    # exist at the closure root and resolve fine at runtime.
    env = dict(os.environ)
    env['npm_config_side_effects_cache'] = 'false'
    run([NPM, 'install', '--omit=dev', '--no-audit', '--no-fund', '--legacy-peer-deps'],
        cwd=out, env=env)

    # 4. npm installs `file:` directory dependencies as symlinks/junctions;
    #    dereference @muse into real copies so the tree zips correctly.
    sys.path.insert(0, str(ROOT / 'frontend' / 'client' / 'scripts' / 'lib'))
    from muse_windows import copy_tree, rmtree_nofollow  # noqa: E402

    muse_nm = out / 'node_modules' / '@muse'
    dereferenced = 0
    if muse_nm.is_dir():
        for entry in sorted(muse_nm.iterdir()):
            try:
                linked = entry.is_symlink()
                attrs = getattr(entry.lstat(), "st_file_attributes", 0)
                linked = linked or bool(attrs & 0x400)
            except OSError:
                continue
            if not linked:
                continue
            target = entry.resolve()
            rmtree_nofollow(entry)
            copy_tree(target, entry, exclude_names=('node_modules',))
            dereferenced += 1
            print(f'    dereferenced {entry.name}', flush=True)
    if dereferenced == 0:
        print('    (no @muse junctions found)', flush=True)

    print(f'closure at {out}')
    return 0


def _pack_muse(pkg_dir, dest) -> None:
    manifest_path = pkg_dir / 'package.json'
    manifest = json.loads(manifest_path.read_text(encoding='utf-8'))
    rewritten = False
    for kind in ('dependencies', 'peerDependencies', 'optionalDependencies'):
        deps = manifest.get(kind)
        if not deps:
            continue
        removed = [name for name, spec in deps.items()
                   if isinstance(spec, str) and (spec.startswith('link:') or spec.startswith('workspace:'))]
        for name in removed:
            del deps[name]
            rewritten = True
    if not rewritten:
        run([NPM, 'pack', str(pkg_dir), '--pack-destination', str(dest)], cwd=ROOT)
        return
    with tempfile.TemporaryDirectory(dir=str(ROOT / 'tmp')) as raw:
        staging = Path(raw) / 'pkg'
        shutil.copytree(pkg_dir, staging, ignore=shutil.ignore_patterns('node_modules'))
        (staging / 'package.json').write_text(json.dumps(manifest, indent=2) + '\n', encoding='utf-8')
        run([NPM, 'pack', str(staging), '--pack-destination', str(dest)], cwd=ROOT)


if __name__ == '__main__':
    try:
        raise SystemExit(main())
    except subprocess.CalledProcessError as error:
        raise SystemExit(error.returncode) from error
    except (FileNotFoundError, RuntimeError) as error:
        print(error, file=sys.stderr)
        raise SystemExit(1) from error
