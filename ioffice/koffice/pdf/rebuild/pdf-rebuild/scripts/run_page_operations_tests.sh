#!/usr/bin/env bash
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PREBUILT="$ROOT/3rdparty/prebuilt"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/ksp-page-ops.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

clang++ -std=c++17 -O1 -g -Wall -Wextra -Werror \
  -I"$PREBUILT/include" \
  "$ROOT/src/ksp/product/PageOperations.cpp" \
  "$ROOT/src/ksp/product/ProductBridge.cpp" \
  "$ROOT/src/ksp/runtime/PdfiumLifecycle.cpp" \
  "$ROOT/harness/page_operations_tests.cpp" \
  -L"$PREBUILT/lib" -lpdfium \
  -o "$WORK/page_operations_tests"
DYLD_LIBRARY_PATH="$PREBUILT/lib" "$WORK/page_operations_tests"
