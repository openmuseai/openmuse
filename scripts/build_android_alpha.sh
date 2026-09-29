#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
alpha_dir="$repo_root/target/android-alpha"
keystore="$alpha_dir/openmuse-alpha.p12"
mkdir -p "$alpha_dir"
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
apksigner="$ANDROID_HOME/build-tools/36.0.0/apksigner"
"$apksigner" verify --verbose --print-certs "$apk" | tee "$alpha_dir/apksigner-report.txt"
if unzip -l "$apk" | grep -Eiq '(^|/)(node|helix|pty)(/|$)|libnode|flutter_pty|dsh-closure'; then
  echo "Android Alpha contains a Desktop runtime" >&2
  exit 1
fi
cp "$apk" "$alpha_dir/OpenMuse-Android-Alpha-arm64.apk"
