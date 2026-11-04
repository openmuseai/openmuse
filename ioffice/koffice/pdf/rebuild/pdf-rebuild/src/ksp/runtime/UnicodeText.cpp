#include "UnicodeText.h"

#include <fpdf_edit.h>

#include <algorithm>
#include <dlfcn.h>
#include <vector>

namespace ksp::runtime {
namespace {

struct RawBidiRun {
    int start;
    int length;
    int isRtl;
};

using ResolveVisualRuns = int (*)(const unsigned short*, int, int,
                                  RawBidiRun*, int);

ResolveVisualRuns bidiResolverExtension() {
#ifdef __EMSCRIPTEN__
    return reinterpret_cast<ResolveVisualRuns>(&KSPBidi_GetVisualRuns);
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<ResolveVisualRuns>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPBidi_GetVisualRuns"));
    return function;
#endif
}

bool highSurrogate(char16_t value) { return value >= 0xd800 && value <= 0xdbff; }
bool lowSurrogate(char16_t value) { return value >= 0xdc00 && value <= 0xdfff; }

bool combiningMark(char16_t value) {
    return (value >= 0x0300 && value <= 0x036f) ||
           (value >= 0x0591 && value <= 0x05c7) ||
           (value >= 0x0610 && value <= 0x061a) ||
           (value >= 0x064b && value <= 0x065f) ||
           value == 0x0670 ||
           (value >= 0x06d6 && value <= 0x06ed) ||
           value == 0x0e31 || (value >= 0x0e34 && value <= 0x0e3a) ||
           (value >= 0x0e47 && value <= 0x0e4e) ||
           (value >= 0x1ab0 && value <= 0x1aff) ||
           (value >= 0x1dc0 && value <= 0x1dff) ||
           (value >= 0x20d0 && value <= 0x20ff) ||
           (value >= 0xfe20 && value <= 0xfe2f);
}

bool variationSelector(char16_t value) {
    return value >= 0xfe00 && value <= 0xfe0f;
}

}  // namespace

bool isUnicodeTextBoundary(const std::u16string& text, int position) {
    if (position < 0 || position > static_cast<int>(text.size())) return false;
    if (position == 0 || position == static_cast<int>(text.size())) return true;
    const char16_t previous = text[static_cast<std::size_t>(position - 1)];
    const char16_t current = text[static_cast<std::size_t>(position)];
    if (highSurrogate(previous) && lowSurrogate(current)) return false;
    if (combiningMark(current) || variationSelector(current)) return false;
    if (previous == 0x200d || current == 0x200d) return false;
    return true;
}

int previousUnicodeTextBoundary(const std::u16string& text, int position) {
    int value = std::clamp(position, 0, static_cast<int>(text.size()));
    if (value == 0) return 0;
    do { --value; } while (value > 0 && !isUnicodeTextBoundary(text, value));
    return value;
}

int nextUnicodeTextBoundary(const std::u16string& text, int position) {
    int value = std::clamp(position, 0, static_cast<int>(text.size()));
    if (value == static_cast<int>(text.size())) return value;
    do { ++value; } while (value < static_cast<int>(text.size()) &&
                           !isUnicodeTextBoundary(text, value));
    return value;
}

bool hasUnsupportedSupplementaryInput(const std::u16string& text) {
    for (std::size_t index = 0; index < text.size(); ++index) {
        const char16_t value = text[index];
        if (highSurrogate(value)) {
            // The original Java editor rejects emoji before native OnChars.
            // Until shaping works on Unicode scalars, reject every
            // supplementary-plane scalar rather than corrupting half-pairs.
            return true;
        }
        if (lowSurrogate(value)) return true;
    }
    return false;
}

bool isStrongRtlCodePoint(char32_t value) {
    // Binary editor-direction approximation for Hebrew/Arabic script blocks.
    // This is deliberately not a complete Unicode Bidi_Class database; mixed
    // embeddings, isolates and shaping remain gated by WT6c2b.
    return (value >= 0x0590 && value <= 0x08ff) ||
           (value >= 0xfb1d && value <= 0xfdff) ||
           (value >= 0xfe70 && value <= 0xfeff) ||
           (value >= 0x10800 && value <= 0x10fff) ||
           (value >= 0x1e800 && value <= 0x1edff);
}

bool isStrongLtrCodePoint(char32_t value) {
    // Formatting, whitespace, punctuation and decimal digits are neutral for
    // base-direction discovery. All remaining assigned graphic characters
    // are treated as LTR for the editor's binary ABI.
    if (value == 0 || value == 0x200e || value == 0x200f ||
        (value >= 0x202a && value <= 0x202e) ||
        (value >= 0x2066 && value <= 0x2069)) return false;
    if (isStrongRtlCodePoint(value)) return false;
    if (value <= 0x20 || (value >= 0x7f && value <= 0xa0)) return false;
    if ((value >= U'0' && value <= U'9') ||
        (value >= 0x0660 && value <= 0x0669) ||
        (value >= 0x06f0 && value <= 0x06f9)) return false;
    if ((value >= 0x21 && value <= 0x2f) ||
        (value >= 0x3a && value <= 0x40) ||
        (value >= 0x5b && value <= 0x60) ||
        (value >= 0x7b && value <= 0x7e)) return false;
    if ((value >= 0x2000 && value <= 0x206f) ||
        (value >= 0x2e00 && value <= 0x2e7f)) return false;
    return true;
}

TextDirection baseTextDirection(const std::u16string& text, TextDirection fallback) {
    for (std::size_t index = 0; index < text.size(); ++index) {
        char32_t value = text[index];
        if (highSurrogate(static_cast<char16_t>(value)) && index + 1 < text.size() &&
            lowSurrogate(text[index + 1])) {
            value = 0x10000 + ((value - 0xd800) << 10) + (text[++index] - 0xdc00);
        }
        if (isStrongRtlCodePoint(value)) return TextDirection::Rtl;
        if (isStrongLtrCodePoint(value)) return TextDirection::Ltr;
    }
    return fallback;
}

std::u16string restoreLogicalRtlRuns(const std::u16string& visualText) {
    std::u16string logical;
    std::vector<int> ignored;
    (void)recoverLogicalRtlRuns(visualText, &logical, &ignored);
    return logical;
}

bool recoverLogicalRtlRuns(const std::u16string& visualText,
                           std::u16string* logicalText,
                           std::vector<int>* logicalGlyphToVisualGlyph) {
    if (!logicalText || !logicalGlyphToVisualGlyph) return false;
    struct Cluster {
        std::u16string text;
        std::vector<int> visualGlyphs;
        int kind = 0;
    };
    std::vector<Cluster> clusters;
    int visualGlyph = 0;
    for (int start = 0; start < static_cast<int>(visualText.size());) {
        const int end = nextUnicodeTextBoundary(visualText, start);
        Cluster cluster;
        cluster.text = visualText.substr(start, end - start);
        for (int offset = start; offset < end;) {
            cluster.visualGlyphs.push_back(visualGlyph++);
            const char16_t value = visualText[static_cast<std::size_t>(offset++)];
            if (highSurrogate(value) && offset < end &&
                lowSurrogate(visualText[static_cast<std::size_t>(offset)])) ++offset;
        }
        const char32_t first = cluster.text.empty() ? 0 : cluster.text.front();
        if (first == U'\r' || first == U'\n' || first == 0x2028 || first == 0x2029)
            cluster.kind = -1;
        else if (isStrongRtlCodePoint(first)) cluster.kind = 1;
        else if (isStrongLtrCodePoint(first) ||
                 (first >= U'0' && first <= U'9') ||
                 (first >= 0x0660 && first <= 0x0669) ||
                 (first >= 0x06f0 && first <= 0x06f9)) cluster.kind = -1;
        clusters.push_back(std::move(cluster));
        start = end;
    }
    for (std::size_t start = 0; start < clusters.size();) {
        if (clusters[start].kind != 1) { ++start; continue; }
        std::size_t scan = start + 1;
        std::size_t lastRtl = start;
        while (scan < clusters.size() && clusters[scan].kind != -1) {
            if (clusters[scan].kind == 1) lastRtl = scan;
            ++scan;
        }
        std::reverse(clusters.begin() + static_cast<std::ptrdiff_t>(start),
                     clusters.begin() + static_cast<std::ptrdiff_t>(lastRtl + 1));
        start = scan;
    }
    logicalText->clear();
    logicalGlyphToVisualGlyph->clear();
    for (const Cluster& cluster : clusters) {
        logicalText->append(cluster.text);
        logicalGlyphToVisualGlyph->insert(logicalGlyphToVisualGlyph->end(),
                                          cluster.visualGlyphs.begin(),
                                          cluster.visualGlyphs.end());
    }
    return true;
}

bool resolveVisualBidiRuns(const std::u16string& logicalText,
                           TextDirection paragraphDirection,
                           std::vector<BidiRun>* result) {
    if (!result) return false;
    result->clear();
    if (logicalText.empty()) return true;
    ResolveVisualRuns resolve = bidiResolverExtension();
    if (!resolve) return false;
    const auto* text = reinterpret_cast<const unsigned short*>(logicalText.data());
    const int direction = paragraphDirection == TextDirection::Rtl ? 1 : 0;
    const int count = resolve(text, static_cast<int>(logicalText.size()), direction,
                              nullptr, 0);
    if (count <= 0) return false;
    std::vector<RawBidiRun> raw(static_cast<std::size_t>(count));
    if (resolve(text, static_cast<int>(logicalText.size()), direction,
                raw.data(), count) != count) return false;
    result->reserve(raw.size());
    int covered = 0;
    for (const RawBidiRun& run : raw) {
        if (run.start < 0 || run.length <= 0 ||
            run.start > static_cast<int>(logicalText.size()) - run.length) {
            result->clear();
            return false;
        }
        result->push_back({run.start, run.length,
                           run.isRtl ? TextDirection::Rtl : TextDirection::Ltr});
        covered += run.length;
    }
    if (covered != static_cast<int>(logicalText.size())) {
        result->clear();
        return false;
    }
    return true;
}

}  // namespace ksp::runtime
