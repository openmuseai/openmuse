#include "JniSupport.h"

namespace ksp::jni {
namespace {
runtime::ErrorCode contractFailure(JNIEnv* env) {
    // This layer uses explicit integer error codes for ABI contract failures.
    // Clear lookup/access exceptions so Java observes JNI_CONTRACT predictably.
    if (env->ExceptionCheck()) env->ExceptionClear();
    runtime::setLastError(runtime::ErrorCode::JniContract);
    return runtime::ErrorCode::JniContract;
}

jfieldID field(JNIEnv* env, jclass cls, const char* name, const char* signature) {
    jfieldID value = env->GetFieldID(cls, name, signature);
    if (!value && env->ExceptionCheck()) env->ExceptionClear();
    return value;
}
}  // namespace

UtfChars::UtfChars(JNIEnv* env, jstring value)
    : env_(env), value_(value), chars_(value ? env->GetStringUTFChars(value, nullptr) : nullptr) {}

UtfChars::~UtfChars() {
    if (chars_) env_->ReleaseStringUTFChars(value_, chars_);
}

runtime::ErrorCode setNativeHandle(JNIEnv* env, jobject nativeHandle,
                                   runtime::Handle value) {
    if (!nativeHandle) {
        runtime::setLastError(runtime::ErrorCode::InvalidArgument);
        return runtime::ErrorCode::InvalidArgument;
    }
    jclass cls = env->GetObjectClass(nativeHandle);
    if (!cls) {
        return contractFailure(env);
    }
    jfieldID valueField = field(env, cls, "mValue", "J");
    env->DeleteLocalRef(cls);
    if (!valueField) return contractFailure(env);
    env->SetLongField(nativeHandle, valueField, static_cast<jlong>(value));
    if (env->ExceptionCheck()) {
        return contractFailure(env);
    }
    runtime::setLastError(runtime::ErrorCode::Ok);
    return runtime::ErrorCode::Ok;
}

runtime::ErrorCode setRectF(JNIEnv* env, jobject rect, float width, float height) {
    return setRectFBounds(env, rect, 0.0f, 0.0f, width, height);
}

runtime::ErrorCode setRectFBounds(JNIEnv* env, jobject rect, float leftValue, float topValue,
                                  float rightValue, float bottomValue) {
    if (!rect) {
        runtime::setLastError(runtime::ErrorCode::InvalidArgument);
        return runtime::ErrorCode::InvalidArgument;
    }
    jclass cls = env->GetObjectClass(rect);
    if (!cls) {
        return contractFailure(env);
    }
    jfieldID left = field(env, cls, "left", "F");
    jfieldID top = left ? field(env, cls, "top", "F") : nullptr;
    jfieldID right = top ? field(env, cls, "right", "F") : nullptr;
    jfieldID bottom = right ? field(env, cls, "bottom", "F") : nullptr;
    env->DeleteLocalRef(cls);
    if (!left || !top || !right || !bottom) {
        return contractFailure(env);
    }
    env->SetFloatField(rect, left, leftValue);
    env->SetFloatField(rect, top, topValue);
    env->SetFloatField(rect, right, rightValue);
    env->SetFloatField(rect, bottom, bottomValue);
    if (env->ExceptionCheck()) {
        return contractFailure(env);
    }
    runtime::setLastError(runtime::ErrorCode::Ok);
    return runtime::ErrorCode::Ok;
}

}  // namespace ksp::jni
