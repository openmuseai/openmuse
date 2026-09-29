#!/usr/bin/env bash
set -euo pipefail

repo_root="$(cd "$(dirname "$0")/.." && pwd)"
apk="${OPENMUSE_ANDROID_APK:-$repo_root/target/android-alpha/OpenMuse-Android-Alpha-arm64.apk}"
package="io.openmuse.openmuse_mobile"
activity="$package/.MainActivity"
background_cycles="${OPENMUSE_BACKGROUND_CYCLES:-50}"
rotation_cycles="${OPENMUSE_ROTATION_CYCLES:-100}"
idle_seconds="${OPENMUSE_SOAK_SECONDS:-0}"
step_delay="${OPENMUSE_SOAK_STEP_DELAY:-0.10}"

command -v adb >/dev/null
test -f "$apk"

serial="${OPENMUSE_ANDROID_SERIAL:-}"
if [[ -z "$serial" ]]; then
  devices="$(adb devices | awk 'NR > 1 && $2 == "device" { print $1 }')"
  if [[ "$(printf '%s\n' "$devices" | sed '/^$/d' | wc -l | tr -d ' ')" -ne 1 ]]; then
    echo "Set OPENMUSE_ANDROID_SERIAL; expected exactly one ready ADB device" >&2
    exit 1
  fi
  serial="$devices"
fi

adb_device=(adb -s "$serial")
if [[ "$(${adb_device[@]} get-state)" != "device" ]]; then
  echo "ADB device $serial is not ready" >&2
  exit 1
fi

timestamp="$(date -u +%Y%m%dT%H%M%SZ)"
evidence_dir="$repo_root/target/android-soak/$timestamp"
mkdir -p "$evidence_dir"

original_auto_rotation="$(${adb_device[@]} shell settings get system accelerometer_rotation | tr -d '\r')"
original_user_rotation="$(${adb_device[@]} shell settings get system user_rotation | tr -d '\r')"
restore_rotation() {
  ${adb_device[@]} shell settings put system accelerometer_rotation "$original_auto_rotation" >/dev/null || true
  ${adb_device[@]} shell settings put system user_rotation "$original_user_rotation" >/dev/null || true
}
trap restore_rotation EXIT

assert_unlocked() {
  local window_state
  window_state="$(${adb_device[@]} shell dumpsys window)"
  if grep -Eiq 'mDreamingLockscreen=true|mShowingLockscreen=true|isStatusBarKeyguard=true' <<<"$window_state"; then
    echo "Device $serial is locked; unlock it before running the soak" >&2
    return 1
  fi
}

assert_healthy() {
  local pid focused
  pid="$(${adb_device[@]} shell pidof "$package" | tr -d '\r')"
  test -n "$pid"
  focused="$(${adb_device[@]} shell dumpsys window | grep -E 'mCurrentFocus|mFocusedApp' | head -2)"
  grep -q "$package" <<<"$focused"
}

capture_ui() {
  local name="$1"
  ${adb_device[@]} exec-out screencap -p >"$evidence_dir/$name.png"
  ${adb_device[@]} shell uiautomator dump "/sdcard/$name.xml" >/dev/null
  ${adb_device[@]} pull "/sdcard/$name.xml" "$evidence_dir/$name.xml" >/dev/null
  grep -q "package=\"$package\"" "$evidence_dir/$name.xml"
  grep -q 'content-desc="OpenMuse"' "$evidence_dir/$name.xml"
}

echo "Installing $(basename "$apk") on $serial"
${adb_device[@]} install -r "$apk" | tee "$evidence_dir/install.txt"
${adb_device[@]} shell input keyevent KEYCODE_WAKEUP
${adb_device[@]} shell wm dismiss-keyguard || true
assert_unlocked
${adb_device[@]} logcat -b all -c
${adb_device[@]} shell am force-stop "$package"
${adb_device[@]} shell am start -W -n "$activity" | tee "$evidence_dir/launch.txt"
sleep 2
assert_healthy
capture_ui before

if [[ "${OPENMUSE_EXPECT_SIGNED_OUT:-1}" == "1" ]]; then
  grep -q 'content-desc="未登录"' "$evidence_dir/before.xml"
  grep -q 'content-desc="请登录以访问 Cloud Workspace"' "$evidence_dir/before.xml"
fi

${adb_device[@]} shell dumpsys package "$package" >"$evidence_dir/package.txt"
${adb_device[@]} shell getprop >"$evidence_dir/device-properties.txt"
shasum -a 256 "$apk" >"$evidence_dir/apk.sha256"

echo "Running $background_cycles background/foreground cycles"
for ((i = 1; i <= background_cycles; i++)); do
  ${adb_device[@]} shell input keyevent KEYCODE_HOME
  sleep "$step_delay"
  ${adb_device[@]} shell am start -n "$activity" >/dev/null
  sleep "$step_delay"
  if ((i % 10 == 0)); then
    assert_healthy
    echo "  lifecycle $i/$background_cycles"
  fi
done

echo "Running $rotation_cycles forced orientation changes"
${adb_device[@]} shell settings put system accelerometer_rotation 0
for ((i = 1; i <= rotation_cycles; i++)); do
  ${adb_device[@]} shell settings put system user_rotation "$((i % 2))"
  sleep "$step_delay"
  if ((i % 20 == 0)); then
    assert_healthy
    echo "  orientation $i/$rotation_cycles"
  fi
done

if ((idle_seconds > 0)); then
  echo "Keeping the foreground app alive for $idle_seconds seconds"
  deadline=$((SECONDS + idle_seconds))
  while ((SECONDS < deadline)); do
    sleep 10
    assert_healthy
  done
fi

capture_ui after
assert_healthy
${adb_device[@]} logcat -d -v threadtime >"$evidence_dir/logcat.txt"
${adb_device[@]} logcat -b crash -d -v threadtime >"$evidence_dir/crash-buffer.txt"
${adb_device[@]} shell dumpsys activity exit-info "$package" >"$evidence_dir/exit-info.txt" || true

if grep -Fq "$package" "$evidence_dir/crash-buffer.txt" || grep -Eiq "ANR in $package" "$evidence_dir/logcat.txt"; then
  echo "A crash or ANR was found in logcat" >&2
  grep -Ei "FATAL EXCEPTION|ANR in $package|Process: $package" "$evidence_dir/crash-buffer.txt" "$evidence_dir/logcat.txt" >&2 || true
  exit 1
fi

model="$(${adb_device[@]} shell getprop ro.product.model | tr -d '\r')"
sdk="$(${adb_device[@]} shell getprop ro.build.version.sdk | tr -d '\r')"
abi="$(${adb_device[@]} shell getprop ro.product.cpu.abi | tr -d '\r')"
apk_digest="$(awk '{print $1}' "$evidence_dir/apk.sha256")"
{
  echo "# OpenMuse Android device soak"
  echo
  echo "- Result: PASS"
  echo "- UTC: $timestamp"
  echo "- Device: $serial / $model / Android API $sdk / $abi"
  echo "- APK SHA-256: $apk_digest"
  echo "- Background/foreground cycles: $background_cycles"
  echo "- Forced orientation changes: $rotation_cycles"
  echo "- Foreground idle seconds: $idle_seconds"
  echo "- UI scope: $([[ ${OPENMUSE_EXPECT_SIGNED_OUT:-1} == 1 ]] && echo signed-out-production || echo externally-provisioned)"
} >"$evidence_dir/report.md"

echo "Android device soak PASS: $evidence_dir"
