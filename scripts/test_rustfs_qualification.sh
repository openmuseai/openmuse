#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-storage-qualification -- --check
cargo +1.85.1 check --package openmuse-storage-qualification --all-targets
cargo +1.85.1 test --package openmuse-storage-qualification
cargo +1.85.1 clippy --package openmuse-storage-qualification --all-targets -- -D warnings
cargo run --quiet --package openmuse-storage-qualification --example evaluate_report -- \
  docs/qualification/storage/st5-rustfs-current.json | rg -q '"status": "ineligible"'
