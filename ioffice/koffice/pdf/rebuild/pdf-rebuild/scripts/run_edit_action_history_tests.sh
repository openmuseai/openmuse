#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")/../.." && pwd)"
OUT_DIR="${TMPDIR:-/tmp}/ksp-edit-action-history"
rm -rf "$OUT_DIR"
mkdir -p "$OUT_DIR"

javac -d "$OUT_DIR" \
  "$ROOT_DIR/pdf-standard-app/app/src/main/java/cn/wps/standalone/EditActionHistory.java" \
  "$ROOT_DIR/pdf-rebuild/harness/java/cn/wps/standalone/EditActionHistoryTest.java"
java -cp "$OUT_DIR" cn.wps.standalone.EditActionHistoryTest
