#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-office-docx -- --check
cargo test --package openmuse-office-docx
cargo clippy --package openmuse-office-docx --all-targets -- -D warnings
