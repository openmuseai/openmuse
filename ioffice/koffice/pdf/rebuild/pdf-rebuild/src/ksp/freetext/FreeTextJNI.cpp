// FreeTextJNI.cpp - JNI glue with the exact WPS export names so the rebuilt
// library is drop-in compatible with Java classes of the same FQCN.
#include "FreeText.h"
#include "../runtime/PdfRuntime.h"
#include "../runtime/PdfiumLifecycle.h"
#include <fpdfview.h>
#include <fpdf_edit.h>
#include <fpdf_save.h>
#include <jni.h>
#include <string>
#include <mutex>

using namespace ksp;


// PDFium standalone build requires explicit library init (font globals).
extern "C" JNIEXPORT jint JNICALL JNI_OnLoad(JavaVM*, void*) {
    ksp::runtime::ensurePdfiumInitialized();
    return JNI_VERSION_1_6;
}
extern "C" {

// ===========================================================================
// WPS surface (exact names; Java FQCN cn.wps.moffice.pdf.core.annot.*)
// ===========================================================================

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_annot_FreeTextAnnotation_native_1setFreeTextColor(
    JNIEnv*, jclass, jlong page, jint color) {
    auto state = ksp::runtime::borrowPage(static_cast<ksp::runtime::Handle>(page));
    if (!state) return;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    (void)ftSetColor(reinterpret_cast<Page>(state->pdf), (uint32_t)color);
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_annot_FreeTextAnnotation_native_1getFreeTextColor(
    JNIEnv*, jclass, jlong page) {
    auto state = ksp::runtime::borrowPage(static_cast<ksp::runtime::Handle>(page));
    if (!state) return 0;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    return (jint)ftGetColor(reinterpret_cast<Page>(state->pdf));
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_annot_FreeTextAnnotation_native_1setSelectFontSize(
    JNIEnv*, jclass, jlong page, jfloat size) {
    auto state = ksp::runtime::borrowPage(static_cast<ksp::runtime::Handle>(page));
    if (!state) return;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    (void)ftSetFontSize(reinterpret_cast<Page>(state->pdf), (float)size);
}

JNIEXPORT jfloat JNICALL
Java_cn_wps_moffice_pdf_core_annot_FreeTextAnnotation_native_1getSelectFontSize(
    JNIEnv*, jclass, jlong page) {
    auto state = ksp::runtime::borrowPage(static_cast<ksp::runtime::Handle>(page));
    if (!state) return 0.0f;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    return (jfloat)ftGetFontSize(reinterpret_cast<Page>(state->pdf));
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_annot_FreeTextAnnotation_native_1setFreeTextRect(
    JNIEnv* env, jclass, jlong page, jobject rect) {
    if (!rect) return;
    jclass rc = env->GetObjectClass(rect);
    jfieldID fL = env->GetFieldID(rc, "left", "F");
    jfieldID fT = env->GetFieldID(rc, "top", "F");
    jfieldID fR = env->GetFieldID(rc, "right", "F");
    jfieldID fB = env->GetFieldID(rc, "bottom", "F");
    Rect r;
    r.left = env->GetFloatField(rect, fL);
    r.top = env->GetFloatField(rect, fT);
    r.right = env->GetFloatField(rect, fR);
    r.bottom = env->GetFloatField(rect, fB);
    env->DeleteLocalRef(rc);
    auto state = ksp::runtime::borrowPage(static_cast<ksp::runtime::Handle>(page));
    if (!state) return;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    (void)ftSetRect(reinterpret_cast<Page>(state->pdf), r);
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_annot_FreeTextAnnotation_native_1getFreeTextRect(
    JNIEnv* env, jclass, jlong page, jlong, jobject rect) {
    if (!rect) return;
    Rect r{0, 0, 0, 0};
    auto state = ksp::runtime::borrowPage(static_cast<ksp::runtime::Handle>(page));
    if (!state) return;
    {
        std::lock_guard<std::mutex> lock(state->document->mutex);
        if (!ftGetRect(reinterpret_cast<Page>(state->pdf), r)) return;
    }
    jclass rc = env->GetObjectClass(rect);
    jfieldID fL = env->GetFieldID(rc, "left", "F");
    jfieldID fT = env->GetFieldID(rc, "top", "F");
    jfieldID fR = env->GetFieldID(rc, "right", "F");
    jfieldID fB = env->GetFieldID(rc, "bottom", "F");
    env->SetFloatField(rect, fL, r.left);
    env->SetFloatField(rect, fT, r.top);
    env->SetFloatField(rect, fR, r.right);
    env->SetFloatField(rect, fB, r.bottom);
    env->DeleteLocalRef(rc);
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_annot_PDFAnnotationEditor_native_1addFreeText(
    JNIEnv*, jclass, jlong doc, jlong page, jfloat left, jfloat top) {
    auto state = ksp::runtime::borrowPage(static_cast<ksp::runtime::Handle>(page), nullptr,
                                          static_cast<ksp::runtime::Handle>(doc));
    if (!state) return JNI_FALSE;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    return ftAddFreeText(reinterpret_cast<Doc>(state->document->pdf),
                         reinterpret_cast<Page>(state->pdf), (float)left, (float)top)
               ? JNI_TRUE
               : JNI_FALSE;
}

// ===========================================================================
// Test helpers (own class cn.wps.rebuild.TestPdf; not part of WPS surface)
// ===========================================================================

JNIEXPORT jlong JNICALL Java_cn_wps_rebuild_TestPdf_nativeNewDoc(JNIEnv*, jclass) {
    return (jlong)FPDF_CreateNewDocument();
}

JNIEXPORT void JNICALL Java_cn_wps_rebuild_TestPdf_nativeCloseDoc(JNIEnv*, jclass,
                                                                  jlong doc) {
    FPDF_CloseDocument((FPDF_DOCUMENT)doc);
}

JNIEXPORT jlong JNICALL Java_cn_wps_rebuild_TestPdf_nativeAddPage(JNIEnv*, jclass,
                                                                  jlong doc,
                                                                  jfloat w, jfloat h) {
    return (jlong)FPDFPage_New((FPDF_DOCUMENT)doc, 0, w, h);
}

JNIEXPORT jlong JNICALL Java_cn_wps_rebuild_TestPdf_nativeGetPage(JNIEnv*, jclass,
                                                                  jlong doc, jint i) {
    return (jlong)FPDF_LoadPage((FPDF_DOCUMENT)doc, i);
}

JNIEXPORT void JNICALL Java_cn_wps_rebuild_TestPdf_nativeClosePage(JNIEnv*, jclass,
                                                                   jlong page) {
    FPDF_ClosePage((FPDF_PAGE)page);
}

namespace {
struct FileWriter {
    FPDF_FILEWRITE base;
    FILE* fp;
};
static int writeBlock(FPDF_FILEWRITE* w, const void* data, unsigned long size) {
    FileWriter* fw = reinterpret_cast<FileWriter*>(w);
    return fwrite(data, 1, size, fw->fp) == size ? 1 : 0;
}
}  // namespace

JNIEXPORT jboolean JNICALL Java_cn_wps_rebuild_TestPdf_nativeSaveDoc(
    JNIEnv* env, jclass, jlong doc, jstring path) {
    const char* p = env->GetStringUTFChars(path, nullptr);
    if (!p) return JNI_FALSE;
    FileWriter fw{};
    fw.base.version = 1;
    fw.base.WriteBlock = writeBlock;
    fw.fp = fopen(p, "wb");
    env->ReleaseStringUTFChars(path, p);
    if (!fw.fp) return JNI_FALSE;
    int ret = FPDF_SaveAsCopy((FPDF_DOCUMENT)doc, &fw.base, 0);
    fclose(fw.fp);
    return ret ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL Java_cn_wps_rebuild_TestPdf_nativeAnnotCount(JNIEnv*, jclass,
                                                                    jlong page) {
    return ftAnnotCount(reinterpret_cast<Page>(page));
}

JNIEXPORT jint JNICALL Java_cn_wps_rebuild_TestPdf_nativeAnnotType(JNIEnv*, jclass,
                                                                   jlong page,
                                                                   jint index) {
    return ftAnnotType(reinterpret_cast<Page>(page), index);
}


JNIEXPORT jboolean JNICALL Java_cn_wps_rebuild_TestPdf_nativeSetContents(
    JNIEnv* env, jclass, jlong page, jstring s) {
    const char* p = env->GetStringUTFChars(s, nullptr);
    if (!p) return JNI_FALSE;
    std::string a(p);
    env->ReleaseStringUTFChars(s, p);
    std::basic_string<FPDF_WCHAR> u;
    for (unsigned char c : a) u.push_back((FPDF_WCHAR)c);
    return ftSetText(reinterpret_cast<Page>(page), u) ? JNI_TRUE : JNI_FALSE;
}


// ---- rebuild-namespaced edit-test surface (no collision with real pdf-sdk) ----
JNIEXPORT jboolean JNICALL Java_cn_wps_rebuild_FreeTextStub_native_1addFreeText(
    JNIEnv*, jclass, jlong doc, jlong page, jfloat left, jfloat top) {
    return ftAddFreeText(reinterpret_cast<Doc>(doc), reinterpret_cast<Page>(page),
                         (float)left, (float)top) ? JNI_TRUE : JNI_FALSE;
}
JNIEXPORT void JNICALL Java_cn_wps_rebuild_FreeTextStub_native_1setFreeTextColor(
    JNIEnv*, jclass, jlong page, jint color) {
    (void)ftSetColor(reinterpret_cast<Page>(page), (uint32_t)color);
}
JNIEXPORT jint JNICALL Java_cn_wps_rebuild_FreeTextStub_native_1getFreeTextColor(
    JNIEnv*, jclass, jlong page) {
    return (jint)ftGetColor(reinterpret_cast<Page>(page));
}
JNIEXPORT void JNICALL Java_cn_wps_rebuild_FreeTextStub_native_1setSelectFontSize(
    JNIEnv*, jclass, jlong page, jfloat size) {
    (void)ftSetFontSize(reinterpret_cast<Page>(page), (float)size);
}
JNIEXPORT jfloat JNICALL Java_cn_wps_rebuild_FreeTextStub_native_1getSelectFontSize(
    JNIEnv*, jclass, jlong page) {
    return (jfloat)ftGetFontSize(reinterpret_cast<Page>(page));
}
JNIEXPORT void JNICALL Java_cn_wps_rebuild_FreeTextStub_native_1setFreeTextRect(
    JNIEnv* env, jclass, jlong page, jobject rect) {
    if (!rect) return;
    jclass rc = env->GetObjectClass(rect);
    Rect r;
    r.left = env->GetFloatField(rect, env->GetFieldID(rc, "left", "F"));
    r.top = env->GetFloatField(rect, env->GetFieldID(rc, "top", "F"));
    r.right = env->GetFloatField(rect, env->GetFieldID(rc, "right", "F"));
    r.bottom = env->GetFloatField(rect, env->GetFieldID(rc, "bottom", "F"));
    (void)ftSetRect(reinterpret_cast<Page>(page), r);
}
JNIEXPORT void JNICALL Java_cn_wps_rebuild_FreeTextStub_native_1getFreeTextRect(
    JNIEnv* env, jclass, jlong page, jlong, jobject rect) {
    if (!rect) return;
    Rect r{0, 0, 0, 0};
    if (!ftGetRect(reinterpret_cast<Page>(page), r)) return;
    jclass rc = env->GetObjectClass(rect);
    env->SetFloatField(rect, env->GetFieldID(rc, "left", "F"), r.left);
    env->SetFloatField(rect, env->GetFieldID(rc, "top", "F"), r.top);
    env->SetFloatField(rect, env->GetFieldID(rc, "right", "F"), r.right);
    env->SetFloatField(rect, env->GetFieldID(rc, "bottom", "F"), r.bottom);
}

}  // extern "C"
