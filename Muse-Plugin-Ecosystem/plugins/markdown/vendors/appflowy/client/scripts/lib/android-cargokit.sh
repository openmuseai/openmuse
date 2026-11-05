#!/usr/bin/env bash
# Build CargoKit Flutter plugin cdylibs into the app jniLibs folder.
#
# irondash_engine_context 0.5.5 looks for com.flutter.gradle.FlutterPlugin.
# This project still applies the legacy groovy FlutterPlugin, so Gradle skips
# CargoKit for that plugin and release APKs miss libirondash_engine_context_native.so.

muse_android_plugin_root() {
  local key="$1"
  local plugins
  plugins="$(muse_flutter_dir)/.flutter-plugins"
  if [[ ! -f "$plugins" ]]; then
    echo "missing $plugins (run flutter pub get)" >&2
    return 1
  fi
  local path
  path="$(awk -F= -v k="$key" '$1 == k { print $2; exit }' "$plugins" | tr -d '\r')"
  if [[ -z "$path" || ! -d "$path" ]]; then
    echo "Flutter plugin $key not in .flutter-plugins" >&2
    return 1
  fi
  printf '%s\n' "${path%/}"
}

muse_android_ndk_revision() {
  local props="${ANDROID_NDK_HOME}/source.properties"
  if [[ ! -f "$props" ]]; then
    echo "missing $props" >&2
    return 1
  fi
  awk -F= '/Pkg.Revision/ { gsub(/^[ \t]+|[ \t]+$/, "", $2); print $2; exit }' "$props"
}

# Usage: muse_android_cargokit_build <plugin_key> <manifest_relpath> <libname>
muse_android_cargokit_build() {
  local plugin_key="$1"
  local manifest_rel="$2"
  local libname="$3"
  local plugin_root runner android_dir flutter_dir out_dir ndk_rev
  plugin_root="$(muse_android_plugin_root "$plugin_key")"
  flutter_dir="$(muse_flutter_dir)"
  android_dir="${flutter_dir}/android"
  runner="${plugin_root}/cargokit/run_build_tool.sh"
  if [[ ! -x "$runner" ]]; then
    chmod +x "$runner"
  fi
  ndk_rev="$(muse_android_ndk_revision)"
  out_dir="${flutter_dir}/build/${plugin_key}/jniLibs/${MODE:-release}"
  mkdir -p "$out_dir" "${flutter_dir}/build/${plugin_key}/build_tool"

  echo "==> CargoKit ${plugin_key} (${libname}, ${MODE:-release}, arm64)"
  export FLUTTER_ROOT="${FLUTTER_HOME:-${FLUTTER_ROOT:-}}"
  env \
    CARGOKIT_ROOT_PROJECT_DIR="$android_dir" \
    CARGOKIT_TOOL_TEMP_DIR="${flutter_dir}/build/${plugin_key}/build_tool" \
    CARGOKIT_MANIFEST_DIR="${plugin_root}/android/${manifest_rel}" \
    CARGOKIT_CONFIGURATION="${MODE:-release}" \
    CARGOKIT_TARGET_TEMP_DIR="${flutter_dir}/build/${plugin_key}/cargokit_target" \
    CARGOKIT_OUTPUT_DIR="$out_dir" \
    CARGOKIT_NDK_VERSION="$ndk_rev" \
    CARGOKIT_SDK_DIR="${ANDROID_HOME:?ANDROID_HOME is required}" \
    CARGOKIT_COMPILE_SDK_VERSION=35 \
    CARGOKIT_MIN_SDK_VERSION=29 \
    CARGOKIT_TARGET_PLATFORMS=android-arm64 \
    CARGOKIT_JAVA_HOME="${JAVA_HOME:?JAVA_HOME is required}" \
    bash "$runner" build-gradle

  local so
  so="$(find "$out_dir" -name "lib${libname}.so" | head -n 1)"
  if [[ -z "$so" || ! -f "$so" ]]; then
    echo "CargoKit did not produce lib${libname}.so under $out_dir" >&2
    return 1
  fi
  local dest="${flutter_dir}/android/app/src/main/jniLibs/arm64-v8a"
  mkdir -p "$dest"
  cp "$so" "$dest/"
  echo "    copied $(basename "$so") -> $dest"
}

muse_android_ensure_plugin_cdylibs() {
  muse_android_cargokit_build irondash_engine_context rust irondash_engine_context_native
}

# cargo-make copies rust-lib/jniLibs over app jniLibs and drops irondash + libc++.
muse_android_stage_pack_jni() {
  local dest cxx
  dest="$(muse_flutter_dir)/android/app/src/main/jniLibs/arm64-v8a"
  mkdir -p "$dest"
  cxx="$(find "${ANDROID_NDK_HOME:?ANDROID_NDK_HOME is required}/toolchains/llvm/prebuilt" \
    -path '*/sysroot/usr/lib/aarch64-linux-android/libc++_shared.so' | head -n 1)"
  if [[ -z "$cxx" || ! -f "$cxx" ]]; then
    echo "libc++_shared.so not found under $ANDROID_NDK_HOME" >&2
    return 1
  fi
  cp "$cxx" "$dest/"
  echo "==> staged $(basename "$cxx") -> $dest"
  muse_android_ensure_plugin_cdylibs
}
