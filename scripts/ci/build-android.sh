#!/usr/bin/env bash
# Build the OpenMuse Android release artifact.
#
# The build logic stays in scripts/build_android_alpha.sh: it compiles the Rust
# mobile artifacts, produces a signed arm64 release APK, and verifies the
# signature, the native symbols and the absence of any desktop runtime. This
# wrapper adds the release staging only: dist/OpenMuse-Android-arm64.apk plus
# SHA256SUMS.txt and build-info.txt, the same artifact shape the desktop flows
# publish.
#
# Signing: the gate signs with the keystore it generates on the spot under
# target/android-alpha. That is an internal build key, not a Play Store upload
# key, so this artifact is for internal distribution; a store release needs the
# real keystore and password supplied through secrets.

set -euo pipefail

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$repo_root"

force_rebuild=false

usage() {
  cat >&2 <<'USAGE'
usage: scripts/ci/build-android.sh [--force-rebuild]

  --force-rebuild  Drop the mobile build outputs and the Gradle cache first
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --force-rebuild) force_rebuild=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage; exit 2 ;;
  esac
done

: "${ANDROID_HOME:?ANDROID_HOME must point at the Android SDK}"

for command in flutter cargo keytool unzip; do
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

# The gate hashes its own evidence with shasum, which ships with perl: macOS
# always has it, but a minimal Linux image can carry only sha256sum. Provide the
# one flag combination the mobile gates use instead of rewriting them.
if ! command -v shasum >/dev/null 2>&1; then
  shim_dir="$(mktemp -d)"
  cat >"$shim_dir/shasum" <<'SHIM'
#!/usr/bin/env bash
if [[ "${1:-}" == "-a" && "${2:-}" == "256" ]]; then shift 2; fi
exec sha256sum "$@"
SHIM
  chmod +x "$shim_dir/shasum"
  export PATH="$shim_dir:$PATH"
  echo "==> shasum is absent; using a sha256sum shim for the gate"
fi

step() { printf '\n==> %s\n' "$1"; }

if [[ "$force_rebuild" == true ]]; then
  step 'force rebuild: dropping mobile build outputs'
  rm -rf target/android-alpha \
    target/office-docx target/office-viewers target/paired-relay \
    target/mobile-gradle-cache
  (cd app/openmuse_mobile && flutter clean)
fi

step 'Android release gate'
"$repo_root/scripts/build_android_alpha.sh"

apk="$repo_root/target/android-alpha/OpenMuse-Android-Alpha-arm64.apk"
[[ -f "$apk" ]] || { echo "the Android gate did not produce $apk" >&2; exit 1; }

step 'package'
mkdir -p dist
cp "$apk" dist/OpenMuse-Android-arm64.apk
if [[ -f target/android-alpha/apksigner-report.txt ]]; then
  cp target/android-alpha/apksigner-report.txt dist/android-apksigner-report.txt
fi
digest="$(sha256 dist/OpenMuse-Android-arm64.apk)"
printf '%s  %s\n' "$digest" 'OpenMuse-Android-arm64.apk' >dist/SHA256SUMS.txt

{
  echo 'product: OpenMuse'
  echo 'profile: release'
  echo "commit: $(git rev-parse HEAD)"
  echo "builtAt: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "flutter: $(flutter --version | head -1)"
  echo "rustc: $(rustc --version)"
  echo "androidSdk: ${ANDROID_HOME}"
  echo 'domains: android=build'
  echo "archive: OpenMuse-Android-arm64.apk sha256=$digest"
} >dist/build-info.txt

printf '\nandroid build complete\n'
printf '    dist/OpenMuse-Android-arm64.apk  %s MB  sha256 %s\n' \
  "$(awk "BEGIN { printf \"%.2f\", $(wc -c <dist/OpenMuse-Android-arm64.apk) / 1048576 }")" "$digest"
