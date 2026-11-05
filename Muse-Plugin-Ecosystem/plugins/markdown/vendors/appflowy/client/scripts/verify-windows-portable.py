#!/usr/bin/env python3
"""Verify a packed / installed Muse tree is out-of-box ready.

Checks:
  1. Flutter exe + muse closure layout (no source-tree MUSE_ROOT required)
  2. Seed @muse + dshmarket into an isolated DSH_HOME (same as the sidecar)
  3. Boot `node <closure>/lib/bin.js web --patch patch.yml` until HTTP ready
  4. Optional: `dsh plugin --profile web add picocolors` through bundled pnpm

Usage:
  python frontend/client/scripts/verify-windows-portable.py
  python frontend/client/scripts/verify-windows-portable.py --root "dist/windows/Muse"
  python frontend/client/scripts/verify-windows-portable.py --skip-plugin
"""
from __future__ import annotations

import argparse
import http.client
import json
import os
import shutil
import subprocess
import sys
import time
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR / "lib"))

import muse_windows as mw  # noqa: E402
from brand_config import load_brand_config  # noqa: E402


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--root",
        type=Path,
        default=None,
        help="Portable or install directory that contains the app exe + muse/.",
    )
    parser.add_argument("--port", default="3091")
    parser.add_argument("--skip-boot", action="store_true")
    parser.add_argument("--skip-plugin", action="store_true")
    parser.add_argument(
        "--package",
        default="picocolors@1.1.1",
        help="Community plugin to one-click install (default: picocolors@1.1.1).",
    )
    return parser.parse_args()


def _host_owned_seed_package(spec: str) -> bool:
    return spec.startswith("@deepseek-ai/")


def _closure_seed_generation(muse: Path) -> str:
    dsh = muse / "closure" / "node_modules" / "@deepseek-ai" / "dsh" / "package.json"
    version = ""
    if dsh.is_file():
        try:
            data = json.loads(dsh.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            data = {}
        if isinstance(data, dict) and isinstance(data.get("version"), str):
            version = data["version"]
    connector = (
        muse / "closure" / "node_modules" / "@muse" / "dsh-appflowy" / "dist" / "src" / "connector.js"
    )
    stamp = int(connector.stat().st_mtime * 1000) if connector.is_file() else 0
    return f"3\t{muse}\t{version}\t{stamp}"


def _seed_closure_plugins(muse: Path, dsh_home: Path) -> int:
    """Mirror DshSidecar.seedClosurePlugins: real copies, no host @deepseek-ai."""
    marker = dsh_home / "profiles" / "web" / ".muse-seeded"
    generation = _closure_seed_generation(muse)
    if marker.is_file() and marker.read_text(encoding="utf-8") == generation:
        return 0
    closure_nm = muse / "closure" / "node_modules"
    dest_roots = [
        dsh_home / "profiles" / "web" / "node_modules",
    ]
    for dest_root in dest_roots:
        leaked_host = dest_root / "@deepseek-ai"
        if leaked_host.exists() or leaked_host.is_symlink():
            mw.rmtree_nofollow(leaked_host)
    roots = ["dshmarket", "dsh-model-capabilities"]
    muse_scope = closure_nm / "@muse"
    if muse_scope.is_dir():
        roots.extend(f"@muse/{entry.name}" for entry in sorted(muse_scope.iterdir()) if entry.is_dir())
    seen: set[str] = set()
    queue = list(roots)
    copied = 0
    while queue:
        spec = queue.pop()
        if spec in seen:
            continue
        seen.add(spec)
        if _host_owned_seed_package(spec):
            continue
        source = closure_nm / spec
        manifest = source / "package.json"
        if not manifest.is_file():
            continue
        for dest_root in dest_roots:
            dest = dest_root / spec
            if dest.exists() or dest.is_symlink():
                mw.rmtree_nofollow(dest)
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copytree(source, dest, dirs_exist_ok=False)
        copied += 1
        try:
            data = json.loads(manifest.read_text(encoding="utf-8"))
        except json.JSONDecodeError:
            continue
        for kind in ("dependencies", "peerDependencies", "optionalDependencies"):
            deps = data.get(kind) or {}
            if not isinstance(deps, dict):
                continue
            for name in deps:
                if _host_owned_seed_package(name):
                    continue
                if (closure_nm / name / "package.json").is_file():
                    queue.append(name)
    marker.parent.mkdir(parents=True, exist_ok=True)
    marker.write_text(generation, encoding="utf-8")
    return copied


def _http_ready(port: str) -> bool:
    try:
        conn = http.client.HTTPConnection("127.0.0.1", int(port), timeout=2)
        conn.request("GET", "/")
        resp = conn.getresponse()
        resp.read()
        conn.close()
        return resp.status < 500
    except Exception:
        return False


def _boot(muse: Path, dsh_home: Path, port: str) -> list[str]:
    node = mw.bundled_node_bin(muse)
    entry = mw.closure_entry(muse)
    patch = muse / "patch.yml"
    env = os.environ.copy()
    env["DSH_HOME"] = str(dsh_home)
    env["DEEPSEEK_API_KEY"] = env.get("DEEPSEEK_API_KEY") or "dummy-oob-key"
    env["NODE_PATH"] = str(muse / "closure" / "node_modules")
    shims = mw.write_desktop_bin_shims(dsh_home, node, muse / "plugin-tools")
    env["PATH"] = os.pathsep.join([str(shims), str(node.parent), env.get("PATH", "")])
    env["npm_config_side_effects_cache"] = "false"
    env["PNPM_CONFIG_SIDE_EFFECTS_CACHE"] = "false"
    cmd = [
        str(node),
        str(entry),
        "web",
        "--patch",
        str(patch),
        "--host",
        "127.0.0.1",
        "--port",
        port,
    ]
    log_path = dsh_home / "boot.log"
    log_file = log_path.open("wb")
    proc = subprocess.Popen(
        cmd,
        cwd=str(muse),
        env=env,
        stdout=log_file,
        stderr=subprocess.STDOUT,
    )
    ready = False
    deadline = time.time() + 180
    try:
        while time.time() < deadline:
            if proc.poll() is not None:
                break
            if _http_ready(port):
                ready = True
                break
            time.sleep(0.4)
    finally:
        proc.kill()
        try:
            proc.wait(timeout=10)
        except Exception:
            pass
        log_file.close()
    text = log_path.read_text(encoding="utf-8", errors="replace") if log_path.is_file() else ""
    lines = text.splitlines()
    if not ready:
        tail = "\n".join(lines[-40:])
        raise RuntimeError(f"closure sidecar did not become ready on :{port}\n{tail}")
    return lines


def _plugin_add(muse: Path, dsh_home: Path, package: str) -> None:
    node = mw.bundled_node_bin(muse)
    entry = mw.closure_entry(muse)
    env = os.environ.copy()
    env["DSH_HOME"] = str(dsh_home)
    env["DEEPSEEK_API_KEY"] = env.get("DEEPSEEK_API_KEY") or "dummy-oob-key"
    shims = dsh_home / ".desktop-bin"
    env["PATH"] = os.pathsep.join([str(shims), str(node.parent), env.get("PATH", "")])
    env["npm_config_side_effects_cache"] = "false"
    env["PNPM_CONFIG_SIDE_EFFECTS_CACHE"] = "false"
    completed = subprocess.run(
        [str(node), str(entry), "plugin", "--profile", "web", "add", package],
        cwd=str(muse),
        env=env,
        capture_output=True,
        text=True,
        encoding="utf-8",
        errors="replace",
    )
    if completed.returncode != 0:
        raise RuntimeError(
            f"plugin add {package} failed ({completed.returncode})\n"
            f"{completed.stdout[-2000:]}\n{completed.stderr[-2000:]}"
        )
    name = package.split("@", 1)[0]
    landed = dsh_home / "profiles" / "web" / "node_modules" / name
    if not landed.exists():
        raise RuntimeError(f"plugin add reported success but {landed} is missing")


def verify(root: Path, *, port: str, skip_boot: bool, skip_plugin: bool, package: str) -> None:
    brand = load_brand_config()
    exe = root / brand.windows_exe
    muse = root / "muse"
    if not exe.is_file():
        raise FileNotFoundError(f"missing {exe}")
    mw.assert_packed_runtime(muse)
    if not mw.looks_like_closure(muse):
        raise RuntimeError(f"{muse} is not a closure layout; out-of-box A requires closure/")

    dsh_home = mw.find_muse_root() / "tmp" / "verify-dsh-home"
    if dsh_home.exists():
        shutil.rmtree(dsh_home, ignore_errors=True)
    dsh_home.mkdir(parents=True, exist_ok=True)

    copied = _seed_closure_plugins(muse, dsh_home)
    print(f"seeded {copied} packages into {dsh_home}", flush=True)
    muse_nm = dsh_home / "profiles" / "web" / "node_modules" / "@muse"
    if not muse_nm.is_dir():
        raise RuntimeError("seed did not materialize profiles/web/node_modules/@muse")

    if not skip_boot:
        lines = _boot(muse, dsh_home, port)
        print(f"BOOT PASS on 127.0.0.1:{port} ({len(lines)} log lines)", flush=True)
    if not skip_plugin:
        _plugin_add(muse, dsh_home, package)
        print(f"PLUGIN PASS add {package}", flush=True)
    print("OUT-OF-BOX PASS", root)


def main() -> int:
    args = parse_args()
    brand = load_brand_config()
    root = args.root
    if root is None:
        root = mw.dist_dir() / "windows" / brand.binary_windows
    verify(
        root.resolve(),
        port=args.port,
        skip_boot=args.skip_boot,
        skip_plugin=args.skip_plugin,
        package=args.package,
    )
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (FileNotFoundError, RuntimeError) as error:
        print(error, file=sys.stderr)
        raise SystemExit(1) from error
