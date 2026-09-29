#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-cloud-resource-authority -- --check
cargo +1.85.1 check --package openmuse-cloud-resource-authority --all-targets
cargo test --package openmuse-cloud-resource-authority
cargo clippy --package openmuse-cloud-resource-authority --all-targets -- -D warnings

if rg -n 'aws_sdk|ListObjects|list_objects|s3://' crates/openmuse-cloud-resource-authority; then
  echo "cloud resource authority must depend on BlobStorePort and metadata, not S3 APIs" >&2
  exit 1
fi
