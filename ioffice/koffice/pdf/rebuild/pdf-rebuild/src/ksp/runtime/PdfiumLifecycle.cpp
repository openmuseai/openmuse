#include "PdfiumLifecycle.h"

#include <fpdfview.h>

#include <mutex>

namespace ksp::runtime {

void ensurePdfiumInitialized() {
    static std::once_flag once;
    std::call_once(once, [] { FPDF_InitLibrary(); });
}

std::mutex& productPdfiumMutex() {
    static std::mutex mutex;
    return mutex;
}

}  // namespace ksp::runtime
