#pragma once

#include "../runtime/Error.h"
#include "../runtime/HandleRegistry.h"

#include <jni.h>
#include <string>

namespace ksp::jni {

class UtfChars {
public:
    UtfChars(JNIEnv* env, jstring value);
    ~UtfChars();
    UtfChars(const UtfChars&) = delete;
    UtfChars& operator=(const UtfChars&) = delete;

    explicit operator bool() const { return chars_ != nullptr; }
    const char* get() const { return chars_; }
    std::string str() const { return chars_ ? std::string(chars_) : std::string(); }

private:
    JNIEnv* env_;
    jstring value_;
    const char* chars_;
};

runtime::ErrorCode setNativeHandle(JNIEnv* env, jobject nativeHandle,
                                   runtime::Handle value);
runtime::ErrorCode setRectF(JNIEnv* env, jobject rect, float width, float height);
runtime::ErrorCode setRectFBounds(JNIEnv* env, jobject rect, float left, float top,
                                  float right, float bottom);

}  // namespace ksp::jni
