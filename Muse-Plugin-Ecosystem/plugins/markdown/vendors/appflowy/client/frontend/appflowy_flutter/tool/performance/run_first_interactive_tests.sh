#!/bin/sh
set -eu

SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
APP_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/../.." && pwd)
FLUTTER_BIN=${FLUTTER_BIN:-flutter}
MUSE_PERF_ITERATIONS=${MUSE_PERF_ITERATIONS:-5}
export MUSE_PERF_ITERATIONS

cd "$APP_DIR"
"$FLUTTER_BIN" test test/performance/viewer_bundle_budget_test.dart
"$FLUTTER_BIN" test integration_test/performance/pty_async_start_test.dart -d macos
"$FLUTTER_BIN" test integration_test/performance/helix_first_interactive_test.dart -d macos
"$FLUTTER_BIN" test integration_test/performance/helix_lsp_tab_budget_test.dart -d macos

echo 'Resource first-interactive performance gates passed.'
