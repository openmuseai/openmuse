#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
(cd "$repo_root/app/openmuse_mobile" && flutter analyze && flutter test && flutter build ios --release --no-codesign)
app="$repo_root/app/openmuse_mobile/build/ios/iphoneos/Runner.app"
test -f "$app/PrivacyInfo.xcprivacy"
if find "$app" -type f | grep -Eiq 'node|helix|dsh-closure|sandbox-worker'; then
  echo "iOS bundle contains downloadable/native execution runtime" >&2
  exit 1
fi
