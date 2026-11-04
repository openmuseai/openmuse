#include "../src/ksp/product/PageOperations.h"
#include "../src/ksp/product/ProductBridge.h"
#include "../src/ksp/runtime/PdfiumLifecycle.h"

#include <fpdf_edit.h>
#include <fpdf_save.h>
#include <fpdfview.h>

#include <cmath>
#include <cstdint>
#include <cstdio>
#include <fstream>
#include <iterator>
#include <string>
#include <vector>

namespace {

struct Writer {
    FPDF_FILEWRITE base{};
    std::vector<uint8_t> bytes;

    Writer() {
        base.version = 1;
        base.WriteBlock = write;
    }
    static int write(FPDF_FILEWRITE* base, const void* data, unsigned long size) {
        auto* self = reinterpret_cast<Writer*>(base);
        const auto* first = static_cast<const uint8_t*>(data);
        self->bytes.insert(self->bytes.end(), first, first + size);
        return 1;
    }
};

std::vector<uint8_t> makePdf(const std::vector<double>& widths) {
    FPDF_DOCUMENT doc = FPDF_CreateNewDocument();
    if (!doc) return {};
    for (std::size_t index = 0; index < widths.size(); ++index) {
        FPDF_PAGE page = FPDFPage_New(doc, static_cast<int>(index), widths[index], 400);
        if (!page) {
            FPDF_CloseDocument(doc);
            return {};
        }
        FPDF_PAGEOBJECT mark = FPDFPageObj_CreateNewRect(10, 10, 20, 20);
        const unsigned int shade = static_cast<unsigned int>(widths[index]);
        if (!mark || !FPDFPageObj_SetFillColor(mark, shade, 0, 0, 255) ||
            !FPDFPath_SetDrawMode(mark, FPDF_FILLMODE_WINDING, 0)) {
            if (mark) FPDFPageObj_Destroy(mark);
            FPDF_ClosePage(page);
            FPDF_CloseDocument(doc);
            return {};
        }
        FPDFPage_InsertObject(page, mark);
        if (!FPDFPage_GenerateContent(page)) {
            FPDF_ClosePage(page);
            FPDF_CloseDocument(doc);
            return {};
        }
        FPDF_ClosePage(page);
    }
    Writer writer;
    const bool saved = FPDF_SaveAsCopy(doc, &writer.base, 0);
    FPDF_CloseDocument(doc);
    return saved ? std::move(writer.bytes) : std::vector<uint8_t>{};
}

std::vector<uint8_t> readAsset(const char* name) {
    const std::string source = __FILE__;
    const std::string path = source.substr(0, source.find_last_of('/')) +
                             "/assets/" + name;
    std::ifstream input(path, std::ios::binary);
    return {std::istreambuf_iterator<char>(input), std::istreambuf_iterator<char>()};
}

bool widthsEqual(const std::vector<uint8_t>& pdf,
                 const std::vector<double>& expected) {
    FPDF_DOCUMENT doc = FPDF_LoadMemDocument64(pdf.data(), pdf.size(), nullptr);
    if (!doc) return false;
    bool ok = FPDF_GetPageCount(doc) == static_cast<int>(expected.size());
    for (std::size_t index = 0; ok && index < expected.size(); ++index) {
        FPDF_PAGE page = FPDF_LoadPage(doc, static_cast<int>(index));
        ok = page && std::abs(FPDF_GetPageWidth(page) - expected[index]) < 0.1 &&
             FPDFPage_CountObjects(page) == 1;
        if (ok) {
            unsigned int red = 0, green = 0, blue = 0, alpha = 0;
            ok = FPDFPageObj_GetFillColor(FPDFPage_GetObject(page, 0),
                                          &red, &green, &blue, &alpha) &&
                 red == static_cast<unsigned int>(expected[index]) &&
                 green == 0 && blue == 0 && alpha == 255;
        }
        if (page) FPDF_ClosePage(page);
    }
    FPDF_CloseDocument(doc);
    return ok;
}

bool check(bool condition, const char* message) {
    if (!condition) std::fprintf(stderr, "FAIL: %s\n", message);
    return condition;
}

}  // namespace

int main() {
    ksp::runtime::ensurePdfiumInitialized();
    const auto first = makePdf({101, 102, 103});
    const auto second = makePdf({201, 202});
    std::vector<uint8_t> output;
    std::string error;
    int failures = 0;

    if (!check(!first.empty() && !second.empty(), "synthetic PDF creation")) ++failures;
    const std::vector<ksp::product::PageSelection> extract = {
        {first.data(), first.size(), {2, 0}},
    };
    if (!check(ksp::product::composePages(extract, &output, &error) &&
                   widthsEqual(output, {103, 101}),
               "non-contiguous extraction preserves requested order")) ++failures;

    const std::vector<ksp::product::PageSelection> merge = {
        {first.data(), first.size(), {0, 1, 2}},
        {second.data(), second.size(), {1, 0, 1}},
    };
    if (!check(ksp::product::composePages(merge, &output, &error) &&
                   widthsEqual(output, {101, 102, 103, 202, 201, 202}),
               "merge preserves source order and repeated pages")) ++failures;

    output = {1, 2, 3};
    const std::vector<ksp::product::PageSelection> invalid = {
        {first.data(), first.size(), {0}},
        {second.data(), second.size(), {2}},
    };
    if (!check(!ksp::product::composePages(invalid, &output, &error) &&
                   output.empty() && error.find("page index") != std::string::npos,
               "invalid later source leaves no partial output")) ++failures;
    if (!check(!ksp::product::composePages({}, &output, &error) && output.empty(),
               "empty source list is rejected")) ++failures;
    const std::vector<ksp::product::PageSelection> emptySelection = {
        {first.data(), first.size(), {}},
    };
    if (!check(!ksp::product::composePages(emptySelection, &output, &error) &&
                   output.empty(),
               "empty page selection is rejected")) ++failures;

    const auto withForm = readAsset("../../3rdparty/src/pdfium/testing/resources/combobox_form.pdf");
    const std::vector<ksp::product::PageSelection> complex = {
        {withForm.data(), withForm.size(), {0}},
    };
    const bool complexSucceeded = ksp::product::composePages(complex, &output, &error);
    if (!check(!withForm.empty() &&
                   !complexSucceeded &&
                   output.empty() && error.find("document features") != std::string::npos,
               "form document is rejected instead of silently losing fields")) ++failures;

    const int32_t firstPages[] = {1, 0};
    const int32_t secondPages[] = {0};
    const KspProductPageSelection abiSources[] = {
        {first.data(), first.size(), firstPages, 2},
        {second.data(), second.size(), secondPages, 1},
    };
    KspProductBytes abiOutput{};
    char abiError[128]{};
    const int32_t abiStatus = ksp_product_compose_pages(
        abiSources, 2, &abiOutput, abiError, sizeof(abiError));
    const std::vector<uint8_t> abiPdf = abiOutput.data
        ? std::vector<uint8_t>(abiOutput.data, abiOutput.data + abiOutput.size)
        : std::vector<uint8_t>{};
    if (!check(ksp_product_abi_version() == 2 && abiStatus == 0 &&
                   widthsEqual(abiPdf, {102, 101, 201}),
               "versioned C ABI composes the same PDF")) ++failures;
    ksp_product_free_bytes(&abiOutput);
    if (!check(!abiOutput.data && abiOutput.size == 0,
               "C ABI output release clears ownership")) ++failures;

    const int32_t badPage[] = {9};
    const KspProductPageSelection badSource = {
        first.data(), first.size(), badPage, 1,
    };
    const int32_t failedAbi = ksp_product_compose_pages(
        &badSource, 1, &abiOutput, abiError, sizeof(abiError));
    if (!check(failedAbi < 0 && !abiOutput.data && abiOutput.size == 0 &&
                   abiError[0] != '\0',
               "C ABI failure has no output and explains the error")) ++failures;

    if (!check(ksp_product_page_count(first.data(), first.size()) == 3 &&
                   ksp_product_page_count(nullptr, 0) < 0,
               "C ABI counts pages and rejects invalid input")) ++failures;

    std::printf("PageOperations PASS=%d FAIL=%d\n", 10 - failures, failures);
    return failures == 0 ? 0 : 1;
}
