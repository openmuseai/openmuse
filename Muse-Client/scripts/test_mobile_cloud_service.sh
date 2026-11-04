#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"

for package in openmuse_mobile_core openmuse_mobile_cloud; do
  (
    cd "$repo_root/packages/$package"
    dart pub get
    dart format --output=none --set-exit-if-changed lib test
    dart analyze
    dart test
  )
done
