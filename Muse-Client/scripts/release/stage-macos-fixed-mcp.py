#!/usr/bin/env python3
"""Stage fixed Store application code inside the signed, read-only app bundle."""
import argparse
import hashlib
import json
from pathlib import Path
import shutil
import zipfile


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--root', type=Path, required=True)
    args = parser.parse_args()
    root = args.root.resolve()
    bundle = root / 'genoffice-pptx.mcpb'
    identity = json.loads((root / 'identity.json').read_text())
    if hashlib.sha256(bundle.read_bytes()).hexdigest() != identity['sha256']:
        raise SystemExit('Fixed MCP bundle digest mismatch.')
    output = root / 'runtime'
    if output.exists():
        shutil.rmtree(output)
    output.mkdir()
    digests, total = {}, 0
    with zipfile.ZipFile(bundle) as archive:
        if len(archive.infolist()) > 1024:
            raise SystemExit('Fixed MCP bundle file limit exceeded.')
        for entry in archive.infolist():
            path = Path(entry.filename)
            if path.is_absolute() or '..' in path.parts or '\\' in entry.filename:
                raise SystemExit('Unsafe fixed MCP bundle path.')
            if entry.is_dir():
                continue
            total += entry.file_size
            if total > 160 * 1024**2:
                raise SystemExit('Fixed MCP expansion limit exceeded.')
            data = archive.read(entry)
            target = output / path
            target.parent.mkdir(parents=True, exist_ok=True)
            target.write_bytes(data)
            target.chmod(0o644)
            digests[entry.filename] = hashlib.sha256(data).hexdigest()
    (root / 'runtime-sha256.json').write_text(json.dumps(digests, indent=2) + '\n')
    print(f'Fixed MCP code staged in app bundle: {len(digests)} files.')


if __name__ == '__main__':
    main()
