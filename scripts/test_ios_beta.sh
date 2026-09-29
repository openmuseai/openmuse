#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
evidence_dir="$repo_root/target/ios-beta"
rm -rf "$evidence_dir"
mkdir -p "$evidence_dir"

OPENMUSE_DOCX_PLATFORMS=ios "$repo_root/scripts/build_office_docx_mobile_artifacts.sh"
OPENMUSE_PAIRED_PLATFORMS=ios "$repo_root/scripts/build_paired_relay_mobile_artifacts.sh"

(cd "$repo_root/app/openmuse_mobile" && flutter analyze && flutter test && flutter build ios --release --no-codesign)

app="$repo_root/app/openmuse_mobile/build/ios/iphoneos/Runner.app"
test -f "$app/PrivacyInfo.xcprivacy"
test -f "$app/Frameworks/Flutter.framework/PrivacyInfo.xcprivacy"
test -f "$app/Runner"
plutil -lint \
  "$app/Info.plist" \
  "$app/PrivacyInfo.xcprivacy" \
  "$app/Frameworks/Flutter.framework/PrivacyInfo.xcprivacy" \
  | tee "$evidence_dir/plist-lint.txt"

bundle_id="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$app/Info.plist")"
version="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$app/Info.plist")"
build="$(/usr/libexec/PlistBuddy -c 'Print :CFBundleVersion' "$app/Info.plist")"
if [[ "$bundle_id" != "io.openmuse.mobile" ]]; then
  echo "Unexpected iOS bundle identifier: $bundle_id" >&2
  exit 1
fi

file "$app/Runner" "$app/Frameworks/App.framework/App" "$app/Frameworks/Flutter.framework/Flutter" \
  | tee "$evidence_dir/architectures.txt"
if ! grep -q 'Runner:.*arm64' "$evidence_dir/architectures.txt"; then
  echo "Runner does not contain the device arm64 architecture" >&2
  exit 1
fi

if find "$app" -type f | grep -Eiq '(^|/)(node|helix|dsh-closure|sandbox-worker)(/|$)|libnode'; then
  echo "iOS bundle contains downloadable/native execution runtime" >&2
  exit 1
fi
nm -gU "$app/Runner" | grep -q '_openmuse_docx_abi_version'
nm -gU "$app/Runner" | grep -q '_openmuse_docx_inspect'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_abi_version'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_device_public'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_issue_offer'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_buffer_free'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_begin_handshake'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_confirm_handshake'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_channel_seal'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_channel_open'
nm -gU "$app/Runner" | grep -q '_openmuse_paired_native_handle_close'

if codesign --verify --deep --strict "$app" 2>"$evidence_dir/codesign.txt"; then
  echo "The no-codesign artifact unexpectedly has a valid signature" >&2
  exit 1
fi
grep -q 'code object is not signed at all' "$evidence_dir/codesign.txt"

(cd "$app" && find . -type f | LC_ALL=C sort | while IFS= read -r path; do shasum -a 256 "$path"; done) \
  >"$evidence_dir/bundle-files.sha256"
ditto -c -k --sequesterRsrc --keepParent "$app" "$evidence_dir/OpenMuse-iOS-Beta-unsigned.app.zip"
shasum -a 256 "$evidence_dir/OpenMuse-iOS-Beta-unsigned.app.zip" >"$evidence_dir/artifact.sha256"

artifact_digest="$(awk '{print $1}' "$evidence_dir/artifact.sha256")"
{
  echo "# OpenMuse iOS unsigned device build gate"
  echo
  echo "- Result: PASS"
  echo "- Bundle identifier: $bundle_id"
  echo "- Version/build: $version ($build)"
  echo "- Architecture: arm64"
  echo "- Privacy manifests: valid plist"
  echo "- Downloadable/Desktop execution runtime scan: PASS"
  echo "- Signing: intentionally absent; not installable on a physical device or TestFlight"
  echo "- Artifact SHA-256: $artifact_digest"
} >"$evidence_dir/report.md"

echo "iOS no-codesign build gate PASS: $evidence_dir"
