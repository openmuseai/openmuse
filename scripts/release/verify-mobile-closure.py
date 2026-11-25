#!/usr/bin/env python3
"""Reject excluded mobile implementations, identifiers and payloads.

A technical exclusion check; passing does not certify copyright ownership.
"""
import argparse
import hashlib
import json
import io
import zlib
from pathlib import Path
import re
import subprocess
import struct
import zipfile

FORBIDDEN = re.compile(r'word_core|openmuse_office_(?:docx|viewers)|pdfium|pdfrx|gridcraft|spreadsheet_engine|pdf_rebuild|word_render|word_editor|siyuan|genoffice|ioffice|kspfreetext|cn[/._]wps', re.I)
FORBIDDEN_PACKAGES = {'openmuse_office_docx', 'openmuse_office_viewers', 'word_editor', 'word_render', 'pdf_rebuild_flutter', 'pdfium_dart', 'pdfium_flutter', 'pdfrx', 'spreadsheet_editor', 'openmuse_siyuan_client', 'openmuse_siyuan_plugin'}
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


# Scan payloads as well as names: Flutter notices, DEX, AOT strings and nested
# offline bundles can retain an excluded implementation under a neutral name.
CONTENT = re.compile(FORBIDDEN.pattern.encode('ascii'), re.I)
MAX_EXPANDED = 512 * 1024 * 1024


def load_profile(name):
    if name is None:
        return None
    if name != 'mobile-1.0.3':
        raise ValueError('Unknown mobile closure profile')
    path = Path(__file__).resolve().parents[2] / 'config/release/mobile-1.0.3-closure.json'
    value = json.loads(path.read_text())
    if (value.get('schema') != 'openmuse.mobile-closure/1'
            or value.get('version') != '1.0.3'
            or value.get('allowedImplementationInPinnedArtifacts') != 'genoffice'
            or value.get('allowedHostIdentifiers') != ['io.openmuse/genoffice-pptx']
            or value.get('allowedLiteralsInPinnedArtifacts') != ['aiOfficeTabs']
            or set(value.get('bundledArtifacts', {})) != {'flutter_assets/assets/bundled/slides-0.9.6.mcpb'}
            or any(not re.fullmatch('[a-f0-9]{64}', v) for v in value['bundledArtifacts'].values())):
        raise ValueError('Invalid mobile closure profile')
    return value


def forbidden_matches(data, profile=None, approved_bundle=False):
    normalized = data.replace(b'\x00', b'')
    if profile:
        for identifier in profile['allowedHostIdentifiers']:
            normalized = normalized.replace(identifier.encode('ascii'), b'')
        if approved_bundle:
            for literal in profile['allowedLiteralsInPinnedArtifacts']:
                normalized = normalized.replace(literal.encode('ascii'), b'')
    return sorted({m.group().decode('ascii').lower() for m in CONTENT.finditer(normalized)
                   if not (approved_bundle and m.group().lower() == b'genoffice')})


def inspect_payload(name, data, depth=0, budget=None, profile=None, approved_bundle=False):
    budget = budget if budget is not None else [MAX_EXPANDED]
    budget[0] -= len(data)
    if budget[0] < 0 or depth > 5:
        raise ValueError('Payload inspection budget exceeded: ' + name)
    bad = []
    if profile:
        for suffix, digest in profile['bundledArtifacts'].items():
            if name == suffix or name.endswith('/' + suffix):
                if hashlib.sha256(data).hexdigest() != digest:
                    return [name + ': reviewed bundled artifact SHA-256 mismatch']
                approved_bundle = True
    # UTF-16 strings appear in Android XML/DEX resources and native metadata.
    matches = forbidden_matches(data, profile, approved_bundle)
    if matches:
        bad.append(name + ': ' + ', '.join(matches))
    if data.startswith(b'PK\x03\x04'):
        with zipfile.ZipFile(io.BytesIO(data)) as nested:
            for item in nested.infolist():
                if item.is_dir():
                    continue
                if item.file_size > budget[0]:
                    raise ValueError('Nested payload too large: ' + name)
                child = name + '!/' + item.filename
                if forbidden_matches(item.filename.encode(), profile, approved_bundle):
                    bad.append(child)
                bad.extend(inspect_payload(child, nested.read(item), depth + 1, budget, profile, approved_bundle))
    elif data.startswith(b'\x1f\x8b') or name.endswith('NOTICES.Z'):
        # Limit decompression before allocating an unbounded notices payload.
        inflater = zlib.decompressobj(16 + zlib.MAX_WBITS if data.startswith(b'\x1f\x8b') else zlib.MAX_WBITS)
        expanded = inflater.decompress(data, max(1, budget[0] + 1))
        if not inflater.eof:
            raise ValueError('Compressed payload incomplete or too large: ' + name)
        bad.extend(inspect_payload(name + '!/expanded', expanded, depth + 1, budget, profile, approved_bundle))
    return bad


def verify(path, profile=None):
    artifact_names = None
    if path.name == 'package_config.json':
        names = {p['name'] for p in json.loads(path.read_text())['packages']}
        bad = sorted(name for name in names if name in FORBIDDEN_PACKAGES or FORBIDDEN.search(name))
    elif path.is_dir():
        artifact_names = [str(p.relative_to(path)) for p in path.rglob('*') if p.is_file()]
        bad = [str(p.relative_to(path)) for p in path.rglob('*') if p.is_file() and forbidden_matches(str(p.relative_to(path)).encode(), profile)]
        for file in path.rglob('*'):
            if file.is_file():
                bad.extend(inspect_payload(str(file.relative_to(path)), file.read_bytes(), profile=profile))
        executable = path / 'Runner'
        if executable.exists():
            symbols = subprocess.check_output(['nm', '-gU', str(executable)], text=True)
            bad.extend(line for line in symbols.splitlines() if forbidden_matches(line.encode(), profile))
    else:
        with zipfile.ZipFile(path) as archive:
            artifact_names = archive.namelist()
            bad = [name for name in archive.namelist() if forbidden_matches(name.encode(), profile)]
            for item in archive.infolist():
                if not item.is_dir():
                    bad.extend(inspect_payload(item.filename, archive.read(item), profile=profile))
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
    if profile and artifact_names is not None:
        for suffix in profile['bundledArtifacts']:
            if sum(name == suffix or name.endswith('/' + suffix) for name in artifact_names) != 1:
                bad.append('Missing or duplicate reviewed bundled artifact: ' + suffix)
    if bad:
        raise SystemExit('Excluded implementations or identifiers in mobile release:\n' + '\n'.join(bad))
    print(f'Mobile dependency and payload closure: PASS ({path})')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('artifact', type=Path)
    parser.add_argument('--profile', choices=['mobile-1.0.3'])
    args = parser.parse_args()
    verify(args.artifact, load_profile(args.profile))
