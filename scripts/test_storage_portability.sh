#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-storage-portability -- --check
cargo +1.85.1 check --package openmuse-storage-portability --all-targets
cargo +1.85.1 test --package openmuse-storage-portability
cargo +1.85.1 clippy --package openmuse-storage-portability --all-targets -- -D warnings

if rg -n 'aws_sdk|s3://|access_key|secret_key' crates/openmuse-storage-portability; then
  echo "storage portability must use provider-neutral ports and must not contain credentials" >&2
  exit 1
fi
