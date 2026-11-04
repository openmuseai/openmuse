#!/usr/bin/env bash
set -euo pipefail

if [[ $# != 2 ]]; then
  echo 'usage: stage_spreadsheet_product_mobile.sh signed-spreadsheet.mcpb publisher-public.pem' >&2
  exit 2
fi
bundle="$(realpath "$1")"
public_key="$(realpath "$2")"
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
dest="$repo_root/app/openmuse_mobile/assets/spreadsheet_product"
mkdir -p "$dest"
python3 - "$bundle" "$public_key" "$dest" <<'PY'
import json, pathlib, shutil, sys, zipfile
bundle, key, dest = map(pathlib.Path, sys.argv[1:])
with zipfile.ZipFile(bundle) as package:
    catalog = json.loads(package.read('catalog.json'))
    signature = json.loads(package.read('catalog.signature.json'))
    product = json.loads(package.read('product/product.json'))
if catalog['productId'] != 'io.openmuse/spreadsheet' or catalog['version'] != product['version']:
    raise SystemExit('Spreadsheet product identity mismatch')
if signature['algorithm'] != 'Ed25519':
    raise SystemExit('Spreadsheet product must use Ed25519')
shutil.copy2(bundle, dest / 'spreadsheet.mcpb')
shutil.copy2(key, dest / 'public.pem')
(dest / 'identity.json').write_text(json.dumps({
    'version': product['version'], 'keyId': signature['keyId']
}, indent=2) + '\n')
PY
