#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo +1.85.1 fmt --package openmuse-draft-transaction -- --check
cargo +1.85.1 test -p openmuse-draft-transaction
cargo +1.85.1 clippy -p openmuse-draft-transaction --all-targets -- -D warnings
