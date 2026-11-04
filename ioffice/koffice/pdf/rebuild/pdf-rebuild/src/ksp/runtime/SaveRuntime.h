#pragma once

#include "PdfRuntime.h"

#include <cstdio>
#include <memory>
#include <mutex>
#include <string>

namespace ksp::runtime {

enum class SavePhase : uint8_t { Created, Started, Finished, Failed, Reset };

struct SaveSessionState {
    SaveSessionState(std::shared_ptr<DocumentState> owner, bool immutable)
        : document(std::move(owner)), annotImmutable(immutable) {}
    ~SaveSessionState();

    std::shared_ptr<DocumentState> document;
    bool annotImmutable = false;
    SavePhase phase = SavePhase::Created;
    std::string path;
    FILE* file = nullptr;
    int progress = 0;
    std::mutex mutex;
};

ErrorCode createSaveSession(Handle document, bool annotImmutable, Handle* result);
ErrorCode startSaveSession(Handle session, const std::string& path, bool* started);
int continueSaveSession(Handle session, int milliseconds, ErrorCode* error = nullptr);
ErrorCode resetSaveSession(Handle session);
ErrorCode closeSaveSession(Handle session);
std::shared_ptr<SaveSessionState> borrowSaveSession(Handle session,
                                                    ErrorCode* error = nullptr);

}  // namespace ksp::runtime
