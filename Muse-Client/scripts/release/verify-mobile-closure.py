#!/usr/bin/env python3
"""Verify the first-launch mobile package excludes unready Office engines."""
import argparse
import json
from pathlib import Path
import re
import subprocess
import struct
import zipfile

FORBIDDEN = re.compile(r'word_core|openmuse_office_(?:docx|viewers)|pdfium|pdfrx|gridcraft|spreadsheet_engine|pdf_rebuild|word_render|word_editor', re.I)
FORBIDDEN_PACKAGES = {'openmuse_office_docx', 'openmuse_office_viewers', 'word_editor', 'word_render', 'pdf_rebuild_flutter', 'pdfium_dart', 'pdfium_flutter', 'pdfrx', 'spreadsheet_editor'}
FORBIDDEN_PERMISSIONS = {
    'android.permission.READ_EXTERNAL_STORAGE',
    'android.permission.WRITE_EXTERNAL_STORAGE',
    'android.permission.READ_MEDIA_IMAGES',
    'android.permission.READ_MEDIA_VIDEO',
    'android.permission.READ_MEDIA_AUDIO',
    'android.permission.READ_MEDIA_VISUAL_USER_SELECTED',
    'android.permission.READ_CONTACTS',
    'android.permission.CAMERA',
    'android.permission.ACCESS_FINE_LOCATION',
    'android.permission.ACCESS_COARSE_LOCATION',
    'android.permission.READ_PHONE_STATE',
    'com.google.android.gms.permission.AD_ID',
}


def verify(path):
    if path.name == 'package_config.json':
        names = {p['name'] for p in json.loads(path.read_text())['packages']}
        bad = sorted(names & FORBIDDEN_PACKAGES)
    elif path.is_dir():
        bad = [str(p.relative_to(path)) for p in path.rglob('*') if p.is_file() and FORBIDDEN.search(str(p.relative_to(path)))]
        executable = path / 'Runner'
        if executable.exists():
            symbols = subprocess.check_output(['nm', '-gU', str(executable)], text=True)
            bad.extend(line for line in symbols.splitlines() if FORBIDDEN.search(line))
    else:
        with zipfile.ZipFile(path) as archive:
            bad = [name for name in archive.namelist() if FORBIDDEN.search(name)]
            manifests = [name for name in archive.namelist()
                         if name == 'AndroidManifest.xml' or name.endswith('/manifest/AndroidManifest.xml')]
            if path.suffix in {'.apk', '.aab'} and not manifests:
                raise SystemExit('Android artifact is missing its compiled manifest')
            for name in manifests:
                manifest = archive.read(name)
                # APK AXML string pools use UTF-8/UTF-16; AAB protobuf XML
                # stores UTF-8 strings. Inspect the actual merged artifact,
                # not only the app's source manifest before library merging.
                restricted = sorted(permission for permission in FORBIDDEN_PERMISSIONS
                                    if any(permission.encode(encoding) in manifest
                                           for encoding in ('utf-8', 'utf-16le')))
                if restricted:
                    raise SystemExit('Unexpected sensitive permissions in mobile release:\n' + '\n'.join(restricted))
            if manifests:
                print('Android merged sensitive-permission scope: PASS')
            native_count = 0
            for name in archive.namelist():
                if not (name.startswith(('lib/arm64-v8a/', 'base/lib/arm64-v8a/')) and name.endswith('.so')):
                    continue
                binary = archive.read(name)
                if binary[:6] != b'\x7fELF\x02\x01':
                    raise SystemExit('Expected an ELF64 little-endian native library: ' + name)
                offset = struct.unpack_from('<Q', binary, 32)[0]
                entry, count = struct.unpack_from('<HH', binary, 54)
                alignments = [struct.unpack_from('<Q', binary, offset + i * entry + 48)[0]
                              for i in range(count)
                              if struct.unpack_from('<I', binary, offset + i * entry)[0] == 1]
                if not alignments or min(alignments) < 16384:
                    raise SystemExit('Native PT_LOAD alignment is below 16 KiB: ' + name)
                native_count += 1
            print(f'Android ELF load-segment alignment: PASS ({native_count} arm64 libraries)')
    if bad:
        raise SystemExit('Unready Office dependencies in mobile release:\n' + '\n'.join(bad))
    print(f'Mobile Office dependency closure: PASS ({path})')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('artifact', type=Path)
    verify(parser.parse_args().artifact)
