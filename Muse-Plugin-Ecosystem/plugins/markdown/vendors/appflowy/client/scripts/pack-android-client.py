#!/usr/bin/env python3
"""Build a production Android APK for OpenMuse (arm64-v8a).

Android does not bundle Node/DSH. The APK talks to public Cloud and
loads session/open webUrl under https://openmuseai.com/u/<hash>/
(see config/openmuse-cloud.json).

Usage (from Muse-Clients):
  python frontend/client/scripts/pack-android-client.py
  python frontend/client/scripts/pack-android-client.py --skip-core --skip-packages
"""
from __future__ import annotations

import argparse
import re
import shutil
import subprocess
import sys
from pathlib import Path

SCRIPT_DIR = Path(__file__).resolve().parent
CLIENT_DIR = SCRIPT_DIR.parent
sys.path.insert(0, str(SCRIPT_DIR / "lib"))

from brand_config import load_brand_config  # noqa: E402


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--debug", action="store_true")
    parser.add_argument("--skip-packages", action="store_true")
    parser.add_argument("--skip-core", action="store_true")
    parser.add_argument(
        "--mobile-config",
        type=Path,
        default=None,
        help="JSON dart-define profile. Release default: config/openmuse-cloud.json",
    )
    parser.add_argument(
        "--skip-verify",
        action="store_true",
        help="Skip verify-android-apk.py after the copy",
    )
    return parser.parse_args()


def pubspec_version(flutter_dir: Path) -> str:
    text = (flutter_dir / "pubspec.yaml").read_text(encoding="utf-8")
    match = re.search(r"^version:\s*([0-9]+\.[0-9]+\.[0-9]+)", text, re.M)
    if not match:
        raise SystemExit("cannot read version from pubspec.yaml")
    return match.group(1)


def main() -> int:
    args = parse_args()
    brand = load_brand_config()
    flutter_dir = CLIENT_DIR / "frontend" / "appflowy_flutter"
    version = pubspec_version(flutter_dir)
    mode = "debug" if args.debug else "release"
    wrapper = SCRIPT_DIR / "build-android-client.sh"
    cmd = ["bash", str(wrapper), f"--{mode}"]
    if args.skip_packages:
        cmd.append("--skip-packages")
    if args.skip_core:
        cmd.append("--skip-core")
    config = args.mobile_config
    if config is None and mode == "release":
        config = CLIENT_DIR / "config" / "openmuse-cloud.json"
    if config is not None:
        config = config.resolve()
        if not config.is_file():
            raise FileNotFoundError(config)
        cmd.extend(["--mobile-config", str(config)])
        print(f"==> Mobile dart-defines from {config}", flush=True)

    print(f"==> Building {brand.name_en} Android APK ({mode}, arm64-v8a)", flush=True)
    subprocess.check_call(cmd)

    dest_dir = CLIENT_DIR / "dist" / "android"
    built = dest_dir / f"{brand.artifact_prefix}-android-{mode}.apk"
    if not built.is_file():
        raise FileNotFoundError(f"expected APK missing: {built}")
    catalog_name = f"{brand.artifact_prefix}-{version}-android-arm64.apk"
    catalog = dest_dir / catalog_name
    shutil.copy2(built, catalog)
    alias = dest_dir / f"{brand.artifact_prefix}-android.apk"
    shutil.copy2(built, alias)
    print(f"APK {built} ({built.stat().st_size / (1024 * 1024):.1f} MB)", flush=True)
    print(f"Catalog name: {catalog}", flush=True)

    if not args.skip_verify:
        verify = SCRIPT_DIR / "verify-android-apk.py"
        subprocess.check_call([sys.executable, str(verify), str(catalog)])

    print("Install on a device: ./frontend/client/scripts/install-android-client.sh")
    print("After manual QA: python frontend/client/scripts/push-android-client.py")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
