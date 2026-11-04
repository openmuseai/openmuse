// RenderJNI.cpp - drawing + rendering exports for the rebuilt library.
// Adds: content generation (text/rect), PDF page rasterization via PDFium.
#include "FreeText.h"
#include "../runtime/PdfRuntime.h"
#include <fpdfview.h>
#include <fpdf_edit.h>
#include <fpdf_annot.h>
#include <fpdf_doc.h>
#include <fpdf_text.h>
#include <fpdf_edit.h>
#include <fpdf_annot.h>
#include <jni.h>
#include <cstring>
#include <string>

static std::basic_string<FPDF_WCHAR> toW16(const char* s) {
    std::basic_string<FPDF_WCHAR> w;
    for (const unsigned char* p = (const unsigned char*)s; *p; ++p) w.push_back((FPDF_WCHAR)*p);
    return w;
}

extern "C" {

// ---- content generation ----
JNIEXPORT jboolean JNICALL Java_cn_wps_rebuild_TestPdf_nativeDrawText(
    JNIEnv* env, jclass, jlong doc, jlong page, jfloat x, jfloat y, jstring text,
    jfloat size, jint r, jint g, jint b, jstring fontPath) {
    const char* t = env->GetStringUTFChars(text, nullptr);
    if (!t) return JNI_FALSE;
    FPDF_FONT font = FPDFText_LoadStandardFont((FPDF_DOCUMENT)doc, "Helvetica");
    if (!font) {
        const char* fp = fontPath ? env->GetStringUTFChars(fontPath, nullptr) : nullptr;
        if (fp) {
            FILE* fh = fopen(fp, "rb");
            if (fh) {
                fseek(fh, 0, SEEK_END); long sz = ftell(fh); fseek(fh, 0, SEEK_SET);
                unsigned char* buf = (unsigned char*)malloc(sz);
                if (buf && fread(buf, 1, sz, fh) == (size_t)sz)
                    font = FPDFText_LoadFont((FPDF_DOCUMENT)doc, buf, sz,
                                             FPDF_FONT_TRUETYPE, 0);
                free(buf);
                fclose(fh);
            }
            env->ReleaseStringUTFChars(fontPath, fp);
        }
    }
    std::basic_string<FPDF_WCHAR> w = toW16(t);
    env->ReleaseStringUTFChars(text, t);
    if (!font) return JNI_FALSE;
    // font size applied ONLY via the matrix (double-scaling produced 36^2 giant glyphs)
    FPDF_PAGEOBJECT obj = FPDFPageObj_CreateTextObj((FPDF_DOCUMENT)doc, font, 0);
    if (!obj) return JNI_FALSE;
    FPDFText_SetText(obj, w.c_str());
    FPDFPageObj_SetFillColor(obj, (unsigned)r, (unsigned)g, (unsigned)b, 255);
    // PDF origin is bottom-left; y passed in top-left convention -> flip
    float yf = FPDF_GetPageHeightF((FPDF_PAGE)page) - y;
    FS_MATRIX m{size, 0, 0, -size, x, yf};
    FPDFPageObj_SetMatrix(obj, &m);
    FPDFPage_InsertObject((FPDF_PAGE)page, obj);
    return FPDFPage_GenerateContent((FPDF_PAGE)page) ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL Java_cn_wps_rebuild_TestPdf_nativeDrawRect(
    JNIEnv*, jclass, jlong page, jfloat x, jfloat y, jfloat w, jfloat h,
    jint r, jint g, jint b) {
    float yf = FPDF_GetPageHeightF((FPDF_PAGE)page) - y;  // top-left -> bottom-left
    FPDF_PAGEOBJECT obj = FPDFPageObj_CreateNewRect(x, yf - h, w, h);
    if (!obj) return JNI_FALSE;
    FPDFPageObj_SetFillColor(obj, (unsigned)r, (unsigned)g, (unsigned)b, 255);
    FPDFPath_SetDrawMode(obj, FPDF_FILLMODE_WINDING, 0);  // fill
    FPDFPage_InsertObject((FPDF_PAGE)page, obj);
    return FPDFPage_GenerateContent((FPDF_PAGE)page) ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jlong JNICALL Java_cn_wps_rebuild_TestPdf_nativeOpenPdf(JNIEnv* env, jclass,
                                                                  jstring path) {
    const char* p = env->GetStringUTFChars(path, nullptr);
    FPDF_DOCUMENT d = p ? FPDF_LoadDocument(p, nullptr) : nullptr;
    if (p) env->ReleaseStringUTFChars(path, p);
    return (jlong)d;
}

JNIEXPORT jfloat JNICALL Java_cn_wps_rebuild_TestPdf_nativeGetPageWidth(JNIEnv*, jclass,
                                                                        jlong page) {
    return FPDF_GetPageWidthF((FPDF_PAGE)page);
}
JNIEXPORT jfloat JNICALL Java_cn_wps_rebuild_TestPdf_nativeGetPageHeight(JNIEnv*, jclass,
                                                                         jlong page) {
    return FPDF_GetPageHeightF((FPDF_PAGE)page);
}


JNIEXPORT jint JNICALL Java_cn_wps_rebuild_TestPdf_nativeGetPageCount(JNIEnv*, jclass,
                                                                       jlong doc) {
    return FPDF_GetPageCount((FPDF_DOCUMENT)doc);
}


JNIEXPORT jstring JNICALL Java_cn_wps_rebuild_TestPdf_nativeGetContents(JNIEnv* env, jclass,
                                                                          jlong page) {
    FPDF_ANNOTATION a = FPDFPage_GetAnnot((FPDF_PAGE)page, 0);
    if (!a) return env->NewStringUTF("");
    unsigned long len = FPDFAnnot_GetStringValue(a, "Contents", nullptr, 0);
    std::string out;
    if (len > 0) {
        std::basic_string<FPDF_WCHAR> ww(len, 0);
        FPDFAnnot_GetStringValue(a, "Contents", &ww[0], len);
        for (FPDF_WCHAR u : ww) {
            unsigned short us = (unsigned short)u;
            if (us >= 32 && us < 127) out += (char)us;
        }
    }
    FPDFPage_CloseAnnot(a);
    return env->NewStringUTF(out.c_str());
}


JNIEXPORT jstring JNICALL Java_cn_wps_rebuild_TestPdf_nativeGetContentsAt(JNIEnv* env, jclass,
                                                                           jlong page, jint idx) {
    if (idx < 0) return env->NewStringUTF("");
    FPDF_ANNOTATION a = FPDFPage_GetAnnot((FPDF_PAGE)page, idx);
    if (!a) return env->NewStringUTF("");
    unsigned long len = FPDFAnnot_GetStringValue(a, "Contents", nullptr, 0);
    std::string out;
    if (len > 0) {
        std::basic_string<FPDF_WCHAR> ww(len, 0);
        FPDFAnnot_GetStringValue(a, "Contents", &ww[0], len);
        for (FPDF_WCHAR u : ww) {
            unsigned short us = (unsigned short)u;
            if (us >= 32 && us < 127) out += (char)us;
        }
    }
    FPDFPage_CloseAnnot(a);
    return env->NewStringUTF(out.c_str());
}

// ---- rasterize page into int[] (ARGB, top-down rows) ----
JNIEXPORT jboolean JNICALL Java_cn_wps_rebuild_TestPdf_nativeRenderPage(
    JNIEnv* env, jclass, jlong page, jint w, jint h, jintArray out) {
    FPDF_BITMAP bm = FPDFBitmap_Create(w, h, 1);  // alpha
    if (!bm) return JNI_FALSE;
    FPDFBitmap_FillRect(bm, 0, 0, w, h, 0xFFFFFFFF);
    // PDF origin bottom-left -> flip rows so the buffer is top-down like Bitmap
    FPDF_RenderPageBitmap(bm, (FPDF_PAGE)page,
                          0, 0, w, h, 0, FPDF_ANNOT | FPDF_RENDER_NO_SMOOTHTEXT);
    jint* dst = env->GetIntArrayElements(out, nullptr);
    const unsigned char* src = (const unsigned char*)FPDFBitmap_GetBuffer(bm);
    int stride = FPDFBitmap_GetStride(bm);
    unsigned int* u32 = (unsigned int*)src;
    for (int row = 0; row < h; ++row) {
        unsigned int* srcRow = u32 + (size_t)row * (stride / 4);  // buffer is already top-down
        for (int col = 0; col < w; ++col) {
            unsigned int px = srcRow[col];  // BGRA
            dst[(size_t)row * w + col] =
                (jint)(0xFF000000u | ((px & 0xFF) << 16) | (px & 0xFF00) | ((px >> 16) & 0xFF));
        }
    }
    env->ReleaseIntArrayElements(out, dst, 0);
    FPDFBitmap_Destroy(bm);
    return JNI_TRUE;
}

// UI-only bridge: render a registry-backed page while all editing operations
// continue to use the original PDFPage JNI contract.
JNIEXPORT jboolean JNICALL Java_cn_wps_rebuild_TestPdf_nativeRenderManagedPage(
    JNIEnv* env, jclass, jlong page, jint w, jint h, jintArray out) {
    ksp::runtime::ErrorCode error;
    auto managed = ksp::runtime::borrowPage(static_cast<ksp::runtime::Handle>(page), &error);
    if (!managed) return JNI_FALSE;
    FPDF_BITMAP bm = FPDFBitmap_Create(w, h, 1);
    if (!bm) return JNI_FALSE;
    FPDFBitmap_FillRect(bm, 0, 0, w, h, 0xFFFFFFFF);
    FPDF_RenderPageBitmap(bm, managed->pdf, 0, 0, w, h, 0,
                          FPDF_ANNOT | FPDF_RENDER_NO_SMOOTHTEXT);
    jint* dst = env->GetIntArrayElements(out, nullptr);
    if (!dst) { FPDFBitmap_Destroy(bm); return JNI_FALSE; }
    const unsigned char* src = static_cast<const unsigned char*>(FPDFBitmap_GetBuffer(bm));
    const int stride = FPDFBitmap_GetStride(bm);
    for (int row = 0; row < h; ++row) {
        const unsigned int* srcRow = reinterpret_cast<const unsigned int*>(src + row * stride);
        for (int col = 0; col < w; ++col) {
            const unsigned int px = srcRow[col];
            dst[static_cast<size_t>(row) * w + col] = static_cast<jint>(
                0xFF000000u | ((px & 0xFF) << 16) | (px & 0xFF00) | ((px >> 16) & 0xFF));
        }
    }
    env->ReleaseIntArrayElements(out, dst, 0);
    FPDFBitmap_Destroy(bm);
    return JNI_TRUE;
}

}  // extern "C"
