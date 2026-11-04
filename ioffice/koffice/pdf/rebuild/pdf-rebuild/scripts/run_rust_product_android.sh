#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SDK="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
NDK="${ANDROID_NDK_HOME:-$SDK/ndk/27.0.12077973}"
case "$(uname -s)" in
  Darwin) HOST_TAG=darwin-x86_64 ;;
  Linux) HOST_TAG=linux-x86_64 ;;
  *) echo "Unsupported Android build host" >&2; exit 2 ;;
esac
LINKER="$NDK/toolchains/llvm/prebuilt/$HOST_TAG/bin/aarch64-linux-android28-clang"
if [[ ! -x "$LINKER" ]]; then
  echo "Android NDK linker not found: $LINKER" >&2
  exit 2
fi

bash "$ROOT/scripts/build_freetext_android.sh" >/dev/null
output="$(mktemp)"
trap 'rm -f "$output"' EXIT
CARGO_TARGET_AARCH64_LINUX_ANDROID_LINKER="$LINKER" \
RUSTFLAGS="${RUSTFLAGS:-} -L native=$ROOT/build -L native=$ROOT/3rdparty/prebuilt/android-arm64/lib" \
  cargo test --manifest-path "$ROOT/product-rust/Cargo.toml" \
    --target aarch64-linux-android --no-run --message-format=json > "$output"
test_binary="$(python3 - "$output" <<'PY'
import json
import sys

executables = []
for line in open(sys.argv[1], encoding="utf-8"):
    try:
        item = json.loads(line)
    except json.JSONDecodeError:
        continue
    if item.get("reason") == "compiler-artifact" and item.get("executable"):
        executables.append(item["executable"])
if len(executables) != 1:
    raise SystemExit(f"expected one Rust test binary, got {len(executables)}")
print(executables[0])
PY
)"

adb_command=(adb)
if [[ -n "${ANDROID_DEVICE_ID:-}" ]]; then
  adb_command+=(-s "$ANDROID_DEVICE_ID")
fi
remote=/data/local/tmp/ksp-rust
"${adb_command[@]}" shell mkdir -p "$remote"
"${adb_command[@]}" push "$test_binary" "$remote/product_test"
"${adb_command[@]}" push "$ROOT/build/libkspfreetext.so" \
  "$ROOT/3rdparty/prebuilt/android-arm64/lib/libpdfium.so" "$remote/"
"${adb_command[@]}" shell chmod 755 "$remote/product_test"
"${adb_command[@]}" shell "LD_LIBRARY_PATH=$remote $remote/product_test --nocapture"
