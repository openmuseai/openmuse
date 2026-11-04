#pragma once

#include <string>
#include <vector>

namespace ksp::runtime {

bool isUnicodeTextBoundary(const std::u16string& text, int position);
int previousUnicodeTextBoundary(const std::u16string& text, int position);
int nextUnicodeTextBoundary(const std::u16string& text, int position);
bool hasUnsupportedSupplementaryInput(const std::u16string& text);

enum class TextDirection : int {
    Ltr = 0,
    Rtl = 1,
};

// Mirrors the native editor ABI: 0 means LTR and 1 means RTL. Neutrals do not
// establish a direction; callers can keep their preceding/base direction.
bool isStrongRtlCodePoint(char32_t codePoint);
bool isStrongLtrCodePoint(char32_t codePoint);
TextDirection baseTextDirection(const std::u16string& text,
                                TextDirection fallback = TextDirection::Ltr);
std::u16string restoreLogicalRtlRuns(const std::u16string& visualText);
bool recoverLogicalRtlRuns(const std::u16string& visualText,
                           std::u16string* logicalText,
                           std::vector<int>* logicalGlyphToVisualGlyph);

struct BidiRun {
    int logicalStart = 0;
    int length = 0;
    TextDirection direction = TextDirection::Ltr;
};

// ICU-backed UBA result supplied by the KSP PDFium extension. Runs are in
// visual order while offsets remain logical UTF-16 offsets.
bool resolveVisualBidiRuns(const std::u16string& logicalText,
                           TextDirection paragraphDirection,
                           std::vector<BidiRun>* result);

}  // namespace ksp::runtime
