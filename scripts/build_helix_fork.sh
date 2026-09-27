#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
source_root="$repo_root/third_party/helix"
engine="$repo_root/plugins/helix/assets/engines/helix"

test -f "$source_root/LICENSE"
test -f "$source_root/Cargo.lock"
cd "$source_root"

# Grammars are already pinned and shipped in the plugin runtime. Building the
# editor executable must not fetch 301 upstream grammar repositories.
export HELIX_DISABLE_AUTO_GRAMMAR_BUILD=1
cargo build -p helix-term --release --locked --target aarch64-apple-darwin
cargo build -p helix-term --release --locked --target x86_64-apple-darwin

"/usr/bin/lipo" -create \
  "$source_root/target/aarch64-apple-darwin/release/hx" \
  "$source_root/target/x86_64-apple-darwin/release/hx" \
  -output "$engine/hx"
chmod 755 "$engine/hx"
test "$(/usr/bin/lipo -archs "$engine/hx")" = "x86_64 arm64"
"$engine/hx" --version | grep -q 'openmuse-nonmodal.3'
shasum -a 256 "$engine/hx"
