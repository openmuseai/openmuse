#!/usr/bin/env bash
# run_diff.sh - run both differential modes on a connected device and print results.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
adb install -r -t --no-streaming "$ROOT/android/ksp-diff.apk" >/dev/null
for mode in rebuilt original; do
  echo "########## $mode ##########"
  "$ROOT/scripts/run_android_mode.sh" "$mode"
done
