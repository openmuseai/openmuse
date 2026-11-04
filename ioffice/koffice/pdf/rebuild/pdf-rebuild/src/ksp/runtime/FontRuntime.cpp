#include "FontRuntime.h"

#include <algorithm>
#include <cctype>
#include <cmath>
#include <dlfcn.h>
#include <filesystem>
#include <fstream>
#include <memory>
#include <mutex>
#include <sstream>
#include <iomanip>
#include <unordered_map>
#include <unordered_set>
#include <vector>

namespace ksp::runtime {
namespace {

using CanEncodeUnicode = FPDF_BOOL (*)(FPDF_FONT, uint32_t);
struct RawShapedGlyph {
    uint32_t glyphId;
    uint32_t cluster;
    float xAdvance;
    float yAdvance;
    float xOffset;
    float yOffset;
};
using ShapeText = int (*)(FPDF_FONT, const unsigned short*, int, int,
                          RawShapedGlyph*, int);

struct PrivateFont {
    std::string path;
    std::shared_ptr<std::vector<uint8_t>> bytes;
    int pdfiumType = FPDF_FONT_TRUETYPE;
};

struct FontRegistry {
    std::mutex mutex;
    std::vector<PrivateFont> fonts;
    std::unordered_set<std::string> supportedReplacementNames;
    std::unordered_map<std::string, std::string> replacements;
};

FontRegistry& registry() {
    static FontRegistry value;
    return value;
}

CanEncodeUnicode canEncodeUnicodeExtension() {
#ifdef __EMSCRIPTEN__
    return &KSPFont_CanEncodeUnicode;
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<CanEncodeUnicode>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPFont_CanEncodeUnicode"));
    return function;
#endif
}

ShapeText shapeTextExtension() {
#ifdef __EMSCRIPTEN__
    return reinterpret_cast<ShapeText>(&KSPFont_ShapeText);
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<ShapeText>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPFont_ShapeText"));
    return function;
#endif
}

std::string normalized(std::string value) {
    if (value.size() > 7 && value[6] == '+') value.erase(0, 7);
    value.erase(std::remove_if(value.begin(), value.end(), [](unsigned char c) {
        return c == ' ' || c == '-' || c == '_' || c == '+';
    }), value.end());
    std::transform(value.begin(), value.end(), value.begin(), [](unsigned char c) {
        return static_cast<char>(std::tolower(c));
    });
    return value;
}

bool sameFontName(const std::string& lhs, const std::string& rhs) {
    std::string left = normalized(lhs);
    std::string right = normalized(rhs);
    constexpr const char suffix[] = "regular";
    auto stripRegular = [&](std::string* value) {
        if (value->size() >= sizeof(suffix) - 1 &&
            value->compare(value->size() - (sizeof(suffix) - 1),
                           sizeof(suffix) - 1, suffix) == 0) {
            value->resize(value->size() - (sizeof(suffix) - 1));
        }
    };
    stripRegular(&left);
    stripRegular(&right);
    return left == right;
}

std::string baseFontName(FPDF_FONT font) {
    const size_t length = font ? FPDFFont_GetBaseFontName(font, nullptr, 0) : 0;
    if (!length) return {};
    std::vector<char> buffer(length);
    if (FPDFFont_GetBaseFontName(font, buffer.data(), buffer.size()) != length) return {};
    return buffer.data();
}

bool loadFile(const std::filesystem::path& path) {
    std::ifstream stream(path, std::ios::binary | std::ios::ate);
    if (!stream) return false;
    const std::streamsize size = stream.tellg();
    if (size <= 0 || size > 128 * 1024 * 1024) return false;
    stream.seekg(0);
    PrivateFont font;
    font.path = path.string();
    font.bytes = std::make_shared<std::vector<uint8_t>>(static_cast<std::size_t>(size));
    if (!stream.read(reinterpret_cast<char*>(font.bytes->data()), size)) return false;
    if (font.bytes->size() >= 4 && (*font.bytes)[0] == 'O' && (*font.bytes)[1] == 'T' &&
        (*font.bytes)[2] == 'T' && (*font.bytes)[3] == 'O') {
        font.pdfiumType = FPDF_FONT_TYPE1;
    }
    FontRegistry& value = registry();
    std::lock_guard<std::mutex> lock(value.mutex);
    auto existing = std::find_if(value.fonts.begin(), value.fonts.end(),
        [&](const PrivateFont& candidate) { return candidate.path == font.path; });
    if (existing == value.fonts.end()) value.fonts.push_back(std::move(font));
    else {
        existing->bytes = std::move(font.bytes);
        existing->pdfiumType = font.pdfiumType;
    }
    return true;
}

}  // namespace

bool loadPrivateFont(const std::string& path) {
    if (path.empty()) return false;
    std::error_code error;
    const std::filesystem::path file(path);
    return std::filesystem::is_regular_file(file, error) && !error && loadFile(file);
}

bool loadPrivateFonts(const std::string& directory) {
    if (directory.empty()) return false;
    std::error_code error;
    const std::filesystem::path root(directory);
    if (!std::filesystem::is_directory(root, error) || error) return false;
    bool loaded = false;
    for (std::filesystem::directory_iterator it(root, error), end; !error && it != end;
         it.increment(error)) {
        if (!it->is_regular_file(error) || error) continue;
        std::string extension = normalized(it->path().extension().string());
        if (extension != ".ttf" && extension != ".otf") continue;
        loaded = loadFile(it->path()) || loaded;
    }
    return loaded;
}

void addSupportedReplacementFont(const std::string& originalName) {
    if (originalName.empty()) return;
    FontRegistry& value = registry();
    std::lock_guard<std::mutex> lock(value.mutex);
    value.supportedReplacementNames.insert(normalized(originalName));
}

bool setReplacementDisplayFont(const std::string& originalName,
                               const std::string& replacementName) {
    if (originalName.empty() || replacementName.empty()) return false;
    FontRegistry& value = registry();
    std::lock_guard<std::mutex> lock(value.mutex);
    const std::string original = normalized(originalName);
    if (value.supportedReplacementNames.count(original) == 0) return false;
    value.replacements[original] = replacementName;
    return true;
}

FPDF_FONT loadReplacementFont(FPDF_DOCUMENT document,
                              const std::string& originalName,
                              uint32_t unicode) {
    if (!document || originalName.empty()) return nullptr;
    CanEncodeUnicode canEncode = canEncodeUnicodeExtension();
    if (!canEncode) return nullptr;
    std::vector<PrivateFont> fonts;
    std::string replacement;
    {
        FontRegistry& value = registry();
        std::lock_guard<std::mutex> lock(value.mutex);
        const std::string original = normalized(originalName);
        if (value.supportedReplacementNames.count(original) == 0) return nullptr;
        auto mapped = value.replacements.find(original);
        if (mapped == value.replacements.end()) return nullptr;
        replacement = normalized(mapped->second);
        fonts = value.fonts;
    }
    for (const PrivateFont& candidate : fonts) {
        if (!candidate.bytes || candidate.bytes->empty()) continue;
        FPDF_FONT font = FPDFText_LoadFont(document, candidate.bytes->data(),
                                           static_cast<uint32_t>(candidate.bytes->size()),
                                           candidate.pdfiumType, true);
        if (!font) continue;
        const bool nameMatches = sameFontName(baseFontName(font), replacement);
        const bool encodes = nameMatches && canEncode(font, unicode);
        if (encodes) return font;
        FPDFFont_Close(font);
    }
    return nullptr;
}

bool findReplacementFontData(const std::string& originalName,
                             std::vector<PrivateFont>* fonts,
                             std::string* replacement) {
    if (!fonts || !replacement) return false;
    FontRegistry& value = registry();
    std::lock_guard<std::mutex> lock(value.mutex);
    const std::string original = normalized(originalName);
    if (value.supportedReplacementNames.count(original) == 0) return false;
    auto mapped = value.replacements.find(original);
    if (mapped == value.replacements.end()) return false;
    *replacement = normalized(mapped->second);
    *fonts = value.fonts;
    return true;
}

std::string hex4(uint32_t value) {
    std::ostringstream stream;
    stream << std::uppercase << std::hex << std::setfill('0')
           << std::setw(4) << (value & 0xffffu);
    return stream.str();
}

bool shapeFontText(FPDF_FONT font, const std::u16string& text, bool rtl,
                   std::vector<ShapedGlyph>* result) {
    if (!font || text.empty() || !result) return false;
    ShapeText shape = shapeTextExtension();
    if (!shape) return false;
    const auto* value = reinterpret_cast<const unsigned short*>(text.data());
    const int count = shape(font, value, static_cast<int>(text.size()), rtl ? 1 : 0,
                            nullptr, 0);
    if (count <= 0 || count > 65535) return false;
    std::vector<RawShapedGlyph> raw(static_cast<std::size_t>(count));
    if (shape(font, value, static_cast<int>(text.size()), rtl ? 1 : 0,
              raw.data(), count) != count) return false;
    result->clear();
    result->reserve(raw.size());
    for (const auto& glyph : raw) {
        if (glyph.cluster >= text.size()) {
            result->clear();
            return false;
        }
        result->push_back({glyph.glyphId, glyph.cluster, glyph.xAdvance,
                           glyph.yAdvance, glyph.xOffset, glyph.yOffset});
    }
    return true;
}

bool buildShapedTextEncoding(FPDF_DOCUMENT document,
                             const std::string& originalName,
                             const std::u16string& text, bool rtl,
                             float fontSize, ShapedTextEncoding* result) {
    if (!document || text.empty() || fontSize <= 0 || !result) return false;
    std::vector<PrivateFont> fonts;
    std::string replacement;
    if (!findReplacementFontData(originalName, &fonts, &replacement)) return false;
    CanEncodeUnicode canEncode = canEncodeUnicodeExtension();
    if (!canEncode) return false;
    for (const PrivateFont& candidate : fonts) {
        if (!candidate.bytes || candidate.bytes->empty()) continue;
        FPDF_FONT shapingFont = FPDFText_LoadFont(
            document, candidate.bytes->data(),
            static_cast<uint32_t>(candidate.bytes->size()),
            candidate.pdfiumType, true);
        if (!shapingFont) continue;
        const bool usable = sameFontName(baseFontName(shapingFont), replacement) &&
            canEncode(shapingFont, static_cast<uint32_t>(text.front()));
        std::vector<ShapedGlyph> glyphs;
        const bool shaped = usable && shapeFontText(shapingFont, text, rtl, &glyphs);
        FPDFFont_Close(shapingFont);
        if (!shaped || glyphs.empty() || glyphs.size() > 65534) continue;

        std::vector<ShapedGlyph> logical = glyphs;
        std::stable_sort(logical.begin(), logical.end(), [](const auto& lhs, const auto& rhs) {
            return lhs.cluster < rhs.cluster;
        });
        std::vector<bool> carrier(logical.size(), false);
        std::vector<uint32_t> clusterEnds(logical.size(), 0);
        std::size_t clusterCount = 0;
        bool requiresIndividualPlacement = false;
        for (std::size_t begin = 0; begin < logical.size();) {
            std::size_t end = begin + 1;
            while (end < logical.size() &&
                   logical[end].cluster == logical[begin].cluster) ++end;
            const uint32_t clusterEnd = end < logical.size()
                ? logical[end].cluster : static_cast<uint32_t>(text.size());
            if (logical[begin].cluster >= clusterEnd || clusterEnd > text.size())
                return false;
            carrier[begin] = true;
            ++clusterCount;
            requiresIndividualPlacement |= end - begin > 1;
            for (std::size_t index = begin; index < end; ++index) {
                clusterEnds[index] = clusterEnd;
                constexpr float kOffsetEpsilon = 0.000001f;
                requiresIndividualPlacement |=
                    std::abs(logical[index].xOffset) > kOffsetEpsilon ||
                    std::abs(logical[index].yOffset) > kOffsetEpsilon ||
                    std::abs(logical[index].yAdvance) > kOffsetEpsilon;
            }
            begin = end;
        }
        std::vector<uint8_t> cidToGid((logical.size() + 1) * 2, 0);
        std::ostringstream cmap;
        cmap << "/CIDInit /ProcSet findresource begin\n12 dict begin\n"
             << "begincmap\n/CIDSystemInfo << /Registry (Adobe) /Ordering (Identity) /Supplement 0 >> def\n"
             << "/CMapName /Adobe-Identity-H def\n/CMapType 2 def\n"
             << "1 begincodespacerange\n<0000> <FFFF>\nendcodespacerange\n"
             << clusterCount << " beginbfchar\n";
        for (std::size_t index = 0; index < logical.size(); ++index) {
            const uint32_t cid = static_cast<uint32_t>(index + 1);
            cidToGid[cid * 2] = static_cast<uint8_t>(logical[index].glyphId >> 8);
            cidToGid[cid * 2 + 1] = static_cast<uint8_t>(logical[index].glyphId);
            if (!carrier[index]) continue;
            const uint32_t begin = logical[index].cluster;
            const uint32_t end = clusterEnds[index];
            cmap << '<' << hex4(cid) << "> <";
            for (uint32_t offset = begin; offset < end; ++offset)
                cmap << hex4(static_cast<uint32_t>(text[offset]));
            cmap << ">\n";
        }
        cmap << "endbfchar\nendcmap\nCMapName currentdict /CMap defineresource pop\nend\nend\n";
        const std::string cmapValue = cmap.str();
        FPDF_FONT custom = FPDFText_LoadCidType2Font(
            document, candidate.bytes->data(),
            static_cast<uint32_t>(candidate.bytes->size()), cmapValue.c_str(),
            cidToGid.data(), static_cast<uint32_t>(cidToGid.size()));
        if (!custom) continue;
        result->font = custom;
        result->charcodes.resize(logical.size());
        result->positions.clear();
        result->positions.reserve(logical.size() > 0 ? logical.size() - 1 : 0);
        result->glyphPositions.clear();
        result->glyphPositions.reserve(logical.size() * 2);
        result->placements.clear();
        result->placements.reserve(logical.size());
        result->requiresIndividualPlacement = requiresIndividualPlacement;
        float position = 0;
        float totalAdvance = 0;
        for (std::size_t index = 0; index < logical.size(); ++index) {
            const uint32_t cid = static_cast<uint32_t>(index + 1);
            result->charcodes[index] = cid;
            const float step = logical[index].xAdvance * fontSize;
            totalAdvance += step;
            if (index > 0) {
                position += rtl ? -step : logical[index - 1].xAdvance * fontSize;
                result->positions.push_back(position);
            }
            result->glyphPositions.push_back(
                position + logical[index].xOffset * fontSize);
            result->glyphPositions.push_back(logical[index].yOffset * fontSize);
            result->placements.push_back({
                cid, logical[index].cluster, clusterEnds[index],
                logical[index].xAdvance, logical[index].yAdvance,
                logical[index].xOffset, logical[index].yOffset, carrier[index]});
        }
        result->advance = totalAdvance;
        return true;
    }
    return false;
}

}  // namespace ksp::runtime
