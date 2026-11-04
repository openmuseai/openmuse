#!/usr/bin/env bash
# Archive an iOS candidate with explicit language and filing policy for US or China.
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
version=''
build_number=''
market='us'
allow_dirty=false
skip_tests=false
while [[ $# -gt 0 ]]; do
  case "$1" in
    --market) market="${2:-}"; shift 2 ;;
    --version) version="${2:-}"; shift 2 ;;
    --build-number) build_number="${2:-}"; shift 2 ;;
    --allow-dirty) allow_dirty=true; shift ;;
    --skip-tests) skip_tests=true; shift ;;
    *) echo "Unknown argument: $1" >&2; exit 2 ;;
  esac
done
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && "$version" != 0.0.1 ]] || {
  echo 'Set an approved release version X.Y.Z.' >&2; exit 2;
}
[[ "$build_number" =~ ^[1-9][0-9]*$ ]] || {
  echo 'Set a positive, increasing build number.' >&2; exit 2;
}
[[ "$market" == us || "$market" == cn ]] || { echo 'Use --market us or --market cn.' >&2; exit 2; }
deployment_profile=global-us
if [[ "$market" == cn ]]; then deployment_profile=china; fi
cd "$repo_root"
python3 "$repo_root/scripts/release/verify-welcome-guide.py"
dirty=false
if [[ -n "$(git status --porcelain)" ]]; then dirty=true; fi
if [[ "$dirty" == true && "$allow_dirty" != true ]]; then
  echo 'Commit and tag the release source first.' >&2; exit 2;
fi

identity='12C7FF3A2A764F0442A08B9FE77EC056B81841F6'
team='SDPXS943DW'
profile_name='OpenMuse Mobile App Store 2026'
profile="${OPENMUSE_IOS_PROFILE_PATH:-$HOME/Library/Developer/Xcode/UserData/Provisioning Profiles/c7bdea16-7c2a-4be6-a7d7-bc9fb1241df0.mobileprovision}"
[[ -f "$profile" ]] || { echo "Missing App Store profile: $profile" >&2; exit 2; }
security find-identity -v -p codesigning | grep -q "$identity" || {
  echo 'The filing Apple Distribution identity is not available in Keychain.' >&2; exit 2;
}
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT
bash "$repo_root/scripts/release/prepare-runtime-config.sh" "$deployment_profile" "$tmp/runtime-defines.json"
runtime_validation_args=()
if [[ "$allow_dirty" == true ]]; then runtime_validation_args+=(--allow-incomplete); fi
python3 "$repo_root/scripts/release/verify-runtime-input.py" "$tmp/runtime-defines.json" \
  --report "$tmp/runtime-input-report.json" "${runtime_validation_args[@]}"
python3 "$repo_root/scripts/release/verify-mobile-closure.py" "$repo_root/app/openmuse_mobile/.dart_tool/package_config.json"
security cms -D -i "$profile" >"$tmp/profile.plist"
python3 - "$tmp/profile.plist" "$profile_name" "$team" <<'PY'
import plistlib, sys
with open(sys.argv[1], 'rb') as f:
    profile = plistlib.load(f)
assert profile['Name'] == sys.argv[2], 'Unexpected provisioning profile name'
assert sys.argv[3] in profile['TeamIdentifier'], 'Unexpected Team ID'
assert profile['Entitlements']['application-identifier'] == f'{sys.argv[3]}.com.openmuseai.mobile', 'Unexpected App ID'
assert 'ProvisionedDevices' not in profile, 'Development or ad hoc profile is not an App Store profile'
PY

OPENMUSE_PAIRED_PLATFORMS=ios "$repo_root/scripts/build_paired_relay_mobile_artifacts.sh"
(cd app/openmuse_mobile && flutter analyze)
if [[ "$skip_tests" != true ]]; then
  (cd app/openmuse_mobile && flutter test)
fi


cat >"$tmp/export-options.plist" <<PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>method</key><string>app-store-connect</string>
<key>teamID</key><string>$team</string>
<key>signingStyle</key><string>manual</string>
<key>signingCertificate</key><string>Apple Distribution</string>
<key>provisioningProfiles</key><dict>
<key>com.openmuseai.mobile</key><string>$profile_name</string>
</dict>
</dict></plist>
PLIST

(cd app/openmuse_mobile && flutter build ipa --release --target lib/main.dart \
  --build-name "$version" --build-number "$build_number" \
  "--dart-define=OPENMUSE_DISTRIBUTION_CHANNEL=ios-store" \
  "--dart-define-from-file=$tmp/runtime-defines.json" \
  --export-options-plist "$tmp/export-options.plist")

ipa_dir="$repo_root/app/openmuse_mobile/build/ios/ipa"
ipa_count="$(find "$ipa_dir" -maxdepth 1 -name '*.ipa' -type f | wc -l | tr -d ' ')"
[[ "$ipa_count" == 1 ]] || { echo 'Expected exactly one exported IPA.' >&2; exit 1; }
source_ipa="$(find "$ipa_dir" -maxdepth 1 -name '*.ipa' -type f -print -quit)"
unzip -q "$source_ipa" -d "$tmp/ipa"
app="$tmp/ipa/Payload/Runner.app"
[[ -d "$app" ]] || { echo 'IPA is missing Runner.app.' >&2; exit 1; }
python3 "$repo_root/scripts/release/verify-ios-bundle.py" "$app"
python3 "$repo_root/scripts/release/verify-mobile-closure.py" "$app"
codesign --verify --deep --strict "$app"
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$app/Info.plist")" == com.openmuseai.mobile ]] || {
  echo 'Exported IPA has an unexpected Bundle ID.' >&2; exit 1;
}
codesign -dvv "$app" 2>"$tmp/codesign.txt"
grep -q "TeamIdentifier=$team" "$tmp/codesign.txt" || {
  echo 'Exported IPA has an unexpected signing Team.' >&2; exit 1;
}
codesign -d --extract-certificates="$tmp/app-cert-" "$app" >/dev/null 2>&1
actual_identity="$(openssl x509 -inform DER -in "$tmp/app-cert-0" -noout -fingerprint -sha1 | cut -d= -f2 | tr -d ': ' | tr '[:lower:]' '[:upper:]')"
[[ "$actual_identity" == "$identity" ]] || {
  echo "Exported IPA certificate does not match the filing certificate: $actual_identity" >&2; exit 1;
}
[[ -f "$app/embedded.mobileprovision" ]] || {
  echo 'Exported IPA is missing its provisioning profile.' >&2; exit 1;
}
security cms -D -i "$app/embedded.mobileprovision" >"$tmp/embedded-profile.plist"
python3 - "$tmp/embedded-profile.plist" "$profile_name" "$team" "$identity" <<'PY'
import hashlib, plistlib, sys
with open(sys.argv[1], 'rb') as f:
    profile = plistlib.load(f)
assert profile['Name'] == sys.argv[2], 'Exported IPA has an unexpected profile'
assert sys.argv[3] in profile['TeamIdentifier'], 'Exported IPA profile has an unexpected Team'
assert profile['Entitlements']['application-identifier'] == f'{sys.argv[3]}.com.openmuseai.mobile', 'Exported IPA profile has an unexpected App ID'
assert sys.argv[4].lower() in [hashlib.sha1(c).hexdigest() for c in profile['DeveloperCertificates']], 'Exported IPA profile does not include the filing certificate'
PY

output_dir="$repo_root/dist/mobile/${version}-${build_number}/ios-$market"
mkdir -p "$output_dir"
cp "${tmp}/runtime-input-report.json" "$output_dir/runtime-input-report.json"
artifact="$output_dir/OpenMuse-ios-${market}-${version}-${build_number}.ipa"
cp "$source_ipa" "$artifact"
cp "$tmp/codesign.txt" "$output_dir/signature-report.txt"
python3 - "$artifact" "$output_dir/release.json" "$version" "$build_number" "$identity" "$dirty" "$skip_tests" "${OPENMUSE_APP_FILING_NUMBER:-}" "$market" "$tmp/runtime-defines.json" <<'PY'
import hashlib, json, pathlib, subprocess, sys
artifact, output, version, number, cert, dirty, skipped, filing, market, config_path = sys.argv[1:]
release = json.loads(json.loads(pathlib.Path(config_path).read_text())['OPENMUSE_RELEASE_JSON'])
snapshot = next(region['bundle']['snapshot'] for region in release['regions'] if region['available'])
language = snapshot['localization']['defaultLocale']
filing = snapshot.get('regulatory', {}).get('appFilingNumber', '')
runtime_report = json.loads(pathlib.Path(output).with_name('runtime-input-report.json').read_text())
record = {
    'status': 'internal-test-only' if not runtime_report['productionInputsComplete'] or dirty == 'true' or skipped == 'true' or (market == 'cn' and not filing) else 'store-candidate',
    'lane': f'ios-{market}',
    'defaultLanguage': language,
    'targetStorefronts': ['US'] if market == 'us' else ['CN'],
    'showChinaFiling': bool(filing),
    'artifact': pathlib.Path(artifact).name,
    'sha256': hashlib.sha256(pathlib.Path(artifact).read_bytes()).hexdigest(),
    'commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'dirtySource': dirty == 'true',
    'runtimeProductionInputsComplete': runtime_report['productionInputsComplete'],
    'onlineSignedPublicationVerified': False,
    'testsPassed': skipped != 'true',
    'englishDistributionTestsPassed': skipped != 'true',
    'appFilingNumber': filing or None,
    'bundleId': 'com.openmuseai.mobile',
    'versionName': version,
    'buildNumber': int(number),
    'filingCertificateSha1': cert.lower(),
    'teamId': 'SDPXS943DW',
    'remainingGates': 'See app/openmuse_mobile/release_materials/09-国内海外上架流水线.zh-CN.md G0-G6',
}
pathlib.Path(output).write_text(json.dumps(record, ensure_ascii=False, indent=2) + '\n')
print(f"{artifact}\nSHA-256: {record['sha256']}")
PY
