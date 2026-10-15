#!/usr/bin/env bash
set -euo pipefail

profile="release"
skip_tests=0
build_name=""
build_number=""
update_channel="stable"
while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile) profile="$2"; shift 2 ;;
    --skip-tests) skip_tests=1; shift ;;
    --build-name) build_name="$2"; shift 2 ;;
    --build-number) build_number="$2"; shift 2 ;;
    --update-channel) update_channel="$2"; shift 2 ;;
    -h|--help)
      echo "usage: package_macos.sh [--profile release|debug] [--skip-tests] [--build-name x.y.z] [--build-number n] [--update-channel stable|canary]"
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
case "$profile" in
  release) product_dir="Release" ;;
  debug) product_dir="Debug" ;;
  *) echo "profile must be release or debug" >&2; exit 2 ;;
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
app_root="$repo_root/app/openmuse_host"
dist_root="$repo_root/dist"
app_path="$app_root/build/macos/Build/Products/${product_dir}/OpenMuse.app"
archive_path="$dist_root/OpenMuse-macos.zip"
dsh_closure="$repo_root/target/dsh-closure"
node_runtime="$repo_root/target/node-v22.19.0-universal"

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
flutter analyze
if [[ "$skip_tests" -eq 0 ]]; then
  flutter test
fi
flutter_build_args=("--${profile}" "--dart-define=OPENMUSE_UPDATE_CHANNEL=$update_channel")
if [[ -n "$build_name" ]]; then flutter_build_args+=("--build-name=$build_name"); fi
if [[ -n "$build_number" ]]; then flutter_build_args+=("--build-number=$build_number"); fi
flutter build macos "${flutter_build_args[@]}"

# The CLI plugin ships a native launcher beside the GUI. It remains callable
# from a regular terminal and is placed on PATH for the in-app console.
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
    codesign --force --sign - "$app_path"
  fi
fi
codesign --verify --deep --strict "$app_path"
ditto -c -k --sequesterRsrc --keepParent "$app_path" "$archive_path"
echo "$archive_path"
