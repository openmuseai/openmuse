#!/bin/bash
set -euo pipefail

archive="$1"
app="$2"
expected_version="$3"
expected_build="$4"
parent_pid="$5"
log="${archive}.install.log"
exec >>"$log" 2>&1

fail() { echo "update failed: $*" >&2; exit 1; }
[[ -f "$archive" && -d "$app" && "$(basename "$app")" == OpenMuse.app ]] || fail 'invalid update paths'
[[ "$expected_version" =~ ^[0-9]+\.[0-9]+\.[0-9]+$ && "$expected_build" =~ ^[1-9][0-9]*$ ]] || fail 'invalid version'
[[ "$parent_pid" =~ ^[1-9][0-9]*$ ]] || fail 'invalid parent PID'

# This process is copied outside OpenMuse.app so the bundle can be replaced.
for _ in {1..240}; do
  if ! kill -0 "$parent_pid" 2>/dev/null; then break; fi
  sleep 0.25
done
if kill -0 "$parent_pid" 2>/dev/null; then fail 'application did not exit'; fi

parent="$(dirname "$app")"
stage="$(mktemp -d "$parent/.openmuse-update.XXXXXX")"
backup="$parent/.OpenMuse-backup-$$.app"
restore() {
  status=$?
  if [[ $status -ne 0 && -d "$backup" ]]; then
    if [[ -d "$app" ]]; then mv "$app" "$stage/failed.app" || true; fi
    mv "$backup" "$app" || true
    open "$app" || true
  fi
  if [[ -d "$stage" ]]; then rm -rf "$stage"; fi
  exit "$status"
}
trap restore EXIT

# Only a single OpenMuse.app bundle is allowed in the ZIP. Reject traversal
# paths before extraction; the signed bundle is checked again afterwards.
unzip -Z1 "$archive" | awk '
  /^OpenMuse\.app\// && !/(^|\/)\.\.($|\/)/ && !/(^|\/)\.($|\/)/ {next}
  /^__MACOSX\/OpenMuse\.app\// && !/(^|\/)\.\.($|\/)/ && !/(^|\/)\.($|\/)/ {next}
  /^__MACOSX\/$/ {next}
  {bad=1}
  END {exit bad}
' || fail 'unsafe ZIP member'
ditto -x -k "$archive" "$stage"
candidate="$stage/OpenMuse.app"
[[ -d "$candidate" ]] || fail 'missing OpenMuse.app'
codesign --verify --deep --strict "$app" || fail 'installed app signature invalid'
codesign --verify --deep --strict "$candidate" || fail 'new app signature invalid'
old_team="$(codesign -dv --verbose=4 "$app" 2>&1 | sed -n 's/^TeamIdentifier=//p')"
new_team="$(codesign -dv --verbose=4 "$candidate" 2>&1 | sed -n 's/^TeamIdentifier=//p')"
[[ "$old_team" == "$new_team" ]] || fail 'signing team changed'
plist="$candidate/Contents/Info.plist"
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleIdentifier' "$plist")" == com.openmuseai.office ]] || fail 'wrong bundle ID'
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleShortVersionString' "$plist")" == "$expected_version" ]] || fail 'wrong version'
[[ "$(/usr/libexec/PlistBuddy -c 'Print :CFBundleVersion' "$plist")" == "$expected_build" ]] || fail 'wrong build'

mv "$app" "$backup"
mv "$candidate" "$app"
open "$app" || fail 'could not relaunch'
echo "installed OpenMuse $expected_version ($expected_build)"
# Keep one known-good rollback copy until the next successful update. It can
# be removed by release maintenance after the new app has been observed.
trap - EXIT
rm -rf "$stage"
