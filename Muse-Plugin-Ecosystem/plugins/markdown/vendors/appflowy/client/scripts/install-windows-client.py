#!/usr/bin/env python3
"""Install the packed Windows client for the current user.

Usage:
  python frontend/client/scripts/install-windows-client.py
  python frontend/client/scripts/install-windows-client.py --launch
"""
from __future__ import annotations

import argparse
import json
import os
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
    parser.add_argument("--launch", action="store_true")
    return parser.parse_args()


def _start_menu_shortcut(target: Path, workdir: Path, name: str) -> None:
    start_menu = Path(os.environ.get("APPDATA", "")) / "Microsoft" / "Windows" / "Start Menu" / "Programs"
    start_menu.mkdir(parents=True, exist_ok=True)
    shortcut = start_menu / f"{name}.lnk"
    script = (
        "$ws = New-Object -ComObject WScript.Shell; "
        f"$s = $ws.CreateShortcut({json.dumps(str(shortcut))}); "
        f"$s.TargetPath = {json.dumps(str(target))}; "
        f"$s.WorkingDirectory = {json.dumps(str(workdir))}; "
        f'$s.Description = {json.dumps(name)}; '
        "$s.Save()"
    )
    subprocess.run(
        ["powershell.exe", "-NoProfile", "-Command", script],
        check=False,
    )


def main() -> int:
    args = parse_args()
    brand = load_brand_config()
    src = mw.dist_dir() / "windows" / brand.binary_windows
    exe = src / brand.windows_exe
    muse = src / "muse"
    if not exe.is_file() or not (
        mw.looks_like_closure(muse) or mw.looks_like_legacy_runtime(muse)
    ):
        raise FileNotFoundError(
            f"packed client missing: {src}\n"
            "Run python frontend/client/scripts/pack-windows-client.py first."
        )
    dest = Path(os.environ.get("LOCALAPPDATA", str(Path.home() / "AppData" / "Local"))) / "Programs" / brand.binary_windows
    print(f"==> Installing {src} -> {dest}", flush=True)
    if dest.exists():
        print(f"Removing existing {dest}", flush=True)
        shutil.rmtree(dest)
    shutil.copytree(src, dest)
    installed = dest / brand.windows_exe
    _start_menu_shortcut(installed, dest, brand.name_en)
    print(f"Installed. Launch with: {installed}")
    if args.launch:
        os.startfile(installed)  # type: ignore[attr-defined]
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except (FileNotFoundError, RuntimeError) as error:
        print(error, file=sys.stderr)
        raise SystemExit(1) from error
