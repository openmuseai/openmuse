#!/usr/bin/env bash
set -euo pipefail
repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"
cargo +1.85.1 fmt --package openmuse-cli-registry -- --check
cargo +1.85.1 test -p openmuse-cli-registry
cargo +1.85.1 clippy -p openmuse-cli-registry --all-targets -- -D warnings
