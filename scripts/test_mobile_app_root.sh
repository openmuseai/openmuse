#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"
gradle_home="${OPENMUSE_GRADLE_HOME:-$repo_root/target/mobile-gradle-cache}"
export GRADLE_USER_HOME="$gradle_home"

(cd packages/openmuse_host_shell && flutter analyze && flutter test)
(cd app/openmuse_mobile && flutter analyze && flutter test && flutter build apk --debug && flutter build ios --debug --no-codesign)
(cd app/openmuse_host && flutter analyze && flutter test)

apk="app/openmuse_mobile/build/app/outputs/flutter-apk/app-debug.apk"
test -f "$apk"
if unzip -l "$apk" | grep -Eiq '(^|/)(node|helix|pty)(/|$)|libnode|flutter_pty'; then
  echo "Mobile artifact unexpectedly contains a Desktop runtime" >&2
  exit 1
fi
if rg -n "openmuse_dsh_plugin|openmuse_helix_plugin|flutter_pty|NativeProcess" \
  app/openmuse_mobile/lib app/openmuse_mobile/pubspec.yaml packages/openmuse_host_shell; then
  echo "Mobile composition imports a concrete Desktop Plugin/runtime" >&2
  exit 1
fi
