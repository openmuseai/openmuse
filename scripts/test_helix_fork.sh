#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
source_root="$repo_root/third_party/helix"
cd "$source_root"
export HELIX_DISABLE_AUTO_GRAMMAR_BUILD=1

cargo fmt --all -- --check
cargo test -p helix-view -p helix-term --lib --locked
if [[ "$(uname -s)" == "Darwin" && "$(uname -m)" == "arm64" ]]; then
  test -f runtime/grammars/rust.dylib
  test -f runtime/grammars/python.dylib
  test -f runtime/grammars/html.dylib
  cargo test -p helix-term --features integration --test integration --locked
  python3 "$repo_root/scripts/test_helix_control_bridge.py"
else
  echo "Full Helix integration fixtures are currently macOS arm64 only" >&2
fi
