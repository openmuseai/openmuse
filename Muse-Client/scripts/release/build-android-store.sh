#!/usr/bin/env bash
# Build a signed Android store candidate from a pinned, clean source tree.
# This does not submit to a store or assert legal/compliance readiness.
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
lane=''
version=''
build_number=''
allow_dirty=false
skip_tests=false

usage() {
  cat >&2 <<'USAGE'
usage: scripts/release/build-android-store.sh --lane cn|play --version X.Y.Z --build-number N [--allow-dirty] [--skip-tests]

  cn    Build a formally signed APK for Chinese Android markets.
  play  Build a formally signed AAB upload candidate for Google Play.

  --allow-dirty is for local rehearsal only; the output metadata records this.
  --skip-tests is for internal investigation only; such a build is never
  marked ready for store submission.
  Store submission still requires the legal, privacy, content and device gates
  in app/openmuse_mobile/release_materials/09-国内海外上架流水线.zh-CN.md.
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --lane) lane="${2:-}"; shift 2 ;;
    --version) version="${2:-}"; shift 2 ;;
    --build-number) build_number="${2:-}"; shift 2 ;;
    --allow-dirty) allow_dirty=true; shift ;;
    --skip-tests) skip_tests=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "Unknown argument: $1" >&2; usage; exit 2 ;;
  esac
done

[[ "$lane" == cn || "$lane" == play ]] || { usage; exit 2; }
[[ "$version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && "$version" != 0.0.1 ]] || {
  echo 'Use an approved release version X.Y.Z, not the current 0.0.1 test version.' >&2
  exit 2
}
[[ "$build_number" =~ ^[1-9][0-9]*$ && "$build_number" -le 2100000000 ]] || {
  echo 'Use a positive, increasing build number no greater than 2100000000.' >&2
  exit 2
}
[[ -z "${OPENMUSE_ANDROID_RELEASE_APPLICATION_ID_SUFFIX:-}" ]] || {
  echo 'A release applicationId suffix would break the registered package name.' >&2
  exit 2
}
: "${ANDROID_HOME:?Set ANDROID_HOME to the Android SDK}"
for tool in flutter cargo keytool openssl python3 unzip; do
  command -v "$tool" >/dev/null || { echo "Missing $tool" >&2; exit 2; }
done
if [[ "$lane" == play && ! -x "$ANDROID_HOME/cmdline-tools/latest/bin/apkanalyzer" ]]; then
  echo 'Install Android SDK command-line tools (latest) before building AAB; Flutter needs apkanalyzer to verify native debug symbols.' >&2
  exit 2
fi

cd "$repo_root"
python3 "$repo_root/scripts/release/verify-welcome-guide.py"
dirty=false
if [[ -n "$(git status --porcelain)" ]]; then dirty=true; fi
if [[ "$dirty" == true && "$allow_dirty" != true ]]; then
  echo 'The source tree is dirty. Commit and tag the release source first.' >&2
  exit 2
fi

OPENMUSE_PAIRED_PLATFORMS=android "$repo_root/scripts/build_paired_relay_mobile_artifacts.sh"

(cd app/openmuse_mobile && flutter analyze)
if [[ "$skip_tests" != true ]]; then
  (cd app/openmuse_mobile && flutter test)
fi

deployment_profile=global-us
if [[ "$lane" == cn ]]; then
  deployment_profile=china
  kind=apk
  source_artifact="$repo_root/app/openmuse_mobile/build/app/outputs/flutter-apk/app-release.apk"
  extension=apk
else
  kind=appbundle
  source_artifact="$repo_root/app/openmuse_mobile/build/app/outputs/bundle/release/app-release.aab"
  extension=aab
fi

runtime_tmp="$(mktemp -d)"
trap 'rm -rf "$runtime_tmp"' EXIT
bash "$repo_root/scripts/release/prepare-runtime-config.sh" "$deployment_profile" "$runtime_tmp/runtime-defines.json"
runtime_validation_args=()
if [[ "$allow_dirty" == true ]]; then runtime_validation_args+=(--allow-incomplete); fi
python3 "$repo_root/scripts/release/verify-runtime-input.py" "$runtime_tmp/runtime-defines.json" \
  --report "$runtime_tmp/runtime-input-report.json" "${runtime_validation_args[@]}"
python3 "$repo_root/scripts/release/verify-mobile-closure.py" "$repo_root/app/openmuse_mobile/.dart_tool/package_config.json"

(cd app/openmuse_mobile && "$repo_root/scripts/filing/with-android-release-signing.sh" \
  flutter build "$kind" --release --target lib/main.dart --target-platform android-arm64 \
  --build-name "$version" --build-number "$build_number" \
  "--dart-define=OPENMUSE_DISTRIBUTION_CHANNEL=android-$lane" \
  "--dart-define-from-file=$runtime_tmp/runtime-defines.json")
python3 "$repo_root/scripts/release/verify-mobile-closure.py" "$source_artifact"
[[ -s "$source_artifact" ]] || { echo "Missing $source_artifact" >&2; exit 1; }

if [[ "$lane" == cn ]]; then
  native_prefix='lib/'
else
  native_prefix='base/lib/'
fi
native_abis="$(unzip -Z1 "$source_artifact" | sed -n "s#^${native_prefix}\\([^/]*\\)/.*#\\1#p" | LC_ALL=C sort -u)"
[[ "$native_abis" == arm64-v8a ]] || {
  echo "Unexpected native ABIs in $source_artifact: $native_abis" >&2
  exit 1
}

public_cert="$repo_root/app/openmuse_mobile/release_materials/assets/OpenMuse-Android-Release-2026.cer"
[[ -f "$public_cert" ]] || { echo 'Missing the archived public release certificate.' >&2; exit 1; }
expected="$(openssl x509 -in "$public_cert" -noout -fingerprint -sha256 | cut -d= -f2 | tr -d ': ' | tr '[:upper:]' '[:lower:]')"
if [[ "$lane" == cn ]]; then
  apksigner=''
  for candidate in "$ANDROID_HOME"/build-tools/*/apksigner; do
    [[ -x "$candidate" ]] && apksigner="$candidate"
  done
  [[ -n "$apksigner" ]] || { echo 'Missing Android SDK apksigner.' >&2; exit 1; }
  report="$("$apksigner" verify --verbose --print-certs "$source_artifact")"
  actual="$(sed -n 's/^.*certificate SHA-256 digest: //p' <<<"$report" | head -1 | tr '[:upper:]' '[:lower:]')"
else
  report="$(keytool -printcert -jarfile "$source_artifact")"
  actual="$(sed -n 's/^[[:space:]]*SHA256: //p' <<<"$report" | head -1 | tr -d ': ' | tr '[:upper:]' '[:lower:]')"
fi
[[ -n "$actual" && "$actual" == "$expected" ]] || {
  echo "Release certificate mismatch (expected $expected; got ${actual:-none})." >&2
  exit 1
}

output_dir="$repo_root/dist/mobile/${version}-${build_number}/${lane}"
mkdir -p "$output_dir"
cp "${runtime_tmp}/runtime-input-report.json" "$output_dir/runtime-input-report.json"
output_artifact="$output_dir/OpenMuse-${lane}-${version}-${build_number}.${extension}"
cp "$source_artifact" "$output_artifact"
printf '%s\n' "$report" >"$output_dir/signature-report.txt"
python3 - "$output_artifact" "$output_dir/release.json" "$lane" "$version" "$build_number" "$expected" "$dirty" "$skip_tests" "${OPENMUSE_APP_FILING_NUMBER:-}" <<'PY'
import hashlib, json, pathlib, subprocess, sys
artifact, output, lane, version, number, cert, dirty, skipped, filing = sys.argv[1:]
hasher = hashlib.sha256()
with pathlib.Path(artifact).open('rb') as stream:
    for chunk in iter(lambda: stream.read(1024 * 1024), b''):
        hasher.update(chunk)
digest = hasher.hexdigest()
runtime_report = json.loads(pathlib.Path(output).with_name('runtime-input-report.json').read_text())
record = {
    'status': 'internal-test-only' if not runtime_report['productionInputsComplete'] or dirty == 'true' or skipped == 'true' or (lane == 'cn' and not filing) else 'store-candidate',
    'lane': lane,
    'artifact': pathlib.Path(artifact).name,
    'sha256': digest,
    'commit': subprocess.check_output(['git', 'rev-parse', 'HEAD'], text=True).strip(),
    'dirtySource': dirty == 'true',
    'runtimeProductionInputsComplete': runtime_report['productionInputsComplete'],
    'onlineSignedPublicationVerified': False,
    'testsPassed': skipped != 'true',
    'appFilingNumber': filing or None,
    'packageId': 'com.openmuseai.mobile',
    'versionName': version,
    'versionCode': int(number),
    'uploadCertificateSha256': cert,
    'remainingGates': 'See app/openmuse_mobile/release_materials/09-国内海外上架流水线.zh-CN.md G0-G6',
}
pathlib.Path(output).write_text(json.dumps(record, ensure_ascii=False, indent=2) + '\n')
print(f'{artifact}\nSHA-256: {digest}')
PY
