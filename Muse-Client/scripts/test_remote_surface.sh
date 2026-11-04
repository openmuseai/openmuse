#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"

cd "$repo_root"
cargo fmt --package openmuse-remote-surface -- --check
cargo test --package openmuse-remote-surface

dart format --output=none --set-exit-if-changed \
  packages/muse_remote_surface_contract \
  packages/muse_remote_surface_core \
  plugins/remote-workbench

(
  cd packages/muse_remote_surface_contract
  flutter analyze
  flutter test
)
(
  cd packages/muse_remote_surface_core
  flutter analyze
  flutter test
)
(
  cd plugins/remote-workbench
  flutter analyze
  flutter test
)
(
  cd plugins/workspace-paired
  flutter test test/remote_surface_gateway_test.dart
)
(
  cd app/openmuse_mobile
  flutter test test/remote_workbench_shell_test.dart test/remote_workbench_gateway_test.dart test/mobile_app_test.dart
)
(
  cd app/openmuse_host
  flutter test test/remote_workbench_desktop_test.dart
)
