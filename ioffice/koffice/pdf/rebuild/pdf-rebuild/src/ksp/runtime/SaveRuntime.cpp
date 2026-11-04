#include "SaveRuntime.h"

#include <fpdf_save.h>

#include <cerrno>
#include <cstdio>

namespace ksp::runtime {
namespace {
struct PdfFileWriter {
    FPDF_FILEWRITE base{};
    FILE* file = nullptr;
};

int writeBlock(FPDF_FILEWRITE* writer, const void* data, unsigned long size) {
    auto* value = reinterpret_cast<PdfFileWriter*>(writer);
    return value->file && std::fwrite(data, 1, size, value->file) == size ? 1 : 0;
}

void store(ErrorCode value, ErrorCode* out) {
    setLastError(value);
    if (out) *out = value;
}
}  // namespace

SaveSessionState::~SaveSessionState() {
    if (file) std::fclose(file);
}

std::shared_ptr<SaveSessionState> borrowSaveSession(Handle session, ErrorCode* error) {
    BorrowedHandle value = handles().borrow(session, HandleType::SaveSession);
    if (error) *error = value.error;
    return value ? value.as<SaveSessionState>() : nullptr;
}

ErrorCode createSaveSession(Handle document, bool annotImmutable, Handle* result) {
    if (!result) {
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
    auto state = std::make_shared<SaveSessionState>(doc, annotImmutable);
    Handle value = handles().insert(HandleType::SaveSession, state,
                                    document, document, &error);
    if (!value) return error;
    *result = value;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode startSaveSession(Handle session, const std::string& path, bool* started) {
    if (!started || path.empty()) {
        if (started) *started = false;
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *started = false;
    ErrorCode error;
    auto state = borrowSaveSession(session, &error);
    if (!state) return error;
    std::lock_guard<std::mutex> lock(state->mutex);
    if (state->phase != SavePhase::Created) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    errno = 0;
    state->file = std::fopen(path.c_str(), "wb");
    if (!state->file) {
        state->phase = SavePhase::Failed;
        setLastError(ErrorCode::Io);
        return ErrorCode::Io;
    }
    state->path = path;
    state->phase = SavePhase::Started;
    state->progress = 0;
    *started = true;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

int continueSaveSession(Handle session, int milliseconds, ErrorCode* outError) {
    if (milliseconds < 0) {
        store(ErrorCode::InvalidArgument, outError);
        return -1;
    }
    ErrorCode error;
    auto state = borrowSaveSession(session, &error);
    if (!state) {
        if (outError) *outError = error;
        return -1;
    }
    std::lock_guard<std::mutex> sessionLock(state->mutex);
    if (state->phase == SavePhase::Finished) {
        store(ErrorCode::Ok, outError);
        return 100;
    }
    if (state->phase != SavePhase::Started || !state->file) {
        store(ErrorCode::InvalidArgument, outError);
        return -1;
    }

    PdfFileWriter writer;
    writer.base.version = 1;
    writer.base.WriteBlock = writeBlock;
    writer.file = state->file;
    errno = 0;
    int saved = 0;
    {
        std::lock_guard<std::mutex> documentLock(state->document->mutex);
        saved = state->document->pdf
                    ? FPDF_SaveAsCopy(state->document->pdf, &writer.base, 0)
                    : 0;
    }
    const int ioError = errno;
    const bool closeOk = std::fclose(state->file) == 0;
    state->file = nullptr;
    if (!saved || !closeOk) {
        state->phase = SavePhase::Failed;
        std::remove(state->path.c_str());
        store(ErrorCode::Io, outError);
        return ioError == ENOSPC ? -2 : -1;
    }
    state->phase = SavePhase::Finished;
    state->progress = 100;
    store(ErrorCode::Ok, outError);
    return 100;
}

ErrorCode resetSaveSession(Handle session) {
    ErrorCode error;
    auto state = borrowSaveSession(session, &error);
    if (!state) return error;
    std::lock_guard<std::mutex> lock(state->mutex);
    // The public PDFium save path does not mutate the live document. The
    // original ResetDocument rollback therefore maps to an explicit no-op.
    if (state->phase == SavePhase::Created || state->phase == SavePhase::Started ||
        state->phase == SavePhase::Finished) {
        if (state->phase == SavePhase::Started && state->file) {
            std::fclose(state->file);
            state->file = nullptr;
            std::remove(state->path.c_str());
        }
        state->phase = SavePhase::Reset;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    setLastError(ErrorCode::InvalidArgument);
    return ErrorCode::InvalidArgument;
}

ErrorCode closeSaveSession(Handle session) {
    ErrorCode result = handles().invalidate(session, HandleType::SaveSession);
    setLastError(result);
    return result;
}

}  // namespace ksp::runtime
