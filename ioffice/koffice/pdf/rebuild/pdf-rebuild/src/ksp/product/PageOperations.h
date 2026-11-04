#pragma once

#include <cstddef>
#include <cstdint>
#include <string>
#include <vector>

namespace ksp::product {

// Byte buffers must stay alive until composePages returns. Every page index is
// zero-based and explicit; callers generate 0..pageCount-1 to merge all pages.
// Repeated indexes intentionally create repeated pages in the output.
struct PageSelection {
    const uint8_t* pdf = nullptr;
    std::size_t pdfSize = 0;
    std::vector<int> pageIndices;
};

// Creates a new PDF without changing any input. Requires FPDF_InitLibrary() to
// have been called by the owning product runtime. The output is committed only
// after a complete, reopenable PDF has been produced.
bool composePages(const std::vector<PageSelection>& selections,
                  std::vector<uint8_t>* output, std::string* error);

}  // namespace ksp::product
