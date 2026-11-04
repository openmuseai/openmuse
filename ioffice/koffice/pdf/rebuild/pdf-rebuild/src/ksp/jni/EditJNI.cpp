#include "../runtime/EditRuntime.h"
#include "../runtime/FontRuntime.h"
#include "../runtime/ObjectRuntime.h"
#include "../runtime/UnicodeText.h"
#include "JniSupport.h"

#include <jni.h>

#include <algorithm>
#include <cmath>

using ksp::runtime::ErrorCode;
using ksp::runtime::Handle;

namespace {
jboolean booleanResult(ErrorCode code) { return code == ErrorCode::Ok ? JNI_TRUE : JNI_FALSE; }

Handle editorPage(Handle handle) {
    if (ksp::runtime::handles().isValid(handle, ksp::runtime::HandleType::Page))
        return handle;
    Handle page = 0;
    return ksp::runtime::resolveTextEditorPage(handle, &page) == ErrorCode::Ok
               ? page : 0;
}

jstring toJavaString(JNIEnv* env, const std::u16string& value) {
    return env->NewString(reinterpret_cast<const jchar*>(value.data()),
                          static_cast<jsize>(value.size()));
}

jobjectArray selectionRectArray(JNIEnv* env,
                               const std::vector<ksp::runtime::TextSelectionRect>& rects) {
    jclass cls = env->FindClass("android/graphics/RectF");
    if (!cls) return nullptr;
    jmethodID ctor = env->GetMethodID(cls, "<init>", "(FFFF)V");
    jobjectArray result = ctor ? env->NewObjectArray(static_cast<jsize>(rects.size()), cls,
                                                      nullptr) : nullptr;
    if (result) {
        for (std::size_t index = 0; index < rects.size(); ++index) {
            const auto& rect = rects[index];
            jobject value = env->NewObject(cls, ctor, rect.left, rect.bottom,
                                           rect.right, rect.top);
            if (!value) break;
            env->SetObjectArrayElement(result, static_cast<jsize>(index), value);
            env->DeleteLocalRef(value);
        }
    }
    env->DeleteLocalRef(cls);
    return result;
}
}

extern "C" {

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1setInsertStatus(
    JNIEnv*, jobject, jlong document, jint status) {
    (void)ksp::runtime::setInsertStatus(static_cast<Handle>(document), status);
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1setEditStatus(
    JNIEnv*, jobject, jlong document, jint status) {
    (void)ksp::runtime::setEditStatus(static_cast<Handle>(document), status);
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_nGetInsertStatus(
    JNIEnv*, jobject, jlong document) {
    int status = -1;
    return ksp::runtime::getInsertStatus(static_cast<Handle>(document), &status) == ErrorCode::Ok
               ? status : -1;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_nGetEditStatus(
    JNIEnv*, jobject, jlong document) {
    int status = -1;
    return ksp::runtime::getEditStatus(static_cast<Handle>(document), &status) == ErrorCode::Ok
               ? status : -1;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1syncEditPageObj(
    JNIEnv*, jobject, jlong document, jlong page, jboolean forSave) {
    return booleanResult(ksp::runtime::syncEditPage(
        static_cast<Handle>(document), static_cast<Handle>(page), forSave == JNI_TRUE));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1ClearEditPageData(
    JNIEnv*, jobject, jlong document, jlong page) {
    return booleanResult(ksp::runtime::clearEditPageData(
        static_cast<Handle>(document), static_cast<Handle>(page)));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1ClearEditFocus(
    JNIEnv*, jobject, jlong document, jlong page) {
    return booleanResult(ksp::runtime::clearEditFocus(
        static_cast<Handle>(document), static_cast<Handle>(page)));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1ClearHoverHandle(
    JNIEnv*, jobject, jlong document, jlong page) {
    return booleanResult(ksp::runtime::clearHoverHandle(
        static_cast<Handle>(document), static_cast<Handle>(page)));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1separateSharedDataInEditPage(
    JNIEnv*, jobject, jlong document, jint pageIndex) {
    return booleanResult(ksp::runtime::separateSharedDataInEditPage(
        static_cast<Handle>(document), pageIndex));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1EditBackup(
    JNIEnv*, jobject, jlong document, jlong page, jint editStatus) {
    return booleanResult(ksp::runtime::backupEditPage(
        static_cast<Handle>(document), static_cast<Handle>(page), editStatus));
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1regenerateContent(
    JNIEnv*, jobject, jlong page) {
    (void)ksp::runtime::regeneratePageContent(static_cast<Handle>(page));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1IsModified(
    JNIEnv*, jobject, jlong page) {
    bool modified = false;
    return ksp::runtime::isPageModified(static_cast<Handle>(page), &modified) == ErrorCode::Ok && modified
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1CheckFuncModified(
    JNIEnv*, jobject, jlong page, jint editStatus) {
    bool modified = false;
    return ksp::runtime::isPageFunctionModified(
               static_cast<Handle>(page), editStatus, &modified) == ErrorCode::Ok && modified
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnLButtonDown(
    JNIEnv*, jobject, jlong page, jfloat x, jfloat y, jint flags) {
    bool focused = false;
    return ksp::runtime::focusTextAtPoint(static_cast<Handle>(page), x, y, flags, &focused)
                   == ErrorCode::Ok && focused ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnMouseMove(
    JNIEnv*, jobject, jlong page, jfloat x, jfloat y, jint flags) {
    bool handled = false;
    return ksp::runtime::moveTextSelectionAtPoint(static_cast<Handle>(page), x, y, flags, &handled)
                   == ErrorCode::Ok && handled ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetHoverType(
    JNIEnv*, jobject, jlong page, jfloat x, jfloat y) {
    int type = 0;
    return ksp::runtime::getTextHoverType(static_cast<Handle>(page), x, y, &type) == ErrorCode::Ok
               ? type : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnLButtonDoubleClick(
    JNIEnv*, jobject, jlong page, jfloat x, jfloat y) {
    bool selected = false;
    return ksp::runtime::selectTextWordAtPoint(static_cast<Handle>(page), x, y, &selected)
                   == ErrorCode::Ok && selected ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnLButtonTripleClick(
    JNIEnv*, jobject, jlong page, jfloat x, jfloat y) {
    bool selected = false;
    return ksp::runtime::selectTextObjectAtPoint(static_cast<Handle>(page), x, y, &selected)
                   == ErrorCode::Ok && selected ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnLButtonUp(
    JNIEnv*, jobject, jlong page, jfloat, jfloat) {
    bool handled = false;
    return ksp::runtime::endTextSelectionDrag(static_cast<Handle>(page), &handled)
                   == ErrorCode::Ok && handled ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetFocusType(
    JNIEnv*, jobject, jlong page) {
    int value = 0;
    return ksp::runtime::getTextFocusState(static_cast<Handle>(page), &value, nullptr,
                                           nullptr, nullptr) == ErrorCode::Ok ? value : 0;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetFocusStatus(
    JNIEnv*, jobject, jlong page) {
    int value = 0;
    return ksp::runtime::getTextFocusState(static_cast<Handle>(page), nullptr, &value,
                                           nullptr, nullptr) == ErrorCode::Ok ? value : 0;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetFocusObjectCount(
    JNIEnv*, jobject, jlong page) {
    int value = 0;
    return ksp::runtime::getTextFocusState(static_cast<Handle>(page), nullptr, nullptr,
                                           &value, nullptr) == ErrorCode::Ok ? value : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetFocusObjectRect(
    JNIEnv* env, jobject, jlong page, jobject rect) {
    ksp::runtime::EditableObjectRecord value;
    if (ksp::runtime::getTextFocusState(static_cast<Handle>(page), nullptr, nullptr,
                                        nullptr, &value) != ErrorCode::Ok) return JNI_FALSE;
    return ksp::jni::setRectFBounds(env, rect, value.pageLeft, value.pageTop,
                                    value.pageRight, value.pageBottom) == ErrorCode::Ok
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1SetFontSize(
    JNIEnv*, jobject, jlong page, jfloat size) {
    return booleanResult(ksp::runtime::setFocusedTextFontSize(static_cast<Handle>(page), size));
}

JNIEXPORT jfloat JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetFontSize(
    JNIEnv*, jobject, jlong page) {
    float size = 0;
    return ksp::runtime::getFocusedTextFontSize(static_cast<Handle>(page), &size) == ErrorCode::Ok
               ? size : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1SetTextColor(
    JNIEnv*, jobject, jlong page, jint color) {
    return booleanResult(ksp::runtime::setFocusedTextColor(
        static_cast<Handle>(page), static_cast<uint32_t>(color)));
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetTextColor(
    JNIEnv*, jobject, jlong page) {
    uint32_t color = 0;
    return ksp::runtime::getFocusedTextColor(static_cast<Handle>(page), &color) == ErrorCode::Ok
               ? static_cast<jint>(color) : 0;
}

JNIEXPORT jstring JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetFontName(
    JNIEnv* env, jobject, jlong page) {
    std::string name;
    return ksp::runtime::getFocusedTextFontName(static_cast<Handle>(page), &name) == ErrorCode::Ok
               ? env->NewStringUTF(name.c_str()) : nullptr;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1SetFontName(
    JNIEnv* env, jobject, jlong page, jstring requested) {
    ksp::jni::UtfChars value(env, requested);
    return value ? booleanResult(ksp::runtime::setFocusedTextFontName(
                       static_cast<Handle>(page), value.str()))
                 : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1setTextBold(
    JNIEnv*, jobject, jlong page, jboolean bold) {
    return ksp::runtime::setFocusedTextBold(static_cast<Handle>(page), bold == JNI_TRUE) ==
                   ErrorCode::Ok
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1isTextBold(
    JNIEnv*, jobject, jlong page) {
    bool result = false;
    return ksp::runtime::getFocusedTextBold(static_cast<Handle>(page), &result) ==
                       ErrorCode::Ok && result
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1setAlignment(
    JNIEnv*, jobject, jlong page, jint alignment) {
    return booleanResult(ksp::runtime::setFocusedTextAlignment(
        static_cast<Handle>(page), static_cast<int>(alignment)));
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getAlignment(
    JNIEnv*, jobject, jlong page) {
    int alignment = 0;
    return ksp::runtime::getFocusedTextAlignment(static_cast<Handle>(page), &alignment)
                   == ErrorCode::Ok
               ? static_cast<jint>(alignment) : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1setLeading(
    JNIEnv*, jobject, jlong page, jfloat leading) {
    return booleanResult(ksp::runtime::setFocusedTextLeading(
        static_cast<Handle>(page), static_cast<float>(leading)));
}

JNIEXPORT jfloat JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1getLeading(
    JNIEnv*, jobject, jlong page) {
    float leading = 1.0f;
    return ksp::runtime::getFocusedTextLeading(static_cast<Handle>(page), &leading)
                   == ErrorCode::Ok
               ? static_cast<jfloat>(leading) : 1.0f;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetTextLength(
    JNIEnv*, jobject, jlong page) {
    std::u16string value;
    return ksp::runtime::getFocusedText(static_cast<Handle>(page), &value) == ErrorCode::Ok
               ? static_cast<jint>(value.size()) : 0;
}

JNIEXPORT jstring JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetText(
    JNIEnv* env, jobject, jlong page) {
    std::u16string value;
    return ksp::runtime::getFocusedText(static_cast<Handle>(page), &value) == ErrorCode::Ok
               ? toJavaString(env, value) : nullptr;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetEditAllTextLength(
    JNIEnv*, jobject, jlong page) {
    std::u16string value;
    return ksp::runtime::getFocusedDomainText(static_cast<Handle>(page), &value) ==
                   ErrorCode::Ok
               ? static_cast<jint>(value.size()) : 0;
}

JNIEXPORT jstring JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetEditAllText(
    JNIEnv* env, jobject, jlong page) {
    std::u16string value;
    return ksp::runtime::getFocusedDomainText(static_cast<Handle>(page), &value) ==
                   ErrorCode::Ok
               ? toJavaString(env, value) : nullptr;
}

JNIEXPORT jintArray JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetTextSelectRange(
    JNIEnv* env, jobject, jlong page) {
    int start = 0, end = 0;
    if (ksp::runtime::getTextSelection(static_cast<Handle>(page), &start, &end)
        != ErrorCode::Ok) return nullptr;
    const jint values[] = {start, end};
    jintArray result = env->NewIntArray(2);
    if (result) env->SetIntArrayRegion(result, 0, 2, values);
    return result;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1SetTextSelectRange(
    JNIEnv*, jobject, jlong page, jint start, jint end) {
    return booleanResult(ksp::runtime::setTextSelection(static_cast<Handle>(page), start, end));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFPageEditor_native_1hasSelection(
    JNIEnv*, jobject, jlong page) {
    int start = 0, end = 0;
    return ksp::runtime::getTextSelection(static_cast<Handle>(page), &start, &end) ==
                   ErrorCode::Ok && start != end ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFPageEditor_native_1startSelection(
    JNIEnv*, jobject, jlong page) {
    int start = -1, end = -1;
    return ksp::runtime::getTextSelection(static_cast<Handle>(page), &start, &end) ==
                   ErrorCode::Ok ? start : -1;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFPageEditor_native_1endSelection(
    JNIEnv*, jobject, jlong page) {
    int start = -1, end = -1;
    return ksp::runtime::getTextSelection(static_cast<Handle>(page), &start, &end) ==
                   ErrorCode::Ok ? end : -1;
}

JNIEXPORT jobjectArray JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFPageEditor_native_1selectionRects(
    JNIEnv* env, jobject, jlong page) {
    std::vector<ksp::runtime::TextSelectionRect> rects;
    if (ksp::runtime::getTextSelectionRects(static_cast<Handle>(page), &rects) !=
            ErrorCode::Ok || rects.empty()) return nullptr;
    return selectionRectArray(env, rects);
}

JNIEXPORT jobjectArray JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFPageEditor_native_1selectionRectsEx(
    JNIEnv* env, jobject, jlong page) {
    std::vector<ksp::runtime::TextSelectionRect> rects;
    if (ksp::runtime::getTextSelectionRects(static_cast<Handle>(page), &rects) !=
            ErrorCode::Ok || rects.empty()) return nullptr;
    jobjectArray rectangles = selectionRectArray(env, rects);
    jintArray directions = env->NewIntArray(static_cast<jsize>(rects.size()));
    if (!rectangles || !directions) return nullptr;
    std::vector<jint> values;
    values.reserve(rects.size());
    for (const auto& rect : rects) values.push_back(rect.direction);
    env->SetIntArrayRegion(directions, 0, static_cast<jsize>(values.size()), values.data());
    jclass objectClass = env->FindClass("java/lang/Object");
    jobjectArray result = objectClass ? env->NewObjectArray(2, objectClass, nullptr) : nullptr;
    if (result) {
        env->SetObjectArrayElement(result, 0, rectangles);
        env->SetObjectArrayElement(result, 1, directions);
    }
    if (objectClass) env->DeleteLocalRef(objectClass);
    env->DeleteLocalRef(rectangles);
    env->DeleteLocalRef(directions);
    return result;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1getSelDirection(
    JNIEnv*, jobject, jlong handle) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    int direction = 0;
    return page && ksp::runtime::getTextSelectionDirection(page, &direction) ==
                   ErrorCode::Ok ? direction : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1isEditing(
    JNIEnv*, jobject, jlong handle) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    std::u16string text;
    return page && ksp::runtime::getFocusedDomainText(page, &text) == ErrorCode::Ok
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1length(
    JNIEnv*, jobject, jlong handle) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    std::u16string text;
    return page && ksp::runtime::getFocusedDomainText(page, &text) == ErrorCode::Ok
               ? static_cast<jint>(text.size()) : -1;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1setCursor(
    JNIEnv*, jobject, jlong handle, jint position) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    return page ? booleanResult(ksp::runtime::setTextSelection(
                      page, position, position)) : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1cursor(
    JNIEnv*, jobject, jlong handle) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    int start = -1, end = -1;
    return page && ksp::runtime::getTextSelection(page, &start, &end) == ErrorCode::Ok
               ? end : -1;
}

JNIEXPORT jstring JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1text(
    JNIEnv* env, jobject, jlong handle, jint start, jint end) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    std::u16string text;
    if (!page || ksp::runtime::getFocusedDomainText(page, &text) != ErrorCode::Ok ||
        start < 0 || end < start || end > static_cast<jint>(text.size())) return nullptr;
    return toJavaString(env, text.substr(static_cast<std::size_t>(start),
                                        static_cast<std::size_t>(end - start)));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1getCaretInfo(
    JNIEnv* env, jobject, jlong handle, jobject rect, jboolean) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    int start = -1, end = -1;
    ksp::runtime::TextCaretGeometry caret;
    if (!page || !rect ||
        ksp::runtime::getTextSelection(page, &start, &end) != ErrorCode::Ok ||
        start != end ||
        ksp::runtime::getTextCaretGeometry(page, end, &caret) != ErrorCode::Ok)
        return JNI_FALSE;
    const float left = std::min(caret.bottomX, caret.topX);
    const float right = std::max(caret.bottomX, caret.topX) + 0.1f;
    const float top = std::max(caret.bottomY, caret.topY);
    const float bottom = std::min(caret.bottomY, caret.topY);
    return ksp::jni::setRectFBounds(env, rect, left, top, right, bottom) ==
                   ErrorCode::Ok
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jstring JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1fontName(
    JNIEnv* env, jobject, jlong handle, jlong) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    std::string name;
    return page && ksp::runtime::getFocusedTextFontName(page, &name) == ErrorCode::Ok
               ? env->NewStringUTF(name.c_str()) : nullptr;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1setFontName(
    JNIEnv* env, jobject, jlong handle, jlong, jstring name) {
    const Handle page = editorPage(static_cast<Handle>(handle));
    ksp::jni::UtfChars value(env, name);
    return page && value
               ? booleanResult(ksp::runtime::setFocusedTextFontName(page, value.str()))
               : JNI_FALSE;
}

JNIEXPORT jstring JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetTextSelectByRange(
    JNIEnv* env, jobject, jlong page, jint start, jint end) {
    std::u16string value;
    if (ksp::runtime::getFocusedDomainText(static_cast<Handle>(page), &value) != ErrorCode::Ok ||
        start < 0 || end < start || end > static_cast<jint>(value.size())) return nullptr;
    return toJavaString(env, value.substr(start, end - start));
}

JNIEXPORT jstring JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetEditSelText(
    JNIEnv* env, jobject, jlong page) {
    int start = 0, end = 0;
    std::u16string value;
    if (ksp::runtime::getTextSelection(static_cast<Handle>(page), &start, &end) != ErrorCode::Ok ||
        ksp::runtime::getFocusedDomainText(static_cast<Handle>(page), &value) != ErrorCode::Ok)
        return nullptr;
    return toJavaString(env, value.substr(start, end - start));
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetRangeIndexAtPos(
    JNIEnv*, jobject, jlong page, jfloat x, jfloat y) {
    int value = -1;
    return ksp::runtime::getTextRangeAtPoint(static_cast<Handle>(page), x, y, &value)
                   == ErrorCode::Ok ? value : -1;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1SetCaret(
    JNIEnv*, jobject, jlong page, jint position) {
    return booleanResult(ksp::runtime::setTextSelection(static_cast<Handle>(page), position,
                                                        position));
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetCaret(
    JNIEnv*, jobject, jlong page) {
    int start = -1, end = -1;
    return ksp::runtime::getTextSelection(static_cast<Handle>(page), &start, &end)
                   == ErrorCode::Ok ? end : -1;
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetCaretInfo(
    JNIEnv* env, jobject, jlong page, jobject rect) {
    int start = -1, end = -1;
    ksp::runtime::TextCaretGeometry caret;
    if (ksp::runtime::getTextSelection(static_cast<Handle>(page), &start, &end) != ErrorCode::Ok ||
        start != end ||
        ksp::runtime::getTextCaretGeometry(static_cast<Handle>(page), end, &caret) != ErrorCode::Ok)
        return;
    const float dx = caret.topY - caret.bottomY;
    const float dy = -(caret.topX - caret.bottomX);
    const float length = std::max(0.001f, std::hypot(dx, dy));
    const float tx = dx / length * 0.1f, ty = dy / length * 0.1f;
    const float left = std::min(caret.bottomX, caret.topX);
    const float right = std::max(caret.bottomX + tx, caret.topX + tx);
    const float top = std::min(caret.bottomY, caret.topY);
    const float bottom = std::max(caret.bottomY + ty, caret.topY + ty);
    (void)ksp::jni::setRectFBounds(env, rect, left, top, right, bottom);
}

JNIEXPORT jobjectArray JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetCaretPoints(
    JNIEnv* env, jobject, jlong page) {
    int start = -1, end = -1;
    ksp::runtime::TextCaretGeometry caret;
    if (ksp::runtime::getTextSelection(static_cast<Handle>(page), &start, &end) != ErrorCode::Ok ||
        start != end ||
        ksp::runtime::getTextCaretGeometry(static_cast<Handle>(page), end, &caret) != ErrorCode::Ok)
        return nullptr;
    jclass cls = env->FindClass("android/graphics/PointF");
    if (!cls) return nullptr;
    jmethodID ctor = env->GetMethodID(cls, "<init>", "(FF)V");
    if (!ctor) { env->DeleteLocalRef(cls); return nullptr; }
    jobjectArray result = env->NewObjectArray(2, cls, nullptr);
    jobject top = env->NewObject(cls, ctor, caret.topX, caret.topY);
    jobject bottom = env->NewObject(cls, ctor, caret.bottomX, caret.bottomY);
    if (result && top && bottom) {
        env->SetObjectArrayElement(result, 0, top);
        env->SetObjectArrayElement(result, 1, bottom);
    }
    if (top) env->DeleteLocalRef(top);
    if (bottom) env->DeleteLocalRef(bottom);
    env->DeleteLocalRef(cls);
    return result;
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetEditSelectionRect(
    JNIEnv* env, jobject, jlong page, jint position, jobject rect) {
    if (position <= 0) return;
    ksp::runtime::TextCharacterGeometry character;
    if (ksp::runtime::getTextCharacterGeometry(static_cast<Handle>(page), position - 1,
                                                &character) != ErrorCode::Ok) return;
    float left = character.quad[0], right = character.quad[0];
    float top = character.quad[1], bottom = character.quad[1];
    for (int i = 1; i < 4; ++i) {
        left = std::min(left, character.quad[i * 2]);
        right = std::max(right, character.quad[i * 2]);
        top = std::max(top, character.quad[i * 2 + 1]);
        bottom = std::min(bottom, character.quad[i * 2 + 1]);
    }
    (void)ksp::jni::setRectFBounds(env, rect, left, top, right, bottom);
}

JNIEXPORT jfloatArray JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1GetEditSelectionQuadRect(
    JNIEnv* env, jobject, jlong page, jint position) {
    jfloat values[8] = {0};
    if (position > 0) {
        ksp::runtime::TextCharacterGeometry character;
        if (ksp::runtime::getTextCharacterGeometry(static_cast<Handle>(page), position - 1,
                                                    &character) == ErrorCode::Ok)
            std::copy(character.quad, character.quad + 8, values);
    }
    jfloatArray result = env->NewFloatArray(8);
    if (result) env->SetFloatArrayRegion(result, 0, 8, values);
    return result;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnChars(
    JNIEnv* env, jobject, jlong page, jstring chars, jboolean, jint) {
    if (!chars) return JNI_FALSE;
    const jsize length = env->GetStringLength(chars);
    const jchar* data = env->GetStringChars(chars, nullptr);
    if (!data) return JNI_FALSE;
    std::u16string value(reinterpret_cast<const char16_t*>(data),
                         static_cast<std::size_t>(length));
    env->ReleaseStringChars(chars, data);
    return booleanResult(ksp::runtime::replaceFocusedText(static_cast<Handle>(page), value));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnChar(
    JNIEnv*, jobject, jlong page, jint character, jint) {
    std::u16string value;
    if (character >= 0 && character <= 0xFFFF) {
        value.push_back(static_cast<char16_t>(character));
    } else if (character <= 0x10FFFF) {
        const uint32_t cp = static_cast<uint32_t>(character) - 0x10000;
        value.push_back(static_cast<char16_t>(0xD800 + (cp >> 10)));
        value.push_back(static_cast<char16_t>(0xDC00 + (cp & 0x3FF)));
    } else return JNI_FALSE;
    return booleanResult(ksp::runtime::replaceFocusedText(static_cast<Handle>(page), value));
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnKeyDown(
    JNIEnv*, jobject, jlong page, jint keyCode, jint) {
    bool handled = false;
    return ksp::runtime::handleTextKeyDown(static_cast<Handle>(page), keyCode, &handled)
                   == ErrorCode::Ok && handled ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFPage_native_1OnKeyUp(
    JNIEnv*, jobject, jlong, jint keyCode, jint) {
    return (keyCode == 8 || keyCode == 46 || keyCode == 67 || keyCode == 112)
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_formfill_PDFFormFill_native_1undoEdit(
    JNIEnv*, jclass, jlong command) {
    (void)ksp::runtime::undoEditCommand(static_cast<Handle>(command));
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_formfill_PDFFormFill_native_1redoEdit(
    JNIEnv*, jclass, jlong command) {
    (void)ksp::runtime::redoEditCommand(static_cast<Handle>(command));
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_formfill_PDFFormFill_native_1deleteTransEdit(
    JNIEnv*, jclass, jlong command) {
    (void)ksp::runtime::deleteEditCommand(static_cast<Handle>(command));
}

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_formfill_PDFFormFill_native_1getFromHandle(
    JNIEnv*, jobject, jlong formFill) {
    Handle editor = 0;
    return ksp::runtime::getFormFillEditor(static_cast<Handle>(formFill), &editor) ==
                   ErrorCode::Ok
               ? static_cast<jlong>(editor) : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_formfill_PDFFormFill_native_1onLButtonDown(
    JNIEnv*, jobject, jlong formFill, jlong page, jfloat x, jfloat y) {
    if (ksp::runtime::bindFormFillPage(static_cast<Handle>(formFill),
                                       static_cast<Handle>(page)) != ErrorCode::Ok)
        return JNI_FALSE;
    bool focused = false;
    return ksp::runtime::focusTextAtPoint(static_cast<Handle>(page), x, y, 0,
                                          &focused) == ErrorCode::Ok && focused
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_formfill_PDFFormFill_native_1onLButtonUp(
    JNIEnv*, jobject, jlong formFill, jlong page, jfloat, jfloat) {
    if (ksp::runtime::bindFormFillPage(static_cast<Handle>(formFill),
                                       static_cast<Handle>(page)) != ErrorCode::Ok)
        return JNI_FALSE;
    bool handled = false;
    return ksp::runtime::endTextSelectionDrag(static_cast<Handle>(page), &handled) ==
                   ErrorCode::Ok
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_formfill_PDFFormFill_native_1onMouseMove(
    JNIEnv*, jobject, jlong formFill, jlong page, jfloat x, jfloat y) {
    if (ksp::runtime::bindFormFillPage(static_cast<Handle>(formFill),
                                       static_cast<Handle>(page)) != ErrorCode::Ok)
        return JNI_FALSE;
    bool handled = false;
    ErrorCode code = ksp::runtime::moveTextSelectionAtPoint(
        static_cast<Handle>(page), x, y, 0, &handled);
    if (code == ErrorCode::Ok && handled) return JNI_TRUE;
    int hover = 0;
    return ksp::runtime::getTextHoverType(static_cast<Handle>(page), x, y, &hover) ==
                   ErrorCode::Ok && hover != 0
               ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_formfill_PDFFormFill_native_1onChars(
    JNIEnv* env, jobject, jlong formFill, jlong page, jstring value) {
    if (!value || ksp::runtime::bindFormFillPage(static_cast<Handle>(formFill),
              static_cast<Handle>(page)) != ErrorCode::Ok) return JNI_FALSE;
    const jchar* chars = env->GetStringChars(value, nullptr);
    if (!chars) return JNI_FALSE;
    const jsize length = env->GetStringLength(value);
    std::u16string text(reinterpret_cast<const char16_t*>(chars),
                        static_cast<std::size_t>(length));
    env->ReleaseStringChars(value, chars);
    return booleanResult(ksp::runtime::replaceFocusedText(
        static_cast<Handle>(page), text));
}

JNIEXPORT jlong JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeTakePendingEditUndo(
    JNIEnv*, jclass, jlong page) {
    Handle result = 0;
    return ksp::runtime::takePendingEditUndo(static_cast<Handle>(page), &result)
                   == ErrorCode::Ok ? static_cast<jlong>(result) : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeGetEditUndoDamage(
    JNIEnv* env, jclass, jlong command, jfloatArray bounds) {
    if (!bounds || env->GetArrayLength(bounds) < 4) return JNI_FALSE;
    ksp::runtime::EditDamageRect value;
    if (ksp::runtime::getEditCommandDamage(static_cast<Handle>(command), &value) !=
        ErrorCode::Ok) return JNI_FALSE;
    const jfloat result[] = {value.left, value.bottom, value.right, value.top};
    env->SetFloatArrayRegion(bounds, 0, 4, result);
    return env->ExceptionCheck() ? (env->ExceptionClear(), JNI_FALSE) : JNI_TRUE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeTextSelectionDirection(
    JNIEnv*, jclass, jlong page) {
    int direction = 0;
    return ksp::runtime::getTextSelectionDirection(static_cast<Handle>(page), &direction) ==
                   ErrorCode::Ok ? direction : 0;
}

JNIEXPORT jintArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeResolveBidiRuns(
    JNIEnv* env, jclass, jstring text, jint paragraphDirection) {
    if (!text || (paragraphDirection != 0 && paragraphDirection != 1)) return nullptr;
    const jchar* chars = env->GetStringChars(text, nullptr);
    if (!chars) return nullptr;
    const jsize length = env->GetStringLength(text);
    std::u16string logical(reinterpret_cast<const char16_t*>(chars),
                          static_cast<std::size_t>(length));
    env->ReleaseStringChars(text, chars);
    std::vector<ksp::runtime::BidiRun> runs;
    if (!ksp::runtime::resolveVisualBidiRuns(
            logical, paragraphDirection ? ksp::runtime::TextDirection::Rtl
                                        : ksp::runtime::TextDirection::Ltr,
            &runs)) return nullptr;
    std::vector<jint> flattened;
    flattened.reserve(runs.size() * 3);
    for (const auto& run : runs) {
        flattened.push_back(run.logicalStart);
        flattened.push_back(run.length);
        flattened.push_back(static_cast<jint>(run.direction));
    }
    jintArray result = env->NewIntArray(static_cast<jsize>(flattened.size()));
    if (result && !flattened.empty()) {
        env->SetIntArrayRegion(result, 0, static_cast<jsize>(flattened.size()),
                               flattened.data());
    }
    return result;
}

JNIEXPORT jintArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeShapeReplacement(
    JNIEnv* env, jclass, jlong document, jstring originalName, jstring text,
    jint direction) {
    if (!originalName || !text || (direction != 0 && direction != 1)) return nullptr;
    ksp::jni::UtfChars name(env, originalName);
    const jchar* chars = env->GetStringChars(text, nullptr);
    if (!name || !chars) return nullptr;
    const jsize length = env->GetStringLength(text);
    std::u16string logical(reinterpret_cast<const char16_t*>(chars),
                           static_cast<std::size_t>(length));
    env->ReleaseStringChars(text, chars);
    ErrorCode error = ErrorCode::Ok;
    auto doc = ksp::runtime::borrowDocument(static_cast<Handle>(document), &error);
    if (!doc || logical.empty()) return nullptr;
    FPDF_FONT font = ksp::runtime::loadReplacementFont(
        doc->pdf, name.str(), static_cast<uint32_t>(logical.front()));
    if (!font) return nullptr;
    std::vector<ksp::runtime::ShapedGlyph> glyphs;
    const bool shaped = ksp::runtime::shapeFontText(
        font, logical, direction == 1, &glyphs);
    FPDFFont_Close(font);
    if (!shaped) return nullptr;
    std::vector<jint> values;
    ksp::runtime::ShapedTextEncoding encoding;
    const bool encoded = ksp::runtime::buildShapedTextEncoding(
        doc->pdf, name.str(), logical, direction == 1, 12.0f, &encoding);
    values.reserve(4 + glyphs.size() * 6);
    values.push_back(encoded ? 1 : 0);
    values.push_back(encoded ? static_cast<jint>(encoding.charcodes.size()) : 0);
    values.push_back(encoded ? static_cast<jint>(encoding.positions.size()) : 0);
    values.push_back(encoded ? static_cast<jint>(std::lround(encoding.advance * 1000.0f)) : 0);
    if (encoding.font) FPDFFont_Close(encoding.font);
    for (const auto& glyph : glyphs) {
        values.push_back(static_cast<jint>(glyph.glyphId));
        values.push_back(static_cast<jint>(glyph.cluster));
        values.push_back(static_cast<jint>(std::lround(glyph.xAdvance * 1000000.0f)));
        values.push_back(static_cast<jint>(std::lround(glyph.yAdvance * 1000000.0f)));
        values.push_back(static_cast<jint>(std::lround(glyph.xOffset * 1000000.0f)));
        values.push_back(static_cast<jint>(std::lround(glyph.yOffset * 1000000.0f)));
    }
    jintArray result = env->NewIntArray(static_cast<jsize>(values.size()));
    if (result && !values.empty()) {
        env->SetIntArrayRegion(result, 0, static_cast<jsize>(values.size()), values.data());
    }
    return result;
}

JNIEXPORT jlong JNICALL
Java_cn_wps_rebuild_EditHistoryBridge_nativeTakePendingEditUndo(
    JNIEnv*, jclass, jlong page) {
    Handle result = 0;
    return ksp::runtime::takePendingEditUndo(static_cast<Handle>(page), &result)
                   == ErrorCode::Ok ? static_cast<jlong>(result) : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_rebuild_EditHistoryBridge_nativeGetEditUndoDamage(
    JNIEnv* env, jclass, jlong command, jfloatArray bounds) {
    if (!bounds || env->GetArrayLength(bounds) < 4) return JNI_FALSE;
    ksp::runtime::EditDamageRect value;
    if (ksp::runtime::getEditCommandDamage(static_cast<Handle>(command), &value) !=
        ErrorCode::Ok) return JNI_FALSE;
    const jfloat result[] = {value.left, value.bottom, value.right, value.top};
    env->SetFloatArrayRegion(bounds, 0, 4, result);
    return env->ExceptionCheck() ? (env->ExceptionClear(), JNI_FALSE) : JNI_TRUE;
}

// Test-only probes exercise the central mutation path before object mutators exist.
JNIEXPORT jint JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeStagePageMutation(
    JNIEnv*, jclass, jlong page, jint editStatus, jboolean focus, jboolean hover) {
    return static_cast<jint>(ksp::runtime::asInt(ksp::runtime::stagePageMutation(
        static_cast<Handle>(page), editStatus, focus == JNI_TRUE, hover == JNI_TRUE)));
}

JNIEXPORT jlongArray JNICALL
Java_cn_wps_rebuild_RuntimeProbe_nativeEditDiagnostics(
    JNIEnv* env, jclass, jlong page) {
    ksp::runtime::EditDiagnostics value;
    if (ksp::runtime::getEditDiagnostics(static_cast<Handle>(page), &value) != ErrorCode::Ok)
        return nullptr;
    const jlong fields[] = {
        static_cast<jlong>(value.dirtyFunctions), static_cast<jlong>(value.stagedEpoch),
        static_cast<jlong>(value.committedEpoch), static_cast<jlong>(value.textCacheEpoch),
        value.hasBackup ? 1 : 0, value.hasFocus ? 1 : 0, value.hasHover ? 1 : 0};
    jlongArray result = env->NewLongArray(7);
    if (result) env->SetLongArrayRegion(result, 0, 7, fields);
    return result;
}

}  // extern "C"
