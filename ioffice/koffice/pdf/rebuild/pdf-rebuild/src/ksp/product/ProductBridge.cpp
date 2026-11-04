#include "ProductBridge.h"

#include "PageOperations.h"
#include "../runtime/PdfiumLifecycle.h"

#include <algorithm>
#include <fpdfview.h>
#include <cstdlib>
#include <cstring>
#include <exception>
#include <limits>
#include <mutex>
#include <string>
#include <vector>

namespace {

void setError(char* error, uint32_t capacity, const std::string& message) {
    if (!error || capacity == 0) return;
    const auto length = std::min<std::size_t>(capacity - 1, message.size());
    std::memcpy(error, message.data(), length);
    error[length] = '\0';
}

}  // namespace

extern "C" {

uint32_t ksp_product_abi_version() { return 2; }

int32_t ksp_product_page_count(const uint8_t* pdf, uint64_t pdfSize) {
    std::lock_guard<std::mutex> lock(ksp::runtime::productPdfiumMutex());
    if (!pdf || pdfSize == 0 || pdfSize > (512ULL << 20) ||
        pdfSize > std::numeric_limits<std::size_t>::max()) return -1001;
    try {
        ksp::runtime::ensurePdfiumInitialized();
        FPDF_DOCUMENT document = FPDF_LoadMemDocument64(
            pdf, static_cast<std::size_t>(pdfSize), nullptr);
        if (!document) return -2;
        const int count = FPDF_GetPageCount(document);
        FPDF_CloseDocument(document);
        return count > 0 ? count : -2;
    } catch (...) {
        return -1;
    }
}

void ksp_product_free_bytes(KspProductBytes* bytes) {
    if (!bytes) return;
    std::free(bytes->data);
    bytes->data = nullptr;
    bytes->size = 0;
}

int32_t ksp_product_compose_pages(const KspProductPageSelection* sources,
                                  uint32_t sourceCount,
                                  KspProductBytes* output,
                                  char* error,
                                  uint32_t errorCapacity) {
    std::lock_guard<std::mutex> lock(ksp::runtime::productPdfiumMutex());
    if (output) {
        output->data = nullptr;
        output->size = 0;
    }
    setError(error, errorCapacity, "");
    if (!output || !sources || sourceCount == 0 || sourceCount > 1000) {
        setError(error, errorCapacity, "invalid source list or output pointer");
        return -1001;
    }
    try {
        std::vector<ksp::product::PageSelection> selections;
        selections.reserve(sourceCount);
        for (uint32_t index = 0; index < sourceCount; ++index) {
            const auto& source = sources[index];
            if (!source.pdf || source.pdf_size == 0 ||
                source.pdf_size > std::numeric_limits<std::size_t>::max() ||
                !source.page_indices || source.page_count == 0 || source.page_count > 10000) {
                setError(error, errorCapacity, "invalid PDF bytes or page selection");
                return -1001;
            }
            ksp::product::PageSelection selection;
            selection.pdf = source.pdf;
            selection.pdfSize = static_cast<std::size_t>(source.pdf_size);
            selection.pageIndices.assign(source.page_indices,
                                         source.page_indices + source.page_count);
            selections.push_back(std::move(selection));
        }
        ksp::runtime::ensurePdfiumInitialized();
        std::vector<uint8_t> bytes;
        std::string message;
        if (!ksp::product::composePages(selections, &bytes, &message)) {
            setError(error, errorCapacity, message);
            return -1;
        }
        auto* allocated = static_cast<uint8_t*>(std::malloc(bytes.size()));
        if (!allocated) {
            setError(error, errorCapacity, "cannot allocate output PDF");
            return -1007;
        }
        std::memcpy(allocated, bytes.data(), bytes.size());
        output->data = allocated;
        output->size = bytes.size();
        return 0;
    } catch (const std::exception& failure) {
        setError(error, errorCapacity, failure.what());
        return -1;
    } catch (...) {
        setError(error, errorCapacity, "unexpected product core failure");
        return -1;
    }
}

}  // extern "C"
