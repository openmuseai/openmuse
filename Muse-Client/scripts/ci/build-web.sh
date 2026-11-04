#!/usr/bin/env bash
# Build the OpenMuse web release.
#
# The Flutter host loads the DSH pane from app/openmuse_web/web/dsh-pane.js,
# which web/dsh-pane/build.mjs generates from web/dsh-pane/src. That bundle is
# committed, so a release rebuilds it from source first; otherwise the published
# host can serve a pane that no longer matches its source.
#
# Mirrors scripts/ci/build-windows.ps1 and scripts/ci/build-macos.sh: the release
# artifacts land in dist/ with SHA256SUMS.txt and build-info.txt.

set -euo pipefail

# Git Bash rewrites a lone "/" argument into its own install root, which would
# hand flutter a --base-href of "C:/Program Files/Git/". Scoped to this script
# so it cannot leak into the caller's environment; other shells ignore it.
export MSYS_NO_PATHCONV=1

repo_root="$(cd "$(dirname "$0")/../.." && pwd)"
cd "$repo_root"

profile=release
base_href=
region=us
skip_tests=false
force_rebuild=false

usage() {
  cat >&2 <<'USAGE'
usage: scripts/ci/build-web.sh [--profile release|debug] [--region cn|us] [--base-href PATH]
                               [--skip-tests] [--force-rebuild]

  --profile        Flutter build profile (default release)
  --region        cn or us (default us); installs a fixed-region configuration
  --base-href      Must match the region profile: cn /, us /app/.
  --skip-tests     Skip flutter analyze and flutter test
  --force-rebuild  Ignore every cache: flutter clean, drop node_modules/build
USAGE
}

while [[ $# -gt 0 ]]; do
  case "$1" in
    --profile) profile="${2:-}"; shift 2 ;;
    --region) region="${2:-}"; shift 2 ;;
    --base-href) base_href="${2:-}"; shift 2 ;;
    --skip-tests) skip_tests=true; shift ;;
    --force-rebuild) force_rebuild=true; shift ;;
    -h|--help) usage; exit 0 ;;
    *) echo "unknown argument: $1" >&2; usage; exit 2 ;;
  esac
done

case "$profile" in
  release|debug) ;;
  *) echo "--profile must be release or debug" >&2; exit 2 ;;
esac

case "$region" in cn|us) ;; *) echo '--region must be cn or us' >&2; exit 2 ;; esac
expected_base="$(python3 -c 'import json,sys; print(json.load(open("config/web-deployments.json"))["regions"][sys.argv[1]]["baseHref"])' "$region")"
[[ -n "$base_href" ]] || base_href="$expected_base"
[[ "$base_href" == "$expected_base" ]] || { echo 'Base href differs from selected region deployment' >&2; exit 2; }

# The host is deployed under a path prefix, so the base href must be anchored on
# both sides ("/" or "/app/"); a bare "app" would resolve against the document.
[[ -n "$base_href" ]] || { echo "--base-href must not be empty" >&2; exit 2; }
[[ "$base_href" == /* ]] || base_href="/$base_href"
[[ "$base_href" == */ ]] || base_href="$base_href/"

for command in flutter npm node git zip unzip; do
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

step 'Fixed-region installation'
artifact_dir="$repo_root/dist/web/$region"
web_defines="$artifact_dir/defines.json"
python3 scripts/release/prepare-web-config.py --region "$region" --output "$web_defines"

step 'DSH pane bundle'
(
  cd web/dsh-pane
  if [[ "$force_rebuild" == true ]]; then rm -rf node_modules; fi
  npm ci --no-audit --no-fund
  # Name the test files explicitly: node --test only treats its arguments as
  # paths to load, so a bare directory is resolved as a module.
  pane_tests=(src/*.test.js)
  [[ -e "${pane_tests[0]}" ]] || { echo 'no pane test files found under web/dsh-pane/src' >&2; exit 1; }
  node --test "${pane_tests[@]}"
  npm run build
)

step 'Flutter web host'
(
  cd app/openmuse_web
  if [[ "$force_rebuild" == true ]]; then flutter clean; fi
  flutter pub get
  if [[ "$skip_tests" == false ]]; then
    flutter analyze
    flutter test
  fi
  flutter build web "--$profile" --base-href "$base_href" --dart-define-from-file="$web_defines" --no-web-resources-cdn
)

out='app/openmuse_web/build/web'
step 'verify'
for required in index.html dsh-pane.js dsh-pane.css; do
  [[ -f "$out/$required" ]] || { echo "missing $required under $out" >&2; exit 1; }
done
[[ -f "$out/main.dart.js" || -f "$out/flutter_bootstrap.js" ]] || {
  echo "missing the Flutter entry bundle under $out" >&2
  exit 1
}
grep -q "<base href=\"$base_href\">" "$out/index.html" || {
  echo "index.html does not carry the requested base href $base_href" >&2
  exit 1
}
printf '    base href   %s\n' "$base_href"
printf '    dsh-pane.js %s MB\n' "$(awk "BEGIN { printf \"%.1f\", $(wc -c <"$out/dsh-pane.js") / 1048576 }")"
printf '    dsh-pane.css %s MB\n' "$(awk "BEGIN { printf \"%.1f\", $(wc -c <"$out/dsh-pane.css") / 1048576 }")"

if ! git diff --quiet --ignore-cr-at-eol -- app/openmuse_web/web/dsh-pane.js app/openmuse_web/web/dsh-pane.css 2>/dev/null; then
  echo '    note: the rebuilt pane bundle differs from the committed one; the release ships the rebuilt one'
fi

step 'package'
mkdir -p "$artifact_dir"
rm -f "$artifact_dir/OpenMuse-web.zip"
(
  cd "$out"
  zip -q -r -X "$artifact_dir/OpenMuse-web.zip" .
)
# The host is served from the archive root, so index.html has to be there and
# not inside a subdirectory. -Z1 lists one entry per line and tr strips the CR
# that the Windows unzip appends, so the match is exact on every platform.
unzip -Z1 "$artifact_dir/OpenMuse-web.zip" | tr -d '\r' | grep -qx 'index.html' || {
  echo 'index.html is not at the root of the archive' >&2
  exit 1
}
digest="$(sha256 "$artifact_dir/OpenMuse-web.zip")"
printf '%s  %s\n' "$digest" 'OpenMuse-web.zip' >"$artifact_dir/SHA256SUMS.txt"

{
  echo 'product: OpenMuse'
  echo "profile: $profile"
  echo "commit: $(git rev-parse HEAD)"
  echo "builtAt: $(date -u +%Y-%m-%dT%H:%M:%SZ)"
  echo "flutter: $(flutter --version | head -1)"
  echo "baseHref: $base_href"
  echo "dataRegion: $region"
  echo 'domains: web=build'
  echo "archive: OpenMuse-web.zip sha256=$digest"
} >"$artifact_dir/build-info.txt"

printf '\nweb build complete\n'
printf '    %s/OpenMuse-web.zip  %s MB  sha256 %s\n' "$artifact_dir" \
  "$(awk "BEGIN { printf \"%.2f\", $(wc -c <"$artifact_dir/OpenMuse-web.zip") / 1048576 }")" "$digest"
