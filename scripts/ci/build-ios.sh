#!/usr/bin/env bash
# Build the OpenMuse iOS release artifact.
#
# The build logic stays in scripts/test_ios_beta.sh: it compiles the Rust mobile
# artifacts, produces the device arm64 Runner.app with --no-codesign, and
# verifies the privacy manifests, the bundle identifier, the linked native
# symbols and the absence of any downloadable/desktop runtime. This wrapper adds
# the release staging only: dist/OpenMuse-iOS-unsigned.app.zip plus its gate
# report, SHA256SUMS.txt and build-info.txt.
#
# Signing: the gate asserts the artifact is NOT signed, because signing needs a
# distribution certificate and a provisioning profile. The staged zip is a
# build/QA artifact; a TestFlight or App Store release needs those credentials
# supplied through secrets, and then an .ipa.

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$repo_root"

force_rebuild=false

usage() {
  cat >&2 <<'USAGE'
usage: scripts/ci/build-ios.sh [--force-rebuild]

  --force-rebuild  Drop the mobile build outputs before the gate runs
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --force-rebuild) force_rebuild=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage; exit 2 ;;
  esac
done

[[ "$(uname -s)" == "Darwin" ]] || {
  echo 'the iOS gate needs macOS with Xcode' >&2
  exit 1
}

for command in flutter cargo nm plutil xcodebuild; do
  command -v "$command" >/dev/null 2>&1 || {
    echo "$command is not on PATH" >&2
    exit 1
  }
done

sha256() {
  if command -v sha256sum >/dev/null 2>&1; then
    sha256sum "$1" | awk '{print $1}'
  else
    shasum -a 256 "$1" | awk '{print $1}'
  fi
}

step() { printf '\n==> %s\n' "$1"; }

if [[ "$force_rebuild" == true ]]; then
  step 'force rebuild: dropping mobile build outputs'
  rm -rf target/ios-beta \
    target/office-docx target/office-viewers target/paired-relay
  (cd app/openmuse_mobile && flutter clean)
fi

step 'iOS release gate'
"$repo_root/scripts/test_ios_beta.sh"

source_zip="$repo_root/target/ios-beta/OpenMuse-iOS-Beta-unsigned.app.zip"
[[ -f "$source_zip" ]] || { echo "the iOS gate did not produce $source_zip" >&2; exit 1; }

step 'package'
mkdir -p dist
cp "$source_zip" dist/OpenMuse-iOS-unsigned.app.zip
if [[ -f target/ios-beta/report.md ]]; then
  cp target/ios-beta/report.md dist/ios-gate-report.md
fi
digest="$(sha256 dist/OpenMuse-iOS-unsigned.app.zip)"
printf '%s  %s\n' "$digest" 'OpenMuse-iOS-unsigned.app.zip' >dist/SHA256SUMS.txt

{
  echo 'product: OpenMuse'
  echo 'profile: release'
  echo "commit: $(git rev-parse HEAD)"
  echo "builtAt: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "flutter: $(flutter --version | head -1)"
  echo "rustc: $(rustc --version)"
  echo 'signing: none (no-codesign build gate)'
  echo 'domains: ios=build'
  echo "archive: OpenMuse-iOS-unsigned.app.zip sha256=$digest"
} >dist/build-info.txt

printf '\nios build complete\n'
printf '    dist/OpenMuse-iOS-unsigned.app.zip  %s MB  sha256 %s\n' \
  "$(awk "BEGIN { printf \"%.2f\", $(wc -c <dist/OpenMuse-iOS-unsigned.app.zip) / 1048576 }")" "$digest"
