#!/usr/bin/env bash
# Print China app-filing identity fields for Android, iOS, and Harmony.
#
#   ./scripts/filing/app-filing-identity.sh all
#   ./scripts/filing/app-filing-identity.sh android --apk dist/OpenMuse-Android-arm64.apk
#   OPENMUSE_ANDROID_STORE_PASSWORD=... ./scripts/filing/app-filing-identity.sh android --keystore release.p12
#   ./scripts/filing/app-filing-identity.sh ios --cert distribution.cer
#   OPENMUSE_IOS_P12_PASSWORD=... ./scripts/filing/app-filing-identity.sh ios --p12 distribution.p12
#   ./scripts/filing/app-filing-identity.sh harmony --cert harmony.cer --bundle-id com.example.app
#
# Passwords are read from the environment and are not printed:
#   OPENMUSE_ANDROID_STORE_PASSWORD, OPENMUSE_ANDROID_KEY_ALIAS
#   OPENMUSE_IOS_P12_PASSWORD
#   OPENMUSE_HARMONY_P12_PASSWORD
set -euo pipefail

script_dir="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
exec python3 "${script_dir}/app_filing_identity.py" "$@"
