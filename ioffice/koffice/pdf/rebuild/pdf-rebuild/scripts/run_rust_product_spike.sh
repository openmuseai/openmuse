#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
if [[ "$(uname -s)" != Darwin ]]; then
  echo "This native ABI spike currently runs on macOS arm64" >&2
  exit 2
fi
bash "$ROOT/scripts/build_freetext.sh" >/dev/null
RUSTFLAGS="${RUSTFLAGS:-} -L native=$ROOT/build -C link-arg=-Wl,-rpath,$ROOT/build" \
  cargo test --manifest-path "$ROOT/product-rust/Cargo.toml"
