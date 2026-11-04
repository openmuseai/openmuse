#!/usr/bin/env bash
# Run one Android contract mode in a fresh app process. The APK must be installed.
set -euo pipefail
MODE="${1:-}"
case "$MODE" in original|rebuilt|shape-original|shape-rebuilt|paper-original|paper-rebuilt) ;; *) echo "usage: $0 original|rebuilt|shape-original|shape-rebuilt|paper-original|paper-rebuilt" >&2; exit 64;; esac
ROOT="$(cd "$(dirname "$0")/.." && pwd)"

echo "mode=$MODE"
echo "device_serial=$(adb get-serialno)"
echo "device_model=$(adb shell getprop ro.product.model | tr -d '\r')"
echo "device_api=$(adb shell getprop ro.build.version.sdk | tr -d '\r')"
echo "apk_sha256=$(shasum -a 256 "$ROOT/android/ksp-diff.apk" | awk '{print $1}')"
echo "input_pdf_sha256=$(shasum -a 256 "$ROOT/harness/assets/testfile_pdf_clip.pdf" | awk '{print $1}')"
echo "password_pdf_sha256=$(shasum -a 256 "$ROOT/harness/assets/password-user123.pdf" | awk '{print $1}')"
echo "editable_pdf_sha256=$(shasum -a 256 "$ROOT/harness/assets/editable-helvetica.pdf" | awk '{print $1}')"
echo "page_object_pdf_sha256=$(shasum -a 256 "$ROOT/harness/assets/editable-objects.pdf" | awk '{print $1}')"
echo "glyph_caret_pdf_sha256=$(shasum -a 256 "$ROOT/harness/assets/glyph-caret-corpus.pdf" | awk '{print $1}')"
echo "form_edit_pdf_sha256=$(shasum -a 256 "$ROOT/harness/assets/form-edit-corpus.pdf" | awk '{print $1}')"
echo "arabic_shape_pdf_sha256=$(shasum -a 256 "$ROOT/harness/assets/arabic-shape-corpus.pdf" | awk '{print $1}')"
echo "arabic_shape_font_sha256=$(shasum -a 256 "$ROOT/harness/assets/fonts/NotoSansArabic-WT6-subset.ttf" | awk '{print $1}')"
for corpus in "$ROOT"/harness/assets/edit-domain-corpus/*.pdf; do
  corpus_name="$(basename "$corpus")"
  echo "domain_corpus_${corpus_name}_sha256=$(shasum -a 256 "$corpus" | awk '{print $1}')"
done
echo "pdfium_so_sha256=$(shasum -a 256 "$ROOT/3rdparty/prebuilt/android-arm64/lib/libpdfium.so" | awk '{print $1}')"
echo "rebuilt_so_sha256=$(shasum -a 256 "$ROOT/build/libkspfreetext.so" | awk '{print $1}')"
echo "original_so_sha256=$(shasum -a 256 "$ROOT/../pdf-core/jni/arm64-v8a/libkwopdf.so" | awk '{print $1}')"
echo "android_contract_sources_sha256=$(while IFS= read -r rel; do shasum -a 256 "$ROOT/harness/java/$rel"; done < "$ROOT/harness/android-sources.txt" | shasum -a 256 | awk '{print $1}')"

RUN_ID="$(date +%s)-$$"
RESULT="/sdcard/Android/data/cn.wps.rebuild/files/diff_${MODE}.txt"
adb shell am force-stop cn.wps.rebuild >/dev/null 2>&1 || true
adb shell rm -f "$RESULT"
adb logcat -c
adb shell am start -n cn.wps.rebuild/.DiffActivity --es mode "$MODE" \
  --es runId "$RUN_ID" --ei paperPage "${PAPER_PAGE:-0}" \
  --ei paperWarmupFrom "${PAPER_WARMUP_FROM:-1}" \
  --ei paperWarmupThrough "${PAPER_WARMUP_THROUGH:-0}" --ei algGen "${ALG_GEN:-0}" \
  --ei paperEditStatus "${PAPER_EDIT_STATUS:-1}" \
  --ei paperFocusEditStatus "${PAPER_FOCUS_EDIT_STATUS:-${PAPER_EDIT_STATUS:-1}}" \
  --ez paperRecognizeWarmup "${PAPER_RECOGNIZE_WARMUP:-true}" \
  --ez paperProbeEditorHits "${PAPER_PROBE_EDITOR_HITS:-false}" \
  --ef paperHitX "${PAPER_HIT_X:--100000}" --ef paperHitY "${PAPER_HIT_Y:--100000}"

# Logcat is a bounded ring buffer: full corpus output can evict DONE and PASS.
# A fresh run id prevents accepting a previous successful run after a crash.
LOGS=""
for ((attempt=0; attempt<${CONTRACT_TIMEOUT_SECONDS:-240}; attempt++)); do
  LOGS="$(adb shell cat "$RESULT" 2>/dev/null | tr -d '\r' || true)"
  if grep -qx "RUN_ID=$RUN_ID" <<<"$LOGS" && grep -qx "DONE_$MODE" <<<"$LOGS"; then break; fi
  sleep 1
done
printf '%s\n' "$LOGS"
CRASHES="$(adb logcat -d -b crash 2>/dev/null | grep -cE 'signal|FATAL EXCEPTION' || true)"
echo "crashes=$CRASHES"

grep -qx "RUN_ID=$RUN_ID" <<<"$LOGS" || { echo "BLOCKED missing current run id"; exit 3; }
grep -qx "DONE_$MODE" <<<"$LOGS" || { echo "BLOCKED missing completion marker"; exit 3; }
grep -Eq 'FAIL |EXC ' <<<"$LOGS" && { echo "FAIL contract log contains failure"; exit 4; }
[[ "$CRASHES" == "0" ]] || { echo "FAIL crash buffer is non-empty"; exit 5; }
if [[ "$MODE" == paper-original || "$MODE" == paper-rebuilt ]]; then
  grep -q "RealPaper.open=0" <<<"$LOGS"
  grep -Eq "RealPaper.page[0-9]+.recognized=true,count=[0-9]+" <<<"$LOGS"
  if [[ "${ALG_GEN:-0}" != "0" ]]; then
    grep -qx "paper.algGen.actual=${ALG_GEN}" <<<"$LOGS"
  fi
  if [[ "$MODE" == paper-rebuilt ]]; then
    grep -Eq '^rebuiltRecognitionContext=invocations:[1-9][0-9]*,uniquePages:[1-9][0-9]*,lastPage:[0-9]+,lastDomains:[0-9]+,generation:-?[0-9]+,cachedPages:[1-9][0-9]*,cachedDomains:[0-9]+,replacements:[0-9]+$' <<<"$LOGS"
    if [[ "${PAPER_PAGE:-0}" != "0" ]]; then
      grep -Eq '^rebuiltRealPaper\.page[0-9]+\.repeatRecognized=true,count=[0-9]+$' <<<"$LOGS"
      grep -Eq 'replacements:[1-9][0-9]*$' <<<"$LOGS"
    fi
  fi
  echo "PASS android-$MODE"
elif [[ "$MODE" == shape-original ]]; then
  grep -q 'originalShape.fontLoaded=true,directory=true,mapped=true,fallback=true' <<<"$LOGS"
  grep -q 'originalShape.extractedBefore=' <<<"$LOGS"
  grep -q 'originalShape.extractedBefore=.*recognized=true,editors=8' <<<"$LOGS"
  grep -q 'originalArabicInsert.*insert=true.*fontSet=true' <<<"$LOGS"
  grep -q 'originalArabicInsert.saved=true' <<<"$LOGS"
  echo "PASS android-shape-original"
elif [[ "$MODE" == shape-rebuilt ]]; then
  grep -q 'rebuiltShape.fontLoaded=true,directory=true,mapped=true,fallback=true' <<<"$LOGS"
  grep -q 'PASS rebuilt HarfBuzz RTL glyph clusters and CID encoding' <<<"$LOGS"
  grep -q 'PASS rebuilt multi-glyph mark cluster encoding oracle' <<<"$LOGS"
  grep -q 'PASS rebuilt FormFill document singleton' <<<"$LOGS"
  grep -q 'PASS rebuilt TextEditor FormFill singleton' <<<"$LOGS"
  grep -q 'PASS rebuilt Arabic logical insert' <<<"$LOGS"
  grep -q 'PASS rebuilt mark cluster rejects interior caret' <<<"$LOGS"
  grep -q 'PASS rebuilt mark cluster has one visual selection rectangle' <<<"$LOGS"
  grep -q 'PASS rebuilt FormFill editor RTL direction' <<<"$LOGS"
  grep -q 'PASS rebuilt Arabic undo logical text' <<<"$LOGS"
  grep -q 'PASS rebuilt Arabic redo logical text' <<<"$LOGS"
  grep -q 'PASS rebuilt Arabic saved logical text' <<<"$LOGS"
  grep -q 'PASS rebuilt saved shaped run is editable as logical text' <<<"$LOGS"
  grep -q 'PASS rebuilt saved shaped cluster replaces without orphan glyphs' <<<"$LOGS"
  grep -q 'PASS rebuilt TextEditor rejects stale page' <<<"$LOGS"
  grep -q 'PASS rebuilt TextEditor rejects closed document' <<<"$LOGS"
  grep -q 'rebuiltArabicInsert.saved=true' <<<"$LOGS"
  echo "PASS android-shape-rebuilt"
elif [[ "$MODE" == original ]]; then
  grep -q 'moduleInit=0' <<<"$LOGS"
  grep -q 'openPDF=0' <<<"$LOGS"
  grep -Eq 'pageCount=[1-9][0-9]*' <<<"$LOGS"
  grep -q 'getPage(index0)=0' <<<"$LOGS"
  grep -q 'originalEditRoundTrip=7,insert=4' <<<"$LOGS"
  grep -q 'originalEditableTextEditorCount=' <<<"$LOGS"
  grep -q 'originalSectionRecognize=true,editorCount=1' <<<"$LOGS"
  grep -q 'originalImageCount=1' <<<"$LOGS"
  grep -q 'PASS original image delete removes object' <<<"$LOGS"
  grep -q 'PASS original image delete persisted' <<<"$LOGS"
  grep -q 'originalPathHit=0' <<<"$LOGS"
  grep -q 'PASS original saver new' <<<"$LOGS"
  grep -q 'originalSaveProgress=100' <<<"$LOGS"
  grep -q 'PASS original saved reopen' <<<"$LOGS"
  grep -q 'PASS original optimize open' <<<"$LOGS"
  grep -q 'originalOptimizeSteps=100' <<<"$LOGS"
  grep -q 'PASS original optimized reopen' <<<"$LOGS"
  grep -q 'closePDF=0' <<<"$LOGS"
  echo "PASS android-contract-original-document-lifecycle"
  if grep -q 'addFreeText=false' <<<"$LOGS"; then
    echo "LIMITATION original annotation flow lacks a real annotation handle"
  fi
else
  grep -q 'PASS seed save' <<<"$LOGS"
  grep -q 'PASS rebuilt save' <<<"$LOGS"
  grep -q 'PASS registry openPDF' <<<"$LOGS"
  grep -q 'PASS registry getPage' <<<"$LOGS"
  grep -q 'rebuiltEditRoundTrip=7,insert=4' <<<"$LOGS"
  grep -q 'rebuiltEditSync=true' <<<"$LOGS"
  grep -q 'rebuiltEditableTextEditorCount=0' <<<"$LOGS"
  grep -q 'rebuiltSectionRecognize=true,editorCount=1' <<<"$LOGS"
  grep -q 'rebuiltImageCount=1' <<<"$LOGS"
  grep -q 'rebuiltPathHit=0' <<<"$LOGS"
  grep -q 'rebuiltPathInventoryCount=1' <<<"$LOGS"
  grep -q 'PASS rebuilt image delete removes object' <<<"$LOGS"
  grep -q 'PASS rebuilt image delete undo restore' <<<"$LOGS"
  grep -q 'PASS rebuilt image delete redo remove' <<<"$LOGS"
  grep -q 'PASS rebuilt image delete persisted' <<<"$LOGS"
  grep -q 'PASS rebuilt image resize bounds' <<<"$LOGS"
  grep -q 'PASS rebuilt image rotated bounds' <<<"$LOGS"
  grep -q 'PASS rebuilt image reverse preserves bounds' <<<"$LOGS"
  grep -q 'PASS rebuilt image rotate undo' <<<"$LOGS"
  grep -q 'PASS rebuilt image rotate redo' <<<"$LOGS"
  grep -q 'PASS rebuilt image geometry persisted' <<<"$LOGS"
  grep -q 'PASS rebuilt image geometry preserves path' <<<"$LOGS"
  grep -q 'registryWrongType=-1003' <<<"$LOGS"
  grep -q 'registryPasswordOpen=-3' <<<"$LOGS"
  grep -q 'PASS registry password reopen' <<<"$LOGS"
  grep -q 'PASS rebuilt saver new' <<<"$LOGS"
  grep -q 'rebuiltSaveProgress=100' <<<"$LOGS"
  grep -q 'PASS rebuilt saved reopen' <<<"$LOGS"
  grep -q 'PASS rebuilt optimize open' <<<"$LOGS"
  grep -q 'rebuiltOptimizeSteps=100' <<<"$LOGS"
  grep -q 'PASS rebuilt optimized reopen' <<<"$LOGS"
  grep -q 'PASS rebuilt opaque undo handle' <<<"$LOGS"
  grep -q 'PASS rebuilt undo apply' <<<"$LOGS"
  grep -q 'PASS rebuilt redo apply' <<<"$LOGS"
  grep -q 'PASS rebuilt cancel restored' <<<"$LOGS"
  grep -q 'PASS rebuilt style saved size' <<<"$LOGS"
  grep -q 'PASS rebuilt style saved color' <<<"$LOGS"
  grep -q 'PASS rebuilt style font undo' <<<"$LOGS"
  grep -q 'PASS rebuilt style font redo' <<<"$LOGS"
  grep -q 'PASS rebuilt style alignment undo' <<<"$LOGS"
  grep -q 'PASS rebuilt style alignment redo' <<<"$LOGS"
  grep -q 'PASS rebuilt style right alignment read' <<<"$LOGS"
  grep -q 'PASS rebuilt style right alignment undo' <<<"$LOGS"
  grep -q 'PASS rebuilt style saved font' <<<"$LOGS"
  grep -q 'PASS rebuilt style saved alignment' <<<"$LOGS"
  grep -q 'PASS rebuilt style leading undo' <<<"$LOGS"
  grep -q 'PASS rebuilt style leading redo' <<<"$LOGS"
  grep -q 'PASS rebuilt style saved leading' <<<"$LOGS"
  grep -q 'PASS rebuilt Form child replace' <<<"$LOGS"
  grep -q 'PASS rebuilt same-page Form isolation' <<<"$LOGS"
  grep -q 'PASS rebuilt cross-page Form isolation' <<<"$LOGS"
  grep -q 'PASS rebuilt Form saved first instance' <<<"$LOGS"
  grep -q 'PASS rebuilt Form saved same-page isolation' <<<"$LOGS"
  grep -q 'PASS rebuilt Form saved cross-page isolation' <<<"$LOGS"
  grep -q 'PASS rebuilt pointer hover' <<<"$LOGS"
  grep -q 'PASS rebuilt pointer drag selection' <<<"$LOGS"
  grep -q 'PASS rebuilt pointer word selection' <<<"$LOGS"
  grep -q 'PASS rebuilt pointer object selection' <<<"$LOGS"
  grep -q 'registryLiveHandles=0' <<<"$LOGS"
  echo "PASS android-contract-rebuilt"
fi
