#!/usr/bin/env python3
"""Prepare signed Mobile catalog files for deployment to an HTTPS origin.

Usage: python3 scripts/publish_mobile_market.py PRIVATE_KEY_PEM HTTPS_ORIGIN OUTPUT_DIR SEQUENCE
Private key is supplied from outside the repository; never commit it.
SEQUENCE must increase for each release from the same signing key.
"""
import base64
import hashlib
import json
import shutil
import sys
from datetime import datetime, timedelta, timezone
from pathlib import Path
from urllib.parse import urlparse

from cryptography.hazmat.primitives import serialization


def main() -> None:
    if len(sys.argv) != 5:
        raise SystemExit(__doc__)
    key_path, origin, output, sequence_arg = sys.argv[1:]
    sequence = int(sequence_arg)
    if sequence < 0:
        raise SystemExit('sequence must be nonnegative')
    parsed = urlparse(origin)
    if parsed.scheme != 'https' or not parsed.netloc or parsed.path not in ('', '/'):
        raise SystemExit('origin must be a bare HTTPS origin')
    private_key = serialization.load_pem_private_key(Path(key_path).read_bytes(), password=None)
    root = Path(__file__).resolve().parents[1] / 'packages/openmuse_mobile_plugin_runtime/assets'
    destination = Path(output)
    destination.mkdir(parents=True, exist_ok=True)
    entries = json.loads((root / 'market.json').read_text())
    for entry in entries:
        source = root / entry['bundle']
        data = source.read_bytes()
        if hashlib.sha256(data).hexdigest() != entry['sha256']:
            raise SystemExit(f"digest mismatch: {source}")
        shutil.copyfile(source, destination / entry['bundle'])
        entry['bundle'] = f"{origin.rstrip('/')}/{source.name}"
        for medium in entry.get('media', []):
            for field in ('uri', 'poster'):
                location = medium.get(field)
                if not location:
                    continue
                prefix = 'asset:packages/openmuse_market_ui/assets/'
                if not location.startswith(prefix):
                    raise SystemExit(f'unsupported bundled media: {location}')
                media_source = root.parent.parent / 'openmuse_market_ui' / 'assets' / location[len(prefix):]
                shutil.copyfile(media_source, destination / media_source.name)
                medium[field] = f"{origin.rstrip('/')}/{media_source.name}"
    payload = json.dumps({
        'schema': 'openmuse-mobile-market/v1',
        'sequence': sequence,
        'expiresAt': (datetime.now(timezone.utc) + timedelta(days=30)).isoformat(),
        'plugins': entries,
    }, ensure_ascii=False, separators=(',', ':')).encode()
    signature = private_key.sign(payload)
    (destination / 'index.json').write_text(json.dumps({
        'payload': base64.b64encode(payload).decode(),
        'signature': base64.b64encode(signature).decode(),
    }, separators=(',', ':')) + '\n')
    public_key = private_key.public_key().public_bytes(
        encoding=serialization.Encoding.Raw,
        format=serialization.PublicFormat.Raw,
    )
    print('OPENMUSE_MOBILE_MARKET_PUBLIC_KEY=' + base64.b64encode(public_key).decode())
    print('Upload output directory to ' + origin)


if __name__ == '__main__':
    main()
