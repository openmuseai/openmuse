#!/bin/bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
version="${1:-1.0.7}"
build="${2:-8}"
source_app="$repo_root/dist/update-e2e/silent-baseline/OpenMuse.app"
archive="$repo_root/dist/OpenMuse-macos.zip"
test_root="$(mktemp -d "$repo_root/dist/update-e2e/helper-validation.XXXXXX")"
test_app="$test_root/OpenMuse.app"
test -d "$source_app"
test -f "$archive"
ditto "$source_app" "$test_app"
mkdir -p "$test_root/bin"
cat >"$test_root/bin/open" <<'OPEN_STUB'
#!/bin/bash
printf '%s\n' "$1" >"$OPENMUSE_TEST_OPEN_MARKER"
OPEN_STUB
chmod 700 "$test_root/bin/open"
export OPENMUSE_TEST_OPEN_MARKER="$test_root/open-called.txt"
PATH="$test_root/bin:/usr/bin:/bin" \
  /bin/bash "$repo_root/scripts/update/install_macos.sh" \
  "$archive" "$test_app" "$version" "$build" 999999
test "$(cat "$OPENMUSE_TEST_OPEN_MARKER")" = "$test_app"
test "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$test_app/Contents/Info.plist")" = "$version"
test "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleVersion' "$test_app/Contents/Info.plist")" = "$build"
codesign --verify --deep --strict "$test_app"
echo "Validated helper replacement and relaunch request: $test_root"
