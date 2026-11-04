#include "JniSupport.h"
#include "../runtime/PdfRuntime.h"

#include <jni.h>
#include <fpdf_transformpage.h>
#include <cmath>

using ksp::runtime::ErrorCode;
using ksp::runtime::Handle;
using ksp::runtime::HandleType;

namespace {
jint code(ErrorCode value) { return static_cast<jint>(ksp::runtime::asInt(value)); }

jint fail(ErrorCode value) {
    ksp::runtime::setLastError(value);
    return code(value);
}

jint pageState(jlong rawHandle) {
    ErrorCode error;
    auto page = ksp::runtime::borrowPage(static_cast<Handle>(rawHandle), &error);
    return page ? 2 : code(error);  // PDFProgressiveState.FINISHED in the SDK contract.
}

bool rectValues(JNIEnv* env, jobject rect, float* left, float* top,
                float* right, float* bottom) {
    if (!rect || !left || !top || !right || !bottom) return false;
    jclass type = env->GetObjectClass(rect);
    if (!type) return false;
    jfieldID leftId = env->GetFieldID(type, "left", "F");
    jfieldID topId = env->GetFieldID(type, "top", "F");
    jfieldID rightId = env->GetFieldID(type, "right", "F");
    jfieldID bottomId = env->GetFieldID(type, "bottom", "F");
    if (!leftId || !topId || !rightId || !bottomId || env->ExceptionCheck()) {
        env->ExceptionClear();
        env->DeleteLocalRef(type);
        return false;
    }
    *left = env->GetFloatField(rect, leftId);
    *top = env->GetFloatField(rect, topId);
    *right = env->GetFloatField(rect, rightId);
    *bottom = env->GetFloatField(rect, bottomId);
    env->DeleteLocalRef(type);
    return !env->ExceptionCheck() && std::isfinite(*left) && std::isfinite(*top) &&
           std::isfinite(*right) && std::isfinite(*bottom);
}
}  // namespace

extern "C" {

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1SetSectionRecognizeAlgGen(
    JNIEnv*, jobject, jint generation) {
    return ksp::runtime::setSectionRecognizeAlgorithmGeneration(generation)
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1GetSectionRecognizeAlgGen(
    JNIEnv*, jobject) {
    return static_cast<jint>(ksp::runtime::sectionRecognizeAlgorithmGeneration());
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1openPDF(
    JNIEnv* env, jclass, jstring path, jobject nativeHandle) {
    if (!path || !nativeHandle) return fail(ErrorCode::InvalidArgument);
    ksp::jni::UtfChars utf(env, path);
    if (!utf) return fail(ErrorCode::JniContract);
    Handle document = 0;
    ErrorCode result = ksp::runtime::openDocument(utf.str(), &document);
    if (!document) return code(result);
    const ErrorCode openResult = result;
    result = ksp::jni::setNativeHandle(env, nativeHandle, document);
    if (result != ErrorCode::Ok) {
        (void)ksp::runtime::closeDocument(document);
        return code(result);
    }
    ksp::runtime::setLastError(openResult);
    return code(openResult);
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1closePDF(
    JNIEnv*, jobject, jlong document) {
    return code(ksp::runtime::closeDocument(static_cast<Handle>(document)));
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1reopenInPassword(
    JNIEnv* env, jobject, jlong document, jstring password) {
    if (!password) return fail(ErrorCode::InvalidArgument);
    ksp::jni::UtfChars utf(env, password);
    if (!utf) return fail(ErrorCode::JniContract);
    return code(ksp::runtime::reopenWithPassword(
        static_cast<Handle>(document), utf.str()));
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1reopen(
    JNIEnv* env, jobject, jlong document, jstring path) {
    ksp::jni::UtfChars value(env, path);
    if (!value) {
        ksp::runtime::setLastError(ErrorCode::InvalidArgument);
        return code(ErrorCode::InvalidArgument);
    }
    return code(ksp::runtime::reopenDocument(static_cast<Handle>(document), value.str()));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1isValid(
    JNIEnv*, jobject, jlong document) {
    return ksp::runtime::handles().isValid(static_cast<Handle>(document), HandleType::Document)
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1getPageCount(
    JNIEnv*, jobject, jlong document) {
    int count = 0;
    const ErrorCode result = ksp::runtime::getPageCount(static_cast<Handle>(document), &count);
    return result == ErrorCode::Ok ? static_cast<jint>(count) : code(result);
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1getPage(
    JNIEnv* env, jobject, jlong document, jint index, jobject nativeHandle) {
    if (!nativeHandle) return fail(ErrorCode::InvalidArgument);
    Handle page = 0;
    ErrorCode result = ksp::runtime::loadPage(static_cast<Handle>(document), index, &page);
    if (result != ErrorCode::Ok) return code(result);
    result = ksp::jni::setNativeHandle(env, nativeHandle, page);
    if (result != ErrorCode::Ok) {
        (void)ksp::runtime::closePage(page);
        return code(result);
    }
    return 0;
}

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1getFormfill(
    JNIEnv*, jobject, jlong document) {
    Handle form = 0;
    return ksp::runtime::getFormFill(static_cast<Handle>(document), &form) ==
                   ErrorCode::Ok
               ? static_cast<jlong>(form)
               : 0;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1getPageSizeFromPageIndex(
    JNIEnv* env, jobject, jlong document, jint index, jobject rect) {
    float width = 0.0f;
    float height = 0.0f;
    ErrorCode result = ksp::runtime::getPageSizeAt(
        static_cast<Handle>(document), index, &width, &height);
    if (result != ErrorCode::Ok) return code(result);
    return code(ksp::jni::setRectF(env, rect, width, height));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1isValid(
    JNIEnv*, jobject, jlong page) {
    return ksp::runtime::handles().isValid(static_cast<Handle>(page), HandleType::Page)
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getPageIndex(
    JNIEnv*, jobject, jlong page) {
    ErrorCode error;
    auto state = ksp::runtime::borrowPage(static_cast<Handle>(page), &error);
    return state ? static_cast<jint>(state->index) : code(error);
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getPageSize(
    JNIEnv* env, jobject, jlong page, jobject rect) {
    float width = 0.0f;
    float height = 0.0f;
    ErrorCode result = ksp::runtime::getPageSize(static_cast<Handle>(page), &width, &height);
    if (result == ErrorCode::Ok) (void)ksp::jni::setRectF(env, rect, width, height);
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getDisplayMatirx(
    JNIEnv* env, jobject, jlong page, jfloatArray output, jobject displayRect,
    jint rotation) {
    if (!output || env->GetArrayLength(output) < 9 || !displayRect) {
        return fail(ErrorCode::InvalidArgument);
    }
    ErrorCode error;
    auto state = ksp::runtime::borrowPage(static_cast<Handle>(page), &error);
    if (!state) return code(error);
    float dl = 0, dt = 0, dr = 0, db = 0;
    if (!rectValues(env, displayRect, &dl, &dt, &dr, &db) || dr <= dl || db <= dt)
        return fail(ErrorCode::InvalidArgument);
    float left = 0, bottom = 0, right = 0, top = 0;
    {
        std::lock_guard<std::mutex> lock(state->document->mutex);
        if (!FPDFPage_GetCropBox(state->pdf, &left, &bottom, &right, &top)) {
            left = 0;
            bottom = 0;
            right = FPDF_GetPageWidthF(state->pdf);
            top = FPDF_GetPageHeightF(state->pdf);
        }
    }
    if (right <= left || top <= bottom) return fail(ErrorCode::General);
    int degrees = rotation;
    if (degrees >= -3 && degrees <= 3) degrees *= 90;
    degrees = ((degrees % 360) + 360) % 360;
    const float dw = dr - dl, dh = db - dt;
    float values[9] = {0, 0, 0, 0, 0, 0, 0, 0, 1};
    if (degrees == 0) {
        const float sx = dw / (right - left), sy = dh / (top - bottom);
        values[0] = sx; values[2] = dl - sx * left;
        values[4] = -sy; values[5] = dt + sy * top;
    } else if (degrees == 90) {
        const float sx = dw / (top - bottom), sy = dh / (right - left);
        values[1] = sx; values[2] = dl - sx * bottom;
        values[3] = sy; values[5] = dt - sy * left;
    } else if (degrees == 180) {
        const float sx = dw / (right - left), sy = dh / (top - bottom);
        values[0] = -sx; values[2] = dl + sx * right;
        values[4] = sy; values[5] = dt - sy * bottom;
    } else if (degrees == 270) {
        const float sx = dw / (top - bottom), sy = dh / (right - left);
        values[1] = -sx; values[2] = dl + sx * top;
        values[3] = -sy; values[5] = dt + sy * right;
    } else {
        return fail(ErrorCode::InvalidArgument);
    }
    env->SetFloatArrayRegion(output, 0, 9, values);
    if (env->ExceptionCheck()) {
        env->ExceptionClear();
        return fail(ErrorCode::JniContract);
    }
    ksp::runtime::setLastError(ErrorCode::Ok);
    return 0;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1unloadPage2(
    JNIEnv*, jobject, jlong page) {
    return code(ksp::runtime::closePage(static_cast<Handle>(page)));
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1startLoading(
    JNIEnv*, jobject, jlong page) { return pageState(page); }

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1continueLoading(
    JNIEnv*, jobject, jlong page, jlong) { return pageState(page); }

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getLoadState(
    JNIEnv*, jobject, jlong page) { return pageState(page); }

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1stopLoading(
    JNIEnv*, jobject, jlong page) { return pageState(page); }

// Test-only diagnostics. They expose error classification, never raw pointers.
JNIEXPORT jint JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeLastErrorCode(JNIEnv*, jclass) {
    return code(ksp::runtime::lastError());
}

JNIEXPORT jstring JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeLastErrorName(JNIEnv* env, jclass) {
    return env->NewStringUTF(ksp::runtime::errorName(ksp::runtime::lastError()));
}

JNIEXPORT jint JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeValidatePageOwner(
    JNIEnv*, jclass, jlong page, jlong document) {
    ErrorCode error;
    auto value = ksp::runtime::borrowPage(static_cast<Handle>(page), &error,
                                         static_cast<Handle>(document));
    return value ? 0 : code(error);
}

JNIEXPORT jint JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeLiveHandleCount(JNIEnv*, jclass) {
    return static_cast<jint>(ksp::runtime::handles().liveCount());
}

JNIEXPORT jlongArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeRecognitionContextDiagnostics(
    JNIEnv* env, jclass, jlong document) {
    ksp::runtime::RecognitionContextDiagnostics value;
    if (ksp::runtime::getRecognitionContextDiagnostics(
            static_cast<Handle>(document), &value) != ErrorCode::Ok) {
        return nullptr;
    }
    const jlong fields[] = {
        static_cast<jlong>(value.invocationCount),
        static_cast<jlong>(value.uniquePageCount),
        static_cast<jlong>(value.lastPageIndex),
        static_cast<jlong>(value.lastDomainCount),
        static_cast<jlong>(value.lastAlgorithmGeneration),
        static_cast<jlong>(value.cachedPageCount),
        static_cast<jlong>(value.cachedDomainCount),
        static_cast<jlong>(value.replacementCount),
    };
    jlongArray result = env->NewLongArray(8);
    if (!result) return nullptr;
    env->SetLongArrayRegion(result, 0, 8, fields);
    return env->ExceptionCheck() ? nullptr : result;
}

JNIEXPORT jlongArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeRecognitionProcessorDiagnostics(
    JNIEnv* env, jclass, jlong document) {
    ksp::runtime::RecognitionProcessorDiagnostics diagnostics;
    const auto code = ksp::runtime::getRecognitionProcessorDiagnostics(
        static_cast<Handle>(document), &diagnostics);
    if (code != ErrorCode::Ok) return nullptr;
    const jlong values[] = {
        static_cast<jlong>(diagnostics.commitCount),
        static_cast<jlong>(diagnostics.recognizedPageCount),
        static_cast<jlong>(diagnostics.formulaPageCount),
        static_cast<jlong>(diagnostics.formulaDomainCount),
        static_cast<jlong>(diagnostics.formulaSpanCount),
        static_cast<jlong>(diagnostics.compoundFormulaCount),
        static_cast<jlong>(diagnostics.maxFormulaSpanCount),
        static_cast<jlong>(diagnostics.lastPageIndex),
        static_cast<jlong>(diagnostics.algorithmGeneration),
    };
    jlongArray result = env->NewLongArray(9);
    if (!result) return nullptr;
    env->SetLongArrayRegion(result, 0, 9, values);
    return result;
}

JNIEXPORT jintArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeRecognitionPageDiagnostics(
    JNIEnv* env, jclass, jlong document, jint pageIndex) {
    ksp::runtime::RecognitionPageDiagnostics value;
    if (ksp::runtime::getRecognitionPageDiagnostics(
            static_cast<Handle>(document), static_cast<int>(pageIndex), &value) !=
        ErrorCode::Ok) {
        return nullptr;
    }
    const jint fields[] = {
        value.domainCount, value.nodeCount, value.sectionCount,
        value.paragraphCount, value.lineCount, value.spanCount,
        value.formulaCount, value.tableCount, value.tableRowCount,
        value.tableCellCount,
    };
    jintArray result = env->NewIntArray(10);
    if (!result) return nullptr;
    env->SetIntArrayRegion(result, 0, 10, fields);
    return env->ExceptionCheck() ? nullptr : result;
}

JNIEXPORT jintArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeRecognitionDomainDiagnostics(
    JNIEnv* env, jclass, jlong document, jint pageIndex) {
    std::vector<ksp::runtime::RecognitionDomainDiagnostics> domains;
    const auto code = ksp::runtime::getRecognitionDomainDiagnostics(
        static_cast<Handle>(document), static_cast<int>(pageIndex), &domains);
    if (code != ErrorCode::Ok) return nullptr;
    std::vector<jint> values;
    values.reserve(domains.size() * 4);
    for (const auto& domain : domains) {
        values.push_back(static_cast<jint>(domain.kind));
        values.push_back(static_cast<jint>(domain.lineCount));
        values.push_back(static_cast<jint>(domain.objectCount));
        values.push_back(static_cast<jint>(domain.spanCount));
    }
    jintArray result = env->NewIntArray(static_cast<jsize>(values.size()));
    if (!result) return nullptr;
    if (!values.empty()) {
        env->SetIntArrayRegion(result, 0, static_cast<jsize>(values.size()),
                               values.data());
    }
    return result;
}

}  // extern "C"
