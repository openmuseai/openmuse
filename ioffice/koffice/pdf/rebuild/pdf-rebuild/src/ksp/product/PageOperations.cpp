#include "PageOperations.h"

#include <fpdf_edit.h>
#include <fpdf_annot.h>
#include <fpdf_attachment.h>
#include <fpdf_catalog.h>
#include <fpdf_doc.h>
#include <fpdf_formfill.h>
#include <fpdf_ppo.h>
#include <fpdf_save.h>
#include <fpdf_signature.h>
#include <fpdfview.h>

#include <memory>
#include <new>
#include <type_traits>

namespace ksp::product {
namespace {

constexpr std::size_t kMaxInputBytes = 512ULL * 1024 * 1024;
constexpr std::size_t kMaxOutputBytes = 512ULL * 1024 * 1024;
constexpr std::size_t kMaxPages = 10000;

struct DocumentCloser {
    void operator()(FPDF_DOCUMENT document) const {
        if (document) FPDF_CloseDocument(document);
    }
};
using Document = std::unique_ptr<std::remove_pointer_t<FPDF_DOCUMENT>, DocumentCloser>;

struct MemoryWriter {
    FPDF_FILEWRITE base{};
    std::vector<uint8_t> bytes;
    bool limitExceeded = false;

    MemoryWriter() {
        base.version = 1;
        base.WriteBlock = write;
    }

    static int write(FPDF_FILEWRITE* base, const void* data, unsigned long size) {
        auto* writer = reinterpret_cast<MemoryWriter*>(base);
        if (!data || size > kMaxOutputBytes - writer->bytes.size()) {
            writer->limitExceeded = true;
            return 0;
        }
        try {
            const auto* first = static_cast<const uint8_t*>(data);
            writer->bytes.insert(writer->bytes.end(), first, first + size);
        } catch (const std::bad_alloc&) {
            writer->limitExceeded = true;
            return 0;
        }
        return 1;
    }
};

bool fail(std::vector<uint8_t>* output, std::string* error,
          const std::string& message) {
    if (output) output->clear();
    if (error) *error = message;
    return false;
}

}  // namespace

bool composePages(const std::vector<PageSelection>& selections,
                  std::vector<uint8_t>* output, std::string* error) {
    if (!output || !error) return false;
    output->clear();
    error->clear();
    if (selections.empty()) return fail(output, error, "no source documents");

    std::size_t totalPages = 0;
    for (const auto& source : selections) {
        if (!source.pdf || source.pdfSize == 0 || source.pdfSize > kMaxInputBytes)
            return fail(output, error, "source PDF is empty or exceeds the input limit");
        if (source.pageIndices.empty())
            return fail(output, error, "source page selection is empty");
        if (source.pageIndices.size() > kMaxPages - totalPages)
            return fail(output, error, "too many output pages");
        totalPages += source.pageIndices.size();
    }

    Document result(FPDF_CreateNewDocument());
    if (!result) return fail(output, error, "cannot create output PDF");

    int insertAt = 0;
    for (std::size_t sourceIndex = 0; sourceIndex < selections.size(); ++sourceIndex) {
        const auto& source = selections[sourceIndex];
        Document input(FPDF_LoadMemDocument64(source.pdf, source.pdfSize, nullptr));
        if (!input)
            return fail(output, error, "cannot open source PDF " + std::to_string(sourceIndex + 1));
        const int count = FPDF_GetPageCount(input.get());
        if (count <= 0)
            return fail(output, error, "source PDF " + std::to_string(sourceIndex + 1) + " has no pages");
        if (FPDF_GetSignatureCount(input.get()) > 0 ||
            FPDF_GetFormType(input.get()) != FORMTYPE_NONE ||
            FPDFBookmark_GetFirstChild(input.get(), nullptr) ||
            FPDFCatalog_IsTagged(input.get()) ||
            FPDFDoc_GetAttachmentCount(input.get()) > 0)
            return fail(output, error, "source PDF " + std::to_string(sourceIndex + 1) +
                                       " has document features that page composition cannot preserve");
        for (int page : source.pageIndices) {
            if (page < 0 || page >= count)
                return fail(output, error, "page index outside source PDF " + std::to_string(sourceIndex + 1));
            FPDF_PAGE sourcePage = FPDF_LoadPage(input.get(), page);
            if (!sourcePage)
                return fail(output, error, "cannot load selected page from source PDF " +
                                           std::to_string(sourceIndex + 1));
            const int annotations = FPDFPage_GetAnnotCount(sourcePage);
            FPDF_ClosePage(sourcePage);
            if (annotations != 0)
                return fail(output, error, "source PDF " + std::to_string(sourceIndex + 1) +
                                           " has page annotations that composition cannot preserve");
        }
        if (!FPDF_ImportPagesByIndex(result.get(), input.get(),
                                     source.pageIndices.data(),
                                     static_cast<unsigned long>(source.pageIndices.size()),
                                     insertAt))
            return fail(output, error, "PDFium could not import pages from source PDF " +
                                       std::to_string(sourceIndex + 1));
        insertAt += static_cast<int>(source.pageIndices.size());
    }

    MemoryWriter writer;
    if (!FPDF_SaveAsCopy(result.get(), &writer.base, 0))
        return fail(output, error, writer.limitExceeded ? "output PDF exceeds the memory limit" :
                                                         "PDFium could not save the composed PDF");
    Document reopened(FPDF_LoadMemDocument64(writer.bytes.data(), writer.bytes.size(), nullptr));
    if (!reopened || FPDF_GetPageCount(reopened.get()) != insertAt)
        return fail(output, error, "composed PDF did not reopen with the expected page count");
    *output = std::move(writer.bytes);
    return true;
}

}  // namespace ksp::product
