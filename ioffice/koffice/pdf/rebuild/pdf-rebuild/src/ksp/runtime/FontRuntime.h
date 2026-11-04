#pragma once

#include <fpdf_edit.h>

#include <cstdint>
#include <string>
#include <vector>

namespace ksp::runtime {

bool loadPrivateFont(const std::string& path);
bool loadPrivateFonts(const std::string& directory);
void addSupportedReplacementFont(const std::string& originalName);
bool setReplacementDisplayFont(const std::string& originalName,
                               const std::string& replacementName);

// Returns an open PDFium font handle owned by the caller. The handle may be
// closed after all TextObjects that reference it have been created.
FPDF_FONT loadReplacementFont(FPDF_DOCUMENT document,
                              const std::string& originalName,
                              uint32_t unicode);

struct ShapedGlyph {
    uint32_t glyphId = 0;
    uint32_t cluster = 0;
    float xAdvance = 0;
    float yAdvance = 0;
    float xOffset = 0;
    float yOffset = 0;
};

struct ShapedTextEncoding {
    struct Placement {
        uint32_t cid = 0;
        uint32_t clusterStart = 0;
        uint32_t clusterEnd = 0;
        float xAdvance = 0;
        float yAdvance = 0;
        float xOffset = 0;
        float yOffset = 0;
        bool logicalCarrier = false;
    };

    FPDF_FONT font = nullptr;
    std::vector<uint32_t> charcodes;
    std::vector<float> positions;
    std::vector<float> glyphPositions;  // x,y pairs in text-object local points.
    std::vector<Placement> placements;
    float advance = 0;
    // FPDFText_SetPositions only carries a one-dimensional baseline offset.
    // Such runs need the persistent shaped-run writer before page mutation.
    bool requiresIndividualPlacement = false;
};

// Calls the fixed-version PDFium HarfBuzz extension. Offsets are logical
// UTF-16 indexes and metrics are normalized to the font em.
bool shapeFontText(FPDF_FONT font, const std::u16string& text, bool rtl,
                   std::vector<ShapedGlyph>* result);

// Builds a custom Identity-H CID font whose CIDs select HarfBuzz glyph IDs
// while ToUnicode keeps the original logical UTF-16 text. `positions` is in
// page points and is suitable for FPDFText_SetPositions(). The caller owns the
// returned font handle.
bool buildShapedTextEncoding(FPDF_DOCUMENT document,
                             const std::string& originalName,
                             const std::u16string& text, bool rtl,
                             float fontSize, ShapedTextEncoding* result);

}  // namespace ksp::runtime
