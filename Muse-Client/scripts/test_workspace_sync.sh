#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-workspace-sync -- --check
cargo +1.85.1 check --package openmuse-workspace-sync --all-targets
cargo +1.85.1 test --package openmuse-workspace-sync
cargo +1.85.1 clippy --package openmuse-workspace-sync --all-targets -- -D warnings

if rg -n 'aws_sdk|s3://|openmuse_cloud_resource_authority' crates/openmuse-workspace-sync; then
  echo "workspace sync must depend on provider-neutral ports, not S3 or Cloud Authority implementations" >&2
  exit 1
fi
