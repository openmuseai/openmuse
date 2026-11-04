#include "JniSupport.h"
#include "../runtime/ObjectRuntime.h"

#include <jni.h>
#include <fpdf_edit.h>

#include <iomanip>
#include <sstream>

using ksp::runtime::EditableObjectRecord;
using ksp::runtime::ErrorCode;
using ksp::runtime::Handle;

namespace {

jlongArray pageObjectArray(JNIEnv* env, Handle page, int type, bool nullIfEmpty = false) {
    std::vector<Handle> handles;
    if (ksp::runtime::getPageObjectHandles(page, type, &handles) != ErrorCode::Ok)
        return nullptr;
    if (nullIfEmpty && handles.empty()) return nullptr;
    jlongArray result = env->NewLongArray(static_cast<jsize>(handles.size()));
    if (!result) return nullptr;
    std::vector<jlong> values(handles.begin(), handles.end());
    if (!values.empty())
        env->SetLongArrayRegion(result, 0, static_cast<jsize>(values.size()), values.data());
    return result;
}

bool setObjectRect(JNIEnv* env, jobject rect, Handle page, Handle object, int type) {
    if (!rect) {
        ksp::runtime::setLastError(ErrorCode::InvalidArgument);
        return false;
    }
    EditableObjectRecord value;
    if (ksp::runtime::getPageObjectRect(page, object, type, &value) != ErrorCode::Ok)
        return false;
    return ksp::jni::setRectFBounds(env, rect, value.pageLeft, value.pageBottom,
                                    value.pageRight, value.pageTop) == ErrorCode::Ok;
}

bool getRectF(JNIEnv* env, jobject rect, float* left, float* top,
              float* right, float* bottom) {
    if (!rect || !left || !top || !right || !bottom) return false;
    jclass cls = env->GetObjectClass(rect);
    if (!cls) return false;
    jfieldID leftId = env->GetFieldID(cls, "left", "F");
    jfieldID topId = env->GetFieldID(cls, "top", "F");
    jfieldID rightId = env->GetFieldID(cls, "right", "F");
    jfieldID bottomId = env->GetFieldID(cls, "bottom", "F");
    if (!leftId || !topId || !rightId || !bottomId || env->ExceptionCheck()) {
        env->ExceptionClear();
        return false;
    }
    *left = env->GetFloatField(rect, leftId);
    *top = env->GetFloatField(rect, topId);
    *right = env->GetFloatField(rect, rightId);
    *bottom = env->GetFloatField(rect, bottomId);
    return !env->ExceptionCheck();
}

}  // namespace

extern "C" {

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1onAfterLoadPage(
    JNIEnv*, jobject, jlong page) {
    // Original arm64 implementation is a single `ret`; retain validation for
    // deterministic errors without triggering edit-domain recognition.
    (void)ksp::runtime::handles().isValid(static_cast<Handle>(page),
                                          ksp::runtime::HandleType::Page);
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1SectionRecognize(
    JNIEnv*, jobject, jlong page) {
    return ksp::runtime::recognizeTextDomains(static_cast<Handle>(page)) == ErrorCode::Ok
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getTextEditorCount(
    JNIEnv*, jobject, jlong page) {
    int result = 0;
    return ksp::runtime::getTextEditorCount(static_cast<Handle>(page), &result) == ErrorCode::Ok
               ? result : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getTextEditorRect(
    JNIEnv* env, jobject, jlong page, jint index, jobject rect,
    jfloatArray matrix, jboolean reverse) {
    if (!rect || !matrix || env->GetArrayLength(matrix) < 9) {
        ksp::runtime::setLastError(ErrorCode::InvalidArgument);
        return JNI_FALSE;
    }
    EditableObjectRecord value;
    ErrorCode code = ksp::runtime::getTextEditorGeometry(
        static_cast<Handle>(page), index, reverse == JNI_TRUE, &value);
    if (code != ErrorCode::Ok) return JNI_FALSE;
    code = ksp::jni::setRectFBounds(
        env, rect, value.left, value.top, value.right, value.bottom);
    if (code != ErrorCode::Ok) return JNI_FALSE;
    env->SetFloatArrayRegion(matrix, 0, 9, value.matrix);
    if (env->ExceptionCheck()) {
        env->ExceptionClear();
        ksp::runtime::setLastError(ErrorCode::JniContract);
        return JNI_FALSE;
    }
    ksp::runtime::setLastError(ErrorCode::Ok);
    return JNI_TRUE;
}

JNIEXPORT jlongArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeTextObjectIds(
    JNIEnv* env, jclass, jlong page) {
    std::vector<uint64_t> ids;
    if (ksp::runtime::getTextEditorIds(static_cast<Handle>(page), &ids) != ErrorCode::Ok)
        return nullptr;
    jlongArray result = env->NewLongArray(static_cast<jsize>(ids.size()));
    if (!result) return nullptr;
    std::vector<jlong> values(ids.begin(), ids.end());
    if (!values.empty()) env->SetLongArrayRegion(result, 0, values.size(), values.data());
    return result;
}

JNIEXPORT jobjectArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeTextObjectDiagnostics(
    JNIEnv* env, jclass, jlong page) {
    std::vector<ksp::runtime::TextObjectDiagnostic> values;
    if (ksp::runtime::getTextObjectDiagnostics(static_cast<Handle>(page), &values) !=
        ErrorCode::Ok) return nullptr;
    jclass stringClass = env->FindClass("java/lang/String");
    if (!stringClass) return nullptr;
    jobjectArray result = env->NewObjectArray(static_cast<jsize>(values.size()),
                                              stringClass, nullptr);
    if (!result) return nullptr;
    for (std::size_t index = 0; index < values.size(); ++index) {
        const auto& value = values[index];
        std::ostringstream prefix;
        prefix << std::fixed << std::setprecision(3)
               << "id=" << value.localId
               << ",box=" << value.pageLeft << "," << value.pageBottom << ","
               << value.pageRight << "," << value.pageTop
               << ",group=" << value.groupLeft << "," << value.groupBottom << ","
               << value.groupRight << "," << value.groupTop
               << ",fontSize=" << value.fontSize
               << ",fontName=" << value.fontName
               << ",fill=" << value.fillColor
               << ",renderMode=" << value.renderMode
               << ",formDepth=" << value.formDepth
               << ",matrix=" << value.matrix[0] << "," << value.matrix[1] << ","
               << value.matrix[2] << "," << value.matrix[3] << ","
               << value.matrix[4] << "," << value.matrix[5] << ",text=";
        std::u16string line;
        const std::string ascii = prefix.str();
        line.assign(ascii.begin(), ascii.end());
        line.append(value.text);
        jstring item = env->NewString(reinterpret_cast<const jchar*>(line.data()),
                                      static_cast<jsize>(line.size()));
        if (!item) return nullptr;
        env->SetObjectArrayElement(result, static_cast<jsize>(index), item);
        env->DeleteLocalRef(item);
    }
    return result;
}

JNIEXPORT jlongArray JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getImages(
    JNIEnv* env, jobject, jlong page) {
    // Original Android oracle returns null, rather than a zero-length array,
    // after the last image is removed.
    return pageObjectArray(env, static_cast<Handle>(page), FPDF_PAGEOBJ_IMAGE, true);
}

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getImageObjectAtPoint(
    JNIEnv*, jobject, jlong page, jfloat x, jfloat y) {
    Handle result = 0;
    (void)ksp::runtime::getPageObjectAtPoint(
        static_cast<Handle>(page), FPDF_PAGEOBJ_IMAGE, x, y, &result);
    return static_cast<jlong>(result);
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getImageRect(
    JNIEnv* env, jobject, jlong page, jlong image, jobject rect) {
    (void)setObjectRect(env, rect, static_cast<Handle>(page),
                        static_cast<Handle>(image), FPDF_PAGEOBJ_IMAGE);
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1removeImageFromPage(
    JNIEnv*, jobject, jlong page, jlong image) {
    return ksp::runtime::removePageObject(static_cast<Handle>(page),
                                          static_cast<Handle>(image),
                                          FPDF_PAGEOBJ_IMAGE) == ErrorCode::Ok
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1resizeImageRect(
    JNIEnv* env, jobject, jlong page, jlong image, jobject rect) {
    float left = 0, bottom = 0, right = 0, top = 0;
    if (!getRectF(env, rect, &left, &bottom, &right, &top)) {
        ksp::runtime::setLastError(ErrorCode::JniContract);
        return 0;
    }
    return ksp::runtime::resizeImageObject(static_cast<Handle>(page),
                                            static_cast<Handle>(image),
                                            left, bottom, right, top) == ErrorCode::Ok ? 1 : 0;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1rotateImageClockwise(
    JNIEnv*, jobject, jlong page, jlong image) {
    return ksp::runtime::rotateImageObjectClockwise(static_cast<Handle>(page),
                                                     static_cast<Handle>(image)) == ErrorCode::Ok
               ? 1 : 0;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1reverseImageHorizontal(
    JNIEnv*, jobject, jlong page, jlong image) {
    return ksp::runtime::reverseImageObjectHorizontal(static_cast<Handle>(page),
                                                       static_cast<Handle>(image)) == ErrorCode::Ok
               ? 1 : 0;
}

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getPathObjectAtPoint(
    JNIEnv*, jobject, jlong page, jfloat, jfloat) {
    // The original SO returns 0 for ordinary page-content paths. Its helper
    // applies an edit-domain classifier that belongs to CE2; a bounding-box
    // hit here would expose objects the original UI cannot select.
    if (!ksp::runtime::handles().isValid(static_cast<Handle>(page),
                                         ksp::runtime::HandleType::Page))
        return 0;
    ksp::runtime::setLastError(ErrorCode::Ok);
    return 0;
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getPathRect(
    JNIEnv* env, jobject, jlong page, jlong path, jobject rect) {
    (void)setObjectRect(env, rect, static_cast<Handle>(page),
                        static_cast<Handle>(path), FPDF_PAGEOBJ_PATH);
}

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1FindImageObjectWithPoint(
    JNIEnv*, jobject, jlong page, jfloat x, jfloat y) {
    Handle result = 0;
    (void)ksp::runtime::getPageObjectAtPoint(
        static_cast<Handle>(page), FPDF_PAGEOBJ_IMAGE, x, y, &result);
    return static_cast<jlong>(result);
}

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1FindImageObjectWithImageIndex(
    JNIEnv*, jobject, jlong page, jint imageIndex) {
    if (imageIndex <= 0) {
        ksp::runtime::setLastError(ErrorCode::OutOfRange);
        return 0;
    }
    std::vector<Handle> values;
    if (ksp::runtime::getPageObjectHandles(
            static_cast<Handle>(page), FPDF_PAGEOBJ_IMAGE, &values) != ErrorCode::Ok)
        return 0;
    if (imageIndex > static_cast<jint>(values.size())) {
        ksp::runtime::setLastError(ErrorCode::OutOfRange);
        return 0;
    }
    return static_cast<jlong>(values[static_cast<std::size_t>(imageIndex - 1)]);
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetImageRectFromPage(
    JNIEnv* env, jobject, jlong page, jlong image, jobject rect) {
    return setObjectRect(env, rect, static_cast<Handle>(page),
                         static_cast<Handle>(image), FPDF_PAGEOBJ_IMAGE)
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetImageIndex(
    JNIEnv*, jobject, jlong page, jlong image) {
    int result = 0;
    return ksp::runtime::getPageObjectIndex(
               static_cast<Handle>(page), static_cast<Handle>(image),
               FPDF_PAGEOBJ_IMAGE, &result) == ErrorCode::Ok
               ? result : 0;
}

JNIEXPORT jlongArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativePageObjectHandles(
    JNIEnv* env, jclass, jlong page, jint type) {
    return pageObjectArray(env, static_cast<Handle>(page), type);
}

}  // extern "C"
