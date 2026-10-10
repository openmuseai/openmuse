#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
alpha_dir="$repo_root/target/android-alpha"
keystore="$alpha_dir/openmuse-alpha.p12"
mkdir -p "$alpha_dir"
OPENMUSE_DOCX_PLATFORMS=android "$repo_root/scripts/build_office_docx_mobile_artifacts.sh"
OPENMUSE_PAIRED_PLATFORMS=android "$repo_root/scripts/build_paired_relay_mobile_artifacts.sh"
OPENMUSE_VIEWERS_PLATFORMS=android "$repo_root/scripts/build_office_viewers_mobile_artifacts.sh"
if [[ ! -f "$keystore" ]]; then
  keytool -genkeypair -keystore "$keystore" -storetype PKCS12 -storepass openmuse-alpha \
    -alias openmuse-alpha -keypass openmuse-alpha -keyalg RSA -keysize 3072 -validity 3650 \
    -dname "CN=OpenMuse Internal Alpha,O=OpenMuse,OU=Mobile" >/dev/null
fi
export GRADLE_USER_HOME="$repo_root/target/mobile-gradle-cache"
export OPENMUSE_ANDROID_KEYSTORE="$keystore"
export OPENMUSE_ANDROID_STORE_PASSWORD="openmuse-alpha"
export OPENMUSE_ANDROID_KEY_ALIAS="openmuse-alpha"
export OPENMUSE_ANDROID_KEY_PASSWORD="openmuse-alpha"
(cd "$repo_root/app/openmuse_mobile" && flutter analyze && flutter test && flutter build apk --release --target-platform android-arm64)
apk="$repo_root/app/openmuse_mobile/build/app/outputs/flutter-apk/app-release.apk"
# The SDK ships several build-tools revisions and the newest one is not a fixed
# number across developer machines and CI images, so resolve apksigner instead
# of pinning a path that only exists on one box.
: "${ANDROID_HOME:?ANDROID_HOME must point at the Android SDK}"
apksigner=""
for revision in $(ls "$ANDROID_HOME/build-tools" 2>/dev/null | sort -t. -k1,1n -k2,2n -k3,3n); do
  if [[ -x "$ANDROID_HOME/build-tools/$revision/apksigner" ]]; then
    apksigner="$ANDROID_HOME/build-tools/$revision/apksigner"
  fi
done
if [[ -z "$apksigner" ]]; then
  echo "apksigner is missing under $ANDROID_HOME/build-tools" >&2
  exit 1
fi
"$apksigner" verify --verbose --print-certs "$apk" | tee "$alpha_dir/apksigner-report.txt"
apk_listing="$(unzip -Z1 "$apk")"
if grep -Eiq '(^|/)(node|helix|pty)(/|$)|libnode|flutter_pty|dsh-closure' <<<"$apk_listing"; then
  echo "Android Alpha contains a Desktop runtime" >&2
  exit 1
fi
grep -q 'lib/arm64-v8a/libopenmuse_office_docx.so' <<<"$apk_listing"
grep -q 'lib/arm64-v8a/libopenmuse_paired_relay.so' <<<"$apk_listing"
grep -q 'lib/arm64-v8a/libopenmuse_office_viewers.so' <<<"$apk_listing"
artifact_check_dir="$(mktemp -d)"
trap 'rm -rf "$artifact_check_dir"' EXIT
unzip -p "$apk" lib/arm64-v8a/libopenmuse_office_docx.so > "$artifact_check_dir/libopenmuse_office_docx.so"
unzip -p "$apk" lib/arm64-v8a/libopenmuse_paired_relay.so > "$artifact_check_dir/libopenmuse_paired_relay.so"
unzip -p "$apk" lib/arm64-v8a/libopenmuse_office_viewers.so > "$artifact_check_dir/libopenmuse_office_viewers.so"
for symbol in abi_version inspect export_simple buffer_free; do
  nm -D "$artifact_check_dir/libopenmuse_office_docx.so" | grep -q "openmuse_docx_$symbol"
done
for symbol in abi_version device_public issue_offer begin_handshake confirm_handshake channel_seal channel_open native_handle_close; do
  nm -D "$artifact_check_dir/libopenmuse_paired_relay.so" | grep -q "openmuse_paired_$symbol"
done
for symbol in office_viewers_abi_version xlsx_inspect pptx_inspect pdf_inspect office_viewer_buffer_free; do
  nm -D "$artifact_check_dir/libopenmuse_office_viewers.so" | grep -q "openmuse_$symbol"
done
cp "$apk" "$alpha_dir/OpenMuse-Android-Alpha-arm64.apk"
shasum -a 256 "$alpha_dir/OpenMuse-Android-Alpha-arm64.apk" > "$alpha_dir/artifact.sha256"
