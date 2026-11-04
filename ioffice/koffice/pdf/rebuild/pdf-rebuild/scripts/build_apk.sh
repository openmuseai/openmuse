#!/usr/bin/env bash
# build_apk.sh - manual APK build: javac -> d8 -> aapt2 -> zip dex/libs -> sign.
# Produces android/ksp-diff.apk containing BOTH libs (rebuilt + original).
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SDK="${ANDROID_HOME:-$HOME/Library/Android/sdk}"
BT="$SDK/build-tools/36.1.0"
PLATFORM="$SDK/platforms/android-34/android.jar"
JH="${JAVA_HOME:-$(/usr/libexec/java_home -v 17)}"
OUT="$ROOT/android/build"
APK="$ROOT/android/ksp-diff.apk"

# The APK embeds the rebuilt JNI library. Recompile it here so a manual
# package cannot silently test a stale native binary.
"$ROOT/scripts/build_freetext_android.sh"

rm -rf "$OUT"; mkdir -p "$OUT/classes"
SRCS=()
while IFS= read -r rel; do
  [[ -z "$rel" || "$rel" == \#* ]] && continue
  SRCS+=("$ROOT/harness/java/$rel")
done < "$ROOT/harness/android-sources.txt"

echo "==[ javac ]=="
"$JH/bin/javac" -source 8 -target 8 -bootclasspath "$PLATFORM" \
  -classpath "$PLATFORM" -d "$OUT/classes" "${SRCS[@]}"

echo "==[ d8 dex ]=="
"$BT/d8" --release --lib "$PLATFORM" --output "$OUT" \
  $(find "$OUT/classes" -name "*.class")

echo "==[ aapt2 link ]=="
"$BT/aapt2" link -o "$OUT/base.apk" \
  -I "$PLATFORM" \
  --manifest "$ROOT/android/AndroidManifest.xml" \
  -A "$ROOT/harness/assets" \
  --min-sdk-version 26 --target-sdk-version 34

echo "==[ add dex + native libs ]=="
mkdir -p "$OUT/lib/arm64-v8a"
cp "$ROOT/build/libkspfreetext.so" "$OUT/lib/arm64-v8a/"
cp "$ROOT/3rdparty/prebuilt/android-arm64/lib/libpdfium.so" "$OUT/lib/arm64-v8a/"
cp "$ROOT/../pdf-core/jni/arm64-v8a/libkwopdf.so" "$OUT/lib/arm64-v8a/"
cd "$OUT"
zip -q "$OUT/base.apk" classes.dex
zip -q -r "$OUT/base.apk" lib

echo "==[ sign ]=="
KS="$ROOT/android/debug.keystore"
[ -f "$KS" ] || "$JH/bin/keytool" -genkeypair -keystore "$KS" -alias androiddebugkey \
  -storepass android -keypass android -dname "CN=Debug" -keyalg RSA -validity 10000
"$BT/apksigner" sign --ks "$KS" --ks-pass pass:android --ks-key-alias androiddebugkey \
  --key-pass pass:android --out "$APK" --in "$OUT/base.apk"
"$BT/apksigner" verify --print-certs "$APK" >/dev/null
echo "==[ contents ]=="
unzip -l "$APK" | grep -E "classes.dex|lib/|AndroidManifest"
echo "APK OK: $APK ($(du -h "$APK" | cut -f1))"
