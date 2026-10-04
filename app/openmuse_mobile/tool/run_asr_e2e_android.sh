#!/usr/bin/env bash
set -euo pipefail

model_dir="${OPENMUSE_ASR_MODEL_DIR:?Set OPENMUSE_ASR_MODEL_DIR to the extracted zh14m model directory}"
audio_file="${OPENMUSE_ASR_TEST_AUDIO:?Set OPENMUSE_ASR_TEST_AUDIO to a mono PCM WAV fixture}"
device_serial="${ANDROID_SERIAL:-}"

app_id="io.openmuse.openmuse_mobile"
device_model_dir="/data/user/0/${app_id}/files/speech-model"
device_audio_dir="/data/user/0/${app_id}/files/speech-test"
apk_path="build/app/outputs/flutter-apk/app-debug.apk"

encoder="${model_dir}/encoder-epoch-99-avg-1.int8.onnx"
decoder="${model_dir}/decoder-epoch-99-avg-1.onnx"
joiner="${model_dir}/joiner-epoch-99-avg-1.int8.onnx"
tokens="${model_dir}/tokens.txt"

for required_file in "$encoder" "$decoder" "$joiner" "$tokens" "$audio_file"; do
  if [[ ! -f "$required_file" ]]; then
    echo "Missing E2E fixture: $required_file" >&2
    exit 2
  fi
done

adb_command=(adb)
if [[ -n "$device_serial" ]]; then
  adb_command+=( -s "$device_serial" )
fi

flutter build apk --debug \
  --dart-define="OPENMUSE_SPEECH_MODEL_DIR=${device_model_dir}" \
  --dart-define="OPENMUSE_SPEECH_TEST_AUDIO=${device_audio_dir}/input.wav" \
  --dart-define=OPENMUSE_SPEECH_E2E_AUTORUN=true

"${adb_command[@]}" install -r -t "$apk_path"
"${adb_command[@]}" push "$encoder" /data/local/tmp/openmuse-asr-encoder.onnx
"${adb_command[@]}" push "$decoder" /data/local/tmp/openmuse-asr-decoder.onnx
"${adb_command[@]}" push "$joiner" /data/local/tmp/openmuse-asr-joiner.onnx
"${adb_command[@]}" push "$tokens" /data/local/tmp/openmuse-asr-tokens.txt
"${adb_command[@]}" push "$audio_file" /data/local/tmp/openmuse-asr-input.wav

"${adb_command[@]}" shell run-as "$app_id" mkdir -p files/speech-model files/speech-test
"${adb_command[@]}" shell run-as "$app_id" cp /data/local/tmp/openmuse-asr-encoder.onnx files/speech-model/encoder-epoch-99-avg-1.int8.onnx
"${adb_command[@]}" shell run-as "$app_id" cp /data/local/tmp/openmuse-asr-decoder.onnx files/speech-model/decoder-epoch-99-avg-1.onnx
"${adb_command[@]}" shell run-as "$app_id" cp /data/local/tmp/openmuse-asr-joiner.onnx files/speech-model/joiner-epoch-99-avg-1.int8.onnx
"${adb_command[@]}" shell run-as "$app_id" cp /data/local/tmp/openmuse-asr-tokens.txt files/speech-model/tokens.txt
"${adb_command[@]}" shell run-as "$app_id" cp /data/local/tmp/openmuse-asr-input.wav files/speech-test/input.wav

"${adb_command[@]}" logcat -c
"${adb_command[@]}" shell am force-stop "$app_id"
"${adb_command[@]}" shell input keyevent KEYCODE_WAKEUP
"${adb_command[@]}" shell wm dismiss-keyguard
"${adb_command[@]}" shell am start -W -n "${app_id}/.MainActivity"

result=""
for _ in $(seq 1 45); do
  logs="$("${adb_command[@]}" logcat -d -v brief)"
  if error_line="$(printf '%s\n' "$logs" | grep -F 'OPENMUSE_ASR_E2E_ERROR=' | tail -1)" && [[ -n "$error_line" ]]; then
    echo "$error_line" >&2
    exit 3
  fi
  if final_line="$(printf '%s\n' "$logs" | grep -F 'OPENMUSE_ASR_E2E_FINAL=' | tail -1)" && [[ -n "$final_line" ]]; then
    result="${final_line#*OPENMUSE_ASR_E2E_FINAL=}"
    break
  fi
  sleep 1
done

if [[ -z "$result" ]]; then
  echo 'Timed out waiting for a non-empty ASR final result.' >&2
  exit 4
fi

composer_verified=false
for _ in $(seq 1 10); do
  "${adb_command[@]}" shell uiautomator dump /sdcard/openmuse-asr-window.xml >/dev/null
  ui_xml="$("${adb_command[@]}" shell cat /sdcard/openmuse-asr-window.xml)"
  if printf '%s\n' "$ui_xml" | grep -F "package=\"${app_id}\"" >/dev/null && \
      printf '%s\n' "$ui_xml" | grep -F "text=\"${result}\"" >/dev/null; then
    composer_verified=true
    break
  fi
  sleep 1
done
if [[ "$composer_verified" != true ]]; then
  echo "ASR returned text, but Composer did not contain it: $result" >&2
  exit 5
fi

echo "Android ASR E2E passed: $result"
