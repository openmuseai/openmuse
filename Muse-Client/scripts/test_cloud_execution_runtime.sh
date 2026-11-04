#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo +1.85.1 fmt --package openmuse-cloud-execution-runtime -- --check
cargo +1.85.1 test -p openmuse-cloud-execution-runtime
cargo +1.85.1 clippy -p openmuse-cloud-execution-runtime --all-targets -- -D warnings
