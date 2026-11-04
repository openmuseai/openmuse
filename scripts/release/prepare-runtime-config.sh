#!/usr/bin/env bash
# Produce one validated, reviewable public deployment input for a release.
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
deployment_profile="${1:?deployment profile required}"
output="${2:?output path required}"
[[ "$deployment_profile" =~ ^[a-z0-9][a-z0-9-]*$ ]] || { echo 'Invalid deployment profile.' >&2; exit 2; }
bundle_tmp="$(mktemp)"
trap 'rm -f "$bundle_tmp"' EXIT
python3 - "$repo_root" "$deployment_profile" "$bundle_tmp" <<'PY'
import json, os, pathlib, sys
root, profile, target = sys.argv[1:]
market = os.environ.get('OPENMUSE_RELEASE_MARKET') or ('china' if profile == 'china' else 'overseas')
if market not in {'china','overseas'}:
    raise SystemExit('OPENMUSE_RELEASE_MARKET must be china or overseas')
source = os.environ.get('OPENMUSE_RELEASE_CONFIG_FILE')
release = json.loads(pathlib.Path(source).read_text()) if source else json.loads((pathlib.Path(root) / 'packages/openmuse_runtime_config/assets/releases' / f'{market}.json').read_text())
provided = os.environ.get('OPENMUSE_DEPLOYMENT_DEFINES_FILE')
if provided:
    defines = json.loads(pathlib.Path(provided).read_text())
    if 'OPENMUSE_RELEASE_JSON' in defines:
        release = json.loads(defines['OPENMUSE_RELEASE_JSON'])
    else:
        bundle = json.loads(defines['OPENMUSE_DEPLOYMENT_JSON'])
        matching = [region for region in release['regions'] if region['available'] and region['id'] == bundle['snapshot']['scope']['dataRegion']]
        if len(matching) != 1:
            raise SystemExit('Deployment override does not belong to this release market')
        matching[0]['bundle'] = bundle
if release['market'] != market:
    raise SystemExit('Release input and selected market do not match')
registered = json.loads((pathlib.Path(root) / 'packages/openmuse_i18n/assets/catalog.json').read_text())
tags = {locale['tag'] for locale in registered['metadata']['locales']}
for region in release['regions']:
    if not region['available']:
        continue
    bundle = region['bundle']
    if 'OPENMUSE_APP_FILING_NUMBER' in os.environ:
        bundle['snapshot'].setdefault('regulatory', {})['appFilingNumber'] = os.environ['OPENMUSE_APP_FILING_NUMBER']
    if 'OPENMUSE_UPDATE_CHANNEL' in os.environ:
        bundle['snapshot']['updates']['channel'] = os.environ['OPENMUSE_UPDATE_CHANNEL']
    if not set(bundle['snapshot']['localization']['enabledLocales']) <= tags:
        raise SystemExit('Installed release must start with the bundled fallback languages')
pathlib.Path(target).write_text(json.dumps(release, ensure_ascii=False))
PY
cd "$repo_root/packages/openmuse_runtime_config"
# A checkout may have been used by a Linux builder. Its package_config contains
# absolute paths from that machine, so existence alone does not make it usable.
flutter pub get --enforce-lockfile
dart run bin/compile_release.dart --input "$bundle_tmp" --output "$output"
