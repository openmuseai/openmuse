#!/usr/bin/env bash
set -euo pipefail

# The independently licensed Helix binary/runtime is a pinned repository
# input. Packaging must never fetch it from another checkout or environment.
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
engine="$repo_root/plugins/helix/assets/engines/helix"
test -x "$engine/hx"
test -f "$engine/LICENSE.MPL-2.0"
test -f "$engine/runtime/languages.toml"
test -d "$engine/runtime/grammars"
test -d "$engine/runtime/queries"
test -f "$engine/runtime/themes/onelight.toml"
test -f "$engine/runtime/themes/openmuse_dark.toml"
expected="cb81c9fba915c66f1d2c5b42debb7585eaffa64d74c3f9d2f06570c0781acae8"
actual="$(shasum -a 256 "$engine/hx" | awk '{ print $1 }')"
if [[ "$actual" != "$expected" ]]; then
  echo "pinned Helix binary checksum mismatch" >&2
  exit 1
fi
test "$(/usr/bin/lipo -archs "$engine/hx")" = "x86_64 arm64"
"$engine/hx" --version
"$engine/hx" --version | grep -q 'openmuse-nonmodal.3'
