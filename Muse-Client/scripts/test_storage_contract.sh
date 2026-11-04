#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-storage-contract --package openmuse-storage-tck -- --check
cargo test --package openmuse-storage-contract --package openmuse-storage-tck
cargo clippy --package openmuse-storage-contract --package openmuse-storage-tck --all-targets -- -D warnings

if rg -n 'aws_sdk_s3|aws-sdk-s3|ByteStream|SdkError' crates/openmuse-storage-contract; then
  echo "storage contract leaked an S3 SDK type" >&2
  exit 1
fi
