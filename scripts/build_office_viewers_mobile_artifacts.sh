#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

out_root="${OPENMUSE_VIEWERS_ARTIFACT_DIR:-$repo_root/target/office-viewers}"
platforms="${OPENMUSE_VIEWERS_PLATFORMS:-all}"
android_out="$out_root/android"
ios_out="$out_root/ios"
header="$repo_root/crates/openmuse-office-viewers/include/openmuse_office_viewers.h"

if [[ "$platforms" != "all" && "$platforms" != "android" && "$platforms" != "ios" ]]; then
  echo "OPENMUSE_VIEWERS_PLATFORMS must be all, android, or ios" >&2
  exit 1
fi
mkdir -p "$android_out" "$ios_out"

if [[ "$platforms" == "all" || "$platforms" == "android" ]]; then
  cargo ndk -t arm64-v8a -o "$android_out" \
    build --release --package openmuse-office-viewers
  android_library="$android_out/arm64-v8a/libopenmuse_office_viewers.so"
  test -f "$android_library"
  file "$android_library" | grep -q 'ARM aarch64'
fi

if [[ "$platforms" == "all" || "$platforms" == "ios" ]]; then
  for target in aarch64-apple-ios aarch64-apple-ios-sim; do
    if ! rustup target list --installed | grep -qx "$target"; then rustup target add "$target"; fi
    cargo build --release --package openmuse-office-viewers --target "$target"
  done
  device_library="$repo_root/target/aarch64-apple-ios/release/libopenmuse_office_viewers.a"
  simulator_library="$repo_root/target/aarch64-apple-ios-sim/release/libopenmuse_office_viewers.a"
  xcframework="$ios_out/OpenMuseOfficeViewers.xcframework"
  if [[ -e "$xcframework" ]]; then
    backup="$ios_out/OpenMuseOfficeViewers.xcframework.previous"
    if [[ -e "$backup" ]]; then rm -rf "$backup"; fi
    mv "$xcframework" "$backup"
  fi
  xcodebuild -create-xcframework \
    -library "$device_library" -headers "$(dirname "$header")" \
    -library "$simulator_library" -headers "$(dirname "$header")" \
    -output "$xcframework"
  rm -rf "$ios_out/OpenMuseOfficeViewers.xcframework.previous"
  test -f "$xcframework/Info.plist"
fi

grep -q 'OPENMUSE_OFFICE_VIEWERS_ABI_VERSION 1u' "$header"
manifest="$out_root/artifacts.sha256"
: >"$manifest"
if [[ -n "${android_library:-}" ]]; then
  (cd "$out_root" && shasum -a 256 "android/arm64-v8a/libopenmuse_office_viewers.so") >>"$manifest"
fi
if [[ -n "${xcframework:-}" ]]; then
  (cd "$out_root" && find ios/OpenMuseOfficeViewers.xcframework -type f -print0 | sort -z | xargs -0 shasum -a 256) >>"$manifest"
fi
echo "Office viewers artifact manifest: $manifest"
