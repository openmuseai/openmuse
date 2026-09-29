#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"

cd "$repo_root"
dart format --output=none --set-exit-if-changed \
  packages/muse_resource_bridge/lib/src/authority.dart \
  packages/muse_resource_bridge/test/authority_test.dart
(
  cd packages/muse_resource_contract
  flutter analyze --no-pub
  flutter test --no-pub
)
(
  cd packages/muse_resource_bridge
  flutter analyze --no-pub
  flutter test --no-pub
)
