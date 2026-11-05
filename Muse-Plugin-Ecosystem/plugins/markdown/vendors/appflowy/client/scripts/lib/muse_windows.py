#!/usr/bin/env python3
"""Shared helpers for packing DSH Office on Windows.

Mirrors frontend/client/scripts/lib/muse-macos.sh for the Windows portable
layout: `{exeDir}/muse/{node,dsh,packages,patch.yml}`.
"""
from __future__ import annotations

import json
import os
import shutil
import stat
import subprocess
import sys
import tarfile
import tempfile
import urllib.request
import zipfile
from pathlib import Path

NODE_VERSION = "22.19.0"
DSHMARKET_SPEC = os.environ.get("MUSE_DSHMARKET_SPEC", "dshmarket@1.31.1")
# Bundled pnpm for one-click plugin installs; mirrors the version DSH Desktop pins.
PNPM_SPEC = os.environ.get("MUSE_PNPM_SPEC", "pnpm@10.34.5")

# Vendored upstream plugins that the composition patch mounts by name
# (`name: dsh-model-capabilities`, see @muse/dsh-appflowy/cordis.patch.yml) and
# that ship a prebuilt entry file (no build step): bare names, and @muse-scoped
# client-UI packages whose shipped file is `lib/` rather than a built `dist/`.
# The Loader resolves names from the profile node_modules, so each package must
# (a) exist in the packed closure and (b) for bare names be seeded by
# DshSidecar.seedClosurePlugins (@muse/* is seeded by the same walk).
VENDORED_PLUGIN_DIRS = (
    ("plugins/dsh-model-capabilities", "dsh-model-capabilities"),
    ("plugins/dsh-client-ui-workspace-binding", "@muse/dsh-client-ui-workspace-binding"),
    (
        "plugins/dsh-client-ui-resource-reference",
        "@muse/dsh-client-ui-resource-reference",
    ),
)

PACKAGE_DIRS = (
    ("core/protocol/host-bridge", "host-bridge"),
    ("core/contract-resource", "contract-resource"),
    ("core/contract-presentation", "contract-presentation"),
    ("core/contract-engine-session", "contract-engine-session"),
    ("core/resource-host", "resource-host"),
    ("core/dsh-resource-presentation", "dsh-resource-presentation"),
    ("core/plugin-facets", "plugin-facets"),
    ("core/plugin-kit", "plugin-kit"),
    ("core/plugin-graph", "plugin-graph"),
    ("core/context-broker", "context-broker"),
    ("core/contract-document", "contract-document"),
    ("plugins/appflowy-view-reference", "plugin-appflowy-view-reference"),
    ("plugins/appflowy-view-rename", "plugin-appflowy-view-rename"),
    ("plugins/dsh-resource-presentation-host", "dsh-resource-presentation-host"),
    ("plugins/dsh-client-ui-resource-open", "dsh-client-ui-resource-open"),
    ("plugins/dsh-tool-resource-present", "dsh-tool-resource-present"),
    ("plugins/appflowy-markdown", "plugin-appflowy-markdown"),
    ("plugins/appflowy-workspace", "plugin-appflowy-workspace"),
    ("plugins/dsh-mobile-surface", "dsh-mobile-surface"),
    ("plugins/dsh-mobile-input", "dsh-mobile-input"),
    ("plugins/dsh-appflowy", "dsh-appflowy"),
)

LIB_DIR = Path(__file__).resolve().parent
WIRE_SCRIPT = LIB_DIR / "wire-muse-node-modules.py"


def find_muse_root(start: Path | None = None) -> Path:
    env = os.environ.get("MUSE_ROOT", "").strip()
    if env:
        root = Path(env).expanduser().resolve()
        if root.is_dir():
            return root
    cand = (start or LIB_DIR).resolve()
    while True:
        if (cand / "middlewares" / "dsh").is_dir() and (
            cand / "frontend" / "client"
        ).is_dir():
            return cand
        if cand.parent == cand:
            break
        cand = cand.parent
    raise FileNotFoundError(
        f"cannot find Muse repo root from {start or LIB_DIR}; set MUSE_ROOT"
    )


def client_dir(root: Path | None = None) -> Path:
    return (root or find_muse_root()) / "frontend" / "client"


def frontend_dir(root: Path | None = None) -> Path:
    return client_dir(root) / "frontend"


def flutter_dir(root: Path | None = None) -> Path:
    return frontend_dir(root) / "appflowy_flutter"


def dist_dir(root: Path | None = None) -> Path:
    return client_dir(root) / "dist"


def packages_root(root: Path | None = None) -> Path:
    return (root or find_muse_root()) / "middlewares" / "dsh"


def harness_dir(root: Path | None = None) -> Path:
    return (root or find_muse_root()) / "vendors" / "deepseek-harness"


def dsh_patch(root: Path | None = None) -> Path:
    return packages_root(root) / "plugins" / "dsh-appflowy" / "cordis.patch.yml"


def overlay_dsh_appflowy_dist(muse_dir: Path, root: Path | None = None) -> None:
    """Replace packed @muse/dsh-appflowy JS without rebuilding the whole closure."""
    src = packages_root(root) / "plugins" / "dsh-appflowy" / "dist"
    dest = (
        muse_dir / "closure" / "node_modules" / "@muse" / "dsh-appflowy" / "dist"
    )
    if not src.is_dir() or not dest.parent.is_dir():
        return
    if dest.exists():
        rmtree_nofollow(dest)
    shutil.copytree(src, dest)
    print(f"==> Overlayed @muse/dsh-appflowy from {src}", flush=True)


def which(name: str) -> str | None:
    found = shutil.which(name)
    if found:
        return found
    if os.name == "nt":
        for suffix in (".cmd", ".bat", ".exe"):
            found = shutil.which(name + suffix)
            if found:
                return found
    return None


def decode_output(data: bytes | None) -> str:
    if not data:
        return ""
    for encoding in ("utf-8", "gb18030", "mbcs"):
        try:
            return data.decode(encoding)
        except UnicodeDecodeError:
            continue
    return data.decode("utf-8", errors="replace")


def run_capture(
    args: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    check: bool = True,
) -> subprocess.CompletedProcess[str]:
    completed = subprocess.run(
        args,
        cwd=cwd,
        env=env,
        check=check,
        capture_output=True,
    )
    stdout = decode_output(completed.stdout)
    stderr = decode_output(completed.stderr)
    return subprocess.CompletedProcess(
        completed.args,
        completed.returncode,
        stdout,
        stderr,
    )


def run(
    args: list[str],
    *,
    cwd: Path | None = None,
    env: dict[str, str] | None = None,
    check: bool = True,
) -> subprocess.CompletedProcess[str]:
    cmd = list(args)
    resolved = which(cmd[0])
    if resolved:
        cmd[0] = resolved
    print("+", " ".join(cmd), flush=True)
    return subprocess.run(
        cmd,
        cwd=cwd,
        env=env,
        check=check,
    )


def _is_flutter_sdk(home: Path) -> bool:
    return (home / "bin" / "flutter.bat").is_file() or (home / "bin" / "flutter").is_file()


def flutter_sdk_version(home: Path) -> str:
    version_file = home / "version"
    if version_file.is_file():
        return version_file.read_text(encoding="utf-8", errors="replace").strip()
    return ""


def flutter_home() -> Path:
    candidates: list[Path] = []
    env = os.environ.get("FLUTTER_HOME", "").strip()
    if env:
        candidates.append(Path(env).expanduser())
    candidates.append(Path.home() / "sdks" / "flutters" / "flutter")
    candidates.append(Path(r"D:\muse\flutter-3.27.4"))
    flutter = which("flutter")
    if flutter:
        candidates.append(Path(flutter).resolve().parent.parent)

    found: list[Path] = []
    seen: set[Path] = set()
    for candidate in candidates:
        if not _is_flutter_sdk(candidate):
            continue
        resolved = candidate.resolve()
        if resolved in seen:
            continue
        seen.add(resolved)
        found.append(resolved)
    for home in found:
        if flutter_sdk_version(home).startswith("3.27."):
            return home
    if found:
        return found[0]
    raise FileNotFoundError(
        "Flutter SDK not found. Set FLUTTER_HOME to a 3.27.x SDK."
    )


def require_flutter_327() -> None:
    home = flutter_home()
    version = flutter_sdk_version(home)
    if not version:
        flutter = home / "bin" / ("flutter.bat" if os.name == "nt" else "flutter")
        out = run_capture([str(flutter), "--version"], check=True)
        version = (out.stdout or out.stderr).splitlines()[0] if (out.stdout or out.stderr) else ""
    if "3.27." not in version:
        raise RuntimeError(
            f"Desktop packs need Flutter 3.27.x; got: {version or 'unknown'} "
            f"from {home}. Set FLUTTER_HOME."
        )
    print(f"Flutter: {version} ({home})", flush=True)


def find_vcvars() -> Path:
    vswhere = (
        Path(os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)"))
        / "Microsoft Visual Studio"
        / "Installer"
        / "vswhere.exe"
    )
    if not vswhere.is_file():
        raise FileNotFoundError(
            "vswhere.exe not found; install Visual Studio with MSVC and CMake."
        )
    out = run_capture(
        [
            str(vswhere),
            "-latest",
            "-products",
            "*",
            "-requires",
            "Microsoft.VisualStudio.Component.VC.Tools.x86.x64",
            "-property",
            "installationPath",
        ],
        check=True,
    )
    install = out.stdout.strip()
    vcvars = Path(install) / "VC" / "Auxiliary" / "Build" / "vcvars64.bat"
    if not vcvars.is_file():
        raise FileNotFoundError(f"vcvars64.bat missing at {vcvars}")
    return vcvars


def run_vs(command: str, *, cwd: Path) -> None:
    vcvars = find_vcvars()
    flutter_bin = flutter_home() / "bin"
    extras = [
        str(flutter_bin),
        str(Path(os.environ.get("LOCALAPPDATA", "")) / "Pub" / "Cache" / "bin"),
        r"D:\Flutter\.cache\bin",
        r"D:\Rust\.cargo\bin",
    ]
    prefix = ";".join(p for p in extras if p and Path(p).exists())
    inner = (
        f'call "{vcvars}" && set "PATH={prefix};%PATH%" && {command}'
    )
    print("+", inner, flush=True)
    completed = subprocess.run(inner, shell=True, cwd=cwd)
    if completed.returncode != 0:
        raise RuntimeError(f"command failed ({completed.returncode}): {command}")


def is_reparse(path: Path) -> bool:
    try:
        attrs = getattr(os.lstat(path), "st_file_attributes", 0)
        return bool(attrs & stat.FILE_ATTRIBUTE_REPARSE_POINT)
    except OSError:
        return False


def rmtree_nofollow(path: Path) -> None:
    """Delete a tree. Junctions/symlinks are unlinked, never followed."""
    if not path.exists() and not is_reparse(path):
        try:
            if not path.is_symlink():
                return
        except OSError:
            return
    walk: list[Path] = [path]
    files: list[Path] = []
    links: list[Path] = []
    dirs: list[Path] = []
    seen: set[str] = set()
    while walk:
        current = walk.pop()
        key = str(current).casefold()
        if key in seen:
            continue
        seen.add(key)
        try:
            if is_reparse(current) or current.is_symlink():
                links.append(current)
                continue
            if current.is_dir():
                dirs.append(current)
                with os.scandir(current) as entries:
                    walk.extend(Path(entry.path) for entry in entries)
            else:
                files.append(current)
        except OSError:
            continue
    for item in files:
        try:
            item.unlink()
        except OSError:
            pass
    for item in links:
        try:
            os.rmdir(item)
        except OSError:
            try:
                item.unlink()
            except OSError:
                pass
    for item in reversed(dirs):
        try:
            item.rmdir()
        except OSError:
            pass


def copy_tree(
    source: Path,
    destination: Path,
    *,
    exclude_names: tuple[str, ...] = (),
) -> None:
    """Copy a tree without exploding or recursing through pnpm junctions."""
    source = source.resolve()
    if not source.exists():
        raise FileNotFoundError(f"missing source tree: {source}")
    destination.mkdir(parents=True, exist_ok=True)

    copied_real: dict[str, Path] = {}
    copied_files = 0
    stack: list[tuple[Path, Path]] = []

    def real_key(path: Path) -> str:
        try:
            return str(path.resolve()).casefold()
        except OSError:
            return str(path).casefold()

    def try_rel_link(target: Path, dest: Path) -> bool:
        dest.parent.mkdir(parents=True, exist_ok=True)
        if dest.exists() or dest.is_symlink() or is_reparse(dest):
            return True
        relative = os.path.relpath(target, dest.parent)
        try:
            os.symlink(relative, dest, target_is_directory=target.is_dir())
            return True
        except OSError:
            return False

    def copy_file(src: Path, dest: Path) -> None:
        nonlocal copied_files
        dest.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(src, dest)
        copied_files += 1
        if copied_files % 2000 == 0:
            print(f"    copied {copied_files} files...", flush=True)

    def schedule_dir(src: Path, dest: Path) -> None:
        key = real_key(src)
        previous = copied_real.get(key)
        if previous is not None:
            try_rel_link(previous, dest)
            return
        dest.mkdir(parents=True, exist_ok=True)
        copied_real[key] = dest
        stack.append((src, dest))

    schedule_dir(source, destination)
    while stack:
        src, dest = stack.pop()
        try:
            entries = os.scandir(src)
        except OSError:
            continue
        with entries:
            for entry in entries:
                if entry.name in exclude_names:
                    continue
                src_item = Path(entry.path)
                dest_item = dest / entry.name
                try:
                    linked = entry.is_symlink() or is_reparse(src_item)
                    is_dir = entry.is_dir(follow_symlinks=False)
                    is_file = entry.is_file(follow_symlinks=False)
                except OSError:
                    continue
                if linked:
                    try:
                        real = src_item.resolve()
                    except OSError:
                        continue
                    if real.is_dir():
                        schedule_dir(real, dest_item)
                    elif real.is_file():
                        key = real_key(real)
                        previous = copied_real.get(key)
                        if previous is not None:
                            try_rel_link(previous, dest_item)
                        else:
                            copy_file(real, dest_item)
                            copied_real[key] = dest_item
                    continue
                if is_dir:
                    schedule_dir(src_item, dest_item)
                    continue
                if is_file:
                    copy_file(src_item, dest_item)

    print(f"    copied {copied_files} files from {source}", flush=True)


def _download(url: str, dest: Path) -> None:
    dest.parent.mkdir(parents=True, exist_ok=True)
    print(f"==> Downloading {url}", flush=True)
    with urllib.request.urlopen(url) as response, dest.open("wb") as handle:
        shutil.copyfileobj(response, handle)


def stage_node(destination: Path, cache_dir: Path | None = None) -> Path:
    name = f"node-v{NODE_VERSION}-win-x64"
    cache = cache_dir or (dist_dir() / "cache")
    cache.mkdir(parents=True, exist_ok=True)
    zip_path = cache / f"{name}.zip"
    if not zip_path.is_file():
        _download(f"https://nodejs.org/dist/v{NODE_VERSION}/{name}.zip", zip_path)
    with tempfile.TemporaryDirectory(prefix="muse-node-") as raw:
        extract = Path(raw)
        with zipfile.ZipFile(zip_path) as archive:
            archive.extractall(extract)
        payload = extract / name
        if not (payload / "node.exe").is_file():
            raise FileNotFoundError(f"Node zip did not contain {name}/node.exe")
        if destination.exists():
            shutil.rmtree(destination)
        shutil.copytree(payload, destination)
    node = destination / "node.exe"
    bin_dir = destination / "bin"
    bin_dir.mkdir(exist_ok=True)
    shutil.copy2(node, bin_dir / "node.exe")
    print(f"Staged Node {NODE_VERSION} at {node}", flush=True)
    return node


def copy_dsh_packages(destination: Path, root: Path | None = None) -> None:
    pkg_root = packages_root(root)
    destination.mkdir(parents=True, exist_ok=True)
    for rel, short in PACKAGE_DIRS:
        pkg = pkg_root / Path(*rel.split("/"))
        dist = pkg / "dist"
        if not dist.is_dir():
            raise FileNotFoundError(
                f"missing built dist for {rel} "
                "(run frontend/client/scripts/pack-windows-client.py without --skip-packages)"
            )
        print(f"    copy {rel} -> {destination / short}", flush=True)
        out = destination / short
        if out.exists():
            shutil.rmtree(out)
        out.mkdir(parents=True)
        shutil.copy2(pkg / "package.json", out / "package.json")
        for extra in ("cordis.patch.yml", "README.md"):
            src = pkg / extra
            if src.is_file():
                shutil.copy2(src, out / extra)
        shutil.copytree(dist, out / "dist")


def wire_muse_node_modules(harness: Path, node: Path, root: Path | None = None) -> None:
    run(
        [sys.executable, str(WIRE_SCRIPT), str(harness), str(root or find_muse_root()), str(node)]
    )


def _dshmarket_version() -> str:
    return DSHMARKET_SPEC.rsplit("@", 1)[-1]


def repair_dshmarket_peers(harness: Path) -> None:
    dest = harness / "node_modules" / "dshmarket"
    scoped = dest / "node_modules" / "@deepseek-ai"
    scoped.mkdir(parents=True, exist_ok=True)
    pairs = {
        "cordis": harness / "vendor" / "cordis",
        "schemastery": harness / "vendor" / "schemastery",
        "dsh-settings": harness / "packages" / "settings" / "settings",
    }
    for name, src in pairs.items():
        if not src.exists():
            continue
        link = scoped / name
        # Junction-safe copy: the packed harness tree re-creates pnpm junctions,
        # and a plain copytree(symlinks=False) would follow them into .pnpm and
        # recurse for minutes to hours. node_modules is never needed for peers —
        # the host profile supplies those — so it is excluded instead of copied.
        rmtree_nofollow(link)
        copy_tree(src, link, exclude_names=("node_modules",))


def _make_writable(directory: Path) -> None:
    """Clear read-only bits a tarball's POSIX modes applied on Windows.

    npm tarballs (pnpm's in particular) ship directories with modes like 0555;
    tarfile materializes those as read-only on Windows, which then blocks every
    later delete/pack step. Call after extraction and before any rmtree/copy.
    """
    for current in directory.rglob("*"):
        try:
            if current.is_symlink():
                continue
            current.chmod(current.stat().st_mode | stat.S_IWRITE)
        except OSError:
            pass


def _extract_npm_tarball(tarball: Path, destination: Path) -> None:
    """Unpack an `npm pack` tarball, stripping the leading `package/` directory.

    Extracts into `destination/_raw` and removes the staging dir when done.
    Plain os.mkdir paths only (no tempfile): sandboxed/CI runners deny entry
    into tempfile's 0700 directories, while ordinary directories work.
    """
    destination.mkdir(parents=True, exist_ok=True)
    temp = destination / "_raw-tmp"
    rmtree_nofollow(temp)
    raw = destination / "_raw"
    rmtree_nofollow(raw)
    raw.mkdir()
    try:
        with tarfile.open(tarball, "r:gz") as archive:
            archive.extractall(raw)
        _make_writable(raw)
        package = raw / "package"
        if not package.is_dir():
            children = [item for item in raw.iterdir() if item.is_dir()]
            if len(children) != 1:
                raise FileNotFoundError(f"unexpected npm pack layout in {tarball}")
            package = children[0]
        for item in package.iterdir():
            shutil.move(str(item), str(destination / item.name))
    finally:
        rmtree_nofollow(raw)
        rmtree_nofollow(temp)
    _make_writable(destination)


def stage_dshmarket(harness: Path) -> None:
    version = _dshmarket_version()
    dest = harness / "node_modules" / "dshmarket"
    if (dest / "package.json").is_file() and (dest / "lib" / "index.js").is_file():
        print(f"dshmarket already staged at {dest}", flush=True)
        repair_dshmarket_peers(harness)
        return
    cache = dist_dir() / "cache"
    cache.mkdir(parents=True, exist_ok=True)
    (harness / "node_modules").mkdir(parents=True, exist_ok=True)
    tarball = cache / f"dshmarket-{version}.tgz"
    if not tarball.is_file():
        print(f"==> Fetching {DSHMARKET_SPEC} from npm", flush=True)
        run(["npm", "pack", DSHMARKET_SPEC], cwd=cache)
    if not tarball.is_file():
        raise FileNotFoundError(f"npm pack did not produce {tarball}")
    extract = cache / f"dshmarket-extract-{version}"
    rmtree_nofollow(extract)
    try:
        _extract_npm_tarball(tarball, extract)
        # Published tarball already includes lib/; npm install here trips
        # npm "edgesOut" bugs on Windows and is not needed for runtime.
        if dest.exists():
            rmtree_nofollow(dest)
        shutil.copytree(extract, dest)
    finally:
        rmtree_nofollow(extract)
    if not (dest / "lib" / "index.js").is_file():
        raise FileNotFoundError("staged dshmarket is missing lib/index.js")
    repair_dshmarket_peers(harness)
    print(f"Staged dshmarket {version} at {dest}", flush=True)


def stage_vendored_dsh_plugins(muse_dir: Path, root: Path | None = None) -> None:
    """Copy vendored bare-name plugins into the packed runtime node_modules.

    Mirrors middlewares/scripts/stage-dsh-runtime.sh for the packed layouts:
    patch.yml mounts these plugins by id/name, so each one has to sit where the
    Loader — and DshSidecar.seedClosurePlugins, which only seeds `dshmarket` plus
    the `@muse/*` scope — can resolve them. Fresh closure builds already install
    them through build-muse-closure.py; this keeps `--reuse-runtime` packs and
    legacy `dsh/` layouts in sync instead of silently dropping the row.
    """
    root = root or find_muse_root()
    targets = (
        muse_dir / "closure" / "node_modules",
        muse_dir / "dsh" / "node_modules",
    )
    for rel, name in VENDORED_PLUGIN_DIRS:
        src = packages_root(root) / Path(*rel.split("/"))
        if not (src / "package.json").is_file():
            raise FileNotFoundError(f"vendored plugin missing at {src}")
        for target in targets:
            if not target.is_dir():
                continue
            dest = target / name
            if dest.exists():
                rmtree_nofollow(dest)
            copy_tree(src, dest, exclude_names=("node_modules", ".git"))
            print(f"Staged vendored plugin {name} at {dest}", flush=True)


def _pnpm_version() -> str:
    return PNPM_SPEC.rsplit("@", 1)[-1]


def stage_plugin_tools(destination: Path) -> None:
    """Stage the bundled pnpm + lock-recovery runner under `{destination}/plugin-tools`.

    `plugin-tools/pnpm/` is the full `pnpm` npm package (bin/pnpm.cjs is the CLI
    entry); `plugin-tools/pnpm-runner.mjs` is the ported DSH Desktop runner that
    wraps every pnpm operation (Windows locked-rename recovery, idle timeout,
    process-tree kill). The Flutter sidecar writes `$DSH_HOME/.desktop-bin`
    shims pointing here, so `dsh plugin --profile <p> add <pkg>` resolves its
    `pnpm` to the bundled node + pnpm with zero user-side tooling.
    """
    version = _pnpm_version()
    tools = destination / "plugin-tools"
    pnpm_dir = tools / "pnpm"
    entry = pnpm_dir / "bin" / "pnpm.cjs"
    runner = tools / "pnpm-runner.mjs"
    if entry.is_file() and runner.is_file():
        print(f"plugin-tools already staged at {tools}", flush=True)
        return
    cache = dist_dir() / "cache"
    cache.mkdir(parents=True, exist_ok=True)
    tarball = cache / f"pnpm-{version}.tgz"
    if not tarball.is_file():
        print(f"==> Fetching {PNPM_SPEC} from npm", flush=True)
        # Workspace-local npm cache: keeps sandboxed/CI builds from writing the
        # user's %LOCALAPPDATA%\npm-cache and keeps the download offline-repeatable.
        npm_cache = cache / "npm-cache"
        npm_cache.mkdir(parents=True, exist_ok=True)
        run(["npm", "pack", PNPM_SPEC, "--cache", str(npm_cache)], cwd=cache)
    if not tarball.is_file():
        raise FileNotFoundError(f"npm pack did not produce {tarball}")
    tools.mkdir(parents=True, exist_ok=True)
    extract = cache / f"pnpm-extract-{version}"
    rmtree_nofollow(extract)
    try:
        _extract_npm_tarball(tarball, extract)
        if pnpm_dir.exists():
            rmtree_nofollow(pnpm_dir)
        shutil.copytree(extract, pnpm_dir)
    finally:
        rmtree_nofollow(extract)
    if not entry.is_file():
        raise FileNotFoundError(f"staged pnpm is missing {entry}")
    shutil.copy2(WIRE_SCRIPT.parent / "pnpm-runner.mjs", runner)
    print(f"Staged pnpm {version} + pnpm-runner.mjs at {tools}", flush=True)


CLOSURE_ENTRY_REL = "closure/node_modules/@deepseek-ai/dsh/lib/bin.js"
INNO_SETUP_VERSION = "6.7.3"


def closure_entry(muse_dir: Path) -> Path:
    return muse_dir / "closure" / "node_modules" / "@deepseek-ai" / "dsh" / "lib" / "bin.js"


def looks_like_closure(muse_dir: Path) -> bool:
    return (muse_dir / "patch.yml").is_file() and closure_entry(muse_dir).is_file()


def looks_like_legacy_runtime(muse_dir: Path) -> bool:
    return (muse_dir / "dsh" / "apps" / "cli" / "src" / "bin.ts").is_file()


def node_arch_tag() -> str:
    machine = os.uname().machine if hasattr(os, "uname") else ""
    if sys.platform == "darwin":
        return "arm64" if machine == "arm64" else "x64"
    if sys.platform.startswith("linux"):
        return "arm64" if machine in {"aarch64", "arm64"} else "x64"
    return "x64"


def bundled_node_bin(muse_dir: Path) -> Path:
    """Official Node tarball uses `bin/node`; the Windows zip uses `node.exe`."""
    for rel in ("node/bin/node", "node/bin/node.exe", "node/node.exe"):
        candidate = muse_dir / Path(*rel.split("/"))
        if candidate.is_file():
            return candidate
    raise FileNotFoundError(f"bundled Node binary missing under {muse_dir / 'node'}")


def stage_bundled_node_unix(destination: Path, *, node: Path | None = None) -> Path:
    """Place official `bin/node` at `{destination}/node/bin/node`."""
    node_dir = destination / "node"
    dest = node_dir / "bin" / "node"
    if node is not None and Path(node).is_file():
        src = Path(node)
    elif dest.is_file():
        src = dest
    else:
        arch = node_arch_tag()
        name = f"node-v{NODE_VERSION}-darwin-{arch}" if sys.platform == "darwin" else f"node-v{NODE_VERSION}-linux-{arch}"
        cache = dist_dir() / "cache"
        cache.mkdir(parents=True, exist_ok=True)
        archive = cache / f"{name}.tar.gz"
        if not archive.is_file():
            _download(f"https://nodejs.org/dist/v{NODE_VERSION}/{name}.tar.gz", archive)
        extracted = cache / f"{name}-extracted"
        if not (extracted / "bin" / "node").is_file():
            if extracted.exists():
                rmtree_nofollow(extracted)
            extracted.mkdir(parents=True)
            with tarfile.open(archive, "r:gz") as tar:
                try:
                    tar.extractall(extracted, filter="data")
                except TypeError:
                    tar.extractall(extracted)
            payload = extracted / name
            if payload.is_dir():
                for item in payload.iterdir():
                    shutil.move(str(item), str(extracted / item.name))
                payload.rmdir()
        src = extracted / "bin" / "node"
        if not src.is_file():
            raise FileNotFoundError(f"Node tarball did not contain bin/node: {archive}")
    dest.parent.mkdir(parents=True, exist_ok=True)
    if src.resolve() != dest.resolve():
        shutil.copy2(src, dest)
        dest.chmod(dest.stat().st_mode | stat.S_IEXEC)
    print(f"Staged bundled node at {dest}", flush=True)
    return dest


def stage_bundled_node_exe(destination: Path, *, node: Path | None = None) -> Path:
    """Place `node.exe` (+ `bin/node.exe` alias) at `{destination}/node/`.

    Closure packs only need the runtime binary, not the full Node zip (npm/corepack).
    """
    node_dir = destination / "node"
    exe = node_dir / "node.exe"
    if node is not None and Path(node).is_file():
        src = Path(node)
    elif exe.is_file():
        src = exe
    else:
        cache_node = dist_dir() / "cache" / f"node-v{NODE_VERSION}-win-x64-extracted"
        if not (cache_node / "node.exe").is_file():
            stage_node(cache_node)
        src = cache_node / "node.exe"
    node_dir.mkdir(parents=True, exist_ok=True)
    if src.resolve() != exe.resolve():
        shutil.copy2(src, exe)
    bin_dir = node_dir / "bin"
    bin_dir.mkdir(exist_ok=True)
    shutil.copy2(exe, bin_dir / "node.exe")
    print(f"Staged bundled node.exe at {exe}", flush=True)
    return exe


def stage_bundled_node(destination: Path, *, node: Path | None = None) -> Path:
    if sys.platform == "win32":
        return stage_bundled_node_exe(destination, node=node)
    return stage_bundled_node_unix(destination, node=node)


def stage_closure(
    source: Path,
    destination: Path,
    *,
    node: Path | None = None,
    patch: Path | None = None,
) -> None:
    """Stage the symlink-free npm closure into `destination` (the muse/ folder).

    Muse closure layout (see packaging-setup-exe-plan.md):
      muse/{node/node.exe, closure/{package.json,node_modules}, closure-tarballs/,
           plugin-tools/, patch.yml}
    `source` is a `build-muse-closure.py --out` directory (package.json + node_modules
    + tarballs). Tarballs land in closure-tarballs for first-launch profile installs.
    """
    destination.mkdir(parents=True, exist_ok=True)
    stage_bundled_node(destination, node=node)

    if not (source / "package.json").is_file():
        raise FileNotFoundError(f"closure source missing package.json: {source}")
    if not (source / "node_modules").is_dir():
        raise FileNotFoundError(f"closure source missing node_modules: {source}")

    closure_dir = destination / "closure"
    if closure_dir.exists():
        rmtree_nofollow(closure_dir)
    closure_dir.mkdir(parents=True)
    shutil.copy2(source / "package.json", closure_dir / "package.json")
    copy_tree(source / "node_modules", closure_dir / "node_modules")

    tarballs = destination / "closure-tarballs"
    if tarballs.exists():
        rmtree_nofollow(tarballs)
    tarballs.mkdir(parents=True)
    if (source / "tarballs").is_dir():
        copy_tree(source / "tarballs", tarballs)

    stage_plugin_tools(destination)
    patch_src = patch or dsh_patch()
    if not patch_src.is_file():
        raise FileNotFoundError(f"missing DSH patch {patch_src}")
    shutil.copy2(patch_src, destination / "patch.yml")
    if sys.platform == "darwin":
        readme = (
            "OpenMuse bundled DSH runtime (macOS, npm closure).\n\n"
            "Launched as:\n"
            "  node closure/node_modules/@deepseek-ai/dsh/lib/bin.js "
            "web --patch patch.yml --host 127.0.0.1 --port 3080\n"
            "User data lives in ~/Library/Application Support/OpenMuse/\n"
        )
    elif sys.platform == "win32":
        readme = (
            "OpenMuse bundled DSH runtime (Windows, npm closure).\n\n"
            "Launched as:\n"
            "  node closure\\node_modules\\@deepseek-ai\\dsh\\lib\\bin.js "
            "web --patch patch.yml --host 127.0.0.1 --port 3080\n"
            "User data lives in %APPDATA%\\OpenMuse\\\n"
        )
    else:
        readme = (
            "OpenMuse bundled DSH runtime (npm closure).\n\n"
            "Launched as:\n"
            "  node closure/node_modules/@deepseek-ai/dsh/lib/bin.js "
            "web --patch patch.yml --host 127.0.0.1 --port 3080\n"
        )
    (destination / "README.txt").write_text(readme, encoding="utf-8")
    stage_vendored_dsh_plugins(destination)
    print(f"Staged closure layout at {destination} (entry {CLOSURE_ENTRY_REL})", flush=True)


def shell_quote(value: str) -> str:
    return "'" + value.replace("'", "'\\''") + "'"


def write_desktop_bin_shims(
    dsh_home: Path, node: Path, plugin_tools: Path, *, platform: str = os.name
) -> Path:
    """Write `$DSH_HOME/.desktop-bin/{pnpm,node}` shims onto the bundled toolchain.

    Mirrors DSH Desktop's `profile-plugin-command.ts`: the shim directory is
    prepended to the harness process PATH so every `pnpm` by-name invocation
    (from `dsh plugin` or dshmarket) lands on the bundled node + pnpm runner.
    Runtime callers (the Flutter sidecar) own scheduling; this helper exists
    for parity and pack-time verification.
    """
    directory = (dsh_home / ".desktop-bin").resolve()
    directory.mkdir(parents=True, exist_ok=True)
    # Absolute paths only: the shims are executed from arbitrary cwds (the
    # profile directory), where a relative bundle path would not resolve.
    node = node.resolve()
    runner = (plugin_tools / "pnpm-runner.mjs").resolve()
    pnpm_entry = (plugin_tools / "pnpm" / "bin" / "pnpm.cjs").resolve()
    command = [str(runner), str(pnpm_entry)]
    if platform == "nt":
        quoted = " ".join(f'"{part}"' for part in command)
        (directory / "pnpm.cmd").write_text(
            f'@chcp 65001 >nul\r\n@echo off\r\n"{node}" {quoted} %*\r\n',
            encoding="utf-8",
        )
        (directory / "node.cmd").write_text(
            f'@chcp 65001 >nul\r\n@echo off\r\n"{node}" %*\r\n',
            encoding="utf-8",
        )
    else:
        quoted = " ".join(shell_quote(part) for part in command)
        pnpm = directory / "pnpm"
        pnpm.write_text(
            f"#!/bin/sh\nexec {shell_quote(str(node))} {quoted} \"$@\"\n",
            encoding="utf-8",
        )
        chmod = getattr(os, "chmod", None)
        if chmod is not None:
            chmod(pnpm, 0o755)
        node_path = directory / "node"
        node_path.write_text(
            f"#!/bin/sh\nexec {shell_quote(str(node))} \"$@\"\n",
            encoding="utf-8",
        )
        if chmod is not None:
            chmod(node_path, 0o755)
    print(f"Wrote .desktop-bin shims (pnpm -> bundled node + runner) at {directory}", flush=True)
    return directory


def stage_dsh_runtime(destination: Path, root: Path | None = None) -> None:
    root = root or find_muse_root()
    harness = harness_dir(root)
    patch = dsh_patch(root)
    if not (harness / "node_modules").is_dir():
        raise FileNotFoundError(
            f"DSH node_modules missing; run pnpm install in {harness} first."
        )
    if not patch.is_file():
        raise FileNotFoundError(f"missing {patch}")

    destination.mkdir(parents=True, exist_ok=True)
    print(f"==> Staging bundled Node into {destination / 'node'}", flush=True)
    node = stage_node(destination / "node")

    print("==> Staging DSH harness (this is the large copy)", flush=True)
    dsh = destination / "dsh"
    if dsh.exists():
        rmtree_nofollow(dsh)
    copy_tree(
        harness,
        dsh,
        exclude_names=(".git", "website", "python", "coverage", ".turbo", ".cache"),
    )

    print("==> Staging Muse packages", flush=True)
    muse_nm = dsh / "node_modules" / "@muse"
    if muse_nm.exists():
        shutil.rmtree(muse_nm)
    (destination / "packages").mkdir(parents=True, exist_ok=True)
    muse_nm.mkdir(parents=True, exist_ok=True)
    copy_dsh_packages(destination / "packages", root)
    copy_dsh_packages(muse_nm, root)
    wire_muse_node_modules(dsh, node, root)

    print("==> Staging dshmarket", flush=True)
    stage_dshmarket(dsh)

    print("==> Staging vendored plugins (bare-name patch rows)", flush=True)
    stage_vendored_dsh_plugins(destination, root)

    print("==> Staging plugin tools (bundled pnpm + pnpm-runner)", flush=True)
    stage_plugin_tools(destination)

    shutil.copy2(patch, destination / "patch.yml")
    (destination / "README.txt").write_text(
        "Muse bundled DSH runtime (Windows).\n\n"
        "Launched by Muse as:\n"
        "  node --import tsx/esm apps/cli/src/bin.ts --profile web --patch ..\\patch.yml\n"
        "User data lives in %APPDATA%\\Muse\\\n",
        encoding="utf-8",
    )
    print(f"Staged Muse runtime at {destination}", flush=True)


def assert_packed_runtime(muse_dir: Path) -> None:
    node = bundled_node_bin(muse_dir)
    patch = muse_dir / "patch.yml"
    pnpm_entry = muse_dir / "plugin-tools" / "pnpm" / "bin" / "pnpm.cjs"
    pnpm_runner = muse_dir / "plugin-tools" / "pnpm-runner.mjs"
    required = [node, patch, pnpm_entry, pnpm_runner]
    if looks_like_closure(muse_dir):
        required.extend(
            [
                closure_entry(muse_dir),
                muse_dir / "closure" / "node_modules" / "dshmarket" / "lib" / "index.js",
                muse_dir / "closure" / "node_modules" / "@muse" / "dsh-appflowy" / "package.json",
            ]
        )
        # patch.yml mounts these by bare name; missing packages mean the row
        # cannot resolve at boot (see VENDORED_PLUGIN_DIRS).
        required.extend(
            muse_dir / "closure" / "node_modules" / name / "package.json"
            for _rel, name in VENDORED_PLUGIN_DIRS
        )
        kind = "closure"
    elif looks_like_legacy_runtime(muse_dir):
        required.extend(
            [
                muse_dir / "dsh" / "apps" / "cli" / "src" / "bin.ts",
                muse_dir / "dsh" / "node_modules" / "dshmarket" / "lib" / "index.js",
            ]
        )
        required.extend(
            muse_dir / "dsh" / "node_modules" / name / "package.json"
            for _rel, name in VENDORED_PLUGIN_DIRS
        )
        kind = "legacy"
    else:
        raise FileNotFoundError(
            "packed runtime is incomplete under "
            f"{muse_dir}: neither closure nor legacy DSH CLI is present"
        )
    missing = [str(path) for path in required if not path.exists()]
    if missing:
        raise FileNotFoundError(
            "packed runtime is incomplete under "
            f"{muse_dir}: " + ", ".join(missing)
        )
    ver = run_capture([str(node), "--version"], check=True).stdout.strip()
    print(f"Bundled {ver} and {kind} DSH CLI are present.", flush=True)


def build_harness_libs(root: Path | None = None) -> None:
    """Emit DSH `lib/types` so @muse packages can tsc against package.json exports."""
    harness = harness_dir(root)
    marker = harness / "vendor" / "cordis" / "lib" / "types" / "index.d.ts"
    if marker.is_file():
        print("DSH host libraries already present", flush=True)
        return
    print("==> Building DSH host libraries", flush=True)
    run(["pnpm", "run", "build:lib:host"], cwd=harness)
    if not marker.is_file():
        raise FileNotFoundError(
            f"expected {marker} after pnpm run build:lib:host"
        )


def pnpm_install(cwd: Path) -> None:
    run(
        [
            "pnpm",
            "install",
            "--fetch-timeout",
            "1800000",
            "--network-concurrency",
            "1",
            "--fetch-retries",
            "10",
        ],
        cwd=cwd,
    )


def prepare_cloud_dart_defines(*, debug: bool = False, root: Path | None = None) -> list[str]:
    """Pin release desktop builds to the public OpenMuse Cloud endpoints.

    Debug builds keep `.env` localhost so local Cloud still works. Release packs
    without this talk to `http://localhost:8000` and production passwords fail.
    """
    if debug:
        return []
    root = root or find_muse_root()
    src = client_dir(root) / "config" / "openmuse-cloud.json"
    if not src.is_file():
        raise FileNotFoundError(f"missing cloud dart-define profile {src}")
    prepared = dist_dir(root) / "cache" / "openmuse-cloud.dart-define.json"
    prepared.parent.mkdir(parents=True, exist_ok=True)
    script = flutter_dir(root) / "tool" / "prepare_mobile_config.dart"
    dart = which("dart")
    if dart is None:
        home = flutter_home()
        dart = str(home / "bin" / ("dart.bat" if os.name == "nt" else "dart"))
    completed = run_capture([dart, str(script), str(src)], cwd=flutter_dir(root), check=True)
    text = (completed.stdout or "").strip()
    if not text.startswith("{"):
        raise RuntimeError(f"cloud profile prepare failed: {completed.stderr or text}")
    prepared.write_text(text + "\n", encoding="utf-8")
    print(f"==> Cloud dart-defines from {src}", flush=True)
    return ["--dart-define-from-file", str(prepared)]


def build_muse_packages(*, skip_tests: bool = False, root: Path | None = None) -> None:
    root = root or find_muse_root()
    print("==> Building Muse packages", flush=True)
    build_harness_libs(root)
    for rel, _short in PACKAGE_DIRS:
        pkg = packages_root(root) / Path(*rel.split("/"))
        print(f"---- {rel}", flush=True)
        pnpm_install(pkg)
        run(["pnpm", "build"], cwd=pkg)
        if skip_tests:
            continue
        manifest = pkg / "package.json"
        if not manifest.is_file():
            continue
        scripts = json.loads(manifest.read_text(encoding="utf-8")).get("scripts") or {}
        if "test" in scripts:
            run(["pnpm", "test"], cwd=pkg)


def zip_folder(source: Path, zip_path: Path) -> None:
    """Zip a tree that may contain pnpm junctions, without exploding or dropping them.

    `Path.rglob` does not recurse into reparse points, so a naive zip silently
    omits every junctioned package; following junctions naively duplicates shared
    real dirs and can loop. This walk mirrors `copy_tree`'s dedup: each real
    directory is archived once, at the first path that reaches it, keyed by the
    case-folded absolute real path.
    """
    if zip_path.exists():
        zip_path.unlink()
    archived_real: dict[str, str] = {}

    def real_key(path: Path) -> str:
        try:
            return str(path.resolve()).casefold()
        except OSError:
            return str(path).casefold()

    with zipfile.ZipFile(zip_path, "w", zipfile.ZIP_DEFLATED) as archive:
        stack: list[tuple[Path, str]] = [(source, source.name)]
        while stack:
            current, prefix = stack.pop()
            try:
                entries = os.scandir(current)
            except OSError:
                continue
            with entries:
                for entry in sorted(entries, key=lambda item: item.name):
                    src_item = Path(entry.path)
                    rel = f"{prefix}/{entry.name}"
                    try:
                        linked = entry.is_symlink() or bool(
                            entry.stat(follow_symlinks=False).st_file_attributes
                            & stat.FILE_ATTRIBUTE_REPARSE_POINT
                        )
                        is_dir = entry.is_dir(follow_symlinks=False)
                        is_file = entry.is_file(follow_symlinks=False)
                    except OSError:
                        continue
                    if linked:
                        try:
                            real = src_item.resolve()
                        except OSError:
                            continue
                        key = real_key(real)
                        if key in archived_real:
                            continue
                        if real.is_dir():
                            archived_real[key] = rel
                            stack.append((real, rel))
                        elif real.is_file():
                            archived_real[key] = rel
                            archive.write(real, rel)
                        continue
                    key = real_key(src_item)
                    if is_dir:
                        archived_real[key] = rel
                        stack.append((src_item, rel))
                    elif is_file:
                        archived_real[key] = rel
                        archive.write(src_item, rel)


def find_iscc() -> Path | None:
    env = (os.environ.get("INNO_SETUP_HOME") or os.environ.get("MUSE_ISCC") or "").strip()
    if env:
        pointed = Path(env).expanduser()
        if pointed.is_file() and pointed.name.lower() == "iscc.exe":
            return pointed
        nested = pointed / "ISCC.exe"
        if nested.is_file():
            return nested
    for base in (
        os.environ.get("ProgramFiles(x86)", r"C:\Program Files (x86)"),
        os.environ.get("ProgramFiles", r"C:\Program Files"),
        os.environ.get("LOCALAPPDATA", ""),
    ):
        if not base:
            continue
        for rel in (
            Path("Inno Setup 6") / "ISCC.exe",
            Path("Programs") / "Inno Setup 6" / "ISCC.exe",
        ):
            candidate = Path(base) / rel
            if candidate.is_file():
                return candidate
    cache = dist_dir() / "cache" / "innosetup" / "ISCC.exe"
    if cache.is_file():
        return cache
    found = which("iscc")
    return Path(found) if found else None


def ensure_iscc() -> Path | None:
    """Return ISCC.exe, downloading a portable Inno Setup if the machine has none."""
    found = find_iscc()
    if found is not None:
        return found
    cache = dist_dir() / "cache"
    cache.mkdir(parents=True, exist_ok=True)
    installer = cache / f"innosetup-{INNO_SETUP_VERSION}.exe"
    dest = cache / "innosetup"
    url = (
        f"https://github.com/jrsoftware/issrc/releases/download/"
        f"is-{INNO_SETUP_VERSION.replace('.', '_')}/innosetup-{INNO_SETUP_VERSION}.exe"
    )
    if not installer.is_file():
        try:
            _download(url, installer)
        except OSError as error:
            print(f"==> Could not download Inno Setup: {error}", flush=True)
            return None
    dest.mkdir(parents=True, exist_ok=True)
    print(f"==> Installing portable Inno Setup {INNO_SETUP_VERSION} -> {dest}", flush=True)
    completed = subprocess.run(
        [
            str(installer),
            "/PORTABLE=1",
            "/VERYSILENT",
            "/SILENT",
            "/CURRENTUSER",
            "/NORESTART",
            "/SUPPRESSMSGBOXES",
            f"/DIR={dest}",
        ],
        check=False,
    )
    iscc = dest / "ISCC.exe"
    if completed.returncode != 0 or not iscc.is_file():
        print(
            f"==> Portable Inno Setup install failed (exit {completed.returncode})",
            flush=True,
        )
        return None
    return iscc
