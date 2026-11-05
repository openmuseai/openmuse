#!/usr/bin/env python3
"""Check an OpenMuse Android APK before sideload or production upload."""
from __future__ import annotations

import argparse
import subprocess
import sys
import zipfile
from pathlib import Path

REQUIRED_LIBS = (
    "lib/arm64-v8a/libdart_ffi.so",
    "lib/arm64-v8a/libc++_shared.so",
    "lib/arm64-v8a/libapp.so",
    "lib/arm64-v8a/libirondash_engine_context_native.so",
    "lib/arm64-v8a/libsuper_native_extensions.so",
)


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("apk", nargs="?", type=Path)
    return parser.parse_args()


def default_apk(client_dir: Path) -> Path:
    android = client_dir / "dist" / "android"
    matches = sorted(android.glob("OpenMuse-*-android-arm64.apk"), key=lambda p: p.stat().st_mtime)
    if matches:
        return matches[-1]
    fallback = android / "OpenMuse-android-release.apk"
    if fallback.is_file():
        return fallback
    raise SystemExit(f"no APK under {android}")


def main() -> int:
    args = parse_args()
    client_dir = Path(__file__).resolve().parent.parent
    apk = (args.apk or default_apk(client_dir)).resolve()
    if not apk.is_file():
        raise SystemExit(f"APK not found: {apk}")
    print(f"==> {apk} ({apk.stat().st_size / (1024 * 1024):.1f} MB)")
    with zipfile.ZipFile(apk) as zf:
        names = set(zf.namelist())
        missing = [name for name in REQUIRED_LIBS if name not in names]
        if missing:
            raise SystemExit("APK missing: " + ", ".join(missing))
        libapp = zf.read("lib/arm64-v8a/libapp.so")
    print("    arm64 native libs: ok")
    blob = libapp.decode("latin1", errors="ignore")
    if "openmuseai.com" not in blob:
        raise SystemExit("release APK does not contain openmuseai.com (check --mobile-config)")
    print("    dart-define openmuseai.com: ok")
    if "dsh.openmuseai.com" in blob:
        raise SystemExit("release APK still bakes dsh.openmuseai.com; page URL must be same-origin /u/<hash>/")
    print("    dart-define omits retired dsh. host: ok")
    file_out = subprocess.check_output(["file", str(apk)], text=True).strip()
    print(f"    {file_out}")
    print("VERIFY_PASS")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
