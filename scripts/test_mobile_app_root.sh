#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"
gradle_home="${OPENMUSE_GRADLE_HOME:-$repo_root/target/mobile-gradle-cache}"
export GRADLE_USER_HOME="$gradle_home"
"$repo_root/scripts/build_office_docx_mobile_artifacts.sh"
"$repo_root/scripts/build_office_viewers_mobile_artifacts.sh"

(cd packages/openmuse_host_shell && flutter analyze && flutter test)
(cd app/openmuse_mobile && flutter analyze && flutter test && flutter build apk --debug && flutter build ios --debug --no-codesign)
(cd app/openmuse_host && flutter analyze && flutter test)

apk="app/openmuse_mobile/build/app/outputs/flutter-apk/app-debug.apk"
test -f "$apk"
apk_listing="$(unzip -Z1 "$apk")"
if grep -Eiq '(^|/)(node|helix|pty)(/|$)|libnode|flutter_pty' <<<"$apk_listing"; then
  echo "Mobile artifact unexpectedly contains a Desktop runtime" >&2
  exit 1
fi
grep -q 'lib/arm64-v8a/libopenmuse_office_docx.so' <<<"$apk_listing"
if rg -n "openmuse_dsh_plugin|openmuse_helix_plugin|flutter_pty|NativeProcess" \
  app/openmuse_mobile/lib app/openmuse_mobile/pubspec.yaml packages/openmuse_host_shell; then
  echo "Mobile composition imports a concrete Desktop Plugin/runtime" >&2
  exit 1
fi
