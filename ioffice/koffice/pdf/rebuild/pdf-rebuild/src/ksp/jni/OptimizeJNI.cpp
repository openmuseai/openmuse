#include "JniSupport.h"
#include "../runtime/OptimizeRuntime.h"

#include <jni.h>

using ksp::runtime::ErrorCode;
using ksp::runtime::Handle;

namespace {
jlong open(JNIEnv* env, jlong document, jstring path, jint flag) {
    ksp::jni::UtfChars value(env, path);
    if (!value) {
        ksp::runtime::setLastError(ErrorCode::InvalidArgument);
        return 0;
    }
    Handle result = 0;
    return ksp::runtime::openOptimizeSession(static_cast<Handle>(document),
                                             value.str(), flag, &result) == ErrorCode::Ok
               ? static_cast<jlong>(result)
               : 0;
}

jint next(jlong document, jlong session, jint milliseconds) {
    return ksp::runtime::continueOptimizeSession(static_cast<Handle>(document),
                                                 static_cast<Handle>(session),
                                                 milliseconds);
}

void close(jlong document, jlong session) {
    if (document && session) {
        (void)ksp::runtime::closeOptimizeSession(static_cast<Handle>(document),
                                                 static_cast<Handle>(session));
    } else {
        ksp::runtime::setLastError(ErrorCode::InvalidHandle);
    }
}
}  // namespace

extern "C" {

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1openOptimize(
    JNIEnv* env, jobject, jlong document, jstring path, jint flag) {
    return open(env, document, path, flag);
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1continueOptimize(
    JNIEnv*, jobject, jlong document, jlong session, jint milliseconds) {
    return next(document, session, milliseconds);
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFDocument_native_1closeOptimize(
    JNIEnv*, jobject, jlong document, jlong session) {
    close(document, session);
}

JNIEXPORT jlong JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1openOptimize(
    JNIEnv* env, jobject, jlong document, jstring path) {
    return open(env, document, path, 0);
}

JNIEXPORT jint JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1continueOptimize(
    JNIEnv*, jobject, jlong document, jlong session, jint milliseconds) {
    return next(document, session, milliseconds);
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_io_PDFNativeSaver_native_1closeOptimize(
    JNIEnv*, jobject, jlong document, jlong session) {
    close(document, session);
}

}  // extern "C"
