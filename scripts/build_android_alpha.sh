#!/usr/bin/env bash
set -euo pipefail

# --skip-tests mirrors the package_macos.sh flag of the same name: `flutter
# analyze` stays a hard gate because it catches real compile errors, while the
# slower `flutter test` suite is skipped when only the artifact is wanted.
skip_tests=0
while [[ $# -gt 0 ]]; do
  case "$1" in
    --skip-tests) skip_tests=1; shift ;;
    -h|--help)
      echo "usage: build_android_alpha.sh [--skip-tests]"
      exit 0
      ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
alpha_dir="$repo_root/target/android-alpha"
keystore="$alpha_dir/openmuse-alpha.p12"
mkdir -p "$alpha_dir"
OPENMUSE_PAIRED_PLATFORMS=android "$repo_root/scripts/build_paired_relay_mobile_artifacts.sh"
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
if [[ "$skip_tests" -eq 0 ]]; then
  (cd "$repo_root/app/openmuse_mobile" && flutter analyze && flutter test)
else
  (cd "$repo_root/app/openmuse_mobile" && flutter analyze)
fi
flutter_build_args=(build apk --release --target-platform android-arm64)
if [[ -n "${OPENMUSE_MCP_REGISTRY_URL:-}" ]]; then
  flutter_build_args+=("--dart-define=OPENMUSE_MCP_REGISTRY_URL=$OPENMUSE_MCP_REGISTRY_URL")
fi
(cd "$repo_root/app/openmuse_mobile" && flutter "${flutter_build_args[@]}")
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
grep -q 'lib/arm64-v8a/libopenmuse_paired_relay.so' <<<"$apk_listing"
artifact_check_dir="$(mktemp -d)"
trap 'rm -rf "$artifact_check_dir"' EXIT
unzip -p "$apk" lib/arm64-v8a/libopenmuse_paired_relay.so > "$artifact_check_dir/libopenmuse_paired_relay.so"
# Capture each symbol table first. `nm -D ... | grep -q` under `set -o pipefail`
# fails spuriously the moment grep exits on its first match, because nm then
# dies of SIGPIPE and its 141 becomes the pipeline status.
paired_symbols="$(nm -D "$artifact_check_dir/libopenmuse_paired_relay.so")"
for symbol in abi_version device_public issue_offer begin_handshake confirm_handshake channel_seal channel_open native_handle_close; do
  grep -q "openmuse_paired_$symbol" <<<"$paired_symbols"
done
python3 "$repo_root/scripts/release/verify-mobile-closure.py" "$apk"
cp "$apk" "$alpha_dir/OpenMuse-Android-Alpha-arm64.apk"
shasum -a 256 "$alpha_dir/OpenMuse-Android-Alpha-arm64.apk" > "$alpha_dir/artifact.sha256"
