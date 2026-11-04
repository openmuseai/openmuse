#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-storage-credentials --package openmuse-storage-s3 -- --check
cargo +1.85.1 check --package openmuse-storage-s3 --all-targets
cargo test --package openmuse-storage-credentials --package openmuse-storage-s3
cargo clippy --package openmuse-storage-credentials --package openmuse-storage-s3 --all-targets -- -D warnings
bash -n scripts/run_storage_provider_tck.sh

if rg -n 'trace!|debug!|info!|warn!|error!' crates/openmuse-storage-s3 crates/openmuse-storage-credentials; then
  echo "storage security boundary must use reviewed structured audit, not ad-hoc logging" >&2
  exit 1
fi

if rg -n 'secret_access_key|session_token' crates/openmuse-storage-contract; then
  echo "storage contract leaked credential fields" >&2
  exit 1
fi
