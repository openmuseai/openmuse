#!/usr/bin/env python3
"""Verify the actual signed app, after all staging and signature repairs."""
import plistlib
import subprocess
import sys
from pathlib import Path


def main():
    app, expected_file = sys.argv[1:]
    expected = plistlib.loads(Path(expected_file).read_bytes())
    signed = subprocess.run(
        ['codesign', '-d', '--entitlements', ':-', app],
        check=True, capture_output=True,
    ).stdout
    actual = plistlib.loads(signed) if signed.strip() else {}
    mismatches = [key for key, value in expected.items()
                  if key not in actual or actual[key] != value]
    if mismatches:
        raise SystemExit('Signed app entitlement mismatch: ' + ', '.join(mismatches))
    print('Signed app entitlements match ' + Path(expected_file).name)


if __name__ == '__main__':
    main()
