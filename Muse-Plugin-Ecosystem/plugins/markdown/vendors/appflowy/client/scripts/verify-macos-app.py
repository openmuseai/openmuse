#!/usr/bin/env python3
"""Verify a packed macOS .app is out-of-box ready (closure + bundled Node).

Usage:
  python frontend/client/scripts/verify-macos-app.py
  python frontend/client/scripts/verify-macos-app.py --root dist/macos/OpenMuse.app
"""
from __future__ import annotations

import argparse
import importlib.util
import shutil
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR / "lib"))

import muse_windows as mw  # noqa: E402
from brand_config import load_brand_config  # noqa: E402


def _load_windows_verify():
    path = SCRIPT_DIR / "verify-windows-portable.py"
    spec = importlib.util.spec_from_file_location("verify_windows_portable", path)
    if spec is None or spec.loader is None:
        raise RuntimeError(f"cannot load {path}")
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=None)
    parser.add_argument("--port", default="3092")
    parser.add_argument("--skip-boot", action="store_true")
    parser.add_argument("--skip-plugin", action="store_true")
    parser.add_argument("--package", default="picocolors@1.1.1")
    return parser.parse_args()


def verify(app: Path, *, port: str, skip_boot: bool, skip_plugin: bool, package: str) -> None:
    vwp = _load_windows_verify()
    muse = app / "Contents" / "Resources" / "muse"
    exe = app / "Contents" / "MacOS" / load_brand_config().macos_app
    if not exe.is_file():
        raise FileNotFoundError(f"missing {exe}")
    mw.assert_packed_runtime(muse)
    if not mw.looks_like_closure(muse):
        raise RuntimeError(f"{muse} is not a closure layout; out-of-box A requires closure/")

    dsh_home = mw.find_muse_root() / "tmp" / "verify-dsh-home-macos"
    if dsh_home.exists():
        shutil.rmtree(dsh_home, ignore_errors=True)
    dsh_home.mkdir(parents=True, exist_ok=True)

    copied = vwp._seed_closure_plugins(muse, dsh_home)
    print(f"seeded {copied} packages into {dsh_home}", flush=True)
    muse_nm = dsh_home / "profiles" / "web" / "node_modules" / "@muse"
    if not muse_nm.is_dir():
        raise RuntimeError("seed did not materialize profiles/web/node_modules/@muse")

    # Bare-name rows in patch.yml are resolved from the seeded profile
    # node_modules, so a package that is packed but never seeded mounts nothing.
    # assert_packed_runtime already proved the closure copy exists.
    profile_nm = dsh_home / "profiles" / "web" / "node_modules"
    patch_text = (muse / "patch.yml").read_text(encoding="utf-8")
    for _rel, name in mw.VENDORED_PLUGIN_DIRS:
        seeded = profile_nm / name / "package.json"
        if not seeded.is_file():
            raise RuntimeError(f"patch row {name} is not seeded: missing {seeded}")
        # YAML may quote scoped names (`name: '@muse/...'`).
        mounted = (
            f"name: {name}" in patch_text
            or f"name: '{name}'" in patch_text
            or f'name: "{name}"' in patch_text
        )
        if not mounted:
            raise RuntimeError(f"{muse / 'patch.yml'} does not mount {name}")
        print(f"vendored plugin seeded + mounted: {name}", flush=True)

    if not skip_boot:
        lines = vwp._boot(muse, dsh_home, port)
        print(f"BOOT PASS on 127.0.0.1:{port} ({len(lines)} log lines)", flush=True)
    if not skip_plugin:
        vwp._plugin_add(muse, dsh_home, package)
        print(f"PLUGIN PASS add {package}", flush=True)
    print("OUT-OF-BOX PASS", app)


def main() -> int:
    args = parse_args()
    brand = load_brand_config()
    root = args.root
    if root is None:
        root = mw.dist_dir() / "macos" / brand.macos_app_bundle
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
