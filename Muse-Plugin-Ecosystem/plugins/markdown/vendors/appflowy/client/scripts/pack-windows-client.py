#!/usr/bin/env python3
"""Build a distributable Windows Muse + DSH client.

Usage:
  python frontend/client/scripts/pack-windows-client.py
  python frontend/client/scripts/pack-windows-client.py --debug
  python frontend/client/scripts/pack-windows-client.py --debug --skip-app-build --skip-packages

Output:
  dist/windows/Muse/
  dist/windows/Muse-windows-x64.zip
  dist/windows/Muse-windows-x64-setup.exe  (if Inno Setup is installed)
"""
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR / "lib"))

import muse_windows as mw  # noqa: E402
from brand_config import load_brand_config  # noqa: E402


def overlay_flutter_product(product_dir: Path, portable: Path) -> None:
    """Copy Flutter bits into the portable dir without touching muse/."""
    portable.mkdir(parents=True, exist_ok=True)
    for item in product_dir.iterdir():
        if item.name.casefold() == "muse":
            continue
        dest = portable / item.name
        if dest.exists() or dest.is_symlink():
            if dest.is_dir() and not dest.is_symlink():
                mw.rmtree_nofollow(dest)
            else:
                dest.unlink()
        if item.is_dir():
            shutil.copytree(item, dest)
        else:
            shutil.copy2(item, dest)


HELIX_RUNTIME_RELATIVE = (
    Path("data")
    / "flutter_assets"
    / "assets"
    / "engines"
    / "helix"
    / "runtime"
)


def sync_helix_runtime_assets(flutter_app: Path, product_dir: Path) -> None:
    """Complete the bundled Helix runtime inside the built product.

    `flutter build` copies only the files that sit directly in a declared asset
    directory, so the per-language `runtime/queries/<language>/` trees of the
    Helix engine never reach the release bundle. HelixInstall.resolve() checks
    for `runtime/queries` and, when it is absent, extracts the 29 MB
    `runtime.tar` into a brand new temp directory on *every* file open: seconds
    of latency and ~56 MB of temp files per open, cleaned up only by the OS.
    Mirroring the source runtime over the bundled one keeps the resolver on its
    bundled-runtime fast path.
    """
    source = flutter_app / "assets" / "engines" / "helix" / "runtime"
    target = product_dir / HELIX_RUNTIME_RELATIVE
    if not source.is_dir():
        print(f"==> Helix runtime assets missing at {source}; skipping mirror", flush=True)
        return
    if not target.is_dir():
        print(f"==> Bundled Helix runtime missing at {target}; skipping mirror", flush=True)
        return

    copied = 0
    for root_dir, _dirs, files in os.walk(source):
        relative = Path(root_dir).relative_to(source)
        destination = target / relative
        destination.mkdir(parents=True, exist_ok=True)
        for name in files:
            shutil.copy2(Path(root_dir) / name, destination / name)
            copied += 1

    print(
        f"==> Mirrored {copied} Helix runtime files into {target}",
        flush=True,
    )


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--debug", action="store_true")
    parser.add_argument("--skip-app-build", action="store_true")
    parser.add_argument(
        "--skip-core-build",
        action="store_true",
        help="Skip cargo-make (Rust dart_ffi); still run flutter build unless --skip-app-build.",
    )
    parser.add_argument("--skip-packages", action="store_true")
    parser.add_argument("--skip-tests", action="store_true")
    parser.add_argument("--reuse-runtime", action="store_true")
    parser.add_argument("--skip-zip", action="store_true")
    parser.add_argument("--skip-installer", action="store_true")
    parser.add_argument(
        "--closure",
        action=argparse.BooleanOptionalAction,
        default=True,
        help="Stage the symlink-free npm closure (default). --no-closure uses the legacy whole-tree copy.",
    )
    parser.add_argument(
        "--closure-source",
        type=Path,
        default=None,
        help="build-muse-closure.py --out directory (default: <repo>/tmp/muse-closure).",
    )
    parser.add_argument(
        "--rebuild-closure",
        action="store_true",
        help="Run build-muse-closure.py even if --closure-source already has node_modules.",
    )
    return parser.parse_args()


def main() -> int:
    args = parse_args()
    mw.require_flutter_327()
    root = mw.find_muse_root()
    os.environ["MUSE_ROOT"] = str(root)
    frontend = mw.frontend_dir(root)
    flutter_app = mw.flutter_dir(root)
    out = mw.dist_dir(root) / "windows"
    brand = load_brand_config()
    portable = out / brand.binary_windows
    out.mkdir(parents=True, exist_ok=True)

    if not args.skip_packages:
        mw.build_muse_packages(skip_tests=args.skip_tests, root=root)

    if args.debug:
        profile = "development-windows-x86"
        build_flag = "debug"
        product_dir = flutter_app / "build" / "windows" / "x64" / "runner" / "Debug"
        core_task = "appflowy-core-dev"
    else:
        profile = "production-windows-x86"
        build_flag = "release"
        product_dir = flutter_app / "build" / "windows" / "x64" / "runner" / "Release"
        core_task = "appflowy-core-release"

    if not args.skip_app_build:
        print(f"==> Building {brand.name_en} Windows ({profile}, {build_flag})", flush=True)
        if not args.skip_core_build:
            mw.run_vs(f"cargo make --profile {profile} {core_task}", cwd=frontend)
        else:
            print("==> Skipping cargo-make (reusing dart_ffi)", flush=True)
        mw.run_vs("call flutter pub get", cwd=flutter_app)
        win_cmd = f"call flutter build windows --{build_flag}"
        defines = mw.prepare_cloud_dart_defines(debug=args.debug, root=root)
        if defines:
            win_cmd += " " + " ".join(defines)
        mw.run_vs(win_cmd, cwd=flutter_app)

    exe = product_dir / brand.windows_exe
    if not exe.is_file():
        raise FileNotFoundError(
            f"expected app missing: {exe}. Build it first or omit --skip-app-build."
        )

    sync_helix_runtime_assets(flutter_app, product_dir)

    print(f"==> Copying {product_dir} -> {portable}", flush=True)
    muse_res = portable / "muse"
    has_runtime = (muse_res / "dsh").is_dir() or (muse_res / "closure").is_dir()
    if args.reuse_runtime and has_runtime:
        print("==> Overlaying Flutter product, keeping muse/", flush=True)
        overlay_flutter_product(product_dir, portable)
        shutil.copy2(mw.dsh_patch(root), muse_res / "patch.yml")
        if mw.looks_like_closure(muse_res):
            print("==> Reused closure runtime (refreshed patch.yml)", flush=True)
            mw.overlay_dsh_appflowy_dist(muse_res, root)
        else:
            print("==> Refreshing Muse packages in reused runtime", flush=True)
            muse_nm = muse_res / "dsh" / "node_modules" / "@muse"
            if muse_nm.exists():
                shutil.rmtree(muse_nm)
            (muse_res / "packages").mkdir(parents=True, exist_ok=True)
            muse_nm.mkdir(parents=True, exist_ok=True)
            mw.copy_dsh_packages(muse_res / "packages", root)
            mw.copy_dsh_packages(muse_nm, root)
            mw.wire_muse_node_modules(muse_res / "dsh", muse_res / "node" / "node.exe", root)
            mw.stage_dshmarket(muse_res / "dsh")
            mw.stage_plugin_tools(muse_res)
    else:
        if portable.exists():
            mw.rmtree_nofollow(portable)
        shutil.copytree(product_dir, portable)
        if args.closure:
            closure_source = (
                args.closure_source.resolve()
                if args.closure_source
                else root / "tmp" / "muse-closure"
            )
            entry = (
                closure_source / "node_modules" / "@deepseek-ai" / "dsh" / "lib" / "bin.js"
            )
            if args.rebuild_closure or not entry.is_file():
                print(f"==> Building Muse closure at {closure_source}", flush=True)
                script = SCRIPT_DIR / "build-muse-closure.py"
                cmd = [sys.executable, str(script), "--out", str(closure_source)]
                if (closure_source / "tarballs").is_dir() and not args.rebuild_closure:
                    cmd.append("--skip-pack")
                mw.run(cmd, cwd=SCRIPT_DIR)
            print(f"==> Staging closure runtime from {closure_source}", flush=True)
            mw.stage_closure(
                closure_source,
                muse_res,
                patch=mw.dsh_patch(root),
            )
        else:
            mw.stage_dsh_runtime(muse_res, root)

    mw.stage_vendored_dsh_plugins(muse_res, root)
    mw.assert_packed_runtime(muse_res)

    zip_path = out / f"{brand.artifact_prefix}-windows-x64.zip"
    if not args.skip_zip:
        print(f"==> Zipping {zip_path}", flush=True)
        mw.zip_folder(portable, zip_path)
    else:
        print("==> Skipping zip", flush=True)

    if not args.skip_installer:
        iscc = mw.ensure_iscc()
        if iscc is None:
            print(
                "==> Skipping installer (ISCC.exe not found). "
                "Install Inno Setup 6 or set INNO_SETUP_HOME to emit a setup.exe.",
                flush=True,
            )
        else:
            iss_src = frontend / "scripts" / "windows_installer" / "inno_setup_config.iss"
            shutil.copy2(iss_src, out / "inno_setup_config.iss")
            icon_src = flutter_app / "windows" / "runner" / "resources" / "app_icon.ico"
            if icon_src.is_file():
                shutil.copy2(icon_src, out / "flowy_logo.ico")
            print(f"==> Building installer with {iscc}", flush=True)
            setup_name = f"{brand.artifact_prefix}-windows-x64-setup"
            mw.run(
                [
                    str(iscc),
                    f"/F{setup_name}",
                    "inno_setup_config.iss",
                    "/DAppVersion=0.11.4",
                ],
                cwd=out,
            )
            built = out / "Output" / f"{setup_name}.exe"
            dest = out / f"{setup_name}.exe"
            if built.is_file():
                shutil.move(str(built), str(dest))
            print(f"Installer: {dest}", flush=True)

    print()
    print("Distributable client:")
    print(f"  {portable}")
    print(f"  {zip_path}")
    print(f"Install: unzip and run {brand.windows_exe}, or run the setup.exe if built.")
    print("First launch: enter DEEPSEEK_API_KEY in the DeepSeek panel.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except subprocess.CalledProcessError as error:
        raise SystemExit(error.returncode) from error
    except (FileNotFoundError, RuntimeError) as error:
        print(error, file=sys.stderr)
        raise SystemExit(1) from error
