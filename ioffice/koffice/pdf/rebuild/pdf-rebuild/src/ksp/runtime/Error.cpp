#include "Error.h"

namespace ksp::runtime {
namespace {
thread_local ErrorCode g_last_error = ErrorCode::Ok;
}

const char* errorName(ErrorCode code) {
    switch (code) {
        case ErrorCode::Ok: return "OK";
        case ErrorCode::General: return "GENERAL";
        case ErrorCode::FileFormat: return "FILE_FORMAT";
        case ErrorCode::Password: return "PASSWORD";
        case ErrorCode::Certificate: return "CERTIFICATE";
        case ErrorCode::SecurityHandler: return "SECURITY_HANDLER";
        case ErrorCode::Suffix: return "SUFFIX";
        case ErrorCode::InvalidArgument: return "INVALID_ARGUMENT";
        case ErrorCode::InvalidHandle: return "INVALID_HANDLE";
        case ErrorCode::WrongType: return "WRONG_TYPE";
        case ErrorCode::OwnerMismatch: return "OWNER_MISMATCH";
        case ErrorCode::StaleHandle: return "STALE_HANDLE";
        case ErrorCode::OutOfRange: return "OUT_OF_RANGE";
        case ErrorCode::Io: return "IO";
        case ErrorCode::JniContract: return "JNI_CONTRACT";
        case ErrorCode::NotFound: return "NOT_FOUND";
        case ErrorCode::Unsupported: return "UNSUPPORTED";
    }
    return "UNKNOWN";
}

void setLastError(ErrorCode code) { g_last_error = code; }
ErrorCode lastError() { return g_last_error; }

}  // namespace ksp::runtime
