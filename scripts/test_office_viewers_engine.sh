#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-office-viewers -- --check
cargo test --package openmuse-office-viewers
cargo clippy --package openmuse-office-viewers --all-targets -- -D warnings
cargo build --package openmuse-office-viewers
(
  cd packages/openmuse_office_viewers
  dart analyze
  OPENMUSE_VIEWERS_TEST_LIBRARY="$repo_root/target/debug/libopenmuse_office_viewers.dylib" dart test
)
