#!/usr/bin/env bash
set -euo pipefail

# Rebuild the signed product, both native adapters, shared Flutter surface and
# requested Hosts from one checkout. Output is for local validation; release
# signing keys and Android production keystores are supplied separately.
target="${1:-all}"
case "$target" in all|macos|android) ;; *) echo 'usage: spreadsheet-product-e2e.sh [all|macos|android]' >&2; exit 2 ;; esac

repo="$(cd "$(dirname "$0")/../.." && pwd)"
client="$repo/Muse-Client"
product="$repo/Muse-Plugin-Ecosystem/plugins/spreadsheet-product"
bridge="$repo/ioffice/samples/libs/spreadsheet/gridcraft_bridge"
surface="$repo/ioffice/samples/libs/spreadsheet/spreadsheet_editor"
host="$client/app/openmuse_host"
mobile="$client/app/openmuse_mobile"
evidence="$client/dist/spreadsheet-evidence-$(date +%F-%H%M%S)"
mkdir -p "$evidence"

key_dir="${SPREADSHEET_PRODUCT_KEY_DIR:-$HOME/.local/share/openmuse/spreadsheet-product-dev}"
private_key="${SPREADSHEET_PRODUCT_PRIVATE_KEY:-$key_dir/private.pem}"
public_key="${SPREADSHEET_PRODUCT_PUBLIC_KEY:-$key_dir/public.pem}"
key_id="${SPREADSHEET_PRODUCT_KEY_ID:-spreadsheet-local-v1}"
if [[ ! -f "$private_key" || ! -f "$public_key" ]]; then
  mkdir -p "$key_dir"
  openssl genpkey -algorithm ED25519 -out "$private_key"
  chmod 600 "$private_key"
  openssl pkey -in "$private_key" -pubout -out "$public_key"
fi

if [[ ! -d "$repo/Muse-Plugin-Ecosystem/node_modules/jszip" ]]; then
  (cd "$repo/Muse-Plugin-Ecosystem" && npm install)
fi
(cd "$product" && npm run generate && npm run validate &&
  npm run pack -- --private-key "$private_key" --key-id "$key_id" &&
  npm run verify -- "$product/dist/spreadsheet-$(node -p "require('./product/product.json').version").mcpb" "$public_key" "$key_id")
version="$(node -p "require('$product/product/product.json').version")"
bundle="$product/dist/spreadsheet-$version.mcpb"
bash "$client/scripts/stage_spreadsheet_product_mobile.sh" "$bundle" "$public_key"
identity="$mobile/assets/spreadsheet_product/identity.json"

(cd "$bridge" && cargo build --release)
if [[ "$target" == all || "$target" == macos ]]; then
  (cd "$bridge" && cargo build --release --target x86_64-apple-darwin)
fi
(cd "$surface" && flutter analyze && flutter test test/visual_geometry_test.dart test/go_to_reference_test.dart)
mac_library="$bridge/target/release/libopenmuse_gridcraft_bridge.dylib"
flutter_root="$(cd "$(dirname "$(command -v flutter)")/.." && pwd)"
(cd "$host" &&
  SPREADSHEET_PRODUCT_E2E_BUNDLE="$bundle" \
  SPREADSHEET_PRODUCT_E2E_PUBLIC_KEY="$public_key" \
  SPREADSHEET_PRODUCT_E2E_KEY_ID="$key_id" \
  OPENMUSE_GRIDCRAFT_DYLIB="$mac_library" \
  SPREADSHEET_PRODUCT_E2E_SCREENSHOT_DIR="$evidence" \
  SPREADSHEET_PRODUCT_E2E_UI_FONT="${SPREADSHEET_PRODUCT_E2E_UI_FONT:-/System/Library/Fonts/Supplemental/Arial Unicode.ttf}" \
  SPREADSHEET_PRODUCT_E2E_ICON_FONT="${SPREADSHEET_PRODUCT_E2E_ICON_FONT:-$flutter_root/bin/cache/artifacts/material_fonts/MaterialIcons-Regular.otf}" \
  flutter test test/spreadsheet_product_e2e_test.dart test/spreadsheet_geometry_e2e_test.dart)

if [[ "$target" == all || "$target" == macos ]]; then
  (cd "$client" &&
    OPENMUSE_CLEAN_BUILD=0 \
    OPENMUSE_SPREADSHEET_CORE_FFI="$mac_library" \
    OPENMUSE_SPREADSHEET_PRODUCT_BUNDLE="$bundle" \
    OPENMUSE_SPREADSHEET_PRODUCT_PUBLIC_KEY="$public_key" \
    OPENMUSE_SPREADSHEET_PRODUCT_IDENTITY="$identity" \
    bash scripts/package_macos.sh --profile release --skip-tests --components spreadsheet)
  codesign --verify --deep --strict "$host/build/macos/Build/Products/Release/OpenMuse.app"
  mac_zip="$client/dist/OpenMuse-spreadsheet-$version-macos.zip"
  ditto -c -k --sequesterRsrc --keepParent \
    "$host/build/macos/Build/Products/Release/OpenMuse.app" "$mac_zip"
  unzip -tq "$mac_zip" >/dev/null
  embedded_hash="$(unzip -p "$mac_zip" 'OpenMuse.app/Contents/Resources/openmuse/spreadsheet-product/spreadsheet.mcpb' | shasum -a 256 | awk '{print $1}')"
  bundle_hash="$(shasum -a 256 "$bundle" | awk '{print $1}')"
  [[ "$embedded_hash" == "$bundle_hash" ]]
fi

if [[ "$target" == all || "$target" == android ]]; then
  (cd "$bridge" && cargo ndk -t arm64-v8a -o "$mobile/android/app/src/main/jniLibs" build --release)
  (cd "$mobile" && flutter pub get && flutter analyze --no-pub && flutter build apk --release --target-platform android-arm64)
  unsigned="$mobile/build/app/outputs/flutter-apk/app-release.apk"
  signed="$client/dist/OpenMuse-spreadsheet-android-arm64.apk"
  apksigner="${ANDROID_APKSIGNER:-$HOME/Library/Android/sdk/build-tools/36.0.0/apksigner}"
  if "$apksigner" verify "$unsigned" >/dev/null 2>&1; then
    cp "$unsigned" "$signed"
  else
    "$apksigner" sign --ks "$HOME/.android/debug.keystore" \
      --ks-key-alias androiddebugkey --ks-pass pass:android --key-pass pass:android \
      --out "$signed" "$unsigned"
  fi
  "$apksigner" verify "$signed"
  unzip -Z1 "$signed" | awk '/^lib\/arm64-v8a\/libopenmuse_gridcraft_bridge.so$/ { found=1 } END { exit !found }'
  unzip -Z1 "$signed" | awk '/^assets\/flutter_assets\/assets\/spreadsheet_product\/spreadsheet.mcpb$/ { found=1 } END { exit !found }'
  android_apk="$client/dist/OpenMuse-spreadsheet-$version-android-arm64.apk"
  cp "$signed" "$android_apk"
  embedded_hash="$(unzip -p "$android_apk" assets/flutter_assets/assets/spreadsheet_product/spreadsheet.mcpb | shasum -a 256 | awk '{print $1}')"
  bundle_hash="$(shasum -a 256 "$bundle" | awk '{print $1}')"
  [[ "$embedded_hash" == "$bundle_hash" ]]
fi

python3 - "$repo" "$client" "$bundle" "$mac_library" "$evidence" "$target" "$version" <<'PY'
import datetime, hashlib, json, pathlib, subprocess, sys
repo, client, bundle, library, evidence = map(pathlib.Path, sys.argv[1:6]); target, version = sys.argv[6:8]
def digest(path):
    if not path.is_file(): return None
    h = hashlib.sha256()
    with path.open('rb') as f:
        for chunk in iter(lambda: f.read(1024 * 1024), b''): h.update(chunk)
    return {'path': str(path), 'bytes': path.stat().st_size, 'sha256': h.hexdigest()}
def rev(directory):
    try: return subprocess.check_output(['git', '-C', str(directory), 'rev-parse', '--short', 'HEAD'], text=True, stderr=subprocess.DEVNULL).strip()
    except Exception: return None
def dirty(directory):
    try: return bool(subprocess.check_output(['git', '-C', str(directory), 'status', '--porcelain'], text=True, stderr=subprocess.DEVNULL).strip())
    except Exception: return None
roots = {name: repo / path for name, path in {
  'Muse-Client':'Muse-Client', 'Muse-Plugin-Ecosystem':'Muse-Plugin-Ecosystem',
  'GridCraft':'ioffice/vendors/gridcraft'}.items()}
report = {
  'profile': 'openmuse-spreadsheet-e2e/1', 'timeUtc': datetime.datetime.now(datetime.timezone.utc).isoformat(),
  'target': target, 'revisions': {name: {'commit': rev(path), 'dirty': dirty(path)} for name, path in roots.items()},
  'contract': {'product': json.loads((repo/'Muse-Plugin-Ecosystem/plugins/spreadsheet-product/product/product.json').read_text())['version'],
    'engineAbiMajor':1,'surfaceMajor':1},
  'artifacts': { 'product': digest(bundle), 'macLibrary': digest(library),
    'macZip': digest(client/f'dist/OpenMuse-spreadsheet-{version}-macos.zip') if target in ('all','macos') else None,
    'androidApk': digest(client/f'dist/OpenMuse-spreadsheet-{version}-android-arm64.apk') if target in ('all','android') else None },
  'fixture': digest(client/'app/openmuse_host/test/fixtures/spreadsheet-shell.xlsx'),
  'geometryFixture': digest(client/'app/openmuse_host/test/fixtures/spreadsheet-geometry.xlsx'),
  'sources': { 'bridge': digest(repo/'ioffice/samples/libs/spreadsheet/gridcraft_bridge/src/lib.rs'),
    'flutterSurface': digest(repo/'ioffice/samples/libs/spreadsheet/spreadsheet_editor/lib/src/spreadsheet_shell.dart'),
    'productGenerator': digest(repo/'Muse-Plugin-Ecosystem/plugins/spreadsheet-product/scripts/generate.mjs'),
    'pipeline': digest(client/'scripts/spreadsheet-product-e2e.sh') },
  'screenshots': sorted(p.name for p in evidence.glob('*.png')),
}
path = evidence/'report.json'; path.write_text(json.dumps(report, ensure_ascii=False, indent=2)+'\n')
print(path)
PY
