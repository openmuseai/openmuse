#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"
cargo +1.85.1 fmt --package openmuse-worker-supervisor -- --check
cargo +1.85.1 test -p openmuse-worker-supervisor
cargo +1.85.1 clippy -p openmuse-worker-supervisor --all-targets -- -D warnings
