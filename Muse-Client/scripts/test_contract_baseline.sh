#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"

cd "$repo_root"
node scripts/validate_contract_baseline.mjs
cargo fmt --package openmuse-contract -- --check
cargo test --package openmuse-contract

dart format --output=none --set-exit-if-changed packages/openmuse_contract
(
  cd packages/openmuse_contract
  flutter analyze
  flutter test
)

node --experimental-strip-types --test third_party/dsh/plugins/openmuse-contract/test/fixtures.test.ts
