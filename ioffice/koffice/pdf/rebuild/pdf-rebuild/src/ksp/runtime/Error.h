#pragma once

#include <cstdint>

namespace ksp::runtime {

// Public JNI methods keep the legacy 0/-1..-6 document-open contract. Runtime
// contract failures use a separate range so tests and logs can distinguish
// malformed, stale, wrong-type and cross-document handles.
enum class ErrorCode : int32_t {
    Ok = 0,
    General = -1,
    FileFormat = -2,
    Password = -3,
    Certificate = -4,
    SecurityHandler = -5,
    Suffix = -6,

    InvalidArgument = -1001,
    InvalidHandle = -1002,
    WrongType = -1003,
    OwnerMismatch = -1004,
    StaleHandle = -1005,
    OutOfRange = -1006,
    Io = -1007,
    JniContract = -1008,
    NotFound = -1009,
    Unsupported = -1010,
};

const char* errorName(ErrorCode code);
void setLastError(ErrorCode code);
ErrorCode lastError();

inline int asInt(ErrorCode code) { return static_cast<int>(code); }

}  // namespace ksp::runtime
