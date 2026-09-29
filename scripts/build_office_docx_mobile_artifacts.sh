#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

out_root="${OPENMUSE_DOCX_ARTIFACT_DIR:-$repo_root/target/office-docx}"
android_out="$out_root/android"
ios_out="$out_root/ios"
header="$repo_root/crates/openmuse-office-docx/include/openmuse_docx.h"

mkdir -p "$android_out" "$ios_out"

cargo ndk -t arm64-v8a -o "$android_out" \
  build --release --package openmuse-office-docx

android_library="$android_out/arm64-v8a/libopenmuse_office_docx.so"
test -f "$android_library"
file "$android_library" | grep -q 'ARM aarch64'

for target in aarch64-apple-ios aarch64-apple-ios-sim; do
  if ! rustup target list --installed | grep -qx "$target"; then
    rustup target add "$target"
  fi
  cargo build --release --package openmuse-office-docx --target "$target"
done

device_library="$repo_root/target/aarch64-apple-ios/release/libopenmuse_office_docx.a"
simulator_library="$repo_root/target/aarch64-apple-ios-sim/release/libopenmuse_office_docx.a"
test -f "$device_library"
test -f "$simulator_library"

xcframework="$ios_out/OpenMuseDocx.xcframework"
if [[ -e "$xcframework" ]]; then
  artifact_backup="$ios_out/OpenMuseDocx.xcframework.previous"
  if [[ -e "$artifact_backup" ]]; then rm -rf "$artifact_backup"; fi
  mv "$xcframework" "$artifact_backup"
fi
xcodebuild -create-xcframework \
  -library "$device_library" -headers "$(dirname "$header")" \
  -library "$simulator_library" -headers "$(dirname "$header")" \
  -output "$xcframework"
rm -rf "$ios_out/OpenMuseDocx.xcframework.previous"

test -f "$xcframework/Info.plist"
grep -q 'OPENMUSE_DOCX_ABI_VERSION 1u' "$header"

manifest="$out_root/artifacts.sha256"
(
  cd "$out_root"
  shasum -a 256 "android/arm64-v8a/libopenmuse_office_docx.so"
  find "ios/OpenMuseDocx.xcframework" -type f -print0 | sort -z | xargs -0 shasum -a 256
) > "$manifest"

echo "DOCX Android artifact: $android_library"
echo "DOCX iOS artifact: $xcframework"
echo "DOCX artifact manifest: $manifest"
