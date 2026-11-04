#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
cd "$repo_root"

cargo fmt --package openmuse-office-docx -- --check
cargo test --package openmuse-office-docx
cargo clippy --package openmuse-office-docx --all-targets -- -D warnings
cargo build --package openmuse-office-docx

case "$(uname -s)" in
  Darwin) dart_library="$repo_root/target/debug/libopenmuse_office_docx.dylib" ;;
  Linux) dart_library="$repo_root/target/debug/libopenmuse_office_docx.so" ;;
  *) echo "Host DOCX FFI TCK is not configured for this platform" >&2; exit 1 ;;
esac
(
  cd "$repo_root/packages/openmuse_office_docx"
  dart pub get
  dart analyze
  OPENMUSE_DOCX_TEST_LIBRARY="$dart_library" dart test
)
