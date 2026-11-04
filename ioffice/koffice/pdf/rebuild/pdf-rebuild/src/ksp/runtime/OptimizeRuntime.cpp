#include "OptimizeRuntime.h"

#include <fpdf_save.h>

#include <cerrno>
#include <cstdio>

namespace ksp::runtime {
namespace {
struct OptimizeFileWriter {
    FPDF_FILEWRITE base{};
    FILE* file = nullptr;
};

int writeBlock(FPDF_FILEWRITE* writer, const void* data, unsigned long size) {
    auto* value = reinterpret_cast<OptimizeFileWriter*>(writer);
    return value->file && std::fwrite(data, 1, size, value->file) == size ? 1 : 0;
}

void store(ErrorCode value, ErrorCode* out) {
    setLastError(value);
    if (out) *out = value;
}
}  // namespace

OptimizeSessionState::~OptimizeSessionState() {
    if (file) std::fclose(file);
}

std::shared_ptr<OptimizeSessionState> borrowOptimizeSession(
    Handle session, Handle document, ErrorCode* error) {
    BorrowedHandle value = handles().borrow(session, HandleType::OptimizeSession,
                                            document, document);
    if (error) *error = value.error;
    return value ? value.as<OptimizeSessionState>() : nullptr;
}

ErrorCode openOptimizeSession(Handle document, const std::string& path, int flag,
                              Handle* result) {
    if (!result || path.empty()) {
        if (result) *result = 0;
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    ErrorCode error;
    auto doc = borrowDocument(document, &error);
    if (!doc) return error;
    {
        std::lock_guard<std::mutex> lock(doc->mutex);
        if (!doc->pdf) {
            setLastError(ErrorCode::InvalidHandle);
            return ErrorCode::InvalidHandle;
        }
    }
    errno = 0;
    FILE* file = std::fopen(path.c_str(), "wb");
    if (!file) {
        setLastError(ErrorCode::Io);
        return ErrorCode::Io;
    }
    auto state = std::make_shared<OptimizeSessionState>(doc, path, flag, file);
    Handle session = handles().insert(HandleType::OptimizeSession, state,
                                      document, document, &error);
    if (!session) {
        state.reset();
        std::remove(path.c_str());
        return error;
    }
    *result = session;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

int continueOptimizeSession(Handle document, Handle session, int milliseconds,
                            ErrorCode* outError) {
    if (milliseconds < 0) {
        store(ErrorCode::InvalidArgument, outError);
        return -1;
    }
    ErrorCode error;
    if (!borrowDocument(document, &error)) {
        if (outError) *outError = error;
        return -1;
    }
    auto state = borrowOptimizeSession(session, document, &error);
    if (!state) {
        if (outError) *outError = error;
        return -1;
    }
    std::lock_guard<std::mutex> sessionLock(state->mutex);
    if (state->phase == OptimizePhase::Finished) {
        store(ErrorCode::Ok, outError);
        return 100;
    }
    if (state->phase != OptimizePhase::Open || !state->file) {
        store(ErrorCode::InvalidArgument, outError);
        return -1;
    }

    OptimizeFileWriter writer;
    writer.base.version = 1;
    writer.base.WriteBlock = writeBlock;
    writer.file = state->file;
    errno = 0;
    int saved = 0;
    bool parserOpen = false;
    {
        std::lock_guard<std::mutex> documentLock(state->document->mutex);
        parserOpen = state->document->pdf != nullptr;
        if (parserOpen) {
            saved = FPDF_SaveAsCopy(state->document->pdf, &writer.base,
                                    FPDF_NO_INCREMENTAL);
        }
    }
    const bool closeOk = std::fclose(state->file) == 0;
    state->file = nullptr;
    if (!parserOpen || !saved || !closeOk) {
        state->phase = OptimizePhase::Failed;
        std::remove(state->path.c_str());
        store(parserOpen ? ErrorCode::Io : ErrorCode::InvalidHandle, outError);
        return -1;
    }
    state->phase = OptimizePhase::Finished;
    store(ErrorCode::Ok, outError);
    return 100;
}

ErrorCode closeOptimizeSession(Handle document, Handle session) {
    ErrorCode error;
    if (!borrowDocument(document, &error)) return error;
    if (!borrowOptimizeSession(session, document, &error)) return error;
    error = handles().invalidate(session, HandleType::OptimizeSession);
    setLastError(error);
    return error;
}

}  // namespace ksp::runtime
