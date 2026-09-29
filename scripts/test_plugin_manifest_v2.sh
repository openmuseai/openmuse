#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"

cd "$repo_root"
node scripts/validate_plugin_manifest_v2.mjs
cargo fmt --package openmuse-plugin-protocol -- --check
cargo test --package openmuse-plugin-protocol
cargo clippy --package openmuse-plugin-protocol --all-targets -- -D warnings

dart format --output=none --set-exit-if-changed packages/openmuse_plugin_sdk
(
  cd packages/openmuse_plugin_sdk
  flutter analyze --no-pub
  flutter test --no-pub
)
