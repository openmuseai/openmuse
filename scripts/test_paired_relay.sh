#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-paired-relay -- --check
cargo test --package openmuse-paired-relay
cargo clippy --package openmuse-paired-relay --all-targets -- -D warnings
