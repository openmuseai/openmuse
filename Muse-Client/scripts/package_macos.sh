#!/usr/bin/env bash
set -euo pipefail

profile="release"
skip_tests=0
package_zip=0
components="none"
build_name=""
build_number=""
update_channel="stable"
distribution="direct"
usage() {
  cat <<'EOF'
usage: package_macos.sh [--profile release|debug] [--skip-tests]
                        [--components all|none|word,pdf,ppt,spreadsheet]
                        [--package] [--build-name x.y.z] [--build-number n]
                        [--update-channel stable|canary]
                        [--distribution direct|app-store]

Builds dist/OpenMuse.app. Components default to none; select any combination
with --components. --package also creates dist/OpenMuse-macos.zip (off by default).
Component bundle, trust-key and native-library paths can be overridden with
the existing OPENMUSE_* environment variables.
EOF
}
while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile) profile="$2"; shift 2 ;;
    --skip-tests) skip_tests=1; shift ;;
    --package) package_zip=1; shift ;;
    --components) components="$2"; shift 2 ;;
    --build-name) build_name="$2"; shift 2 ;;
    --build-number) build_number="$2"; shift 2 ;;
    --update-channel) update_channel="$2"; shift 2 ;;
    --distribution) distribution="$2"; shift 2 ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
word_selected=0
pdf_selected=0
ppt_selected=0
spreadsheet_selected=0
case "$components" in
  all) word_selected=1; pdf_selected=1; ppt_selected=1; spreadsheet_selected=1 ;;
  none) ;;
  *)
    if [[ ! "$components" =~ ^(word|pdf|ppt|pptx|spreadsheet)(,(word|pdf|ppt|pptx|spreadsheet))*$ ]]; then
      echo "invalid component list: $components" >&2
      usage >&2
      exit 2
    fi
    IFS=, read -r -a requested_components <<< "$components"
    for component in "${requested_components[@]}"; do
      case "$component" in
        word) word_selected=1 ;;
        pdf) pdf_selected=1 ;;
        ppt|pptx) ppt_selected=1 ;;
        spreadsheet) spreadsheet_selected=1 ;;
        *) echo "unsupported component: $component" >&2; usage >&2; exit 2 ;;
      esac
    done
    ;;
esac
case "$profile" in
  release) product_dir="Release"; entitlement_file="Release.entitlements" ;;
  debug) product_dir="Debug"; entitlement_file="DebugProfile.entitlements" ;;
  *) echo "profile must be release or debug" >&2; exit 2 ;;
esac
case "$distribution" in
  direct) ;;
  app-store)
    [[ "$profile" == release ]] || { echo 'App Store requires release profile' >&2; exit 2; }
    entitlement_file="AppStore.entitlements"
    ;;
  *) echo 'distribution must be direct or app-store' >&2; exit 2 ;;
esac
[[ "$update_channel" == "stable" || "$update_channel" == "canary" ]] || {
  echo "unsupported update channel: $update_channel" >&2; exit 2;
}
[[ -z "$build_name" || "$build_name" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ ]] || {
  echo "invalid build name: $build_name" >&2; exit 2;
}
[[ -z "$build_number" || "$build_number" =~ ^[1-9][0-9]*$ ]] || {
  echo "invalid build number: $build_number" >&2; exit 2;
}

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
# Native builds, staging and the final app copy need several GiB of scratch
# space. Fail before writing rather than exhausting the host's shared volume.
python3 - "$repo_root" <<'PY'
import shutil, sys
free = shutil.disk_usage(sys.argv[1]).free
required = 10 * 1024**3
if free < required:
    raise SystemExit(f'macOS packaging stopped: {free / 1024**3:.1f} GiB free; at least 10 GiB required. Free space before rebuilding.')
PY
app_root="$repo_root/app/openmuse_host"
dist_root="$repo_root/dist"
app_path="$app_root/build/macos/Build/Products/${product_dir}/OpenMuse.app"
archive_path="$dist_root/OpenMuse-macos.zip"
dist_app_path="$dist_root/OpenMuse.app"
dsh_closure="$repo_root/target/dsh-closure"
node_runtime="$repo_root/target/node-v22.19.0-universal"
ecosystem_root="$repo_root/../Muse-Plugin-Ecosystem"
office_root="$repo_root/../ioffice"
mobile_assets="$repo_root/app/openmuse_mobile/assets"

current_bundle() {
  python3 - "$ecosystem_root/plugins/$1" "$2" <<'PY'
import json, pathlib, sys
root, name = pathlib.Path(sys.argv[1]), sys.argv[2]
version = json.loads((root / 'package.json').read_text())['version']
bundle = root / 'dist' / f'{name}-{version}.mcpb'
if not bundle.is_file():
    raise SystemExit(f'Missing component bundle: {bundle}')
print(bundle)
PY
}
require_component_file() {
  [[ -f "$2" ]] || { echo "$1 component requires $2" >&2; exit 2; }
}
if (( word_selected )); then
  OPENMUSE_WORD_CORE_FFI="${OPENMUSE_WORD_CORE_FFI:-$office_root/samples/docx-mvp-render/native/target/aarch64-apple-darwin/release/libword_core_ffi.dylib}"
  OPENMUSE_WORD_CORE_FFI_X86_64="${OPENMUSE_WORD_CORE_FFI_X86_64:-$office_root/samples/docx-mvp-render/native/target/x86_64-apple-darwin/release/libword_core_ffi.dylib}"
  OPENMUSE_WORD_PRODUCT_BUNDLE="${OPENMUSE_WORD_PRODUCT_BUNDLE:-$(current_bundle word word)}"
  OPENMUSE_WORD_PRODUCT_PUBLIC_KEY="${OPENMUSE_WORD_PRODUCT_PUBLIC_KEY:-$mobile_assets/word_product/public.pem}"
  OPENMUSE_WORD_PRODUCT_IDENTITY="${OPENMUSE_WORD_PRODUCT_IDENTITY:-$mobile_assets/word_product/identity.json}"
  require_component_file Word "$OPENMUSE_WORD_CORE_FFI"
  require_component_file Word "$OPENMUSE_WORD_PRODUCT_BUNDLE"
  require_component_file Word "$OPENMUSE_WORD_PRODUCT_PUBLIC_KEY"
  require_component_file Word "$OPENMUSE_WORD_PRODUCT_IDENTITY"
  if [[ "$profile" == "release" ]]; then
    require_component_file Word "$OPENMUSE_WORD_CORE_FFI_X86_64"
  fi
fi
if (( pdf_selected )); then
  OPENMUSE_PDF_PRODUCT_BUNDLE="${OPENMUSE_PDF_PRODUCT_BUNDLE:-$(current_bundle pdf-product pdf)}"
  OPENMUSE_PDF_PRODUCT_PUBLIC_KEY="${OPENMUSE_PDF_PRODUCT_PUBLIC_KEY:-$mobile_assets/pdf_product/public.pem}"
  OPENMUSE_PDF_PRODUCT_IDENTITY="${OPENMUSE_PDF_PRODUCT_IDENTITY:-$mobile_assets/pdf_product/identity.json}"
  require_component_file PDF "$OPENMUSE_PDF_PRODUCT_BUNDLE"
  require_component_file PDF "$OPENMUSE_PDF_PRODUCT_PUBLIC_KEY"
  require_component_file PDF "$OPENMUSE_PDF_PRODUCT_IDENTITY"
fi
if (( ppt_selected )); then
  OPENMUSE_PPTX_BUNDLE="${OPENMUSE_PPTX_BUNDLE:-$(current_bundle genoffice-pptx genoffice-pptx)}"
  require_component_file PPT "$OPENMUSE_PPTX_BUNDLE"
fi
if (( spreadsheet_selected )); then
  OPENMUSE_SPREADSHEET_CORE_FFI="${OPENMUSE_SPREADSHEET_CORE_FFI:-$office_root/samples/libs/spreadsheet/gridcraft_bridge/target/release/libopenmuse_gridcraft_bridge.dylib}"
  OPENMUSE_SPREADSHEET_CORE_FFI_X86_64="${OPENMUSE_SPREADSHEET_CORE_FFI_X86_64:-$office_root/samples/libs/spreadsheet/gridcraft_bridge/target/x86_64-apple-darwin/release/libopenmuse_gridcraft_bridge.dylib}"
  OPENMUSE_SPREADSHEET_PRODUCT_BUNDLE="${OPENMUSE_SPREADSHEET_PRODUCT_BUNDLE:-$(current_bundle spreadsheet-product spreadsheet)}"
  OPENMUSE_SPREADSHEET_PRODUCT_PUBLIC_KEY="${OPENMUSE_SPREADSHEET_PRODUCT_PUBLIC_KEY:-$mobile_assets/spreadsheet_product/public.pem}"
  OPENMUSE_SPREADSHEET_PRODUCT_IDENTITY="${OPENMUSE_SPREADSHEET_PRODUCT_IDENTITY:-$mobile_assets/spreadsheet_product/identity.json}"
  require_component_file Spreadsheet "$OPENMUSE_SPREADSHEET_CORE_FFI"
  require_component_file Spreadsheet "$OPENMUSE_SPREADSHEET_PRODUCT_BUNDLE"
  require_component_file Spreadsheet "$OPENMUSE_SPREADSHEET_PRODUCT_PUBLIC_KEY"
  require_component_file Spreadsheet "$OPENMUSE_SPREADSHEET_PRODUCT_IDENTITY"
  if [[ "$profile" == "release" ]]; then
    require_component_file Spreadsheet "$OPENMUSE_SPREADSHEET_CORE_FFI_X86_64"
  fi
fi

cd "$app_root"
"$repo_root/scripts/stage_helix.sh"
"$repo_root/scripts/stage_node_macos.sh"
if [[ ! -f "$dsh_closure/node_modules/@deepseek-ai/dsh/lib/bin.js" ]] ||
   [[ ! -f "$dsh_closure/node_modules/.bin/pnpm" ]] ||
   ! cmp -s "$repo_root/third_party/dsh/package.json" "$dsh_closure/package.json" ||
   ! cmp -s "$repo_root/third_party/dsh/package-lock.json" "$dsh_closure/package-lock.json"; then
  stale_closure="$dsh_closure.stale.$$"
  if [[ -d "$dsh_closure" ]]; then mv "$dsh_closure" "$stale_closure"; fi
  if python3 "$repo_root/scripts/build_dsh_closure.py" --out "$dsh_closure"; then
    if [[ -d "$stale_closure" ]]; then rm -rf "$stale_closure"; fi
  else
    rm -rf "$dsh_closure"
    if [[ -d "$stale_closure" ]]; then mv "$stale_closure" "$dsh_closure"; fi
    exit 1
  fi
fi
cmp "$repo_root/third_party/dsh/package.json" "$dsh_closure/package.json"
cmp "$repo_root/third_party/dsh/package-lock.json" "$dsh_closure/package-lock.json"
python3 "$repo_root/scripts/build_dsh_closure.py" --out "$dsh_closure" --validate-only
python3 "$repo_root/scripts/test_dsh_runtime.py" --node "$node_runtime/node" --closure "$dsh_closure"
if [[ "${OPENMUSE_CLEAN_BUILD:-1}" == "1" ]]; then
  flutter clean
fi
flutter pub get
mkdir -p "$app_root/macos/Flutter/ephemeral"
printf 'OPENMUSE_DISTRIBUTION_CHANNEL = %s\n' "$distribution" > \
  "$app_root/macos/Flutter/ephemeral/OpenMuse-Distribution.xcconfig"
flutter analyze
if [[ "$skip_tests" -eq 0 ]]; then
  flutter test
fi
runtime_config_dir="$(mktemp -d)"
trap 'rm -rf "$runtime_config_dir"' EXIT
OPENMUSE_UPDATE_CHANNEL="$update_channel" bash "$repo_root/scripts/release/prepare-runtime-config.sh" "${OPENMUSE_DEPLOYMENT_PROFILE:-default}" "$runtime_config_dir/defines.json"
flutter_build_args=("--${profile}" "--dart-define-from-file=$runtime_config_dir/defines.json")
if [[ "$distribution" == app-store ]]; then
  flutter_build_args+=("--dart-define=OPENMUSE_MAC_APP_STORE=true")
fi
if [[ -n "$build_name" ]]; then flutter_build_args+=("--build-name=$build_name"); fi
if [[ -n "$build_number" ]]; then flutter_build_args+=("--build-number=$build_number"); fi
if [[ "$distribution" == app-store ]]; then
  # The bundled Notes kernel currently supports Apple Silicon. Do not ship an
  # untested Intel slice that cannot load the native document engine.
  FLUTTER_XCODE_ARCHS=arm64 FLUTTER_XCODE_EXCLUDED_ARCHS=x86_64 \
    flutter build macos "${flutter_build_args[@]}"
else
  flutter build macos "${flutter_build_args[@]}"
fi
/usr/libexec/PlistBuddy -c "Add :OpenMuseDistributionChannel string $distribution" "$app_path/Contents/Info.plist" || \
  /usr/libexec/PlistBuddy -c "Set :OpenMuseDistributionChannel $distribution" "$app_path/Contents/Info.plist"
if [[ "$distribution" == app-store ]]; then
  install -m 644 "$app_root/macos/Runner/PrivacyInfo.xcprivacy" \
    "$app_path/Contents/Resources/PrivacyInfo.xcprivacy"
  python3 "$repo_root/scripts/release/stage-macos-store-menu.py" \
    --client "$repo_root" --app "$app_path"
fi

if (( pdf_selected )); then
  pdf_native_assets="$app_path/Contents/Frameworks/App.framework/Resources/flutter_assets/packages/pdf_rebuild_flutter/assets/native"
  for library in libkspfreetext.dylib libpdfium.dylib libpdfium_viewer.dylib; do
    require_component_file PDF "$pdf_native_assets/$library"
  done
fi

# Flutter may reuse a previous build directory. Clear selectable component
# resources first so --components none and partial selections stay exact.
rm -rf \
  "$app_path/Contents/Resources/openmuse/word-product" \
  "$app_path/Contents/Resources/openmuse/pdf-product" \
  "$app_path/Contents/Resources/openmuse/spreadsheet-product" \
  "$app_path/Contents/Resources/openmuse/genoffice-pptx"
rm -f \
  "$app_path/Contents/Frameworks/libword_core_ffi.dylib" \
  "$app_path/Contents/Frameworks/libopenmuse_gridcraft_bridge.dylib"

# Word's native model/layout library is delivered with the Host, while the
# installed Word product MCPB contains only signed UI resources and commands.
# Keep it optional for Host-only packages; Word validation sets this explicitly.
if (( word_selected )); then
  test -f "$OPENMUSE_WORD_CORE_FFI"
  word_engine_dest="$app_path/Contents/Frameworks/libword_core_ffi.dylib"
  if [[ -n "${OPENMUSE_WORD_CORE_FFI_X86_64:-}" ]]; then
    test -f "$OPENMUSE_WORD_CORE_FFI_X86_64"
    lipo -create "$OPENMUSE_WORD_CORE_FFI" "$OPENMUSE_WORD_CORE_FFI_X86_64" \
      -output "$word_engine_dest"
    chmod 755 "$word_engine_dest"
    lipo "$word_engine_dest" -verify_arch arm64 x86_64
  else
    install -m 755 "$OPENMUSE_WORD_CORE_FFI" "$word_engine_dest"
  fi
  codesign --force --sign - "$word_engine_dest"
fi

# Product chrome remains a separately signed resource package. A local
# validation build may bootstrap it from the app bundle; the Host verifies it
# with the pinned public key before registering the Word editor.
if (( word_selected )); then
  : "${OPENMUSE_WORD_PRODUCT_PUBLIC_KEY:?}"
  : "${OPENMUSE_WORD_PRODUCT_IDENTITY:?}"
  word_product_dest="$app_path/Contents/Resources/openmuse/word-product"
  mkdir -p "$word_product_dest"
  install -m 644 "$OPENMUSE_WORD_PRODUCT_BUNDLE" "$word_product_dest/word.mcpb"
  install -m 644 "$OPENMUSE_WORD_PRODUCT_PUBLIC_KEY" "$word_product_dest/public.pem"
  install -m 644 "$OPENMUSE_WORD_PRODUCT_IDENTITY" "$word_product_dest/identity.json"
  mkdir -p "$app_path/Contents/Resources/WordFonts"
  for font in Roboto-Regular.ttf NotoSansCJK-Regular.ttc LICENSE-Roboto.txt LICENSE-NotoSansCJK.txt; do
    install -m 644 "$office_root/samples/docx-mvp-render/fonts/$font" "$app_path/Contents/Resources/WordFonts/$font"
  done
fi
if (( pdf_selected )); then
  : "${OPENMUSE_PDF_PRODUCT_PUBLIC_KEY:?}"
  : "${OPENMUSE_PDF_PRODUCT_IDENTITY:?}"
  pdf_product_dest="$app_path/Contents/Resources/openmuse/pdf-product"
  mkdir -p "$pdf_product_dest"
  install -m 644 "$OPENMUSE_PDF_PRODUCT_BUNDLE" "$pdf_product_dest/pdf.mcpb"
  install -m 644 "$OPENMUSE_PDF_PRODUCT_PUBLIC_KEY" "$pdf_product_dest/public.pem"
  install -m 644 "$OPENMUSE_PDF_PRODUCT_IDENTITY" "$pdf_product_dest/identity.json"
fi
if (( spreadsheet_selected )); then
  test -f "$OPENMUSE_SPREADSHEET_CORE_FFI"
  spreadsheet_engine_dest="$app_path/Contents/Frameworks/libopenmuse_gridcraft_bridge.dylib"
  if [[ -f "$OPENMUSE_SPREADSHEET_CORE_FFI_X86_64" ]]; then
    lipo -create "$OPENMUSE_SPREADSHEET_CORE_FFI" "$OPENMUSE_SPREADSHEET_CORE_FFI_X86_64" \
      -output "$spreadsheet_engine_dest"
    chmod 755 "$spreadsheet_engine_dest"
    lipo "$spreadsheet_engine_dest" -verify_arch arm64 x86_64
  else
    install -m 755 "$OPENMUSE_SPREADSHEET_CORE_FFI" "$spreadsheet_engine_dest"
  fi
  codesign --force --sign - "$spreadsheet_engine_dest"
fi
if (( spreadsheet_selected )); then
  : "${OPENMUSE_SPREADSHEET_PRODUCT_PUBLIC_KEY:?}"
  : "${OPENMUSE_SPREADSHEET_PRODUCT_IDENTITY:?}"
  spreadsheet_product_dest="$app_path/Contents/Resources/openmuse/spreadsheet-product"
  mkdir -p "$spreadsheet_product_dest"
  install -m 644 "$OPENMUSE_SPREADSHEET_PRODUCT_BUNDLE" "$spreadsheet_product_dest/spreadsheet.mcpb"
  install -m 644 "$OPENMUSE_SPREADSHEET_PRODUCT_PUBLIC_KEY" "$spreadsheet_product_dest/public.pem"
  install -m 644 "$OPENMUSE_SPREADSHEET_PRODUCT_IDENTITY" "$spreadsheet_product_dest/identity.json"
fi
if (( ppt_selected )); then
  pptx_dest="$app_path/Contents/Resources/openmuse/genoffice-pptx"
  mkdir -p "$pptx_dest"
  install -m 644 "$OPENMUSE_PPTX_BUNDLE" "$pptx_dest/genoffice-pptx.mcpb"
  python3 - "$pptx_dest/genoffice-pptx.mcpb" "$pptx_dest/identity.json" <<'PY'
import hashlib, json, pathlib, sys, zipfile
bundle, output = map(pathlib.Path, sys.argv[1:])
with zipfile.ZipFile(bundle) as archive:
    manifest = json.loads(archive.read('manifest.json'))
if manifest['name'] != 'genoffice-pptx':
    raise SystemExit('Expected a GenOffice PPTX MCPB')
output.write_text(json.dumps({
    'version': manifest['version'],
    'sha256': hashlib.sha256(bundle.read_bytes()).hexdigest(),
}, indent=2) + '\n')
PY
fi

# The CLI plugin ships a native launcher beside the GUI. It remains callable
# from a regular terminal and is placed on PATH for the in-app console.
if [[ "$distribution" == direct ]]; then
dart build cli -t bin/openmuse.dart -o "$app_root/build/openmuse-cli"
cli_bundle="$app_root/build/openmuse-cli/bundle"
cli_dest="$app_path/Contents/Resources/openmuse/cli"
mkdir -p "$cli_dest"
rsync -a --delete "$cli_bundle/" "$cli_dest/"
test -x "$cli_dest/bin/openmuse"
for library in "$cli_dest"/lib/*.dylib; do
  if [[ -f "$library" ]]; then codesign --force --sign - "$library"; fi
done
codesign --force --sign - "$cli_dest/bin/openmuse"
else
  rm -rf "$app_path/Contents/Resources/openmuse/cli"
fi

if [[ "$skip_tests" -eq 0 ]]; then
  (
    cd "$repo_root/plugins/helix"
    flutter analyze
    OPENMUSE_REQUIRE_PTY_TEST=1 \
      OPENMUSE_PTY_LIBRARY_DIR="$app_path/Contents/Frameworks" \
      flutter test
  )
fi

test -d "$app_path"
bundle_id="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$app_path/Contents/Info.plist")"
test "$bundle_id" = "com.openmuseai.office"

bundled_helix="$app_path/Contents/Frameworks/App.framework/Resources/flutter_assets/packages/openmuse_helix_plugin/assets/engines/helix"
test -f "$bundled_helix/hx"
# Stage only the editor runtime needed by this distribution. Bundling every
# upstream theme would pull unrelated theme licenses into the product.
rsync -a --delete-excluded \
  --include='/languages.toml' \
  --include='/grammars/***' \
  --include='/queries/***' \
  --include='/themes/' \
  --include='/themes/onelight.toml' \
  --include='/themes/openmuse_dark.toml' \
  --exclude='*' \
  "$repo_root/plugins/helix/assets/engines/helix/runtime/" "$bundled_helix/runtime/"
bash "$repo_root/scripts/ci/stage-helix-grammars-macos.sh" "$bundled_helix/runtime"
test -f "$bundled_helix/runtime/themes/onelight.toml"
test -f "$bundled_helix/runtime/themes/openmuse_dark.toml"
test -f "$bundled_helix/config.toml"

test -x "$node_runtime/node"
test -f "$node_runtime/LICENSE"
test -f "$repo_root/third_party/dsh/LICENSE"
dsh_dest="$app_path/Contents/Resources/openmuse/dsh"
if [[ -L "$dsh_dest/node_modules" ]]; then
  rm "$dsh_dest/node_modules"
fi
mkdir -p "$dsh_dest/node_modules" "$dsh_dest/node/bin" "$dsh_dest/licenses"
rsync -a --delete "$dsh_closure/node_modules/" "$dsh_dest/node_modules/"
if [[ -L "$dsh_dest/node/bin/node" ]]; then
  rm "$dsh_dest/node/bin/node"
fi
install -m 755 "$node_runtime/node" "$dsh_dest/node/bin/node"
cp "$repo_root/third_party/dsh/LICENSE" "$dsh_dest/licenses/DSH-LICENSE"
cp "$node_runtime/LICENSE" "$dsh_dest/licenses/NODE-LICENSE"
"$dsh_dest/node/bin/node" "$dsh_dest/node_modules/@deepseek-ai/dsh/lib/bin.js" --version
test -f "$dsh_dest/node_modules/.bin/pnpm"
test -f "$dsh_dest/node_modules/pnpm/LICENSE"
test "$(PATH="$dsh_dest/node_modules/.bin:$dsh_dest/node/bin:/usr/bin:/bin" \
  "$dsh_dest/node_modules/.bin/pnpm" --version)" = "10.34.1"
codesign --force --sign - "$dsh_dest/node/bin/node"

update_dest="$app_path/Contents/Resources/openmuse/update"
mkdir -p "$update_dest"
install -m 755 "$repo_root/scripts/update/install_macos.sh" "$update_dest/install_macos.sh"
if [[ "$distribution" == app-store ]]; then
  # Slides uses the bundled Node interpreter and its self-contained server.
  # The Store document workspace does not ship the local agent/package manager.
  rm -rf "$update_dest" "$bundled_helix" "$dsh_dest/node_modules"
  codesign --force --sign - --entitlements "$app_root/macos/Runner/$entitlement_file" "$app_path"
fi

mkdir -p "$dist_root"
# Flutter's incremental macOS build can update App.framework without refreshing
# the outer ad-hoc app seal. Refresh only ad-hoc signatures; never replace a
# Developer ID signature here.
app_framework="$app_path/Contents/Frameworks/App.framework"
if ! codesign --verify --strict "$app_framework" >/dev/null 2>&1; then
  framework_signature="$(codesign -dv "$app_framework" 2>&1)"
  if [[ "$framework_signature" == *'Signature=adhoc'* ]]; then
    codesign --force --sign - "$app_framework"
  fi
fi
if ! codesign --verify --deep --strict "$app_path" >/dev/null 2>&1; then
  signature_details="$(codesign -dv "$app_path" 2>&1)"
  if [[ "$signature_details" == *'Signature=adhoc'* ]]; then
    # Re-signing without --entitlements drops the original file-selection and
    # networking permissions, even though the resulting seal verifies.
    codesign --force --sign - \
      --entitlements "$app_root/macos/Runner/$entitlement_file" "$app_path"
  fi
fi
codesign --verify --deep --strict "$app_path"
python3 "$repo_root/scripts/verify_macos_entitlements.py" \
  "$app_path" "$app_root/macos/Runner/$entitlement_file"
rm -rf "$dist_app_path"
ditto "$app_path" "$dist_app_path"
codesign --verify --deep --strict "$dist_app_path"
echo "$dist_app_path"
if (( package_zip )); then
  ditto -c -k --sequesterRsrc --keepParent "$dist_app_path" "$archive_path"
  echo "$archive_path"
fi
