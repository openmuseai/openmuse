#!/usr/bin/env python3
"""Compile a fixed-region Web installation before building Flutter."""
import argparse
import json
import os
from pathlib import Path
import subprocess
import tempfile
from urllib.parse import urlsplit

ROOT = Path(__file__).resolve().parents[2]


def fixed_release(region, release, profile):
    if release['market'] != profile['market']:
        raise ValueError('Release market differs from Web deployment')
    matches = [r for r in release['regions'] if r['id'] == region and r['available']]
    if len(matches) != 1:
        raise ValueError('Web region unavailable')
    snapshot = matches[0]['bundle']['snapshot']
    if snapshot['scope']['dataRegion'] != region:
        raise ValueError('Web data region mismatch')
    for service in ['auth', 'cloud', 'relay']:
        value = snapshot['deployment']['services'].get(service)
        if value is None:
            continue
        url = urlsplit(value['baseUrl'])
        if f'{url.scheme}://{url.netloc}' not in profile['serviceOrigins']:
            raise ValueError(f'{service} origin is outside the regional Web profile')
    release = {**release, 'id': f'official-web-{region}', 'selection': 'fixed', 'fixedRegion': region, 'regions': matches}
    return release


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--region', required=True, choices=['cn', 'us'])
    parser.add_argument('--output', type=Path, required=True)
    parser.add_argument('--release-file', type=Path)
    args = parser.parse_args()
    profile = json.loads((ROOT / 'config/web-deployments.json').read_text())['regions'][args.region]
    source = args.release_file or Path(os.environ.get('OPENMUSE_WEB_RELEASE_FILE') or ROOT / profile['releaseFile'])
    if args.output.resolve() == source.resolve():
        parser.error('Output must not overwrite installed release')
    release = fixed_release(args.region, json.loads(source.read_text()), profile)
    with tempfile.TemporaryDirectory(prefix='openmuse-web-config-') as temp:
        input_file, output_file = Path(temp) / 'release.json', Path(temp) / 'defines.json'
        input_file.write_text(json.dumps(release, ensure_ascii=False))
        package = ROOT / 'packages/openmuse_runtime_config'
        if not (package / '.dart_tool/package_config.json').exists():
            subprocess.run(['flutter', 'pub', 'get'], cwd=package, check=True)
        subprocess.run(['dart', 'run', 'bin/compile_release.dart', '--input', str(input_file), '--output', str(output_file)], cwd=ROOT / 'packages/openmuse_runtime_config', check=True)
        defines = json.loads(output_file.read_text())
        defines.update(OPENMUSE_WEB_DATA_REGION=args.region, OPENMUSE_WEB_ORIGIN=profile['origin'], OPENMUSE_WEB_BASE_PATH=profile['baseHref'])
        args.output.parent.mkdir(parents=True, exist_ok=True)
        args.output.write_text(json.dumps(defines, ensure_ascii=False) + '\n')
    print(f'Validated fixed {args.region} Web installation: {profile["origin"]}{profile["baseHref"]}')


if __name__ == '__main__':
    main()
