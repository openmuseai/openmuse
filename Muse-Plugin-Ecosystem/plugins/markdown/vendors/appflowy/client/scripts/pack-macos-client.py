#!/usr/bin/env python3
"""Build a distributable macOS OpenMuse + DSH client (install-and-run).

Default layout matches Windows plan A: bundled Node + symlink-free npm
closure inside OpenMuse.app/Contents/Resources/muse. Users do not install
Node, pnpm, or a source tree.

Usage:
  python frontend/client/scripts/pack-macos-client.py
  python frontend/client/scripts/pack-macos-client.py --debug
  python frontend/client/scripts/pack-macos-client.py --skip-app-build --skip-packages
"""
from __future__ import annotations

import argparse
import os
import platform
import re
import shutil
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
sys.path.insert(0, str(SCRIPT_DIR / "lib"))

import muse_windows as mw  # noqa: E402
from brand_config import load_brand_config  # noqa: E402


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
    parser.add_argument(
        "--skip-runtime",
        action="store_true",
        help="Copy the .app only; do not stage muse/ closure (for splitting Flutter vs sidecar builds).",
    )
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


def ensure_toolchain() -> None:
    home = mw.flutter_home()
    pub_bin = Path.home() / ".pub-cache" / "bin"
    os.environ["FLUTTER_HOME"] = str(home)
    os.environ["PATH"] = os.pathsep.join(
        [str(pub_bin), str(home / "bin"), os.environ.get("PATH", "")]
    )
    mw.require_flutter_327()
    if shutil.which("protoc-gen-dart") is None:
        raise FileNotFoundError(
            "protoc-gen-dart is not on PATH (expected in $HOME/.pub-cache/bin)"
        )


def app_version(flutter_app: Path) -> str:
    text = (flutter_app / "pubspec.yaml").read_text(encoding="utf-8")
    match = re.search(r"^version:\s*([0-9]+\.[0-9]+\.[0-9]+)", text, re.M)
    return match.group(1) if match else "0.0.0"


def codesign_app(app: Path, exe_name: str, entitlements: Path | None = None) -> None:
    """Ad-hoc sign nested Mach-O then the bundle.

    The Flutter main executable is built with Hardened Runtime. Nested CocoaPods
    frameworks are only ad-hoc *without* runtime. After a download, macOS
    translocates the app and dyld library validation kills it
    (`CwlCatchException.framework` signature not valid). Sign innermost first
    with `--options runtime`, then the bundle with Release entitlements that
    include `disable-library-validation`.
    """
    exe = app / "Contents" / "MacOS" / exe_name
    if not exe.is_file():
        raise FileNotFoundError(f"cannot codesign, missing {exe}")

    nested: list[Path] = []
    frameworks = app / "Contents" / "Frameworks"
    if frameworks.is_dir():
        nested.extend(sorted(frameworks.glob("*.dylib")))
        nested.extend(sorted(frameworks.glob("**/*.xpc")))
        nested.extend(
            p for p in sorted(frameworks.glob("**/*.app")) if p.resolve() != app.resolve()
        )
        nested.extend(sorted(frameworks.glob("*.framework")))
    for target in nested:
        mw.run(
            [
                "codesign",
                "--force",
                "-s",
                "-",
                "--options",
                "runtime",
                "--timestamp=none",
                str(target),
            ]
        )
    cmd = [
        "codesign",
        "--force",
        "-s",
        "-",
        "--options",
        "runtime",
        "--timestamp=none",
    ]
    if entitlements is not None and entitlements.is_file():
        cmd.extend(["--entitlements", str(entitlements)])
    mw.run(cmd + [str(exe)])
    mw.run(cmd + [str(app)])
    mw.run(["codesign", "--verify", "--deep", "--strict", str(app)])


def dir_size_bytes(path: Path) -> int:
    total = 0
    for root, _dirs, files in os.walk(path, followlinks=False):
        for name in files:
            file = Path(root) / name
            try:
                total += file.stat().st_size
            except OSError:
                continue
    return total


def human_mb(nbytes: int) -> str:
    return f"{nbytes / (1024 * 1024):.1f} MB"


def stage_legacy_runtime(muse_res: Path) -> None:
    script = mw.find_muse_root() / "middlewares" / "scripts" / "stage-dsh-runtime.sh"
    if not script.is_file():
        raise FileNotFoundError(f"missing {script}")
    mw.run(["bash", str(script), str(muse_res)])


def ensure_closure(source: Path, *, rebuild: bool) -> None:
    entry = source / "node_modules" / "@deepseek-ai" / "dsh" / "lib" / "bin.js"
    if rebuild or not entry.is_file():
        print(f"==> Building Muse closure at {source}", flush=True)
        script = SCRIPT_DIR / "build-muse-closure.py"
        cmd = [sys.executable, str(script), "--out", str(source)]
        if (source / "tarballs").is_dir() and not rebuild:
            cmd.append("--skip-pack")
        mw.run(cmd, cwd=SCRIPT_DIR)


def main() -> int:
    args = parse_args()
    ensure_toolchain()
    root = mw.find_muse_root()
    os.environ["MUSE_ROOT"] = str(root)
    frontend = mw.frontend_dir(root)
    flutter_app = mw.flutter_dir(root)
    brand = load_brand_config()
    out = mw.dist_dir(root) / "macos"
    app_name = brand.macos_app_bundle
    packed = out / app_name
    out.mkdir(parents=True, exist_ok=True)
    version = app_version(flutter_app)
    machine = platform.machine()

    if not args.skip_packages:
        mw.build_muse_packages(skip_tests=args.skip_tests, root=root)

    if args.debug:
        profile = "development-mac-arm64" if machine == "arm64" else "development-mac-x86_64"
        build_flag = "debug"
        product_dir = flutter_app / "build" / "macos" / "Build" / "Products" / "Debug" / app_name
        core_task = "appflowy-core-dev"
    else:
        profile = "production-mac-arm64" if machine == "arm64" else "production-mac-x86_64"
        build_flag = "release"
        product_dir = (
            flutter_app / "build" / "macos" / "Build" / "Products" / "Release" / app_name
        )
        core_task = "appflowy-core-release"

    if not args.skip_app_build:
        print(f"==> Building {brand.name_en} macOS ({profile}, {build_flag})", flush=True)
        if not args.skip_core_build:
            mw.run(["cargo", "make", "--profile", profile, core_task], cwd=frontend)
        else:
            print("==> Skipping cargo-make (reusing dart_ffi)", flush=True)
        mw.run(["flutter", "pub", "get"], cwd=flutter_app)
        flutter_cmd = ["flutter", "build", "macos", f"--{build_flag}"]
        flutter_cmd.extend(mw.prepare_cloud_dart_defines(debug=args.debug, root=root))
        mw.run(flutter_cmd, cwd=flutter_app)

    if not product_dir.is_dir():
        raise FileNotFoundError(
            f"expected app missing: {product_dir}. Build it first or omit --skip-app-build."
        )

    print(f"==> Copying {product_dir} → {packed}", flush=True)
    if args.skip_runtime:
        if packed.exists():
            shutil.rmtree(packed)
        mw.run(["ditto", str(product_dir), str(packed)])
        print("==> Skipping muse runtime staging", flush=True)
        print(f"Staged app (no sidecar): {packed}")
        return 0

    muse_res = packed / "Contents" / "Resources" / "muse"
    has_runtime = (muse_res / "dsh").is_dir() or (muse_res / "closure").is_dir()
    saved: Path | None = None
    if args.reuse_runtime and packed.is_dir() and has_runtime:
        saved = Path(
            subprocess.check_output(["mktemp", "-d", f"{os.environ.get('TMPDIR', '/tmp')}/muse-runtime.XXXXXX"])
            .decode()
            .strip()
        )
        shutil.move(str(muse_res), str(saved / "muse"))

    if packed.exists():
        shutil.rmtree(packed)
    mw.run(["ditto", str(product_dir), str(packed)])

    muse_res = packed / "Contents" / "Resources" / "muse"
    if saved is not None:
        if muse_res.exists():
            shutil.rmtree(muse_res)
        shutil.move(str(saved / "muse"), str(muse_res))
        shutil.rmtree(saved, ignore_errors=True)
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
            mw.wire_muse_node_modules(muse_res / "dsh", mw.bundled_node_bin(muse_res), root)
            mw.stage_dshmarket(muse_res / "dsh")
            mw.stage_plugin_tools(muse_res)
    elif args.closure:
        closure_source = (
            args.closure_source.resolve()
            if args.closure_source
            else root / "tmp" / "muse-closure"
        )
        ensure_closure(closure_source, rebuild=args.rebuild_closure)
        print(f"==> Staging closure runtime from {closure_source}", flush=True)
        mw.stage_closure(closure_source, muse_res, patch=mw.dsh_patch(root))
    else:
        print("==> Staging legacy whole-tree DSH runtime", flush=True)
        stage_legacy_runtime(muse_res)

    # Closure builds carry VENDORED_PLUGIN_DIRS in their npm manifest; make the
    # reused-runtime paths match so patch.yml's bare-name rows always resolve.
    mw.stage_vendored_dsh_plugins(muse_res, root)

    print("==> Ad-hoc codesign (nested frameworks + disable-library-validation)", flush=True)
    codesign_app(
        packed,
        brand.macos_app,
        entitlements=flutter_app / "macos" / "Runner" / "Release.entitlements",
    )
    mw.assert_packed_runtime(muse_res)

    zip_compat = out / f"{brand.artifact_prefix}-macos.zip"
    zip_catalog = out / f"{brand.artifact_prefix}-{version}-macos-universal.zip"
    if not args.skip_zip:
        print(f"==> Zipping {zip_catalog}", flush=True)
        for path in (zip_compat, zip_catalog):
            if path.exists():
                path.unlink()
        mw.run(["ditto", "-c", "-k", "--keepParent", str(packed), str(zip_catalog)])
        if zip_catalog.resolve() != zip_compat.resolve():
            shutil.copy2(zip_catalog, zip_compat)
    else:
        print("==> Skipping zip", flush=True)

    if shutil.which("create-dmg"):
        print("==> Creating DMG", flush=True)
        dmg = out / f"{brand.macos_app}.dmg"
        if dmg.exists():
            dmg.unlink()
        mw.run(
            [
                "create-dmg",
                "--overwrite",
                "--dmg-title",
                brand.name_en,
                str(packed),
                str(out),
            ]
        )

    app_bytes = dir_size_bytes(packed)
    zip_bytes = zip_catalog.stat().st_size if zip_catalog.is_file() else 0
    print()
    print("Distributable client:")
    print(f"  {packed}  ({human_mb(app_bytes)})")
    if zip_catalog.is_file():
        print(f"  {zip_catalog}  ({human_mb(zip_bytes)})")
        print(f"  {zip_compat}")
    print(f"Install: copy {app_name} to /Applications (or run it from dist/macos).")
    print("First launch: enter DEEPSEEK_API_KEY in the DeepSeek panel.")
    print(f"SIZE app={human_mb(app_bytes)} zip={human_mb(zip_bytes)}")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except subprocess.CalledProcessError as error:
        raise SystemExit(error.returncode) from error
    except (FileNotFoundError, RuntimeError) as error:
        print(error, file=sys.stderr)
        raise SystemExit(1) from error
