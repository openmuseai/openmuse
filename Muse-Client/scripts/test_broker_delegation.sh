#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"

cd "$repo_root"
cargo fmt --package openmuse-plugin-protocol --package openmuse-platform-runtime -- --check
cargo test --package openmuse-platform-runtime
cargo test --package openmuse-plugin-protocol
cargo clippy --package openmuse-platform-runtime --all-targets -- -D warnings
