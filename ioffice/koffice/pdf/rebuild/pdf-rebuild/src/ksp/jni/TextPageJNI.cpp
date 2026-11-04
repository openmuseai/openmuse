// Ordinary-page text selection JNI. Java owns device/page matrix conversion
// and word expansion; this layer owns PDFium text, page-coordinate hit tests,
// range geometry, and UTF-16 extraction.
#include "JniSupport.h"
#include "../runtime/PdfRuntime.h"
#include "../runtime/UnicodeText.h"

#include <fpdf_text.h>
#include <algorithm>
#include <cmath>
#include <jni.h>
#include <mutex>
#include <vector>

using ksp::runtime::ErrorCode;
using ksp::runtime::Handle;

namespace {
void setError(ErrorCode e) { ksp::runtime::setLastError(e); }
jstring empty(JNIEnv* env) { return env->NewString(nullptr, 0); }
jstring utf16(JNIEnv* env, const unsigned short* data, int length) {
    return env->NewString(reinterpret_cast<const jchar*>(data), std::max(0, length));
}
bool range(FPDF_TEXTPAGE pdf, int start, int end, int* count) {
    const int total = FPDFText_CountChars(pdf);
    if (start < 0 || end < start) { setError(ErrorCode::InvalidArgument); return false; }
    if (start >= total || end >= total) { setError(ErrorCode::OutOfRange); return false; }
    *count = end - start + 1;
    return true;
}

// libkwopdf does not call the public FPDFText_GetCharIndexAtPos wrapper. Its
// PDFTextPage JNI thunk calls the text-page object's virtual GetIndexAtPos()
// directly (libkwopdf.so: 0x245d44, vtable slot +0x60). Reproduce the old
// PDFium implementation here because newer PDFium builds do not preserve its
// large-tolerance nearest-character behavior.
int legacyGetIndexAtPos(FPDF_TEXTPAGE pdf, double x, double y,
                        double xTolerance, double yTolerance) {
    const int count = FPDFText_CountChars(pdf);
    int nearPos = -1;
    double xDiff = 5000.0;
    double yDiff = 5000.0;
    for (int pos = 0; pos < count; ++pos) {
        double left = 0.0, right = 0.0, bottom = 0.0, top = 0.0;
        if (!FPDFText_GetCharBox(pdf, pos, &left, &right, &bottom, &top))
            continue;

        // Matches CFX_FloatRect::Contains() followed by Normalize() in the
        // original CPDF_TextPage::GetIndexAtPos implementation.
        if (x >= left && x <= right && y >= bottom && y <= top)
            return pos;
        if (xTolerance <= 0.0 && yTolerance <= 0.0)
            continue;
        if (left > right) std::swap(left, right);
        if (bottom > top) std::swap(bottom, top);
        if (x < left - xTolerance / 2.0 || x > right + xTolerance / 2.0 ||
            y < bottom - yTolerance / 2.0 || y > top + yTolerance / 2.0)
            continue;

        const double currentX = std::min(std::fabs(x - left), std::fabs(x - right));
        const double currentY = std::min(std::fabs(y - bottom), std::fabs(y - top));
        if (currentX + currentY < xDiff + yDiff) {
            xDiff = currentX;
            yDiff = currentY;
            nearPos = pos;
        }
    }
    return nearPos;
}
jstring text(JNIEnv* env, FPDF_TEXTPAGE pdf, int start, int end) {
    int count = 0;
    if (!range(pdf, start, end, &count)) return empty(env);
    std::vector<unsigned short> data(static_cast<size_t>(count) + 1, 0);
    const int written = FPDFText_GetText(pdf, start, count, data.data());
    if (written <= 0) { setError(ErrorCode::General); return empty(env); }
    setError(ErrorCode::Ok);
    return utf16(env, data.data(), std::min(count, written - 1));
}
bool setRect(JNIEnv* env, jobject rect, double left, double top,
             double right, double bottom) {
    if (!rect) { setError(ErrorCode::InvalidArgument); return false; }
    jclass cls = env->GetObjectClass(rect);
    jfieldID l = cls ? env->GetFieldID(cls, "left", "F") : nullptr;
    jfieldID t = cls ? env->GetFieldID(cls, "top", "F") : nullptr;
    jfieldID r = cls ? env->GetFieldID(cls, "right", "F") : nullptr;
    jfieldID b = cls ? env->GetFieldID(cls, "bottom", "F") : nullptr;
    if (cls) env->DeleteLocalRef(cls);
    if (!l || !t || !r || !b || env->ExceptionCheck()) {
        if (env->ExceptionCheck()) env->ExceptionClear();
        setError(ErrorCode::JniContract); return false;
    }
    env->SetFloatField(rect, l, static_cast<jfloat>(left));
    env->SetFloatField(rect, t, static_cast<jfloat>(top));
    env->SetFloatField(rect, r, static_cast<jfloat>(right));
    env->SetFloatField(rect, b, static_cast<jfloat>(bottom));
    return true;
}
bool overlaps(double l1, double t1, double r1, double b1,
              double l2, double t2, double r2, double b2) {
    return std::max(l1, l2) <= std::min(r1, r2) &&
           std::max(std::min(t1, b1), std::min(t2, b2)) <=
               std::min(std::max(t1, b1), std::max(t2, b2));
}

int directionForRect(FPDF_TEXTPAGE pdf, double left, double top,
                     double right, double bottom) {
    for (int index = 0, count = FPDFText_CountChars(pdf); index < count; ++index) {
        double l = 0, r = 0, b = 0, t = 0;
        if (!FPDFText_GetCharBox(pdf, index, &l, &r, &b, &t) ||
            !overlaps(left, top, right, bottom, l, t, r, b)) continue;
        const char32_t value = static_cast<char32_t>(FPDFText_GetUnicode(pdf, index));
        if (ksp::runtime::isStrongRtlCodePoint(value)) return 1;
        if (ksp::runtime::isStrongLtrCodePoint(value)) return 0;
    }
    return 0;
}
jobject newList(JNIEnv* env) {
    jclass cls = env->FindClass("java/util/ArrayList");
    jmethodID init = cls ? env->GetMethodID(cls, "<init>", "()V") : nullptr;
    jobject result = init ? env->NewObject(cls, init) : nullptr;
    if (cls) env->DeleteLocalRef(cls);
    return result;
}
bool add(JNIEnv* env, jobject list, jobject value) {
    jclass cls = list ? env->GetObjectClass(list) : nullptr;
    jmethodID method = cls ? env->GetMethodID(cls, "add", "(Ljava/lang/Object;)Z") : nullptr;
    if (cls) env->DeleteLocalRef(cls);
    if (!method || !value) return false;
    env->CallBooleanMethod(list, method, value);
    return !env->ExceptionCheck();
}
jobject newRect(JNIEnv* env, float l, float t, float r, float b) {
    jclass cls = env->FindClass("android/graphics/RectF");
    jmethodID init = cls ? env->GetMethodID(cls, "<init>", "(FFFF)V") : nullptr;
    jobject result = init ? env->NewObject(cls, init, l, t, r, b) : nullptr;
    if (cls) env->DeleteLocalRef(cls);
    return result;
}
}  // namespace

extern "C" {

JNIEXPORT jlong JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1new(
    JNIEnv*, jobject, jlong page) {
    Handle result = 0;
    return ksp::runtime::createTextPage(static_cast<Handle>(page), &result) == ErrorCode::Ok
               ? static_cast<jlong>(result) : 0;
}
JNIEXPORT jint JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getUnicode(
    JNIEnv*, jobject, jlong handle, jint index) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return -2;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    if (index < 0 || index >= FPDFText_CountChars(s->pdf)) {
        setError(ErrorCode::OutOfRange); return -2;
    }
    setError(ErrorCode::Ok); return static_cast<jint>(FPDFText_GetUnicode(s->pdf, index));
}
JNIEXPORT jstring JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getText(
    JNIEnv* env, jobject, jlong handle, jint start, jint end) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return empty(env);
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    return text(env, s->pdf, start, end);
}
JNIEXPORT jstring JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getPageText(
    JNIEnv* env, jobject, jlong handle) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return empty(env);
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    const int count = FPDFText_CountChars(s->pdf);
    if (!count) { setError(ErrorCode::Ok); return empty(env); }
    return text(env, s->pdf, 0, count - 1);
}
JNIEXPORT jint JNICALL Java_cn_wps_pdf_lib_select_TextPage_nCharCount(
    JNIEnv*, jclass, jlong handle) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return 0;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    setError(ErrorCode::Ok); return FPDFText_CountChars(s->pdf);
}
JNIEXPORT jfloat JNICALL Java_cn_wps_pdf_lib_select_TextPage_nFontSize(
    JNIEnv*, jclass, jlong handle, jint index) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return -1.0f;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    if (index < 0 || index >= FPDFText_CountChars(s->pdf)) {
        setError(ErrorCode::OutOfRange); return -1.0f;
    }
    double size = FPDFText_GetFontSize(s->pdf, index);
    FS_MATRIX matrix{};
    if (FPDFText_GetMatrix(s->pdf, index, &matrix)) {
        const double yScale = std::hypot(static_cast<double>(matrix.c),
                                         static_cast<double>(matrix.d));
        if (yScale > 0) size *= yScale;
    }
    setError(ErrorCode::Ok); return static_cast<jfloat>(size);
}
JNIEXPORT jint JNICALL Java_cn_wps_pdf_lib_select_TextPage_nColor(
    JNIEnv*, jclass, jlong handle, jint index) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return 0;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    if (index < 0 || index >= FPDFText_CountChars(s->pdf)) {
        setError(ErrorCode::OutOfRange); return 0;
    }
    unsigned int r = 0, g = 0, b = 0, a = 0;
    if (!FPDFText_GetFillColor(s->pdf, index, &r, &g, &b, &a)) {
        setError(ErrorCode::General); return 0;
    }
    setError(ErrorCode::Ok);
    (void)a;
    return static_cast<jint>(((r & 255) << 16) | ((g & 255) << 8) | (b & 255));
}
JNIEXPORT jstring JNICALL Java_cn_wps_pdf_lib_select_TextPage_nSysFontName(
    JNIEnv* env, jclass, jlong handle, jint index) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return nullptr;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    if (index < 0 || index >= FPDFText_CountChars(s->pdf)) {
        setError(ErrorCode::OutOfRange); return nullptr;
    }
    int flags = 0;
    unsigned long size = FPDFText_GetFontInfo(s->pdf, index, nullptr, 0, &flags);
    if (!size) { setError(ErrorCode::General); return nullptr; }
    std::vector<char> name(size, 0);
    if (!FPDFText_GetFontInfo(s->pdf, index, name.data(), size, &flags)) {
        setError(ErrorCode::General); return nullptr;
    }
    // The original SO exposes its system-font byte string byte-for-byte as
    // Java UTF-16 (e.g. GBK CB CE CC E5 -> U+00CB U+00CE U+00CC U+00E5),
    // rather than decoding it as UTF-8.
    std::vector<jchar> units;
    units.reserve(size ? size - 1 : 0);
    for (unsigned long i = 0; i + 1 < size && name[i] != 0; ++i)
        units.push_back(static_cast<unsigned char>(name[i]));
    setError(ErrorCode::Ok);
    return env->NewString(units.data(), static_cast<jsize>(units.size()));
}
JNIEXPORT void JNICALL Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1reloadTextPage(
    JNIEnv*, jobject, jlong page) { (void)ksp::runtime::reloadTextPage(page); }

JNIEXPORT jint JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getCPAtPoint(
    JNIEnv*, jobject, jlong handle, jfloat x, jfloat y, jfloat xt, jfloat yt) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return -1;
    if (xt < 0 || yt < 0) { setError(ErrorCode::InvalidArgument); return -1; }
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    const int result = legacyGetIndexAtPos(s->pdf, x, y, xt, yt);
    setError(ErrorCode::Ok);
    return result;
}
JNIEXPORT jint JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getCharRangeIndex(
    JNIEnv* env, jobject, jlong handle, jfloat left, jfloat top, jfloat right,
    jfloat bottom, jintArray position, jboolean) {
    if (!position || env->GetArrayLength(position) < 2 || left > right) {
        setError(ErrorCode::InvalidArgument); return 0;
    }
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return 0;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    int first = -1, last = -1;
    for (int i = 0, n = FPDFText_CountChars(s->pdf); i < n; ++i) {
        double l = 0, r = 0, b = 0, t = 0;
        if (FPDFText_GetCharBox(s->pdf, i, &l, &r, &b, &t) &&
            overlaps(left, top, right, bottom, l, t, r, b)) {
            if (first < 0) first = i; last = i;
        }
    }
    jint pair[2] = {first, first < 0 ? 0 : last - first + 1};
    env->SetIntArrayRegion(position, 0, 2, pair);
    setError(ErrorCode::Ok); return pair[1];
}
JNIEXPORT jint JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1countRects(
    JNIEnv*, jobject, jlong handle, jint start, jint end) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return 0;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    int count = 0; if (!range(s->pdf, start, end, &count)) return 0;
    int result = FPDFText_CountRects(s->pdf, start, count);
    setError(result < 0 ? ErrorCode::OutOfRange : ErrorCode::Ok);
    return std::max(0, result);
}
JNIEXPORT jint JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getRect(
    JNIEnv* env, jobject, jlong handle, jint index, jobject rect) {
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s || !rect) { if (!rect) setError(ErrorCode::InvalidArgument); return 0; }
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    double l = 0, t = 0, r = 0, b = 0;
    if (!FPDFText_GetRect(s->pdf, index, &l, &t, &r, &b)) {
        setError(ErrorCode::OutOfRange); return 0;
    }
    // Original ABI stores PDF lower-y in RectF.top and upper-y in RectF.bottom;
    // the Java display matrix flips this into Android top-down order.
    if (!setRect(env, rect, l, b, r, t)) return 0;
    const int direction = directionForRect(s->pdf, l, t, r, b);
    setError(ErrorCode::Ok); return direction;
}

JNIEXPORT jobject JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getText2(
    JNIEnv* env, jobject, jlong handle, jint start, jint end) {
    jobject result = newList(env);
    if (!result) { setError(ErrorCode::JniContract); return nullptr; }
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return result;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    int count = 0; if (!range(s->pdf, start, end, &count)) return result;
    std::vector<unsigned short> data(static_cast<size_t>(count) + 1, 0);
    int written = FPDFText_GetText(s->pdf, start, count, data.data());
    if (written <= 0) { setError(ErrorCode::General); return result; }
    int length = std::min(count, written - 1), begin = 0;
    for (int i = 0; i <= length; ++i) {
        if (i != length && data[i] != '\r') continue;
        jstring part = utf16(env, data.data() + begin, i - begin);
        add(env, result, part); env->DeleteLocalRef(part);
        if (i + 1 < length && data[i + 1] == '\n') ++i;
        begin = i + 1;
    }
    setError(ErrorCode::Ok); return result;
}
JNIEXPORT void JNICALL Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getPageTextAndRects(
    JNIEnv* env, jobject, jlong handle, jobject texts, jobject rects) {
    if (!texts || !rects) { setError(ErrorCode::InvalidArgument); return; }
    ErrorCode error; auto s = ksp::runtime::borrowTextPage(handle, &error);
    if (!s) return;
    std::lock_guard<std::mutex> lock(s->page->document->mutex);
    const int chars = FPDFText_CountChars(s->pdf);
    int lineStart = 0;
    for (int boundary = 0; boundary <= chars; ++boundary) {
        const unsigned int unicode = boundary < chars ? FPDFText_GetUnicode(s->pdf, boundary) : '\r';
        if (unicode != '\r' && unicode != '\n' && boundary != chars) continue;
        const int lineEnd = boundary - 1;
        double left = 0, right = 0, bottom = 0, top = 0;
        bool hasBox = false;
        for (int i = lineStart; i <= lineEnd; ++i) {
            double l = 0, r = 0, b = 0, t = 0;
            if (!FPDFText_GetCharBox(s->pdf, i, &l, &r, &b, &t)) continue;
            if (!hasBox) { left = l; right = r; bottom = b; top = t; hasBox = true; }
            else { left = std::min(left, l); right = std::max(right, r);
                   bottom = std::min(bottom, b); top = std::max(top, t); }
        }
        if (hasBox && lineEnd >= lineStart) {
            jstring string = text(env, s->pdf, lineStart, lineEnd);
            jobject rect = newRect(env, left, bottom, right, top);
            bool ok = add(env, texts, string) && add(env, rects, rect);
            if (string) env->DeleteLocalRef(string);
            if (rect) env->DeleteLocalRef(rect);
            if (!ok) { setError(ErrorCode::JniContract); return; }
        }
        if (unicode == '\r' && boundary + 1 < chars &&
            FPDFText_GetUnicode(s->pdf, boundary + 1) == '\n') ++boundary;
        lineStart = boundary + 1;
    }
    setError(ErrorCode::Ok);
}

// Rebuild namespace facade for the standalone integration app.
JNIEXPORT jlong JNICALL Java_cn_wps_rebuild_TextSel_native_1openDocument(
    JNIEnv* env, jclass, jstring path) {
    ksp::jni::UtfChars value(env, path);
    if (!value) { setError(ErrorCode::InvalidArgument); return 0; }
    Handle result = 0;
    return ksp::runtime::openDocument(value.str(), &result) == ErrorCode::Ok
               ? static_cast<jlong>(result) : 0;
}
JNIEXPORT jlong JNICALL Java_cn_wps_rebuild_TextSel_native_1loadPage(
    JNIEnv*, jclass, jlong document, jint index) {
    Handle result = 0;
    return ksp::runtime::loadPage(document, index, &result) == ErrorCode::Ok
               ? static_cast<jlong>(result) : 0;
}
JNIEXPORT void JNICALL Java_cn_wps_rebuild_TextSel_native_1closePage(
    JNIEnv*, jclass, jlong page) { if (page) (void)ksp::runtime::closePage(page); }
JNIEXPORT void JNICALL Java_cn_wps_rebuild_TextSel_native_1closeDocument(
    JNIEnv*, jclass, jlong document) {
    if (document) (void)ksp::runtime::closeDocument(document);
}
JNIEXPORT jlong JNICALL Java_cn_wps_rebuild_TextSel_native_1new(JNIEnv* e, jclass, jlong p) {
    return Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1new(e, nullptr, p);
}
JNIEXPORT jint JNICALL Java_cn_wps_rebuild_TextSel_native_1getCPAtPoint(
    JNIEnv* e, jclass, jlong h, jfloat x, jfloat y, jfloat xt, jfloat yt) {
    return Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getCPAtPoint(e,nullptr,h,x,y,xt,yt);
}
JNIEXPORT jint JNICALL Java_cn_wps_rebuild_TextSel_native_1countRects(
    JNIEnv* e, jclass, jlong h, jint s, jint n) {
    return Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1countRects(e,nullptr,h,s,n);
}
JNIEXPORT jint JNICALL Java_cn_wps_rebuild_TextSel_native_1getRect(
    JNIEnv* e, jclass, jlong h, jint i, jobject r) {
    return Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getRect(e,nullptr,h,i,r);
}
JNIEXPORT jstring JNICALL Java_cn_wps_rebuild_TextSel_native_1getText(
    JNIEnv* e, jclass, jlong h, jint s, jint n) {
    return Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getText(e,nullptr,h,s,n);
}
JNIEXPORT jint JNICALL Java_cn_wps_rebuild_TextSel_native_1getUnicode(
    JNIEnv* e, jclass, jlong h, jint i) {
    return Java_cn_wps_moffice_pdf_core_select_PDFTextPage_native_1getUnicode(e,nullptr,h,i);
}
JNIEXPORT jint JNICALL Java_cn_wps_rebuild_TextSel_native_1getCharCount(
    JNIEnv* e, jclass, jlong h) {
    return Java_cn_wps_pdf_lib_select_TextPage_nCharCount(e,nullptr,h);
}
}  // extern "C"
