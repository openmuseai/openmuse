#pragma once

#include "PdfRuntime.h"

#include <cstdio>
#include <memory>
#include <mutex>
#include <string>

namespace ksp::runtime {

enum class OptimizePhase : uint8_t { Open, Finished, Failed };

struct OptimizeSessionState {
    OptimizeSessionState(std::shared_ptr<DocumentState> owner, std::string output,
                         int preset, FILE* stream)
        : document(std::move(owner)), path(std::move(output)), flag(preset), file(stream) {}
    ~OptimizeSessionState();

    std::shared_ptr<DocumentState> document;
    std::string path;
    int flag = 0;
    FILE* file = nullptr;
    OptimizePhase phase = OptimizePhase::Open;
    std::mutex mutex;
};

ErrorCode openOptimizeSession(Handle document, const std::string& path, int flag,
                              Handle* result);
int continueOptimizeSession(Handle document, Handle session, int milliseconds,
                            ErrorCode* error = nullptr);
ErrorCode closeOptimizeSession(Handle document, Handle session);
std::shared_ptr<OptimizeSessionState> borrowOptimizeSession(
    Handle session, Handle document, ErrorCode* error = nullptr);

}  // namespace ksp::runtime
