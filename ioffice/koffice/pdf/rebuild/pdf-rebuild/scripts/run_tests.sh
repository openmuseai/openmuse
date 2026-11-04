#!/usr/bin/env bash
# run_tests.sh - build the dylib, compile and run the Java JNI harness (macOS)
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
PREBUILT="$ROOT/3rdparty/prebuilt"
BUILD="$ROOT/build"
JH="${JAVA_HOME:-$(/usr/libexec/java_home -v 17)}"

"$ROOT/scripts/build_freetext.sh" >/dev/null
bash "$ROOT/scripts/run_page_operations_tests.sh"

WORK="$(mktemp -d "${TMPDIR:-/tmp}/ksp-host-test.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

clang++ -std=c++17 -O1 -g -Wall -Wextra -fsanitize=address,undefined \
  "$ROOT/harness/runtime_tests.cpp" \
  "$ROOT/src/ksp/runtime/Error.cpp" \
  "$ROOT/src/ksp/runtime/HandleRegistry.cpp" \
  "$ROOT/src/ksp/runtime/UnicodeText.cpp" \
  -o "$WORK/runtime_tests"
"$WORK/runtime_tests"

while IFS= read -r rel; do
  [[ -z "$rel" || "$rel" == \#* ]] && continue
  realpath "$ROOT/harness/java/$rel"
done < "$ROOT/harness/host-sources.txt" > "$WORK/java_srcs.txt"
"$JH/bin/javac" -d "$WORK/classes" @"$WORK/java_srcs.txt"

cd "$WORK/classes"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" FreeTextTest
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" DocumentLifecycleTest \
  "$ROOT/harness/assets/testfile_pdf_clip.pdf" \
  "$ROOT/harness/assets/password-user123.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" TextPageLifecycleTest \
  "$ROOT/harness/assets/testfile_pdf_clip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" SaveLifecycleTest \
  "$ROOT/harness/assets/testfile_pdf_clip.pdf" "$WORK/save-roundtrip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" OptimizeLifecycleTest \
  "$ROOT/harness/assets/testfile_pdf_clip.pdf" \
  "$WORK/optimize-roundtrip.pdf" "$WORK/optimize-wrapped.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" EditLifecycleTest \
  "$ROOT/harness/assets/testfile_pdf_clip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" ObjectInventoryTest \
  "$ROOT/harness/assets/editable-helvetica.pdf" \
  "$ROOT/harness/assets/testfile_pdf_clip.pdf" \
  "$ROOT/harness/assets/editable-objects.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" ImageEditTest \
  "$ROOT/harness/assets/editable-objects.pdf" "$WORK/image-edit-roundtrip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" DomainCorpusTest \
  "$ROOT/harness/assets/edit-domain-corpus/multicolumn.pdf" \
  "$ROOT/harness/assets/edit-domain-corpus/rotated.pdf" \
  "$ROOT/harness/assets/edit-domain-corpus/cropbox.pdf" \
  "$ROOT/harness/assets/edit-domain-corpus/form-xobject.pdf"
if [[ -n "${KSP_REAL_EDIT_PDF:-}" ]]; then
  DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
    "$JH/bin/java" -Djava.library.path="$BUILD" RealDocumentDomainTest \
    "$KSP_REAL_EDIT_PDF"
  DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
    "$JH/bin/java" -Djava.library.path="$BUILD" DomainLogicalTextTest \
    "$KSP_REAL_EDIT_PDF"
  DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
    "$JH/bin/java" -Djava.library.path="$BUILD" RealDomainEditTest \
    "$KSP_REAL_EDIT_PDF" "$WORK/real-domain-edit.pdf"
  DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
    "$JH/bin/java" -Djava.library.path="$BUILD" LongDocumentEditScanTest \
    "$KSP_REAL_EDIT_PDF"
fi
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" GlyphCaretTest \
  "$ROOT/harness/assets/glyph-caret-corpus.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" EditPointerInteractionTest \
  "$ROOT/harness/assets/glyph-caret-corpus.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" TextStyleTest \
  "$ROOT/harness/assets/editable-helvetica.pdf" "$WORK/text-style-roundtrip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" EditTransactionTest \
  "$ROOT/harness/assets/editable-helvetica.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" CrossPageEditTransactionTest \
  "$ROOT/harness/assets/editable-two-page.pdf" "$WORK/cross-page-edit.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" EditDamageTest \
  "$ROOT/harness/assets/editable-helvetica.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" TextReflowTest \
  "$ROOT/harness/assets/editable-helvetica.pdf" "$WORK/text-reflow-roundtrip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" MixedTextReflowTest \
  "$ROOT/harness/assets/mixed-reflow-corpus.pdf" "$WORK/mixed-text-reflow-roundtrip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" AdvancedTextReflowTest \
  "$ROOT/harness/assets/edit-domain-corpus/rotated.pdf" \
  "$ROOT/harness/assets/edit-domain-corpus/form-xobject.pdf" \
  "$WORK/advanced-form-reflow-roundtrip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" ComplexTextReflowTest \
  "$ROOT/harness/assets/complex-text-corpus.pdf" \
  "$WORK/complex-text-reflow-roundtrip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" ExternalFontFallbackTest \
  "$ROOT/harness/assets/mixed-reflow-corpus.pdf" \
  "$ROOT/harness/assets/fonts/NotoSansSC-WT6-subset.ttf" \
  "$WORK/external-font-fallback-roundtrip.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" UnicodeEditBoundaryTest \
  "$ROOT/harness/assets/mixed-reflow-corpus.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" RtlEditGeometryTest \
  "$ROOT/harness/assets/rtl-edit-corpus.pdf"
DYLD_LIBRARY_PATH="$PREBUILT/lib:$BUILD" \
  "$JH/bin/java" -Djava.library.path="$BUILD" FormEditTest \
  "$ROOT/harness/assets/form-edit-corpus.pdf" "$WORK/form-edit-roundtrip.pdf"
"$JH/bin/java" SelectionInteractionTest
"$JH/bin/java" AtomicFileCommitTest "$WORK/atomic-file-commit"
echo "== test exit: $? =="
