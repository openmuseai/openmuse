#!/usr/bin/env bash
set -euo pipefail

APP_ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REBUILD_ROOT="$(cd "$APP_ROOT/../pdf-rebuild" && pwd)"
FLUTTER="${FLUTTER_BIN:-flutter}"
TARGET="${1:-shared}"

shared() {
  "$FLUTTER" analyze --no-pub
  "$FLUTTER" test --no-pub
}

macos() {
  if [[ "$(uname -s)" != Darwin ]]; then
    echo "macOS verification requires a macOS host" >&2
    exit 2
  fi
  bash "$APP_ROOT/tool/prepare_macos_native.sh"
  bash "$REBUILD_ROOT/scripts/run_page_operations_tests.sh"
  KSP_MAC_EDIT_DYLIB="$REBUILD_ROOT/build/libkspfreetext.dylib" \
  KSP_MAC_EDIT_TEST_PDF="$REBUILD_ROOT/harness/assets/editable-two-page.pdf" \
  KSP_PAGE_OPS_TEST_PDF="$REBUILD_ROOT/harness/assets/editable-two-page.pdf" \
    "$FLUTTER" test --no-pub
  "$FLUTTER" build macos --release --no-pub
  local bundled_pdfium="$APP_ROOT/build/macos/Build/Products/Release/pdf_rebuild_flutter.app/Contents/Frameworks/App.framework/Versions/A/Resources/flutter_assets/assets/native/libpdfium.dylib"
  cmp "$REBUILD_ROOT/3rdparty/prebuilt/lib/libpdfium.dylib" "$bundled_pdfium"
  local bundled_viewer="$APP_ROOT/build/macos/Build/Products/Release/pdf_rebuild_flutter.app/Contents/Frameworks/App.framework/Versions/A/Resources/flutter_assets/assets/native/libpdfium_viewer.dylib"
  cmp "$REBUILD_ROOT/3rdparty/prebuilt/lib/libpdfium_viewer.dylib" "$bundled_viewer"
  local bundled_editor="$APP_ROOT/build/macos/Build/Products/Release/pdf_rebuild_flutter.app/Contents/Frameworks/App.framework/Versions/A/Resources/flutter_assets/assets/native/libkspfreetext.dylib"
  cmp "$REBUILD_ROOT/build/libkspfreetext.dylib" "$bundled_editor"
  nm -gU "$bundled_editor" | grep '_ksp_product_edit_render_page_region$' > /dev/null
  nm -gU "$bundled_editor" | grep '_ksp_product_edit_read_page_text$' > /dev/null
  nm -gU "$bundled_editor" | grep '_ksp_product_edit_read_page_links$' > /dev/null
  "$FLUTTER" test --no-pub integration_test/product_flow_test.dart -d macos \
    --dart-define=KSP_PRODUCT_READER=true
}

android() {
  bash "$APP_ROOT/tool/prepare_android_native.sh"
  "$FLUTTER" build apk --release --target-platform android-arm64 --no-pub
  local apk="$APP_ROOT/build/app/outputs/flutter-apk/app-release.apk"
  local pdfium_count
  pdfium_count="$(unzip -Z1 "$apk" | grep -c '^lib/arm64-v8a/libpdfium.so$')"
  if [[ "$pdfium_count" != 1 ]]; then
    echo "Expected exactly one Android arm64 PDFium binary in APK" >&2
    exit 2
  fi
  local bundled_sha source_sha
  bundled_sha="$(unzip -p "$apk" lib/arm64-v8a/libpdfium.so | shasum -a 256 | awk '{print $1}')"
  source_sha="$(shasum -a 256 "$REBUILD_ROOT/3rdparty/prebuilt/android-arm64/lib/libpdfium.so" | awk '{print $1}')"
  if [[ "$bundled_sha" != "$source_sha" ]]; then
    echo "APK PDFium does not match the product binary" >&2
    exit 2
  fi
  local viewer_sha viewer_source_sha
  viewer_sha="$(unzip -p "$apk" lib/arm64-v8a/libpdfium_viewer.so | shasum -a 256 | awk '{print $1}')"
  viewer_source_sha="$(shasum -a 256 "$REBUILD_ROOT/3rdparty/prebuilt/android-arm64/lib/libpdfium_viewer.so" | awk '{print $1}')"
  if [[ "$viewer_sha" != "$viewer_source_sha" ]]; then
    echo "APK reader PDFium does not match the product source variant" >&2
    exit 2
  fi
  if [[ "$(unzip -p "$apk" lib/arm64-v8a/libkspfreetext.so | strings | grep -c '^ksp_product_edit_abi_version$' || true)" -lt 1 ]]; then
    echo "APK is missing the product edit ABI" >&2
    exit 2
  fi
  local editor_sha editor_source_sha
  editor_sha="$(unzip -p "$apk" lib/arm64-v8a/libkspfreetext.so | shasum -a 256 | awk '{print $1}')"
  local sdk ndk host_tag stripped_editor
  sdk="${ANDROID_SDK_ROOT:-${ANDROID_HOME:-$HOME/Library/Android/sdk}}"
  ndk="${ANDROID_NDK_HOME:-$sdk/ndk/27.0.12077973}"
  case "$(uname -s)" in
    Darwin) host_tag=darwin-x86_64 ;;
    Linux) host_tag=linux-x86_64 ;;
    *) echo "Unsupported Android build host" >&2; exit 2 ;;
  esac
  stripped_editor="$(mktemp)"
  cp "$REBUILD_ROOT/build/libkspfreetext.so" "$stripped_editor"
  "$ndk/toolchains/llvm/prebuilt/$host_tag/bin/llvm-strip" --strip-unneeded "$stripped_editor"
  editor_source_sha="$(shasum -a 256 "$stripped_editor" | awk '{print $1}')"
  rm -f "$stripped_editor"
  if [[ "$editor_sha" != "$editor_source_sha" ]]; then
    echo "APK product editor does not match the tested native binary" >&2
    exit 2
  fi
  if [[ "$(unzip -p "$apk" lib/arm64-v8a/libkspfreetext.so | strings | grep -c '^ksp_product_edit_render_page_region$' || true)" -lt 1 ]]; then
    echo "APK is missing product ABI v4 region rendering" >&2
    exit 2
  fi
  if [[ "$(unzip -p "$apk" lib/arm64-v8a/libkspfreetext.so | strings | grep -c '^ksp_product_edit_read_page_text$' || true)" -lt 1 ]]; then
    echo "APK is missing product ABI v5 page text" >&2
    exit 2
  fi
  if [[ "$(unzip -p "$apk" lib/arm64-v8a/libkspfreetext.so | strings | grep -c '^ksp_product_edit_read_page_links$' || true)" -lt 1 ]]; then
    echo "APK is missing product ABI v6 page links" >&2
    exit 2
  fi
  if [[ "${ANDROID_DEVICE:-0}" == 1 ]]; then
    bash "$REBUILD_ROOT/scripts/run_android_product_smoke.sh"
    bash "$REBUILD_ROOT/scripts/run_android_edit_smoke.sh"
  fi
  if [[ "${ANDROID_UI:-0}" == 1 ]]; then
    "$FLUTTER" test --no-pub integration_test/product_flow_test.dart \
      -d "${ANDROID_DEVICE_ID:-emulator-5554}" \
      --dart-define=KSP_PRODUCT_READER=true
  fi
}

web() {
  node "$REBUILD_ROOT/harness/web_product_smoke.mjs" \
    "$APP_ROOT/web/ksp_product.mjs" \
    "$REBUILD_ROOT/harness/assets/editable-two-page.pdf"
  node "$REBUILD_ROOT/harness/web_bridge_smoke.mjs" \
    "$APP_ROOT/web/ksp_product_bridge.mjs" \
    "$REBUILD_ROOT/harness/assets/editable-two-page.pdf" \
    "$REBUILD_ROOT/harness/assets/page-order.pdf"
  "$FLUTTER" build web --release --no-pub
  "$FLUTTER" build web --release --wasm --no-pub
  cmp "$APP_ROOT/web/ksp_product.wasm" "$APP_ROOT/build/web/ksp_product.wasm"
  if find "$APP_ROOT/build/web" -type f \( -name pdfium.wasm -o -name pdfium_client.js -o -name pdfium_worker.js \) | grep -q .; then
    echo "Web release contains a second PDFium runtime" >&2
    exit 2
  fi
  echo "Web product ABI, JavaScript bridge and Flutter builds passed."
}

rust() {
  bash "$REBUILD_ROOT/scripts/run_rust_product_spike.sh"
  cargo check --manifest-path "$REBUILD_ROOT/product-rust/Cargo.toml" \
    --target wasm32-unknown-unknown
}

rust_android() {
  bash "$REBUILD_ROOT/scripts/run_rust_product_android.sh"
}

ios() {
  if [[ "$(uname -s)" != Darwin ]]; then
    echo "iOS simulator verification requires a macOS host" >&2
    exit 2
  fi
  bash "$APP_ROOT/tool/prepare_ios_native.sh"
  if [[ "${IOS_SIMULATOR:-0}" == 1 ]]; then
    bash "$REBUILD_ROOT/scripts/run_ios_product_smoke.sh"
    if [[ -z "${IOS_SIMULATOR_UDID:-}" ]]; then
      echo "Set IOS_SIMULATOR_UDID to the booted simulator UUID" >&2
      exit 2
    fi
    "$FLUTTER" test --no-pub integration_test/product_flow_test.dart \
      -d "$IOS_SIMULATOR_UDID"
  fi
  "$FLUTTER" build ios --simulator --no-codesign --no-pub
  test -d "$APP_ROOT/build/ios/iphonesimulator/Runner.app"
  local frameworks="$APP_ROOT/build/ios/iphonesimulator/Runner.app/Frameworks"
  nm -gU "$frameworks/libpdfium.dylib" | grep '_KSPTextObj_SetGlyphPositions$' > /dev/null
  nm -gU "$frameworks/libpdfium_viewer.dylib" | grep '_KSPTextObj_SetGlyphPositions$' > /dev/null
  nm -gU "$frameworks/libksp_product.dylib" | grep '_ksp_product_edit_abi_version$' > /dev/null
  codesign -v "$frameworks/libpdfium.dylib"
  codesign -v "$frameworks/libpdfium_viewer.dylib"
  codesign -v "$frameworks/libksp_product.dylib"
  echo "iOS simulator app built with rebuild PDFium and product edit bridge."
}

cd "$APP_ROOT"
"$FLUTTER" pub get
case "$TARGET" in
  shared) shared ;;
  macos) macos ;;
  android) android ;;
  web) web ;;
  rust) rust ;;
  rust-android) rust_android ;;
  ios) ios ;;
  all)
    shared
    macos
    android
    web
    ;;
  *)
    echo "Usage: $0 {shared|macos|android|web|rust|rust-android|ios|all}" >&2
    exit 2
    ;;
esac
