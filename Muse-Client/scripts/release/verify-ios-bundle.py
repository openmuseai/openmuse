#!/usr/bin/env python3
"""Check iOS privacy declarations, asset binaries and Notes exports before upload."""

import pathlib
import plistlib
import re
import subprocess
import sys


def verify(app: pathlib.Path) -> None:
    info = plistlib.loads((app / 'Info.plist').read_bytes())
    # file_picker's bundled photo picker references protected Photos APIs even
    # when a user only opens documents. Apple checks these in the final binary.
    required_purposes = (
        'NSPhotoLibraryUsageDescription',
        'NSMicrophoneUsageDescription',
        'NSLocalNetworkUsageDescription',
    )
    missing_purposes = [key for key in required_purposes
                        if not isinstance(info.get(key), str)
                        or not info[key].strip()]
    if missing_purposes:
        raise SystemExit('Missing iOS privacy purpose strings:\n' +
                         '\n'.join(missing_purposes))
    print('iOS privacy purpose strings: PASS (Photos, microphone, local network)')
    assets = app / "Frameworks/App.framework/flutter_assets"
    if not assets.is_dir():
        raise SystemExit(f"Missing Flutter assets: {assets}")
    executable_magic = {
        b"\xfe\xed\xfa\xce", b"\xce\xfa\xed\xfe",
        b"\xfe\xed\xfa\xcf", b"\xcf\xfa\xed\xfe",
        b"\xca\xfe\xba\xbe", b"\xbe\xba\xfe\xca",
        b"\xca\xfe\xba\xbf", b"\xbf\xba\xfe\xca",
        b"\x7fELF",
    }
    rejected = []
    for path in assets.rglob("*"):
        if not path.is_file():
            continue
        with path.open("rb") as stream:
            header = stream.read(4)
        if header in executable_magic or path.suffix.lower() in {
            ".dylib", ".so", ".dll", ".exe",
        }:
            rejected.append(str(path.relative_to(app)))
    if rejected:
        raise SystemExit("Native binaries are not permitted as iOS assets:\n" +
                         "\n".join(rejected))
    if (assets / 'packages/openmuse_siyuan_client/assets/runtime.zip').is_file():
        executable = info['CFBundleExecutable']
        exports = ''
        for binary in [app / executable, app / (executable + '.debug.dylib')]:
            if binary.is_file():
                exports += subprocess.check_output(
                    ['xcrun', 'dyld_info', '-exports', str(binary)], text=True,
                )
        required = {'muse_siyuan_' + name for name in
                    ['start', 'status', 'free', 'flush', 'suspend', 'resume']}
        missing = sorted(name for name in required
                         if not re.search(r'\s_' + name + r'(?:\s|$)', exports))
        if missing:
            raise SystemExit('Notes static FFI exports missing from final iOS binary:\n' + '\n'.join(missing))
        print('iOS Notes static FFI exports: PASS (6 entry points)')
    print("iOS asset executable scan: PASS")


if __name__ == "__main__":
    if len(sys.argv) != 2:
        raise SystemExit("Usage: verify-ios-bundle.py /path/to/Runner.app")
    verify(pathlib.Path(sys.argv[1]))
