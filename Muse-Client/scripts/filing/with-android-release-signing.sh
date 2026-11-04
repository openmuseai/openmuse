#!/usr/bin/env bash
# Run a command with the OpenMuse Android release key from this Mac's login Keychain.
set -euo pipefail

if [[ $# -eq 0 ]]; then
  echo "Usage: $0 COMMAND [ARG ...]" >&2
  exit 2
fi

keystore="$HOME/Library/Application Support/OpenMuse/Signing/Android/openmuse-android-release-2026.p12"
if [[ ! -f "$keystore" ]]; then
  echo "Android release keystore is missing: $keystore" >&2
  exit 2
fi

password="$(security find-generic-password \
  -a com.openmuseai.mobile \
  -s 'OpenMuse Android release keystore' -w)"

export OPENMUSE_ANDROID_KEYSTORE="$keystore"
export OPENMUSE_ANDROID_STORE_PASSWORD="$password"
export OPENMUSE_ANDROID_KEY_ALIAS=openmuse-release
export OPENMUSE_ANDROID_KEY_PASSWORD="$password"
unset password

exec "$@"
