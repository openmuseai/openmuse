#!/usr/bin/env bash
# 构建 word_render 随包分发的预编译 native 产物。
#
# 产物落点：word_render/native/<platform>/（提交进 git，作为 flutter asset 随包分发）
#   macos/arm64/libword_core_ffi.dylib   （ad-hoc 签名）
#   linux/x86_64/libword_core_ffi.so
#   windows/x86_64/word_core_ffi.dll
#   android/<abi>/libword_core_ffi.so    （cargo-ndk）
#   ios/...                              （FRB iOS 走静态链接，见 README；脚本为骨架）
#
# 用法：
#   tool/build_native.sh                 # 构建当前宿主机平台
#   tool/build_native.sh android         # 构建 Android 全部 ABI（需 cargo-ndk）
#   tool/build_native.sh ios             # 骨架（未完成）
#
# 前置：cargo/rustc 可用；release 构建不需要 flutter_rust_bridge_codegen
# （生成物已提交在 word_render/lib/src/rust/）。
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"          # samples/
NATIVE_CRATE="$ROOT/docx-mvp-render/native"                          # word_core_ffi crate
PACKAGE="$ROOT/libs/word/word_render/native"                         # 产物落包目录
TARGET="${1:-host}"

cargo_release() {
  cargo build --release -p word_core_ffi
}

case "$TARGET" in
  host)
    case "$(uname -s)-$(uname -m)" in
      Darwin-arm64)
        cargo_release
        mkdir -p "$PACKAGE/macos/arm64"
        cp "$NATIVE_CRATE/target/release/libword_core_ffi.dylib" "$PACKAGE/macos/arm64/"
        codesign --force -s - "$PACKAGE/macos/arm64/libword_core_ffi.dylib"
        echo "→ $PACKAGE/macos/arm64/libword_core_ffi.dylib"
        ;;
      Darwin-*)
        echo "未支持：macOS 非 arm64 构建请先追加分支（x86_64 需交叉目标）" >&2; exit 1
        ;;
      Linux-*)
        (cd "$NATIVE_CRATE" && cargo_release)
        mkdir -p "$PACKAGE/linux/x86_64"
        cp "$NATIVE_CRATE/target/release/libword_core_ffi.so" "$PACKAGE/linux/x86_64/"
        echo "→ $PACKAGE/linux/x86_64/libword_core_ffi.so"
        ;;
      MINGW*|MSYS*|CYGWIN*)
        (cd "$NATIVE_CRATE" && cargo_release)
        mkdir -p "$PACKAGE/windows/x86_64"
        cp "$NATIVE_CRATE/target/release/word_core_ffi.dll" "$PACKAGE/windows/x86_64/"
        echo "→ $PACKAGE/windows/x86_64/word_core_ffi.dll"
        ;;
      *) echo "未识别宿主机 $(uname -s)" >&2; exit 1 ;;
    esac
    ;;

  android)
    command -v cargo-ndk >/dev/null || {
      echo "需要 cargo-ndk：cargo install cargo-ndk" >&2; exit 1; }
    ABIS=(arm64-v8a armeabi-v7a x86_64)
    for abi in "${ABIS[@]}"; do
      cargo ndk -t "$abi" -o "$PACKAGE/android" build --release -p word_core_ffi
      # cargo-ndk 输出 $PACKAGE/android/$abi/libword_core_ffi.so
      echo "→ $PACKAGE/android/$abi/libword_core_ffi.so"
    done
    echo "注意：Android 消费者需把对应 ABI 的 .so 放进 jniLibs（详见 README）。"
    ;;

  ios)
    echo "FRB/iOS 走静态链接（XCFramework + 插件集成），当前为骨架："
    echo "  1) cargo build --release --target aarch64-apple-ios （需 rustup target add）"
    echo "  2) xcodebuild -create-xcframework ... 并作为 flutter plugin 随包分发"
    echo "  3) 见 README「平台支持矩阵」iOS 行"
    ;;

  *)
    echo "用法: $0 [host|android|ios]" >&2; exit 1 ;;
esac