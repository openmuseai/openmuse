#!/usr/bin/env python3
"""Assess public runtime bootstrap material without inventing production keys."""
import argparse
import json
from pathlib import Path
from urllib.parse import urlsplit


def assess(defines):
    release = json.loads(defines['OPENMUSE_RELEASE_JSON'])
    issues = []
    for region in release['regions']:
        if not region['available']:
            continue
        bootstrap = region['bundle']['bootstrap']
        prefix = region['id'] + ': '
        if not bootstrap.get('discoveryUrls'):
            issues.append(prefix + 'missing production discovery URL')
        if not bootstrap.get('publicKeys'):
            issues.append(prefix + 'missing production config verification key')
        if not bootstrap.get('allowedResourceOrigins'):
            issues.append(prefix + 'missing production resource origins')
        for uri in bootstrap.get('discoveryUrls', []):
            parsed = urlsplit(uri)
            if parsed.scheme != 'https' or not parsed.hostname or parsed.username or parsed.password:
                issues.append(prefix + 'discovery URL must be credential-free HTTPS')
        if region['bundle']['snapshot']['environment'] != 'production':
            issues.append(prefix + 'snapshot environment is not production')
    return {'productionInputsComplete': not issues, 'issues': issues,
            'onlineSignedPublicationVerified': False}


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('defines', type=Path)
    parser.add_argument('--report', type=Path, required=True)
    parser.add_argument('--allow-incomplete', action='store_true')
    args = parser.parse_args()
    result = assess(json.loads(args.defines.read_text()))
    args.report.write_text(json.dumps(result, indent=2) + '\n')
    if result['issues']:
        print('Production runtime input incomplete: ' + '; '.join(result['issues']))
        if not args.allow_incomplete:
            raise SystemExit(1)
    else:
        print('Production runtime public inputs: PASS (online publication still requires acceptance)')
