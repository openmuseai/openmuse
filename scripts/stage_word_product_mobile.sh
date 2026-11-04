#!/usr/bin/env bash
set -euo pipefail

# Stage an already signed, immutable Word product snapshot as the Mobile
# bootstrap plugin. Runtime installation may replace it with another package
# signed by the same trusted publisher key.
if [[ $# != 2 ]]; then
  echo 'usage: stage_word_product_mobile.sh signed-word.mcpb publisher-public.pem' >&2
  exit 2
fi
bundle="$(realpath "$1")"
public_key="$(realpath "$2")"
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
dest="$repo_root/app/openmuse_mobile/assets/word_product"
mkdir -p "$dest"
python3 - "$bundle" "$public_key" "$dest" <<'PY'
import json, pathlib, shutil, sys, zipfile
bundle, key, dest = map(pathlib.Path, sys.argv[1:])
with zipfile.ZipFile(bundle) as package:
    catalog = json.loads(package.read('catalog.json'))
    signature = json.loads(package.read('catalog.signature.json'))
    product = json.loads(package.read('product/product.json'))
if catalog['productId'] != 'io.openmuse/word' or catalog['version'] != product['version']:
    raise SystemExit('Word product identity mismatch')
if signature['algorithm'] != 'Ed25519':
    raise SystemExit('Word product must use Ed25519')
shutil.copy2(bundle, dest / 'word.mcpb')
shutil.copy2(key, dest / 'public.pem')
(dest / 'identity.json').write_text(json.dumps({
    'version': product['version'], 'keyId': signature['keyId']
}, indent=2) + '\n')
PY
