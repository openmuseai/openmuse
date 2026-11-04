#pragma once

#include <mutex>

namespace ksp::runtime {

// One initialization path shared by JNI, native product ABI and legacy bridge.
void ensurePdfiumInitialized();

// Serialize PDFium work across independent product documents and page tools.
// Callers must not hold a session mutex before taking this mutex.
std::mutex& productPdfiumMutex();

}  // namespace ksp::runtime
