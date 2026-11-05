#!/usr/bin/env python3
"""Upload a verified Android APK to production /downloads/ (merge, no wipe).

Does not run Cloud deploy-infra.sh. Catalog already lists
/downloads/android/OpenMuse-{version}-android-arm64.apk; this only replaces the
file on the server.

Usage (after packing and installing on a phone):
  python frontend/client/scripts/push-android-client.py
  python frontend/client/scripts/push-android-client.py --apk path/to.apk
"""
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
CLIENT_DIR = SCRIPT_DIR.parent
WORKSPACE = CLIENT_DIR.parent.parent.parent  # openmuse-io
WEBSITE = WORKSPACE / "Muse-WebSite"


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--apk", type=Path, default=None)
    parser.add_argument(
        "--env-file",
        type=Path,
        default=WORKSPACE / "scripts" / "deploy.env",
        help="gitignored deploy.env (OPENMUSE_DEPLOY_*)",
    )
    parser.add_argument("--skip-verify", action="store_true")
    return parser.parse_args()


def find_apk(explicit: Path | None) -> Path:
    if explicit is not None:
        path = explicit.resolve()
        if not path.is_file():
            raise FileNotFoundError(path)
        return path
    android = CLIENT_DIR / "dist" / "android"
    catalog = sorted(
        android.glob("OpenMuse-*-android-arm64.apk"),
        key=lambda p: p.stat().st_mtime,
    )
    if catalog:
        return catalog[-1]
    fallback = android / "OpenMuse-android-release.apk"
    if fallback.is_file():
        return fallback
    raise SystemExit(f"no APK in {android}; run pack-android-client.py first")


def catalog_name(apk: Path) -> str:
    if apk.name.endswith("-android-arm64.apk"):
        return apk.name
    import re

    pubspec = CLIENT_DIR / "frontend" / "appflowy_flutter" / "pubspec.yaml"
    match = re.search(r"^version:\s*([0-9]+\.[0-9]+\.[0-9]+)", pubspec.read_text(), re.M)
    version = match.group(1) if match else "0.11.4"
    return f"OpenMuse-{version}-android-arm64.apk"


def main() -> int:
    args = parse_args()
    apk = find_apk(args.apk)
    if not args.skip_verify:
        subprocess.check_call([sys.executable, str(SCRIPT_DIR / "verify-android-apk.py"), str(apk)])
    if not args.env_file.is_file():
        raise SystemExit(f"missing {args.env_file} (copy scripts/deploy.env.example)")
    if not (WEBSITE / "scripts" / "deploy.py").is_file():
        raise SystemExit(f"Muse-WebSite deploy.py not found at {WEBSITE}")

    public_name = catalog_name(apk)
    dest_dir = WEBSITE / "public" / "downloads" / "android"
    dest_dir.mkdir(parents=True, exist_ok=True)
    dest = dest_dir / public_name
    shutil.copy2(apk, dest)
    print(f"==> Staged {dest} ({dest.stat().st_size / (1024 * 1024):.1f} MB)", flush=True)

    downloads = WEBSITE / "public" / "downloads"
    bak = Path(tempfile.mkdtemp(prefix="om-dl-bak."))
    parked: list[str] = []
    try:
        for name in ("macos", "ios", "windows", "linux"):
            src = downloads / name
            if src.exists():
                shutil.move(str(src), bak / name)
                parked.append(name)
        env = os.environ.copy()
        env["OPENMUSE_DEPLOY_ENV_FILE"] = str(args.env_file.resolve())
        print("==> Uploading android/ only (merge on server)", flush=True)
        subprocess.check_call(
            [sys.executable, "-u", str(WEBSITE / "scripts" / "deploy.py"), "--env", "production", "downloads"],
            cwd=str(WEBSITE),
            env=env,
        )
    finally:
        for name in parked:
            shutil.move(str(bak / name), downloads / name)
        shutil.rmtree(bak, ignore_errors=True)

    url = f"https://openmuseai.com/downloads/android/{public_name}"
    print(f"Public URL: {url}")
    print("HEAD after deploy: curl -sI " + url)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
