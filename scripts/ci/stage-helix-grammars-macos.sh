#!/usr/bin/env bash
set -euo pipefail

if [[ $# -ne 1 ]]; then
  echo 'usage: stage-helix-grammars-macos.sh <bundled-runtime-directory>' >&2
  exit 2
fi

root="$(cd "$(dirname "$0")/../.." && pwd)"
engine="$root/plugins/helix/assets/engines/helix"
destination="$1/grammars"
selection="$root/scripts/helix-grammars.toml"
config_root="$root/target/helix-grammar-build-macos"
mkdir -p "$config_root/helix" "$destination"
cp "$selection" "$config_root/helix/languages.toml"

# The pinned hx is universal. Apple Clang can produce a universal grammar
# library in one build when both architectures are supplied to its C/C++
# compiler. Keep the generated sources and libraries outside tracked assets.
export XDG_CONFIG_HOME="$config_root"
export HELIX_RUNTIME="$engine/runtime"
export CFLAGS="${CFLAGS:-} -arch x86_64 -arch arm64"
export CXXFLAGS="${CXXFLAGS:-} -arch x86_64 -arch arm64"
"$engine/hx" --grammar fetch --strict
"$engine/hx" --grammar build --strict

source_grammars="$config_root/helix/runtime/grammars"
count=0
while IFS= read -r name; do
  source="$source_grammars/$name.dylib"
  target="$destination/$name.dylib"
  if [[ ! -f "$source" ]]; then
    echo "required Helix grammar was not built: $name" >&2
    exit 1
  fi
  archs="$(/usr/bin/lipo -archs "$source")"
  if [[ " $archs " != *' x86_64 '* || " $archs " != *' arm64 '* ]]; then
    echo "Helix grammar is not universal: $name ($archs)" >&2
    exit 1
  fi
  cp "$source" "$target"
  codesign --force --sign - "$target"
  count=$((count + 1))
done < <(grep -oE '"[^"]+"' "$selection" | tr -d '"')

health_config="$root/target/helix-grammar-health-macos"
mkdir -p "$health_config"
for language in rust python javascript; do
  health="$(XDG_CONFIG_HOME="$health_config" HELIX_RUNTIME="$1" "$engine/hx" --health "$language")"
  if ! grep -q 'Tree-sitter parser: ✓' <<<"$health"; then
    echo "bundled Helix parser is unavailable for $language" >&2
    exit 1
  fi
done
echo "    $count universal Tree-sitter grammars staged for macOS"
