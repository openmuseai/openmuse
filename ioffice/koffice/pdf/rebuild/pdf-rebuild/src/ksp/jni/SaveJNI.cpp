#include "JniSupport.h"
#include "../runtime/SaveRuntime.h"

#include <climits>
#include <jni.h>

using ksp::runtime::ErrorCode;
using ksp::runtime::Handle;

extern "C" {

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1new(
    JNIEnv*, jobject, jlong document, jboolean immutable) {
    Handle result = 0;
    return ksp::runtime::createSaveSession(static_cast<Handle>(document),
                                           immutable == JNI_TRUE, &result) == ErrorCode::Ok
               ? static_cast<jlong>(result)
               : 0;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1start(
    JNIEnv* env, jobject, jlong session, jstring path) {
    ksp::jni::UtfChars value(env, path);
    if (!value) {
        ksp::runtime::setLastError(ErrorCode::InvalidArgument);
        return JNI_FALSE;
    }
    bool started = false;
    (void)ksp::runtime::startSaveSession(static_cast<Handle>(session),
                                         value.str(), &started);
    return started ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1continue__JI(
    JNIEnv*, jobject, jlong session, jint milliseconds) {
    return ksp::runtime::continueSaveSession(static_cast<Handle>(session),
                                             milliseconds);
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1continue__J(
    JNIEnv*, jobject, jlong session) {
    return ksp::runtime::continueSaveSession(static_cast<Handle>(session), INT_MAX);
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1delete(
    JNIEnv*, jobject, jlong session) {
    if (session) (void)ksp::runtime::closeSaveSession(static_cast<Handle>(session));
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1ResetDocument(
    JNIEnv*, jobject, jlong session) {
    if (session) (void)ksp::runtime::resetSaveSession(static_cast<Handle>(session));
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1closeParser(
    JNIEnv*, jobject, jlong document) {
    return ksp::runtime::closeDocumentParser(static_cast<Handle>(document)) == ErrorCode::Ok
               ? 0
               : -1;
}

}  // extern "C"
