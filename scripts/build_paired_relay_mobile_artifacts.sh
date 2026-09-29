#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

out_root="${OPENMUSE_PAIRED_ARTIFACT_DIR:-$repo_root/target/paired-relay}"
platforms="${OPENMUSE_PAIRED_PLATFORMS:-all}"
android_out="$out_root/android"
ios_out="$out_root/ios"
header="$repo_root/crates/openmuse-paired-relay/include/openmuse_paired.h"

if [[ "$platforms" != "all" && "$platforms" != "android" && "$platforms" != "ios" ]]; then
  echo "OPENMUSE_PAIRED_PLATFORMS must be all, android, or ios" >&2
  exit 1
fi

mkdir -p "$android_out" "$ios_out"

if [[ "$platforms" == "all" || "$platforms" == "android" ]]; then
  cargo ndk -t arm64-v8a -o "$android_out" \
    build --release --package openmuse-paired-relay
  android_library="$android_out/arm64-v8a/libopenmuse_paired_relay.so"
  test -f "$android_library"
  file "$android_library" | grep -q 'ARM aarch64'
fi

if [[ "$platforms" == "all" || "$platforms" == "ios" ]]; then
  for target in aarch64-apple-ios aarch64-apple-ios-sim; do
    if ! rustup target list --installed | grep -qx "$target"; then
      rustup target add "$target"
    fi
    cargo build --release --package openmuse-paired-relay --target "$target"
  done

  device_library="$repo_root/target/aarch64-apple-ios/release/libopenmuse_paired_relay.a"
  simulator_library="$repo_root/target/aarch64-apple-ios-sim/release/libopenmuse_paired_relay.a"
  test -f "$device_library"
  test -f "$simulator_library"

  xcframework="$ios_out/OpenMusePaired.xcframework"
  if [[ -e "$xcframework" ]]; then
    artifact_backup="$ios_out/OpenMusePaired.xcframework.previous"
    if [[ -e "$artifact_backup" ]]; then rm -rf "$artifact_backup"; fi
    mv "$xcframework" "$artifact_backup"
  fi
  xcodebuild -create-xcframework \
    -library "$device_library" -headers "$(dirname "$header")" \
    -library "$simulator_library" -headers "$(dirname "$header")" \
    -output "$xcframework"
  rm -rf "$ios_out/OpenMusePaired.xcframework.previous"
  test -f "$xcframework/Info.plist"
fi

grep -q 'OPENMUSE_PAIRED_ABI_VERSION 1u' "$header"

manifest="$out_root/artifacts.sha256"
: >"$manifest"
if [[ -n "${android_library:-}" ]]; then
  (cd "$out_root" && shasum -a 256 "android/arm64-v8a/libopenmuse_paired_relay.so") >>"$manifest"
fi
if [[ -n "${xcframework:-}" ]]; then
  (
    cd "$out_root"
    find "ios/OpenMusePaired.xcframework" -type f -print0 | sort -z | xargs -0 shasum -a 256
  ) >>"$manifest"
fi

if [[ -n "${android_library:-}" ]]; then echo "Paired Android artifact: $android_library"; fi
if [[ -n "${xcframework:-}" ]]; then echo "Paired iOS artifact: $xcframework"; fi
echo "Paired artifact manifest: $manifest"
