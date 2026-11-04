#include "../runtime/FontRuntime.h"
#include "JniSupport.h"

#include <jni.h>

extern "C" {

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFFontMgr_native_1loadPrivateFont(
        JNIEnv* env, jclass, jstring path) {
    ksp::jni::UtfChars value(env, path);
    return value && ksp::runtime::loadPrivateFont(value.str()) ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_std_PDFFontMgr_native_1loadPrivateFonts(
        JNIEnv* env, jclass, jstring path) {
    ksp::jni::UtfChars value(env, path);
    return value && ksp::runtime::loadPrivateFonts(value.str()) ? JNI_TRUE : JNI_FALSE;
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1loadFont(
        JNIEnv* env, jobject, jstring path) {
    ksp::jni::UtfChars value(env, path);
    if (value) (void)ksp::runtime::loadPrivateFont(value.str());
}

JNIEXPORT void JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1addSupportReplaceFont(
        JNIEnv* env, jobject, jstring originalName) {
    ksp::jni::UtfChars value(env, originalName);
    if (value) ksp::runtime::addSupportedReplacementFont(value.str());
}

JNIEXPORT jboolean JNICALL
Java_cn_wps_moffice_pdf_core_edit_PDFTextEditor_native_1setReplaceDisplayFont(
        JNIEnv* env, jobject, jstring originalName, jstring replacementName) {
    ksp::jni::UtfChars original(env, originalName);
    ksp::jni::UtfChars replacement(env, replacementName);
    return original && replacement && ksp::runtime::setReplacementDisplayFont(
            original.str(), replacement.str()) ? JNI_TRUE : JNI_FALSE;
}

}  // extern "C"
