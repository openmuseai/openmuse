#include "ObjectRuntime.h"
#include "FontRuntime.h"
#include "UnicodeText.h"

#include <fpdf_edit.h>
#include <fpdf_text.h>

#include <algorithm>
#include <atomic>
#include <cmath>
#include <dlfcn.h>
#include <functional>
#include <limits>
#include <unordered_set>

namespace ksp::runtime {
namespace {

using MarkFormChildDirty = FPDF_BOOL (*)(FPDF_PAGEOBJECT, FPDF_PAGEOBJECT);
using InsertFormObject = FPDF_BOOL (*)(FPDF_PAGEOBJECT, FPDF_PAGEOBJECT,
                                      unsigned long);
using FlushFormContents = FPDF_BOOL (*)(FPDF_PAGEOBJECT);
using SetTextObjectFont = FPDF_BOOL (*)(FPDF_PAGEOBJECT, FPDF_FONT);
using SetTextObjectGlyphPositions = FPDF_BOOL (*)(FPDF_PAGEOBJECT, const float*,
                                                 size_t);
using SetTextObjectActualText = FPDF_BOOL (*)(FPDF_PAGEOBJECT, FPDF_WIDESTRING,
                                             size_t, uint32_t);
using CanEncodeUnicode = FPDF_BOOL (*)(FPDF_FONT, uint32_t);

MarkFormChildDirty markFormChildDirtyExtension() {
#ifdef __EMSCRIPTEN__
    return &KSPFormObj_MarkChildDirty;
#else
    // Android does not retain relocations for an unresolved weak symbol.
    // Resolve lazily from the already-loaded libpdfium instead, while keeping
    // older prebuilt PDFium builds safely unsupported.
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<MarkFormChildDirty>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPFormObj_MarkChildDirty"));
    return function;
#endif
}

InsertFormObject insertFormObjectExtension() {
#ifdef __EMSCRIPTEN__
    return &KSPFormObj_InsertObjectAtIndex;
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<InsertFormObject>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPFormObj_InsertObjectAtIndex"));
    return function;
#endif
}

FlushFormContents flushFormContentsExtension() {
#ifdef __EMSCRIPTEN__
    return &KSPFormObj_FlushContents;
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<FlushFormContents>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPFormObj_FlushContents"));
    return function;
#endif
}

SetTextObjectFont setTextObjectFontExtension() {
#ifdef __EMSCRIPTEN__
    return &KSPTextObj_SetFont;
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<SetTextObjectFont>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPTextObj_SetFont"));
    return function;
#endif
}

SetTextObjectGlyphPositions setTextObjectGlyphPositionsExtension() {
#ifdef __EMSCRIPTEN__
    return &KSPTextObj_SetGlyphPositions;
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<SetTextObjectGlyphPositions>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPTextObj_SetGlyphPositions"));
    return function;
#endif
}

SetTextObjectActualText setTextObjectActualTextExtension() {
#ifdef __EMSCRIPTEN__
    return &KSPTextObj_SetActualText;
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<SetTextObjectActualText>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPTextObj_SetActualText"));
    return function;
#endif
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

struct Matrix {
    float a = 1, b = 0, c = 0, d = 1, e = 0, f = 0;
};

struct Bounds {
    float left = 0, bottom = 0, right = 0, top = 0;
};

// PDFTextEditor.BORDER_CONTENT_DISTANCE in the original Java host is 5.5pt.
// The host expands editor interaction geometry around the native body rect.
// Apply the same page-space fallback here because the rebuild's compatibility
// OnLButtonDown entry point performs both the Java-level and native hit steps.
constexpr float kEditorBorderContentDistance = 5.5f;

struct FocusedGlyph {
    int utf16Start = 0;
    int utf16End = 0;
    float originX = 0, originY = 0;
    float endX = 0, endY = 0;
    float bottomOffset = 0, topOffset = 0;
    float unitX = 1, unitY = 0;
};

struct DomainUndoItem {
    FPDF_PAGEOBJECT object = nullptr;
    std::vector<FPDF_PAGEOBJECT> formAncestors;
    int pageObjectIndex = -1;
    int formObjectIndex = -1;
    std::u16string beforeText;
    std::u16string afterText;
    bool hasBeforeLogicalOverride = false;
    std::u16string beforeLogicalOverride;
    FPDF_FONT beforeFont = nullptr;
    FPDF_FONT afterFont = nullptr;
    bool ownsAfterFont = false;
    std::vector<uint32_t> afterCharcodes;
    std::vector<float> afterPositions;
    std::vector<float> afterGlyphPositions;
    uint32_t afterShapedRunId = 0;
    bool beforePresent = true;
    bool afterPresent = true;
    bool afterDeleted = false;
    bool ownsDetachedObject = false;
    bool hasMatrix = false;
    FS_MATRIX beforeMatrix{1, 0, 0, 1, 0, 0};
    FS_MATRIX afterMatrix{1, 0, 0, 1, 0, 0};
};

struct EditUndoState {
    enum class Kind {
        Text, DomainText, ObjectPresence, FontSize, FillColor, Font,
        LayoutMatrices, ImageMatrix
    };
    Kind kind = Kind::Text;
    std::shared_ptr<PageState> page;
    Handle pageHandle = 0;
    FPDF_PAGEOBJECT object = nullptr;
    std::vector<FPDF_PAGEOBJECT> formAncestors;
    int pageObjectIndex = -1;
    std::u16string beforeText;
    std::u16string afterText;
    int beforeStart = 0, beforeEnd = 0;
    int afterStart = 0, afterEnd = 0;
    bool beforePresent = true;
    bool afterPresent = true;
    bool afterDeleted = false;
    bool currentlyAfter = true;
    bool ownsDetachedObject = false;
    int objectType = FPDF_PAGEOBJ_UNKNOWN;
    float beforeFontSize = 0;
    float afterFontSize = 0;
    uint32_t beforeColor = 0;
    uint32_t afterColor = 0;
    FPDF_FONT beforeFont = nullptr;
    FPDF_FONT afterFont = nullptr;
    bool ownsTextAfterFont = false;
    std::vector<FPDF_PAGEOBJECT> alignedObjects;
    std::vector<std::vector<FPDF_PAGEOBJECT>> alignedFormAncestors;
    std::vector<FS_MATRIX> beforeMatrices;
    std::vector<FS_MATRIX> afterMatrices;
    std::vector<DomainUndoItem> domainItems;
    FPDF_PAGEOBJECT domainFocusObject = nullptr;
    bool beforeHasLayoutBottom = false;
    bool afterHasLayoutBottom = false;
    float beforeLayoutBottom = 0;
    float afterLayoutBottom = 0;
    bool hasDamage = false;
    Bounds damage{0, 0, 0, 0};

    ~EditUndoState() {
        if (ownsDetachedObject && object) FPDFPageObj_Destroy(object);
        if (ownsTextAfterFont && afterFont) FPDFFont_Close(afterFont);
        for (auto& item : domainItems) {
            if (item.ownsDetachedObject && item.object) FPDFPageObj_Destroy(item.object);
            if (item.ownsAfterFont && item.afterFont) FPDFFont_Close(item.afterFont);
        }
    }
};

bool setObjectText(FPDF_PAGEOBJECT object, const std::u16string& text) {
    std::vector<FPDF_WCHAR> wide;
    wide.reserve(text.size() + 1);
    for (char16_t value : text) wide.push_back(static_cast<FPDF_WCHAR>(value));
    wide.push_back(0);
    return FPDFText_SetText(object, wide.data()) != 0;
}

bool applyDomainItemText(DomainUndoItem& item, bool after) {
    // Preserve the original encoded glyph stream for untouched objects. Many
    // embedded subset fonts decode existing charcodes to Unicode but cannot
    // encode that Unicode losslessly again (TOC leaders are a common case).
    if (item.beforePresent && item.afterPresent && !item.afterFont &&
        item.beforeText == item.afterText) {
        return true;
    }
    if (after && item.afterFont && !item.afterCharcodes.empty()) {
        SetTextObjectFont setFont = setTextObjectFontExtension();
        if (!setFont || !setFont(item.object, item.afterFont) ||
            !FPDFText_SetCharcodes(item.object, item.afterCharcodes.data(),
                                   item.afterCharcodes.size())) return false;
        if (!item.afterGlyphPositions.empty()) {
            SetTextObjectGlyphPositions setGlyphPositions =
                setTextObjectGlyphPositionsExtension();
            if (!setGlyphPositions ||
                !setGlyphPositions(item.object, item.afterGlyphPositions.data(),
                                   item.afterCharcodes.size())) return false;
        } else if (item.afterCharcodes.size() > 1 &&
                   !FPDFText_SetPositions(item.object, item.afterPositions.data(),
                                          item.afterPositions.size())) {
            return false;
        }
        SetTextObjectActualText setActualText = setTextObjectActualTextExtension();
        return setActualText && setActualText(
            item.object,
            reinterpret_cast<const unsigned short*>(item.afterText.data()),
            item.afterText.size(), item.afterShapedRunId);
    }
    SetTextObjectActualText setActualText = setTextObjectActualTextExtension();
    if (setActualText) (void)setActualText(item.object, nullptr, 0, 0);
    if (!after && item.beforeFont) {
        SetTextObjectFont setFont = setTextObjectFontExtension();
        if (!setFont || !setFont(item.object, item.beforeFont)) return false;
    }
    return setObjectText(item.object, after ? item.afterText : item.beforeText);
}

uint32_t nextShapedRunId() {
    static std::atomic<uint32_t> next{1};
    uint32_t value = next.fetch_add(1, std::memory_order_relaxed) & 0x7fffffffu;
    if (value == 0) value = next.fetch_add(1, std::memory_order_relaxed) & 0x7fffffffu;
    return value == 0 ? 1 : value;
}

bool markFormWriteback(FPDF_PAGEOBJECT object,
                       const std::vector<FPDF_PAGEOBJECT>& ancestors) {
    if (ancestors.empty()) return true;
    MarkFormChildDirty markDirty = markFormChildDirtyExtension();
    if (!markDirty) return false;
    FPDF_PAGEOBJECT child = object;
    for (auto item = ancestors.rbegin(); item != ancestors.rend(); ++item) {
        if (!markDirty(*item, child)) return false;
        child = *item;
    }
    return true;
}

int formChildIndex(FPDF_PAGEOBJECT form, FPDF_PAGEOBJECT child) {
    if (!form || !child) return -1;
    const int count = FPDFFormObj_CountObjects(form);
    for (int index = 0; index < count; ++index) {
        if (FPDFFormObj_GetObject(form, static_cast<unsigned long>(index)) == child)
            return index;
    }
    return -1;
}

bool propagateFormWriteback(
        const std::vector<FPDF_PAGEOBJECT>& ancestors) {
    MarkFormChildDirty markDirty = markFormChildDirtyExtension();
    if (ancestors.size() < 2) return true;
    if (!markDirty) return false;
    for (std::size_t index = ancestors.size() - 1; index > 0; --index) {
        if (!markDirty(ancestors[index - 1], ancestors[index])) return false;
    }
    return true;
}

bool removeDomainObject(DomainUndoItem& item, PageState& state) {
    if (item.formAncestors.empty())
        return FPDFPage_RemoveObject(state.pdf, item.object) != 0;
    FlushFormContents flush = flushFormContentsExtension();
    FPDF_PAGEOBJECT container = item.formAncestors.back();
    return flush && FPDFFormObj_RemoveObject(container, item.object) &&
           flush(container) && propagateFormWriteback(item.formAncestors);
}

bool insertDomainObject(DomainUndoItem& item, PageState& state) {
    if (item.formAncestors.empty()) {
        const int count = FPDFPage_CountObjects(state.pdf);
        const std::size_t index = static_cast<std::size_t>(
            std::clamp(item.pageObjectIndex, 0, std::max(count, 0)));
        return FPDFPage_InsertObjectAtIndex(state.pdf, item.object, index) != 0;
    }
    InsertFormObject insert = insertFormObjectExtension();
    FPDF_PAGEOBJECT container = item.formAncestors.back();
    const int count = FPDFFormObj_CountObjects(container);
    const unsigned long index = static_cast<unsigned long>(
        std::clamp(item.formObjectIndex, 0, std::max(count, 0)));
    return insert && insert(container, item.object, index) &&
           propagateFormWriteback(item.formAncestors);
}

bool findFormAncestors(FPDF_PAGE page, FPDF_PAGEOBJECT target,
                       std::vector<FPDF_PAGEOBJECT>* ancestors) {
    if (!page || !target || !ancestors) return false;
    auto visit = [&](auto&& self, FPDF_PAGEOBJECT object) -> bool {
        if (object == target) return true;
        if (FPDFPageObj_GetType(object) != FPDF_PAGEOBJ_FORM) return false;
        ancestors->push_back(object);
        const int count = FPDFFormObj_CountObjects(object);
        for (int index = 0; index < count; ++index) {
            FPDF_PAGEOBJECT child = FPDFFormObj_GetObject(object, index);
            if (child && self(self, child)) return true;
        }
        ancestors->pop_back();
        return false;
    };
    const int count = FPDFPage_CountObjects(page);
    for (int index = 0; index < count; ++index) {
        FPDF_PAGEOBJECT object = FPDFPage_GetObject(page, index);
        if (object && visit(visit, object)) return true;
    }
    ancestors->clear();
    return false;
}

Matrix fromPdfium(FPDF_PAGEOBJECT object) {
    FS_MATRIX value{1, 0, 0, 1, 0, 0};
    (void)FPDFPageObj_GetMatrix(object, &value);
    return {value.a, value.b, value.c, value.d, value.e, value.f};
}

Matrix multiply(const Matrix& parent, const Matrix& local) {
    return {
        parent.a * local.a + parent.c * local.b,
        parent.b * local.a + parent.d * local.b,
        parent.a * local.c + parent.c * local.d,
        parent.b * local.c + parent.d * local.d,
        parent.a * local.e + parent.c * local.f + parent.e,
        parent.b * local.e + parent.d * local.f + parent.f};
}

bool inverse(const Matrix& value, Matrix* result) {
    const float determinant = value.a * value.d - value.b * value.c;
    if (!result || std::fabs(determinant) < 1e-8f) return false;
    result->a = value.d / determinant;
    result->b = -value.b / determinant;
    result->c = -value.c / determinant;
    result->d = value.a / determinant;
    result->e = (value.c * value.f - value.d * value.e) / determinant;
    result->f = (value.b * value.e - value.a * value.f) / determinant;
    return true;
}

void transform(const Matrix& matrix, float x, float y, float* tx, float* ty) {
    *tx = matrix.a * x + matrix.c * y + matrix.e;
    *ty = matrix.b * x + matrix.d * y + matrix.f;
}

void storeMatrix(const Matrix& matrix, float (&out)[9]) {
    out[0] = matrix.a; out[1] = matrix.c; out[2] = matrix.e;
    out[3] = matrix.b; out[4] = matrix.d; out[5] = matrix.f;
    out[6] = 0; out[7] = 0; out[8] = 1;
}

bool buildFocusedGlyphsLocked(PageState& state, EditableObjectRecord* record,
                              std::vector<FocusedGlyph>* result) {
    if (!record || !result) return false;
    result->clear();
    FPDF_TEXTPAGE textPage = FPDFText_LoadPage(state.pdf);
    if (!textPage) return false;
    const int count = FPDFText_CountChars(textPage);
    int utf16 = 0;
    std::u16string visualText;
    for (int i = 0; i < count; ++i) {
        if (FPDFText_GetTextObject(textPage, i) != record->object) continue;
        double left = 0, right = 0, bottom = 0, top = 0;
        double ox = 0, oy = 0;
        FS_MATRIX matrix{1, 0, 0, 1, 0, 0};
        if (!FPDFText_GetCharBox(textPage, i, &left, &right, &bottom, &top) ||
            !FPDFText_GetCharOrigin(textPage, i, &ox, &oy)) continue;
        (void)FPDFText_GetMatrix(textPage, i, &matrix);
        float ux = matrix.a, uy = matrix.b;
        const float length = std::hypot(ux, uy);
        if (length < 1e-6f) { ux = 1; uy = 0; }
        else { ux /= length; uy /= length; }
        const float nx = -uy, ny = ux;
        const float cornersX[] = {static_cast<float>(left), static_cast<float>(right),
                                  static_cast<float>(right), static_cast<float>(left)};
        const float cornersY[] = {static_cast<float>(bottom), static_cast<float>(bottom),
                                  static_cast<float>(top), static_cast<float>(top)};
        float minAlong = INFINITY, maxAlong = -INFINITY;
        float minNormal = INFINITY, maxNormal = -INFINITY;
        for (int corner = 0; corner < 4; ++corner) {
            const float dx = cornersX[corner] - static_cast<float>(ox);
            const float dy = cornersY[corner] - static_cast<float>(oy);
            minAlong = std::min(minAlong, dx * ux + dy * uy);
            maxAlong = std::max(maxAlong, dx * ux + dy * uy);
            minNormal = std::min(minNormal, dx * nx + dy * ny);
            maxNormal = std::max(maxNormal, dx * nx + dy * ny);
        }
        const unsigned int codePoint = FPDFText_GetUnicode(textPage, i);
        const int units = codePoint > 0xFFFF ? 2 : 1;
        if (codePoint <= 0xffff) {
            visualText.push_back(static_cast<char16_t>(codePoint));
        } else {
            const unsigned int scalar = codePoint - 0x10000;
            visualText.push_back(static_cast<char16_t>(0xd800 + (scalar >> 10)));
            visualText.push_back(static_cast<char16_t>(0xdc00 + (scalar & 0x3ff)));
        }
        FocusedGlyph glyph;
        glyph.utf16Start = utf16;
        glyph.utf16End = utf16 + units;
        glyph.originX = static_cast<float>(ox);
        glyph.originY = static_cast<float>(oy);
        glyph.endX = glyph.originX + std::max(0.0f, maxAlong) * ux;
        glyph.endY = glyph.originY + std::max(0.0f, maxAlong) * uy;
        glyph.bottomOffset = minNormal;
        glyph.topOffset = maxNormal;
        glyph.unitX = ux;
        glyph.unitY = uy;
        result->push_back(glyph);
        utf16 += units;
    }
    FPDFText_ClosePage(textPage);
    if (result->empty()) return false;
    std::u16string logicalText;
    std::vector<int> logicalToVisual;
    const auto shapedLogical = state.shapedLogicalText.find(record->object);
    if (shapedLogical != state.shapedLogicalText.end()) {
        // Custom shaped CIDs and their absolute positions are emitted in
        // logical order, but TextPage enumerates their negative-positioned
        // glyphs visually. Sort origins from the RTL edge back into logical
        // order instead of applying the legacy content-string heuristic.
        logicalText = shapedLogical->second;
        std::sort(result->begin(), result->end(), [](const auto& lhs, const auto& rhs) {
            return lhs.originX * lhs.unitX + lhs.originY * lhs.unitY >
                   rhs.originX * rhs.unitX + rhs.originY * rhs.unitY;
        });
        int logicalOffset = 0;
        for (FocusedGlyph& glyph : *result) {
            glyph.utf16Start = logicalOffset;
            glyph.utf16End = ++logicalOffset;
        }
    } else if (recoverLogicalRtlRuns(visualText, &logicalText, &logicalToVisual) &&
        logicalToVisual.size() == result->size()) {
        const std::vector<FocusedGlyph> visualGlyphs = *result;
        for (std::size_t index = 0; index < logicalToVisual.size(); ++index) {
            (*result)[index] = visualGlyphs[static_cast<std::size_t>(
                logicalToVisual[index])];
        }
        int logicalOffset = 0;
        for (FocusedGlyph& glyph : *result) {
            const int units = glyph.utf16End - glyph.utf16Start;
            glyph.utf16Start = logicalOffset;
            glyph.utf16End = logicalOffset + units;
            logicalOffset += units;
        }
    } else {
        logicalText = visualText;
    }

    std::vector<BidiRun> bidiRuns;
    if (!resolveVisualBidiRuns(logicalText, baseTextDirection(logicalText),
                               &bidiRuns)) {
        bidiRuns.push_back({0, static_cast<int>(logicalText.size()),
                            baseTextDirection(logicalText)});
    }
    // Advances are resolved within each UBA run. A logical neighbour from a
    // different visual run is not a valid caret edge at a bidi boundary.
    for (const BidiRun& run : bidiRuns) {
        std::vector<std::size_t> indices;
        const int runEnd = run.logicalStart + run.length;
        for (std::size_t index = 0; index < result->size(); ++index) {
            if ((*result)[index].utf16Start >= run.logicalStart &&
                (*result)[index].utf16Start < runEnd) indices.push_back(index);
        }
        for (std::size_t index = 0; index + 1 < indices.size(); ++index) {
            FocusedGlyph& glyph = (*result)[indices[index]];
            const FocusedGlyph& next = (*result)[indices[index + 1]];
            glyph.endX = next.originX;
            glyph.endY = next.originY;
        }
        if (indices.size() > 1) {
            FocusedGlyph& last = (*result)[indices.back()];
            const FocusedGlyph& previous = (*result)[indices[indices.size() - 2]];
            last.endX = last.originX + (last.originX - previous.originX);
            last.endY = last.originY + (last.originY - previous.originY);
        } else if (indices.size() == 1 && run.direction == TextDirection::Rtl) {
            FocusedGlyph& glyph = (*result)[indices.front()];
            const float advance = std::fabs((glyph.endX - glyph.originX) * glyph.unitX +
                                            (glyph.endY - glyph.originY) * glyph.unitY);
            glyph.endX = glyph.originX - advance * glyph.unitX;
            glyph.endY = glyph.originY - advance * glyph.unitY;
        }
    }
    return true;
}

int nearestGlyphBoundary(const std::vector<FocusedGlyph>& glyphs, float x, float y) {
    float bestDistance = INFINITY;
    int bestIndex = 0;
    auto consider = [&](float bx, float by, int boundary) {
        const float dx = x - bx, dy = y - by;
        const float distance = dx * dx + dy * dy;
        if (distance < bestDistance) {
            bestDistance = distance;
            bestIndex = boundary;
        }
    };
    for (const auto& glyph : glyphs) {
        consider(glyph.originX, glyph.originY, glyph.utf16Start);
        consider(glyph.endX, glyph.endY, glyph.utf16End);
    }
    return bestIndex;
}

bool objectRecord(FPDF_PAGEOBJECT object, int topLevelIndex, uint64_t localId,
                  const Matrix& parent, EditableObjectRecord* result) {
    if (!result) return false;
    const int type = FPDFPageObj_GetType(object);
    const Matrix local = fromPdfium(object);
    const Matrix pageMatrix = multiply(parent, local);
    FS_QUADPOINTSF quad{};
    if (!FPDFPageObj_GetRotatedBounds(object, &quad)) {
        float left = 0, bottom = 0, right = 0, top = 0;
        if (!FPDFPageObj_GetBounds(object, &left, &bottom, &right, &top)) return false;
        quad = {left, bottom, right, bottom, right, top, left, top};
    }

    const float xs[] = {quad.x1, quad.x2, quad.x3, quad.x4};
    const float ys[] = {quad.y1, quad.y2, quad.y3, quad.y4};
    for (int i = 0; i < 4; ++i) {
        float x = 0, y = 0;
        transform(parent, xs[i], ys[i], &x, &y);
        if (i == 0) {
            result->pageLeft = result->pageRight = x;
            result->pageTop = result->pageBottom = y;
        } else {
            result->pageLeft = std::min(result->pageLeft, x);
            result->pageRight = std::max(result->pageRight, x);
            result->pageBottom = std::min(result->pageBottom, y);
            result->pageTop = std::max(result->pageTop, y);
        }
    }

    if (type == FPDF_PAGEOBJ_TEXT) {
        // PDFium reports the quad in parent coordinates. Text-editor callers
        // need object-local bounds plus a local-to-page matrix.
        Matrix inverseLocal;
        if (!inverse(local, &inverseLocal)) return false;
        for (int i = 0; i < 4; ++i) {
            float x = 0, y = 0;
            transform(inverseLocal, xs[i], ys[i], &x, &y);
            if (i == 0) {
                result->left = result->right = x;
                result->top = result->bottom = y;
            } else {
                result->left = std::min(result->left, x);
                result->right = std::max(result->right, x);
                result->bottom = std::min(result->bottom, y);
                result->top = std::max(result->top, y);
            }
        }
    } else {
        result->left = result->pageLeft;
        result->right = result->pageRight;
        result->top = result->pageTop;
        result->bottom = result->pageBottom;
    }
    result->pageObjectIndex = topLevelIndex;
    result->localId = localId;
    result->object = object;
    result->type = type;
    storeMatrix(pageMatrix, result->matrix);
    result->groupLeft = result->pageLeft;
    result->groupRight = result->pageRight;
    result->groupTop = result->pageTop;
    result->groupBottom = result->pageBottom;
    return true;
}

void collect(FPDF_PAGEOBJECT object, int topLevelIndex, uint64_t localId,
             const Matrix& parent, const Bounds* parentGroup,
             const std::vector<FPDF_PAGEOBJECT>& formAncestors,
             std::vector<EditableObjectRecord>* output) {
    const int type = FPDFPageObj_GetType(object);
    if (type == FPDF_PAGEOBJ_TEXT || type == FPDF_PAGEOBJ_IMAGE ||
        type == FPDF_PAGEOBJ_PATH) {
        EditableObjectRecord record;
        if (objectRecord(object, topLevelIndex, localId, parent, &record)) {
            record.formAncestors = formAncestors;
            if (parentGroup) {
                record.groupLeft = parentGroup->left;
                record.groupRight = parentGroup->right;
                record.groupTop = parentGroup->top;
                record.groupBottom = parentGroup->bottom;
            }
            output->push_back(record);
        }
        return;
    }
    if (type != FPDF_PAGEOBJ_FORM) return;
    EditableObjectRecord formRecord;
    Bounds formGroup{};
    const Bounds* childGroup = parentGroup;
    if (objectRecord(object, topLevelIndex, localId, parent, &formRecord)) {
        formGroup = {formRecord.pageLeft, formRecord.pageBottom,
                     formRecord.pageRight, formRecord.pageTop};
        childGroup = parentGroup ? parentGroup : &formGroup;
    }
    const Matrix formToPage = multiply(parent, fromPdfium(object));
    std::vector<FPDF_PAGEOBJECT> childAncestors = formAncestors;
    childAncestors.push_back(object);
    const int count = FPDFFormObj_CountObjects(object);
    for (int i = 0; i < count; ++i) {
        FPDF_PAGEOBJECT child = FPDFFormObj_GetObject(object, static_cast<unsigned long>(i));
        const uint64_t childId = (localId ^ static_cast<uint64_t>(i + 1)) * 1099511628211ULL;
        if (child) collect(child, topLevelIndex, childId, formToPage, childGroup,
                           childAncestors, output);
    }
}

bool persistedShapedRun(FPDF_PAGEOBJECT object, uint32_t* runId,
                        std::u16string* logicalText) {
    if (!object || !runId || !logicalText) return false;
    const int marks = FPDFPageObj_CountMarks(object);
    for (int index = 0; index < marks; ++index) {
        FPDF_PAGEOBJECTMARK mark = FPDFPageObj_GetMark(
            object, static_cast<unsigned long>(index));
        int id = 0;
        unsigned long bytes = 0;
        if (!mark || !FPDFPageObjMark_GetParamIntValue(mark, "KSPRunId", &id) ||
            id <= 0 ||
            !FPDFPageObjMark_GetParamBlobValue(mark, "ActualText", nullptr, 0,
                                               &bytes) ||
            bytes < 2 || (bytes & 1u) != 0) {
            continue;
        }
        std::vector<unsigned char> encoded(bytes);
        unsigned long copied = 0;
        if (!FPDFPageObjMark_GetParamBlobValue(mark, "ActualText", encoded.data(),
                                               bytes, &copied) || copied != bytes ||
            encoded[0] != 0xfe || encoded[1] != 0xff) {
            continue;
        }
        logicalText->clear();
        logicalText->reserve((bytes - 2) / 2);
        for (std::size_t offset = 2; offset < encoded.size(); offset += 2) {
            logicalText->push_back(static_cast<char16_t>(
                (static_cast<uint16_t>(encoded[offset]) << 8) |
                encoded[offset + 1]));
        }
        *runId = static_cast<uint32_t>(id);
        return !logicalText->empty();
    }
    return false;
}

ErrorCode rebuildLocked(PageState& page) {
    page.editableObjects.clear();
    const int count = FPDFPage_CountObjects(page.pdf);
    for (int i = 0; i < count; ++i) {
        FPDF_PAGEOBJECT object = FPDFPage_GetObject(page.pdf, i);
        const uint64_t localId = (1469598103934665603ULL ^ static_cast<uint64_t>(i + 1)) *
                                 1099511628211ULL;
        if (object) collect(object, i, localId, Matrix{}, nullptr, {},
                            &page.editableObjects);
    }
    page.persistentShapedRunIds.clear();
    struct PersistedGroup {
        std::u16string text;
        FPDF_PAGEOBJECT carrier = nullptr;
        float carrierRight = -INFINITY;
    };
    std::unordered_map<uint32_t, PersistedGroup> groups;
    for (const auto& record : page.editableObjects) {
        if (record.type != FPDF_PAGEOBJ_TEXT) continue;
        uint32_t runId = 0;
        std::u16string logical;
        if (!persistedShapedRun(record.object, &runId, &logical)) continue;
        page.persistentShapedRunIds[record.object] = runId;
        PersistedGroup& group = groups[runId];
        if (group.text.empty()) group.text = std::move(logical);
        // Shaped runs written here are RTL. Domain atoms are sorted from left
        // to right, so place the one logical payload on the rightmost carrier
        // and make every visual attachment logically empty.
        if (!group.carrier || record.pageRight > group.carrierRight) {
            group.carrier = record.object;
            group.carrierRight = record.pageRight;
        }
    }
    for (const auto& [object, runId] : page.persistentShapedRunIds) {
        const auto group = groups.find(runId);
        if (group != groups.end()) {
            page.shapedLogicalText[object] =
                object == group->second.carrier ? group->second.text : std::u16string{};
        }
    }
    page.editableInventoryValid = true;
    ++page.editableInventoryEpoch;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

float normalizedOrientation(const EditableObjectRecord& item) {
    constexpr float kPi = 3.14159265358979323846f;
    float angle = std::atan2(item.matrix[3], item.matrix[0]);
    while (angle <= -kPi / 2) angle += kPi;
    while (angle > kPi / 2) angle -= kPi;
    return angle;
}

Matrix orientationMatrix(float angle) {
    const float cosine = std::cos(angle);
    const float sine = std::sin(angle);
    return {cosine, sine, -sine, cosine, 0, 0};
}

Bounds pageBoundsInBasis(const Bounds& value, const Matrix& pageToBasis) {
    const float xs[] = {value.left, value.right, value.right, value.left};
    const float ys[] = {value.bottom, value.bottom, value.top, value.top};
    Bounds result{};
    for (int i = 0; i < 4; ++i) {
        float x = 0, y = 0;
        transform(pageToBasis, xs[i], ys[i], &x, &y);
        if (i == 0) result = {x, y, x, y};
        else {
            result.left = std::min(result.left, x);
            result.right = std::max(result.right, x);
            result.bottom = std::min(result.bottom, y);
            result.top = std::max(result.top, y);
        }
    }
    return result;
}

Bounds textBoundsInBasis(const EditableObjectRecord& item, const Matrix& pageToBasis) {
    const Matrix object{item.matrix[0], item.matrix[3], item.matrix[1],
                        item.matrix[4], item.matrix[2], item.matrix[5]};
    const Matrix objectToBasis = multiply(pageToBasis, object);
    const float xs[] = {item.left, item.right, item.right, item.left};
    const float ys[] = {item.bottom, item.bottom, item.top, item.top};
    Bounds result{};
    for (int i = 0; i < 4; ++i) {
        float x = 0, y = 0;
        transform(objectToBasis, xs[i], ys[i], &x, &y);
        if (i == 0) result = {x, y, x, y};
        else {
            result.left = std::min(result.left, x);
            result.right = std::max(result.right, x);
            result.bottom = std::min(result.bottom, y);
            result.top = std::max(result.top, y);
        }
    }
    return result;
}

void include(Bounds value, float* left, float* bottom, float* right, float* top) {
    *left = std::min(*left, value.left);
    *right = std::max(*right, value.right);
    *bottom = std::min(*bottom, value.bottom);
    *top = std::max(*top, value.top);
}

float median(std::vector<float> values, float fallback = 0.0f) {
    if (values.empty()) return fallback;
    const std::size_t middle = values.size() / 2;
    std::nth_element(values.begin(), values.begin() + middle, values.end());
    if (values.size() % 2 != 0) return values[middle];
    const float upper = values[middle];
    std::nth_element(values.begin(), values.begin() + middle - 1, values.end());
    return (values[middle - 1] + upper) * 0.5f;
}

struct LayoutLine {
    float orientation = 0;
    TextDirection direction = TextDirection::Ltr;
    Bounds geometry{};
    Bounds group{};
    std::vector<const EditableObjectRecord*> records;
    std::vector<float> baselines;
    std::vector<float> heights;
    bool hasFormContainer = false;
    bool structuredRow = false;
    bool formulaRow = false;
    bool displayFormulaRow = false;
    bool startsListItem = false;
    bool referenceLabelOnly = false;
    bool startsReferenceBody = false;
    bool containsReferenceLabel = false;
    bool mergeReferenceContinuations = false;

    float baseline() const { return median(baselines); }
    // Use the complete visible line band.  A median of atom heights is wrong
    // for leader lines because dozens of tiny dots would hide the normal text
    // height and make the next baseline look artificially distant.
    float height() const { return std::max(1.0f, geometry.top - geometry.bottom); }
    // TeX/Typst TOC leaders commonly arrive as dozens of tiny TextObjects on
    // one baseline.  Their indentation describes outline depth, not a new
    // editable paragraph, so paragraph-indent splitting must ignore them.
    bool isLeaderLine() const { return records.size() >= 24; }
};

struct LayoutBlock {
    float orientation = 0;
    Bounds geometry{};
    Bounds group{};
    std::vector<LayoutLine*> lines;
    bool hasFormContainer = false;
    bool hasFormula = false;
    bool referenceLabel = false;
    bool referenceBody = false;
};

Bounds itemGeometry(const EditableObjectRecord& item, float orientation) {
    Matrix pageToBasis;
    (void)inverse(orientationMatrix(orientation), &pageToBasis);
    return textBoundsInBasis(item, pageToBasis);
}

Bounds itemGroup(const EditableObjectRecord& item, float orientation) {
    Matrix pageToBasis;
    (void)inverse(orientationMatrix(orientation), &pageToBasis);
    return pageBoundsInBasis(
        {item.groupLeft, item.groupBottom, item.groupRight, item.groupTop}, pageToBasis);
}

float itemBaseline(const EditableObjectRecord& item, float orientation) {
    Matrix pageToBasis;
    (void)inverse(orientationMatrix(orientation), &pageToBasis);
    float x = 0, y = 0;
    transform(pageToBasis, item.matrix[2], item.matrix[5], &x, &y);
    return y;
}

float horizontalGap(const Bounds& first, const Bounds& second) {
    return std::max(0.0f, std::max(first.left - second.right,
                                   second.left - first.right));
}

bool horizontalCompatible(const Bounds& first, const Bounds& second,
                          float scale) {
    const float overlap = std::min(first.right, second.right) -
                          std::max(first.left, second.left);
    const float minWidth = std::max(1.0f, std::min(first.right - first.left,
                                                  second.right - second.left));
    if (overlap >= 0.15f * minWidth) return true;
    const float left = std::fabs(first.left - second.left);
    const float right = std::fabs(first.right - second.right);
    const float center = std::fabs((first.left + first.right) * 0.5f -
                                   (second.left + second.right) * 0.5f);
    return std::min(left, std::min(right, center)) <= 1.5f * scale;
}

void recognizeTextDomainsLocked(PageState& page) {
    page.textDomains.clear();
    std::vector<const EditableObjectRecord*> text;
    for (const auto& item : page.editableObjects) {
        if (item.type == FPDF_PAGEOBJ_TEXT) text.push_back(&item);
    }
    // Newly generated content and some Form/producer streams group objects by
    // style instead of reading order.  A large upward jump cannot be an inline
    // sub/superscript transition, so fall back to geometric order for those
    // streams.  Small baseline excursions remain in stream order because they
    // carry the formula span boundaries recovered below.
    bool contentOrderDisordered = false;
    for (std::size_t i = 1; i < text.size(); ++i) {
        const float previousOrientation = normalizedOrientation(*text[i - 1]);
        const float orientation = normalizedOrientation(*text[i]);
        if (std::fabs(previousOrientation - orientation) > 0.035f) continue;
        const Bounds previousGeometry = itemGeometry(*text[i - 1], previousOrientation);
        const Bounds geometry = itemGeometry(*text[i], orientation);
        const float scale = std::max(previousGeometry.top - previousGeometry.bottom,
                                     geometry.top - geometry.bottom);
        if (itemBaseline(*text[i], orientation) -
                itemBaseline(*text[i - 1], previousOrientation) >
            0.75f * std::max(1.0f, scale)) {
            contentOrderDisordered = true;
            break;
        }
    }
    bool hasMathFont = false;
    for (const auto* item : text) {
        FPDF_FONT font = FPDFTextObj_GetFont(item->object);
        const size_t length = font ? FPDFFont_GetBaseFontName(font, nullptr, 0) : 0;
        if (!length) continue;
        std::vector<char> name(length);
        if (FPDFFont_GetBaseFontName(font, name.data(), name.size()) == length &&
            std::string(name.data()).find("Math") != std::string::npos) {
            hasMathFont = true;
            break;
        }
    }
    if (contentOrderDisordered && !hasMathFont) {
        std::sort(text.begin(), text.end(), [](const auto* lhs, const auto* rhs) {
            const float lhsOrientation = normalizedOrientation(*lhs);
            const float rhsOrientation = normalizedOrientation(*rhs);
            if (std::fabs(lhsOrientation - rhsOrientation) > 0.035f)
                return lhsOrientation < rhsOrientation;
            const float lhsBaseline = itemBaseline(*lhs, lhsOrientation);
            const float rhsBaseline = itemBaseline(*rhs, rhsOrientation);
            if (std::fabs(lhsBaseline - rhsBaseline) > 0.5f)
                return lhsBaseline > rhsBaseline;
            return itemGeometry(*lhs, lhsOrientation).left <
                   itemGeometry(*rhs, rhsOrientation).left;
        });
    }
    // Load the text traits once.  The original layout tree distinguishes an
    // LrFormula row from an ordinary paragraph containing inline math.  The
    // observable discriminator in generated scientific PDFs is a baseline
    // cohort whose visible characters are overwhelmingly carried by the math
    // font.  Classifying the whole cohort is important: applying the tighter
    // formula gutter to every math TextObject fragments normal prose.
    FPDF_TEXTPAGE textPage = FPDFText_LoadPage(page.pdf);
    struct TextTraits {
        std::u16string value;
        bool mathFont = false;
        TextDirection direction = TextDirection::Ltr;
    };
    std::unordered_map<const EditableObjectRecord*, TextTraits> traits;
    TextDirection inheritedDirection = TextDirection::Ltr;
    const EditableObjectRecord* directionPredecessor = nullptr;
    for (const auto* item : text) {
        TextTraits value;
        FPDF_FONT font = FPDFTextObj_GetFont(item->object);
        const size_t fontNameLength = font ? FPDFFont_GetBaseFontName(font, nullptr, 0) : 0;
        if (fontNameLength) {
            std::vector<char> fontName(fontNameLength);
            if (FPDFFont_GetBaseFontName(font, fontName.data(), fontName.size()) ==
                fontNameLength) {
                value.mathFont = std::string(fontName.data()).find("Math") !=
                                 std::string::npos;
            }
        }
        if (textPage) {
            const unsigned long bytes =
                FPDFTextObj_GetText(item->object, textPage, nullptr, 0);
            std::vector<FPDF_WCHAR> buffer(
                (bytes + sizeof(FPDF_WCHAR) - 1) / sizeof(FPDF_WCHAR));
            if (bytes && FPDFTextObj_GetText(item->object, textPage, buffer.data(), bytes) ==
                             bytes) {
                for (FPDF_WCHAR unit : buffer) {
                    if (!unit) break;
                    value.value.push_back(static_cast<char16_t>(unit));
                }
            }
        }
        TextDirection fallbackDirection = TextDirection::Ltr;
        if (directionPredecessor) {
            const float orientation = normalizedOrientation(*item);
            const float previousOrientation = normalizedOrientation(*directionPredecessor);
            if (std::fabs(orientation - previousOrientation) <= 0.035f &&
                std::fabs(itemBaseline(*item, orientation) -
                          itemBaseline(*directionPredecessor, previousOrientation)) <= 0.35f) {
                fallbackDirection = inheritedDirection;
            }
        }
        value.direction = baseTextDirection(value.value, fallbackDirection);
        inheritedDirection = value.direction;
        directionPredecessor = item;
        traits.emplace(item, std::move(value));
    }
    std::unordered_map<const EditableObjectRecord*, bool> formulaCohort;
    std::unordered_map<const EditableObjectRecord*, bool> displayFormulaCohort;
    std::unordered_map<const EditableObjectRecord*, float> formulaCohortHeight;
    float bodyLeft = INFINITY;
    std::vector<float> pageTextHeights;
    for (const auto* item : text) {
        const float orientation = normalizedOrientation(*item);
        if (std::fabs(orientation) <= 0.035f) {
            const Bounds geometry = itemGeometry(*item, orientation);
            bodyLeft = std::min(bodyLeft, geometry.left);
            pageTextHeights.push_back(std::max(1.0f, geometry.top - geometry.bottom));
        }
    }
    for (const auto* origin : text) {
        const float orientation = normalizedOrientation(*origin);
        std::size_t mathCharacters = 0;
        std::size_t otherCharacters = 0;
        float cohortLeft = INFINITY;
        float cohortRight = -INFINITY;
        float cohortHeight = 1.0f;
        for (const auto* candidate : text) {
            if (std::fabs(normalizedOrientation(*candidate) - orientation) > 0.035f ||
                std::fabs(itemBaseline(*candidate, orientation) -
                          itemBaseline(*origin, orientation)) > 0.35f) {
                continue;
            }
            const Bounds geometry = itemGeometry(*candidate, orientation);
            cohortLeft = std::min(cohortLeft, geometry.left);
            cohortRight = std::max(cohortRight, geometry.right);
            cohortHeight = std::max(cohortHeight, geometry.top - geometry.bottom);
            const auto& candidateTraits = traits[candidate];
            for (char16_t unit : candidateTraits.value) {
                if (unit == u' ' || unit == u'\t' || unit == u'\r' || unit == u'\n')
                    continue;
                if (candidateTraits.mathFont) ++mathCharacters;
                else ++otherCharacters;
            }
        }
        const std::size_t visibleCharacters = mathCharacters + otherCharacters;
        const bool formula = visibleCharacters >= 1 &&
                             mathCharacters * 4 >= visibleCharacters * 3;
        formulaCohort[origin] = formula;
        formulaCohortHeight[origin] = cohortHeight;
        displayFormulaCohort[origin] = formula && std::isfinite(bodyLeft) &&
            cohortLeft >= bodyLeft + 5.0f * cohortHeight &&
            cohortRight > cohortLeft;
    }
    const float normalTextHeight = median(pageTextHeights, 1.0f);
    for (const auto* origin : text) {
        if (!displayFormulaCohort[origin]) continue;
        const float orientation = normalizedOrientation(*origin);
        if (formulaCohortHeight[origin] >= 0.80f * normalTextHeight) continue;
        bool hasDisplayParent = false;
        const float baseline = itemBaseline(*origin, orientation);
        for (const auto* candidate : text) {
            if (!displayFormulaCohort[candidate] || candidate == origin ||
                std::fabs(normalizedOrientation(*candidate) - orientation) > 0.035f)
                continue;
            const float parentGap = itemBaseline(*candidate, orientation) - baseline;
            if (parentGap > 0.5f && parentGap <= 0.65f * normalTextHeight) {
                hasDisplayParent = true;
                break;
            }
        }
        displayFormulaCohort[origin] = hasDisplayParent;
    }

    // Stage 1: preserve content-stream order while building baseline runs.
    // This is significant for formulas: a main-baseline atom, a subscript and
    // the following main-baseline atom are three editable spans in the
    // original SO.  Geometry-sorting all main-baseline atoms together loses
    // the intervening baseline transition and incorrectly creates one large
    // editor.  Ordinary style changes and TOC leader atoms remain in one run
    // because their text matrices share the same baseline.
    std::vector<LayoutLine> lines;
    auto startsListItem = [&](const EditableObjectRecord& item) {
        const auto& value = traits[&item].value;
        if (value.empty()) return false;
        return value[0] == 0x2022 || value[0] == 0x25e6 ||
               value[0] == 0x2023 || value[0] == 0x2043;
    };
    auto isReferenceLabel = [&](const EditableObjectRecord& item) {
        const auto& value = traits[&item].value;
        std::size_t begin = 0;
        std::size_t end = value.size();
        auto whitespace = [](char16_t unit) {
            return unit == u' ' || unit == u'\t' || unit == u'\r' ||
                   unit == u'\n' || unit == 0x00a0;
        };
        while (begin < end && whitespace(value[begin])) ++begin;
        while (end > begin && whitespace(value[end - 1])) --end;
        if (end - begin < 3 || value[begin] != u'[' || value[end - 1] != u']')
            return false;
        for (std::size_t i = begin + 1; i + 1 < end; ++i) {
            if (value[i] < u'0' || value[i] > u'9') return false;
        }
        return true;
    };
    for (const auto* item : text) {
        const float orientation = normalizedOrientation(*item);
        const Bounds itemVisibleGeometry = itemGeometry(*item, orientation);
        if (itemVisibleGeometry.right - itemVisibleGeometry.left < 0.01f &&
            itemVisibleGeometry.top - itemVisibleGeometry.bottom < 0.01f) {
            // Empty positioning TextObjects do not contribute glyphs or an
            // editor border.  Treating them as zero-sized lines fragments a
            // TOC into one block per row.
            continue;
        }
        const Bounds geometry = itemGeometry(*item, orientation);
        const Bounds group = itemGroup(*item, orientation);
        LayoutLine* best = nullptr;
        if (std::fabs(orientation) > 0.035f) {
            float bestDistance = INFINITY;
            for (auto& candidate : lines) {
                if (std::fabs(orientation - candidate.orientation) > 0.035f) continue;
                if (traits[item].direction != candidate.direction) continue;
                const float itemHeight =
                    std::max(1.0f, geometry.top - geometry.bottom);
                const float overlap =
                    std::min(candidate.geometry.top, geometry.top) -
                    std::max(candidate.geometry.bottom, geometry.bottom);
                const float centerDistance = std::fabs(
                    (candidate.geometry.top + candidate.geometry.bottom) * 0.5f -
                    (geometry.top + geometry.bottom) * 0.5f);
                const bool sameBand =
                    overlap >= 0.15f * std::min(candidate.height(), itemHeight) ||
                    centerDistance <= 0.60f *
                                          std::max(candidate.height(), itemHeight);
                const float gap = horizontalGap(candidate.geometry, geometry);
                if (sameBand &&
                    gap <= 4.0f * std::max(candidate.height(), itemHeight) &&
                    centerDistance < bestDistance) {
                    best = &candidate;
                    bestDistance = centerDistance;
                }
            }
        } else if (!lines.empty()) {
            LayoutLine& previous = lines.back();
            const float itemHeight = std::max(1.0f, geometry.top - geometry.bottom);
            const float baselineDistance = std::fabs(
                itemBaseline(*item, orientation) - previous.baseline());
            const float baselineTolerance =
                std::max(0.35f, 0.08f * std::min(previous.height(), itemHeight));
            const float gap = horizontalGap(previous.geometry, geometry);
            const bool formulaRow = formulaCohort[item] && previous.formulaRow;
            const float gutterScale = formulaRow ? 1.25f : 4.0f;
            // The original LrParagraph tree keeps a bibliography/list label
            // such as "[45]" in a narrow editor beside the hanging paragraph.
            // It is a sibling structural node even though both nodes share a
            // baseline.  Preserve the whitespace gutter as a hard boundary;
            // ordinary adjacent TextObjects on a line still coalesce.
            const bool referenceLabelBoundary = previous.referenceLabelOnly &&
                gap >= 1.00f * std::max(previous.height(), itemHeight);
            if (std::fabs(orientation - previous.orientation) <= 0.035f &&
                traits[item].direction == previous.direction &&
                baselineDistance <= baselineTolerance &&
                gap <= gutterScale * std::max(previous.height(), itemHeight) &&
                !referenceLabelBoundary) {
                best = &previous;
            }
        }
        if (!best) {
            LayoutLine line;
            line.orientation = orientation;
            line.direction = traits[item].direction;
            line.geometry = geometry;
            line.group = group;
            line.formulaRow = formulaCohort[item];
            line.displayFormulaRow = displayFormulaCohort[item];
            line.startsListItem = startsListItem(*item);
            line.referenceLabelOnly = isReferenceLabel(*item);
            line.containsReferenceLabel = line.referenceLabelOnly;
            line.startsReferenceBody = !lines.empty() &&
                                       lines.back().referenceLabelOnly;
            lines.push_back(std::move(line));
            best = &lines.back();
            if (lines.size() > 1) {
                LayoutLine& previous = lines[lines.size() - 2];
                const float baselineDistance = std::fabs(
                    itemBaseline(*item, orientation) - previous.baseline());
                const float rowBand = 0.65f * std::max(previous.height(), best->height());
                const bool advancesAcrossRow =
                    geometry.left >= previous.geometry.left - 0.5f &&
                    geometry.left <= previous.geometry.right +
                                         4.0f * std::max(previous.height(), best->height());
                if (std::fabs(orientation - previous.orientation) <= 0.035f &&
                    baselineDistance <= rowBand && advancesAcrossRow &&
                    !previous.referenceLabelOnly) {
                    previous.structuredRow = true;
                    best->structuredRow = true;
                }
            }
        } else {
            include(geometry, &best->geometry.left, &best->geometry.bottom,
                    &best->geometry.right, &best->geometry.top);
            include(group, &best->group.left, &best->group.bottom,
                    &best->group.right, &best->group.top);
        }
        best->records.push_back(item);
        best->formulaRow = best->formulaRow && formulaCohort[item];
        best->displayFormulaRow =
            best->displayFormulaRow && displayFormulaCohort[item];
        best->referenceLabelOnly = best->referenceLabelOnly && isReferenceLabel(*item);
        best->containsReferenceLabel =
            best->containsReferenceLabel || isReferenceLabel(*item);
        best->baselines.push_back(itemBaseline(*item, best->orientation));
        best->heights.push_back(std::max(1.0f, geometry.top - geometry.bottom));
        best->hasFormContainer = best->hasFormContainer || !item->formAncestors.empty();
    }
    if (textPage) FPDFText_ClosePage(textPage);
    // The original editor paints every span on a shared baseline with the
    // same row-height envelope.  Keep horizontal ownership per span, but
    // normalize top/bottom across peer runs so short operators still expose
    // a full-height caret and focus frame.
    for (std::size_t i = 0; i < lines.size(); ++i) {
        float rowTop = lines[i].geometry.top;
        float rowBottom = lines[i].geometry.bottom;
        for (std::size_t j = 0; j < lines.size(); ++j) {
            if (std::fabs(lines[i].orientation - lines[j].orientation) > 0.035f ||
                std::fabs(lines[i].baseline() - lines[j].baseline()) > 0.35f) {
                continue;
            }
            rowTop = std::max(rowTop, lines[j].geometry.top);
            rowBottom = std::min(rowBottom, lines[j].geometry.bottom);
        }
        lines[i].geometry.top = rowTop;
        lines[i].geometry.bottom = rowBottom;
    }
    for (auto& line : lines) {
        std::sort(line.records.begin(), line.records.end(), [&](const auto* lhs, const auto* rhs) {
            return itemGeometry(*lhs, line.orientation).left <
                   itemGeometry(*rhs, line.orientation).left;
        });
    }
    // A wide reference label can share its first line with the body.  The
    // original analyzer keeps subsequent lines in that same editor only when
    // every continuation reaches essentially the same right edge; otherwise
    // it exposes the hanging continuation as a sibling.  This distinguishes
    // compact entries such as [106]/[119] from entries whose final DOI or URL
    // line is visibly shorter, without inspecting the citation number.
    const bool pageHasDetachedReferenceLabels = std::any_of(
        lines.begin(), lines.end(), [](const LayoutLine& line) {
            return line.referenceLabelOnly;
        });
    for (std::size_t i = 0; i < lines.size(); ++i) {
        if (!lines[i].containsReferenceLabel || lines[i].referenceLabelOnly)
            continue;
        const float scale = lines[i].height();
        bool sawContinuation = false;
        bool fullWidth = true;
        float previousBaseline = lines[i].baseline();
        for (std::size_t j = i + 1; j < lines.size(); ++j) {
            if (lines[j].containsReferenceLabel) break;
            const float gap = previousBaseline - lines[j].baseline();
            if (gap < 0.5f || gap > 2.5f * std::max(scale, lines[j].height()))
                break;
            if (lines[j].geometry.left - lines[i].geometry.left <= 0.5f * scale)
                break;
            sawContinuation = true;
            if (std::fabs(lines[j].geometry.right - lines[i].geometry.right) >
                0.25f * scale) {
                fullWidth = false;
            }
            previousBaseline = lines[j].baseline();
        }
        lines[i].mergeReferenceContinuations =
            !pageHasDetachedReferenceLabels && sawContinuation && fullWidth;
    }
    // Estimate the normal baseline step from local, column-compatible
    // neighbours.  This value remains fixed while blocks grow, preventing the
    // old transitive whole-page merge.
    std::vector<float> orientationSteps(lines.size(), 0.0f);
    for (std::size_t origin = 0; origin < lines.size(); ++origin) {
        std::vector<float> candidates;
        std::vector<float> heights;
        for (const auto& line : lines) {
            if (std::fabs(line.orientation - lines[origin].orientation) <= 0.035f)
                heights.push_back(line.height());
        }
        for (std::size_t i = 0; i < lines.size(); ++i) {
            if (std::fabs(lines[i].orientation - lines[origin].orientation) > 0.035f)
                continue;
            for (std::size_t j = i + 1; j < lines.size(); ++j) {
                if (std::fabs(lines[j].orientation - lines[i].orientation) > 0.035f)
                    continue;
                const float gap = lines[i].baseline() - lines[j].baseline();
                const float scale = std::max(lines[i].height(), lines[j].height());
                if (gap <= 0.5f || gap > 4.0f * scale) continue;
                if (!horizontalCompatible(lines[i].geometry, lines[j].geometry, scale))
                    continue;
                candidates.push_back(gap);
                break;
            }
        }
        orientationSteps[origin] = median(candidates, 1.65f * median(heights, 1.0f));
    }

    // Stage 2: join lines into editable blocks using the fixed line step and
    // adjacent-line geometry.  A Form container remains a special observed
    // original-SO case: strongly aligned Form and top-level text may share an
    // editor even across a large vertical gap, while output geometry still
    // excludes the Form container envelope.
    std::vector<LayoutBlock> blocks;
    for (std::size_t lineIndex = 0; lineIndex < lines.size(); ++lineIndex) {
        LayoutLine& line = lines[lineIndex];
        LayoutBlock* best = nullptr;
        float bestGap = INFINITY;
        for (auto& block : blocks) {
            if (std::fabs(block.orientation - line.orientation) > 0.035f) continue;
            LayoutLine& previous = *block.lines.back();
            const float gap = previous.baseline() - line.baseline();
            if (gap < -0.5f) continue;
            const float scale = std::max(previous.height(), line.height());
            const float threshold = std::max(2.0f * scale,
                                             1.35f * orientationSteps[lineIndex]);
            bool compatible = horizontalCompatible(previous.geometry, line.geometry, scale) &&
                              gap <= threshold + 0.5f;
            if (line.startsListItem) compatible = false;
            if (line.startsReferenceBody) compatible = false;
            // Reference labels are leaf siblings, not a vertical label
            // column.  Neither a following body nor the next numbered label
            // may absorb them into a shared editable block.
            if (previous.referenceLabelOnly || line.referenceLabelOnly ||
                line.containsReferenceLabel)
                compatible = false;
            // When a wider reference label consumes most of the gutter (for
            // example "[104]"), the original editor keeps the label and first
            // body line together, then exposes the hanging continuation as a
            // sibling editor.
            if (previous.containsReferenceLabel &&
                line.geometry.left - previous.geometry.left > 0.50f * scale &&
                !previous.mergeReferenceContinuations)
                compatible = false;
            // Distinct content-stream runs inside one visual row represent
            // baseline-layer changes (sub/superscripts, formula spans) or a
            // horizontal region boundary.  They must remain separate spans.
            // Paragraph continuation lines are about one full baseline step
            // apart; joining runs substantially closer than that recreates
            // the old formula-overmerge defect.
            const float inlineBandLimit =
                std::max(0.75f, 0.55f * orientationSteps[lineIndex]);
            if (compatible && (gap < inlineBandLimit || previous.structuredRow ||
                               line.structuredRow)) {
                compatible = false;
            }
            // The original analyzer separates first-line indented paragraphs
            // and hanging-list items even when their baseline gap is normal.
            // A first line may legitimately dedent on its continuation, hence
            // a dedent becomes a boundary only after the candidate block has
            // already accumulated at least two lines.
            if (compatible && !previous.isLeaderLine() && !line.isLeaderLine()) {
                const float indentTolerance = std::max(6.0f, 0.65f * scale);
                const float leftDelta = line.geometry.left - previous.geometry.left;
                const bool hangingItemBoundary = block.lines.size() > 1 &&
                                                 leftDelta < -indentTolerance;
                const bool firstLineIndentBoundary =
                    leftDelta > indentTolerance &&
                    gap > 1.25f * orientationSteps[lineIndex];
                if (hangingItemBoundary || firstLineIndentBoundary) compatible = false;
            }
            // LrFormula keeps a vertical span when successive formula rows
            // describe the same aligned column, including a narrow label
            // placed below a wider expression.  This relation is stronger
            // than the generic structured-row break above.
            if (previous.displayFormulaRow && line.displayFormulaRow && gap > 0.5f &&
                gap <= 4.0f * scale) {
                const float leftDelta =
                    std::fabs(previous.geometry.left - line.geometry.left);
                const float rightDelta =
                    std::fabs(previous.geometry.right - line.geometry.right);
                const float previousWidth = previous.geometry.right - previous.geometry.left;
                const float lineWidth = line.geometry.right - line.geometry.left;
                const float overlap = std::min(previous.geometry.right,
                                               line.geometry.right) -
                                      std::max(previous.geometry.left,
                                               line.geometry.left);
                const bool alignedColumn = leftDelta <= 0.35f * scale &&
                                           rightDelta <= 1.0f * scale;
                const bool nestedLabel = gap <= 2.4f * scale &&
                    gap >= 2.0f * scale && overlap > 0.0f &&
                    std::min(previousWidth, lineWidth) <=
                        0.25f * std::max(previousWidth, lineWidth);
                if (alignedColumn || nestedLabel) compatible = true;
            }
            // A full-width prose continuation can cross a slightly enlarged
            // leading interval (for example a theorem immediately following
            // its explanatory line) while retaining one LrParagraph.
            if (!previous.formulaRow && !line.formulaRow &&
                !previous.structuredRow && !line.structuredRow &&
                !line.startsListItem && !line.startsReferenceBody &&
                !previous.containsReferenceLabel && gap > 0.5f &&
                gap <= 2.25f * scale &&
                std::fabs(previous.geometry.left - line.geometry.left) <=
                    0.5f * scale &&
                std::min(previous.geometry.right - previous.geometry.left,
                         line.geometry.right - line.geometry.left) >=
                    20.0f * scale) {
                compatible = true;
            }
            if (!compatible && block.hasFormContainer) {
                const float formScale = std::min(previous.height(), line.height());
                compatible = std::fabs(block.group.left - line.group.left) <=
                                 0.75f * formScale &&
                             gap <= 20.0f * formScale;
            }
            if (compatible && gap < bestGap) {
                best = &block;
                bestGap = gap;
            }
        }
        if (!best) {
            LayoutBlock block;
            block.orientation = line.orientation;
            block.geometry = line.geometry;
            block.group = line.group;
            blocks.push_back(std::move(block));
            best = &blocks.back();
        } else {
            include(line.geometry, &best->geometry.left, &best->geometry.bottom,
                    &best->geometry.right, &best->geometry.top);
            include(line.group, &best->group.left, &best->group.bottom,
                    &best->group.right, &best->group.top);
        }
        best->lines.push_back(&line);
        best->hasFormContainer = best->hasFormContainer || line.hasFormContainer;
        best->hasFormula = best->hasFormula || line.formulaRow ||
                           line.displayFormulaRow;
        best->referenceLabel = best->referenceLabel || line.referenceLabelOnly;
        best->referenceBody = best->referenceBody || line.startsReferenceBody ||
                              line.mergeReferenceContinuations;
    }

    // Preserve stream order within one orientation (formula spans depend on
    // it), while retaining the original public editor ordering across
    // horizontal, slanted and rotated regions.
    std::stable_sort(blocks.begin(), blocks.end(), [](const LayoutBlock& lhs,
                                                       const LayoutBlock& rhs) {
        const auto orientationBucket = [](float value) {
            // Recognition treats angles within roughly two degrees as one
            // orientation.  A wider ordering bucket keeps that equivalence
            // class stable without using a non-transitive fuzzy comparator.
            return static_cast<long>(std::llround(value / 0.10f));
        };
        return orientationBucket(lhs.orientation) < orientationBucket(rhs.orientation);
    });

    // The original recognizer owns a structural LrSect tree and separately
    // exposes writable editors. Keep those concepts separate: the root node is
    // never returned as a second writable rectangle, and every PageObject is
    // owned by exactly one writable domain. This prevents hierarchy work from
    // reintroducing the duplicate ownership seen in the earlier rebuild.
    constexpr uint64_t rootNodeId = 1;
    uint64_t nextNodeId = rootNodeId + 1;
    page.textDomainHierarchy.clear();
    page.textEditorProjections.clear();
    EditableTextDomainNode root;
    root.id = rootNodeId;
    root.kind = EditableTextDomainKind::Section;
    page.textDomainHierarchy.push_back(std::move(root));
    std::unordered_set<uint64_t> ownedObjects;
    for (const auto& block : blocks) {
        EditableTextDomain domain;
        domain.nodeId = nextNodeId++;
        domain.parentNodeId = rootNodeId;
        domain.kind = block.hasFormContainer
                          ? EditableTextDomainKind::FormParagraph
                      : block.referenceLabel && !block.referenceBody
                          ? EditableTextDomainKind::ReferenceLabel
                      : block.referenceBody
                          ? EditableTextDomainKind::ReferenceBody
                      : block.hasFormula
                          ? EditableTextDomainKind::Formula
                          : EditableTextDomainKind::Paragraph;
        domain.orientation = block.orientation;
        domain.left = block.geometry.left;
        domain.right = block.geometry.right;
        domain.top = block.geometry.top;
        domain.bottom = block.geometry.bottom;
        domain.groupLeft = block.group.left;
        domain.groupRight = block.group.right;
        domain.groupTop = block.group.top;
        domain.groupBottom = block.group.bottom;
        storeMatrix(orientationMatrix(block.orientation), domain.matrix);
        for (const LayoutLine* line : block.lines) {
            EditableTextLine outputLine;
            outputLine.left = line->geometry.left;
            outputLine.right = line->geometry.right;
            outputLine.top = line->geometry.top;
            outputLine.bottom = line->geometry.bottom;
            outputLine.baseline = line->baseline();
            outputLine.medianHeight = line->height();
            for (const auto* record : line->records) {
                if (!ownedObjects.insert(record->localId).second) continue;
                outputLine.objectIds.push_back(record->localId);
                domain.objectIds.push_back(record->localId);
            }
            if (!outputLine.objectIds.empty())
                domain.lines.push_back(std::move(outputLine));
        }
        if (domain.objectIds.empty()) continue;
        const int domainIndex = static_cast<int>(page.textDomains.size());
        EditableTextDomainNode structuralDomain;
        structuralDomain.id = domain.nodeId;
        structuralDomain.parentId = rootNodeId;
        structuralDomain.kind = domain.kind;
        structuralDomain.editableDomainIndex = domainIndex;
        page.textDomainHierarchy.front().children.push_back(structuralDomain.id);
        page.textDomainHierarchy.push_back(std::move(structuralDomain));
        const std::size_t domainNodeIndex = page.textDomainHierarchy.size() - 1;
        for (const EditableTextLine& line : domain.lines) {
            EditableTextDomainNode lineNode;
            lineNode.id = nextNodeId++;
            lineNode.parentId = domain.nodeId;
            lineNode.kind = EditableTextDomainKind::Line;
            page.textDomainHierarchy[domainNodeIndex].children.push_back(lineNode.id);
            page.textDomainHierarchy.push_back(std::move(lineNode));
            const std::size_t lineNodeIndex = page.textDomainHierarchy.size() - 1;
            for (std::size_t span = 0; span < line.objectIds.size(); ++span) {
                EditableTextDomainNode spanNode;
                spanNode.id = nextNodeId++;
                spanNode.parentId = page.textDomainHierarchy[lineNodeIndex].id;
                spanNode.kind = EditableTextDomainKind::Span;
                page.textDomainHierarchy[lineNodeIndex].children.push_back(spanNode.id);
                page.textDomainHierarchy.push_back(std::move(spanNode));
            }
        }
        page.textDomains.push_back(std::move(domain));
        const EditableTextDomain& storedDomain = page.textDomains.back();
        TextEditorProjection projection;
        projection.sourceNodeId = storedDomain.nodeId;
        projection.targetDomainIndex = domainIndex;
        projection.kind = storedDomain.kind;
        projection.stage = TextEditorProjectionStage::TextBlock;
        projection.left = storedDomain.left;
        projection.top = storedDomain.top;
        projection.right = storedDomain.right;
        projection.bottom = storedDomain.bottom;
        std::copy(std::begin(storedDomain.matrix), std::end(storedDomain.matrix),
                  projection.matrix);
        page.textEditorProjections.push_back(std::move(projection));
    }
    page.textDomainsRecognized = true;
}

}  // namespace

void invalidateEditableObjects(Handle page, PageState& state) {
    (void)handles().invalidateChildren(page, HandleType::Page, HandleType::PageObject);
    state.editableObjectHandles.clear();
    state.editableInventoryValid = false;
    state.editableObjects.clear();
    state.textDomainsRecognized = false;
    state.textDomains.clear();
    state.textDomainHierarchy.clear();
    state.textEditorProjections.clear();
}

ErrorCode recognizeTextDomains(Handle page) {
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    state->textReflowAnchor = nullptr;
    invalidateEditableObjects(page, *state);
    code = rebuildLocked(*state);
    if (code != ErrorCode::Ok) return code;
    recognizeTextDomainsLocked(*state);
    auto& context = state->document->recognitionContext;
    auto snapshot = std::make_shared<RecognitionPageSnapshot>();
    snapshot->pageIndex = state->index;
    snapshot->algorithmGeneration = sectionRecognizeAlgorithmGeneration();
    snapshot->sourceMutationEpoch = state->document->mutationEpoch;
    snapshot->sourceInventoryEpoch = state->editableInventoryEpoch;
    snapshot->domains = state->textDomains;
    snapshot->hierarchy = state->textDomainHierarchy;
    snapshot->projections = state->textEditorProjections;
    auto cached = context.pages.find(state->index);
    if (cached != context.pages.end()) ++context.replacementCount;
    context.pages[state->index] = std::move(snapshot);
    ++context.invocationCount;
    if (std::find(context.firstSeenPages.begin(), context.firstSeenPages.end(),
                  state->index) == context.firstSeenPages.end()) {
        context.firstSeenPages.push_back(state->index);
    }
    context.lastPageIndex = state->index;
    context.lastDomainCount = static_cast<int>(state->textEditorProjections.size());
    context.lastAlgorithmGeneration = context.pages[state->index]->algorithmGeneration;
    ++context.processorCommitCount;
    context.processorFormulaPageCount = 0;
    context.processorFormulaDomainCount = 0;
    context.processorFormulaSpanCount = 0;
    context.processorCompoundFormulaCount = 0;
    context.processorMaxFormulaSpanCount = 0;
    for (const auto& pageEntry : context.pages) {
        if (!pageEntry.second) continue;
        int pageFormulaCount = 0;
        for (const EditableTextDomain& domain : pageEntry.second->domains) {
            if (domain.kind != EditableTextDomainKind::Formula) continue;
            ++pageFormulaCount;
            ++context.processorFormulaDomainCount;
            const int spans = static_cast<int>(domain.objectIds.size());
            context.processorFormulaSpanCount += spans;
            if (spans > 1) ++context.processorCompoundFormulaCount;
            context.processorMaxFormulaSpanCount =
                std::max(context.processorMaxFormulaSpanCount, spans);
        }
        if (pageFormulaCount > 0) ++context.processorFormulaPageCount;
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode analyzeEditableObjects(Handle page) {
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    invalidateEditableObjects(page, *state);
    return rebuildLocked(*state);
}

ErrorCode getTextEditorCount(Handle page, int* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    *result = state->textDomainsRecognized
                  ? static_cast<int>(state->textEditorProjections.size()) : 0;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextEditorGeometry(Handle page, int index, bool reverse,
                                EditableObjectRecord* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    if (!state->textDomainsRecognized || index < 0 ||
        index >= static_cast<int>(state->textEditorProjections.size())) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    const TextEditorProjection& projection =
        state->textEditorProjections[static_cast<std::size_t>(index)];
    if (projection.targetDomainIndex < 0 ||
        projection.targetDomainIndex >= static_cast<int>(state->textDomains.size())) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    const bool stableFrame = state->textReflowAnchor &&
                             state->focusedTextDomain == projection.targetDomainIndex;
    result->left = stableFrame ? state->textReflowLeft : projection.left;
    result->right = stableFrame ? state->textReflowRight : projection.right;
    result->top = projection.top;
    result->bottom = stableFrame && state->textReflowHasLayoutBottom
                         ? state->textReflowLayoutBottom : projection.bottom;
    std::copy(std::begin(projection.matrix), std::end(projection.matrix), result->matrix);
    if (reverse) {
        const Matrix forward{result->matrix[0], result->matrix[3], result->matrix[1],
                             result->matrix[4], result->matrix[2], result->matrix[5]};
        Matrix reversed;
        if (!inverse(forward, &reversed)) {
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
        storeMatrix(reversed, result->matrix);
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextEditorIds(Handle page, std::vector<uint64_t>* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    result->clear();
    result->reserve(state->editableObjects.size());
    for (const auto& object : state->editableObjects) {
        if (object.type == FPDF_PAGEOBJ_TEXT) result->push_back(object.localId);
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextObjectDiagnostics(Handle page,
                                   std::vector<TextObjectDiagnostic>* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    FPDF_TEXTPAGE textPage = FPDFText_LoadPage(state->pdf);
    if (!textPage) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    result->clear();
    for (const auto& item : state->editableObjects) {
        if (item.type != FPDF_PAGEOBJ_TEXT) continue;
        TextObjectDiagnostic value;
        value.localId = item.localId;
        value.pageLeft = item.pageLeft;
        value.pageBottom = item.pageBottom;
        value.pageRight = item.pageRight;
        value.pageTop = item.pageTop;
        value.groupLeft = item.groupLeft;
        value.groupBottom = item.groupBottom;
        value.groupRight = item.groupRight;
        value.groupTop = item.groupTop;
        value.formDepth = static_cast<int>(item.formAncestors.size());
        std::copy(std::begin(item.matrix), std::end(item.matrix), value.matrix);
        (void)FPDFTextObj_GetFontSize(item.object, &value.fontSize);
        FPDF_FONT font = FPDFTextObj_GetFont(item.object);
        const size_t fontNameLength = font ? FPDFFont_GetBaseFontName(font, nullptr, 0) : 0;
        if (fontNameLength) {
            std::vector<char> fontName(fontNameLength);
            if (FPDFFont_GetBaseFontName(font, fontName.data(), fontName.size()) ==
                fontNameLength) value.fontName.assign(fontName.data());
        }
        unsigned int r = 0, g = 0, b = 0, a = 0;
        if (FPDFPageObj_GetFillColor(item.object, &r, &g, &b, &a)) {
            value.fillColor = (a << 24) | (r << 16) | (g << 8) | b;
        }
        value.renderMode = static_cast<int>(FPDFTextObj_GetTextRenderMode(item.object));
        const unsigned long bytes = FPDFTextObj_GetText(item.object, textPage, nullptr, 0);
        std::vector<FPDF_WCHAR> buffer((bytes + sizeof(FPDF_WCHAR) - 1) /
                                      sizeof(FPDF_WCHAR));
        if (bytes) (void)FPDFTextObj_GetText(item.object, textPage, buffer.data(), bytes);
        for (FPDF_WCHAR unit : buffer) {
            if (!unit) break;
            value.text.push_back(static_cast<char16_t>(unit));
        }
        result->push_back(std::move(value));
    }
    FPDFText_ClosePage(textPage);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

namespace {

Handle tokenForLocked(Handle pageHandle, Handle documentHandle,
                      const std::shared_ptr<PageState>& page,
                      const EditableObjectRecord& record, ErrorCode* error) {
    const auto found = page->editableObjectHandles.find(record.localId);
    if (found != page->editableObjectHandles.end() &&
        handles().isValid(found->second, HandleType::PageObject)) {
        if (error) *error = ErrorCode::Ok;
        return found->second;
    }
    auto state = std::make_shared<PageObjectState>();
    state->page = page;
    state->localId = record.localId;
    state->inventoryEpoch = page->editableInventoryEpoch;
    state->type = record.type;
    const Handle token = handles().insert(HandleType::PageObject, state, documentHandle,
                                          pageHandle, error);
    if (token) page->editableObjectHandles[record.localId] = token;
    return token;
}

}  // namespace

ErrorCode getPageObjectHandles(Handle page, int type, std::vector<Handle>* result) {
    if (!result || (type != FPDF_PAGEOBJ_IMAGE && type != FPDF_PAGEOBJ_PATH &&
                    type != FPDF_PAGEOBJ_TEXT)) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    if (!pageBorrow) return pageBorrow.error;
    auto state = pageBorrow.as<PageState>();
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    result->clear();
    for (const auto& record : state->editableObjects) {
        if (record.type != type) continue;
        ErrorCode code = ErrorCode::Ok;
        const Handle token = tokenForLocked(page, pageBorrow.documentOwner, state, record, &code);
        if (!token) return code;
        result->push_back(token);
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getPageObjectRect(Handle page, Handle object, int expectedType,
                            EditableObjectRecord* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    if (!pageBorrow) return pageBorrow.error;
    const BorrowedHandle objectBorrow = handles().borrow(
        object, HandleType::PageObject, pageBorrow.documentOwner, page);
    if (!objectBorrow) return objectBorrow.error;
    auto pageState = pageBorrow.as<PageState>();
    auto objectState = objectBorrow.as<PageObjectState>();
    std::lock_guard<std::mutex> lock(pageState->document->mutex);
    if (objectState->type != expectedType) {
        setLastError(ErrorCode::WrongType);
        return ErrorCode::WrongType;
    }
    if (!pageState->editableInventoryValid ||
        objectState->inventoryEpoch != pageState->editableInventoryEpoch) {
        setLastError(ErrorCode::StaleHandle);
        return ErrorCode::StaleHandle;
    }
    const auto found = std::find_if(
        pageState->editableObjects.begin(), pageState->editableObjects.end(),
        [&](const EditableObjectRecord& value) {
            return value.localId == objectState->localId && value.type == expectedType;
        });
    if (found == pageState->editableObjects.end()) {
        setLastError(ErrorCode::StaleHandle);
        return ErrorCode::StaleHandle;
    }
    *result = *found;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getPageObjectAtPoint(Handle page, int expectedType, float x, float y,
                               Handle* result) {
    if (!result || !std::isfinite(x) || !std::isfinite(y)) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    if (!pageBorrow) return pageBorrow.error;
    auto state = pageBorrow.as<PageState>();
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    for (auto item = state->editableObjects.rbegin(); item != state->editableObjects.rend(); ++item) {
        if (item->type != expectedType) continue;
        if (x < item->pageLeft || x > item->pageRight ||
            y < item->pageBottom || y > item->pageTop) continue;
        ErrorCode code = ErrorCode::Ok;
        *result = tokenForLocked(page, pageBorrow.documentOwner, state, *item, &code);
        return *result ? ErrorCode::Ok : code;
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getPageObjectIndex(Handle page, Handle object, int expectedType, int* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    if (!pageBorrow) return pageBorrow.error;
    const BorrowedHandle objectBorrow = handles().borrow(
        object, HandleType::PageObject, pageBorrow.documentOwner, page);
    if (!objectBorrow) return objectBorrow.error;
    auto state = pageBorrow.as<PageState>();
    auto target = objectBorrow.as<PageObjectState>();
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (target->type != expectedType) {
        setLastError(ErrorCode::WrongType);
        return ErrorCode::WrongType;
    }
    if (!state->editableInventoryValid ||
        target->inventoryEpoch != state->editableInventoryEpoch) {
        setLastError(ErrorCode::StaleHandle);
        return ErrorCode::StaleHandle;
    }
    int ordinal = 0;
    for (const auto& record : state->editableObjects) {
        if (record.type != expectedType) continue;
        ++ordinal;
        if (record.localId == target->localId) {
            *result = ordinal;  // Original Java contract numbers images from one.
            setLastError(ErrorCode::Ok);
            return ErrorCode::Ok;
        }
    }
    setLastError(ErrorCode::StaleHandle);
    return ErrorCode::StaleHandle;
}

ErrorCode removePageObject(Handle page, Handle object, int expectedType) {
    if (expectedType != FPDF_PAGEOBJ_IMAGE && expectedType != FPDF_PAGEOBJ_PATH) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    if (!pageBorrow) return pageBorrow.error;
    const BorrowedHandle objectBorrow = handles().borrow(
        object, HandleType::PageObject, pageBorrow.documentOwner, page);
    if (!objectBorrow) return objectBorrow.error;
    auto state = pageBorrow.as<PageState>();
    auto target = objectBorrow.as<PageObjectState>();
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (target->type != expectedType) {
        setLastError(ErrorCode::WrongType);
        return ErrorCode::WrongType;
    }
    if (!state->editableInventoryValid ||
        target->inventoryEpoch != state->editableInventoryEpoch) {
        setLastError(ErrorCode::StaleHandle);
        return ErrorCode::StaleHandle;
    }
    const auto found = std::find_if(state->editableObjects.begin(), state->editableObjects.end(),
        [&](const EditableObjectRecord& value) {
            return value.localId == target->localId && value.type == expectedType;
        });
    if (found == state->editableObjects.end()) {
        setLastError(ErrorCode::StaleHandle);
        return ErrorCode::StaleHandle;
    }
    // CE6.1 preserves the exact original object's ownership and z-order. Form
    // children require a paired indexed insertion API and are handled by the
    // later Form object slice rather than silently deleting shared content.
    if (!found->formAncestors.empty()) {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    FPDF_PAGEOBJECT pageObject = found->object;
    const int pageObjectIndex = found->pageObjectIndex;
    if (!FPDFPage_RemoveObject(state->pdf, pageObject)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    auto command = std::make_shared<EditUndoState>();
    command->kind = EditUndoState::Kind::ObjectPresence;
    command->page = state;
    command->pageHandle = page;
    command->object = pageObject;
    command->pageObjectIndex = pageObjectIndex;
    command->objectType = expectedType;
    command->currentlyAfter = true;
    command->ownsDetachedObject = true;
    if (!FPDFPage_GenerateContent(state->pdf)) {
        command.reset();
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    state->dirtyFunctions |= uint32_t{1} << (expectedType == FPDF_PAGEOBJ_IMAGE ? 6 : 4);
    state->stagedEpoch = ++state->document->mutationEpoch;
    invalidateEditableObjects(page, *state);
    rebuildLocked(*state);
    recognizeTextDomainsLocked(*state);
    ErrorCode handleError = ErrorCode::Ok;
    const Handle undoHandle = handles().insert(HandleType::EditUndo, command,
                                               pageBorrow.documentOwner,
                                               pageBorrow.documentOwner,
                                               &handleError);
    if (!undoHandle) {
        command.reset();
        setLastError(handleError);
        return handleError;
    }
    state->editUndoHandles.push_back(undoHandle);
    state->pendingEditUndoHandles.push_back(undoHandle);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

namespace {

ErrorCode setImageMatrix(Handle page, Handle image,
                         const std::function<void(const EditableObjectRecord&,
                                                  const FS_MATRIX&, FS_MATRIX*)>& transform) {
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    if (!pageBorrow) return pageBorrow.error;
    const BorrowedHandle imageBorrow = handles().borrow(
        image, HandleType::PageObject, pageBorrow.documentOwner, page);
    if (!imageBorrow) return imageBorrow.error;
    auto state = pageBorrow.as<PageState>();
    auto target = imageBorrow.as<PageObjectState>();
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (target->type != FPDF_PAGEOBJ_IMAGE) {
        setLastError(ErrorCode::WrongType);
        return ErrorCode::WrongType;
    }
    if (!state->editableInventoryValid ||
        target->inventoryEpoch != state->editableInventoryEpoch) {
        setLastError(ErrorCode::StaleHandle);
        return ErrorCode::StaleHandle;
    }
    auto found = std::find_if(state->editableObjects.begin(), state->editableObjects.end(),
        [&](const EditableObjectRecord& value) {
            return value.localId == target->localId && value.type == FPDF_PAGEOBJ_IMAGE;
        });
    if (found == state->editableObjects.end()) {
        setLastError(ErrorCode::StaleHandle);
        return ErrorCode::StaleHandle;
    }
    if (!found->formAncestors.empty()) {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    FS_MATRIX before{1, 0, 0, 1, 0, 0};
    if (!FPDFPageObj_GetMatrix(found->object, &before)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    FS_MATRIX after = before;
    transform(*found, before, &after);
    if (!FPDFPageObj_SetMatrix(found->object, &after) ||
        !FPDFPage_GenerateContent(state->pdf)) {
        (void)FPDFPageObj_SetMatrix(found->object, &before);
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    auto command = std::make_shared<EditUndoState>();
    command->kind = EditUndoState::Kind::ImageMatrix;
    command->page = state;
    command->pageHandle = page;
    command->object = found->object;
    command->objectType = FPDF_PAGEOBJ_IMAGE;
    command->alignedObjects.push_back(found->object);
    command->alignedFormAncestors.push_back({});
    command->beforeMatrices.push_back(before);
    command->afterMatrices.push_back(after);
    invalidateEditableObjects(page, *state);
    rebuildLocked(*state);
    recognizeTextDomainsLocked(*state);
    state->dirtyFunctions |= uint32_t{1} << 6;  // PDF.EditStatus.IMAGE
    state->stagedEpoch = ++state->document->mutationEpoch;
    ErrorCode handleError = ErrorCode::Ok;
    const Handle undoHandle = handles().insert(HandleType::EditUndo, command,
                                               pageBorrow.documentOwner,
                                               pageBorrow.documentOwner, &handleError);
    if (!undoHandle) {
        setLastError(handleError);
        return handleError;
    }
    state->editUndoHandles.push_back(undoHandle);
    state->pendingEditUndoHandles.push_back(undoHandle);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

}  // namespace

ErrorCode resizeImageObject(Handle page, Handle image, float left, float bottom,
                            float right, float top) {
    if (!std::isfinite(left) || !std::isfinite(bottom) || !std::isfinite(right) ||
        !std::isfinite(top) || !(left < right) || !(bottom < top)) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    return setImageMatrix(page, image,
        [=](const EditableObjectRecord&, const FS_MATRIX&, FS_MATRIX* after) {
            *after = FS_MATRIX{right - left, 0, 0, top - bottom, left, bottom};
        });
}

ErrorCode rotateImageObjectClockwise(Handle page, Handle image) {
    return setImageMatrix(page, image,
        [](const EditableObjectRecord& record, const FS_MATRIX& before, FS_MATRIX* after) {
            const float centerX = (record.pageLeft + record.pageRight) * 0.5f;
            const float centerY = (record.pageBottom + record.pageTop) * 0.5f;
            after->a = before.b;
            after->b = -before.a;
            after->c = before.d;
            after->d = -before.c;
            after->e = centerX + before.f - centerY;
            after->f = centerY - before.e + centerX;
        });
}

ErrorCode reverseImageObjectHorizontal(Handle page, Handle image) {
    return setImageMatrix(page, image,
        [](const EditableObjectRecord& record, const FS_MATRIX& before, FS_MATRIX* after) {
            const float centerX = (record.pageLeft + record.pageRight) * 0.5f;
            after->a = -before.a;
            after->b = before.b;
            after->c = -before.c;
            after->d = before.d;
            after->e = 2.0f * centerX - before.e;
            after->f = before.f;
        });
}

namespace {

EditableObjectRecord* focusedRecordLocked(PageState& state) {
    if (!state.focusedTextObjectId) return nullptr;
    auto found = std::find_if(state.editableObjects.begin(), state.editableObjects.end(),
        [&](const EditableObjectRecord& value) {
            return value.type == FPDF_PAGEOBJ_TEXT &&
                   value.localId == state.focusedTextObjectId;
        });
    return found == state.editableObjects.end() ? nullptr : &*found;
}

EditableTextDomain* focusedDomainLocked(PageState& state) {
    if (state.focusedTextDomain < 0 ||
        state.focusedTextDomain >= static_cast<int>(state.textDomains.size())) return nullptr;
    return &state.textDomains[static_cast<std::size_t>(state.focusedTextDomain)];
}

std::u16string objectText(FPDF_TEXTPAGE textPage, FPDF_PAGEOBJECT object) {
    if (!textPage || !object) return {};
    const unsigned long bytes = FPDFTextObj_GetText(object, textPage, nullptr, 0);
    std::vector<FPDF_WCHAR> buffer((bytes + sizeof(FPDF_WCHAR) - 1) /
                                  sizeof(FPDF_WCHAR));
    if (bytes) (void)FPDFTextObj_GetText(object, textPage, buffer.data(), bytes);
    std::u16string result;
    for (FPDF_WCHAR value : buffer) {
        if (!value) break;
        result.push_back(static_cast<char16_t>(value));
    }
    return restoreLogicalRtlRuns(result);
}

std::u16string objectTextLocked(PageState& state, FPDF_PAGEOBJECT object) {
    const auto override = state.shapedLogicalText.find(object);
    if (override != state.shapedLogicalText.end()) return override->second;
    FPDF_TEXTPAGE textPage = FPDFText_LoadPage(state.pdf);
    if (!textPage) return {};
    std::u16string result = objectText(textPage, object);
    FPDFText_ClosePage(textPage);
    return result;
}

struct DomainTextRun {
    uint64_t localId = 0;
    int lineIndex = 0;
    int start = 0;
    int end = 0;
};

struct DomainTextLayout {
    std::u16string text;
    std::vector<DomainTextRun> runs;
};

bool buildDomainTextLocked(PageState& state, DomainTextLayout* result) {
    EditableTextDomain* domain = focusedDomainLocked(state);
    if (!domain || !result) return false;
    FPDF_TEXTPAGE textPage = FPDFText_LoadPage(state.pdf);
    if (!textPage) return false;
    result->text.clear();
    result->runs.clear();
    for (std::size_t lineIndex = 0; lineIndex < domain->lines.size(); ++lineIndex) {
        if (lineIndex != 0) result->text.push_back(u'\n');
        for (uint64_t id : domain->lines[lineIndex].objectIds) {
            const auto found = std::find_if(state.editableObjects.begin(),
                                            state.editableObjects.end(),
                [&](const EditableObjectRecord& item) {
                    return item.type == FPDF_PAGEOBJ_TEXT && item.localId == id;
                });
            if (found == state.editableObjects.end()) continue;
            DomainTextRun run;
            run.localId = id;
            run.lineIndex = static_cast<int>(lineIndex);
            run.start = static_cast<int>(result->text.size());
            const auto override = state.shapedLogicalText.find(found->object);
            result->text.append(override != state.shapedLogicalText.end()
                ? override->second : objectText(textPage, found->object));
            run.end = static_cast<int>(result->text.size());
            result->runs.push_back(run);
        }
    }
    FPDFText_ClosePage(textPage);
    return true;
}

const DomainTextRun* domainRunById(const DomainTextLayout& layout, uint64_t id) {
    const auto found = std::find_if(layout.runs.begin(), layout.runs.end(),
        [&](const DomainTextRun& run) { return run.localId == id; });
    return found == layout.runs.end() ? nullptr : &*found;
}

const DomainTextRun* domainRunAtPosition(const DomainTextLayout& layout, int position,
                                         uint64_t preferredId = 0) {
    if (preferredId) {
        const DomainTextRun* preferred = domainRunById(layout, preferredId);
        if (preferred && position >= preferred->start && position <= preferred->end)
            return preferred;
    }
    for (const auto& run : layout.runs) {
        if (position >= run.start && position < run.end) return &run;
    }
    for (auto item = layout.runs.rbegin(); item != layout.runs.rend(); ++item) {
        if (position == item->end) return &*item;
    }
    return nullptr;
}

EditableObjectRecord* textRecordById(PageState& state, uint64_t id) {
    const auto found = std::find_if(state.editableObjects.begin(), state.editableObjects.end(),
        [&](const EditableObjectRecord& record) {
            return record.type == FPDF_PAGEOBJ_TEXT && record.localId == id;
        });
    return found == state.editableObjects.end() ? nullptr : &*found;
}

int domainPositionForObjectPointLocked(PageState& state,
                                       const EditableObjectRecord& record,
                                       float x, float y) {
    DomainTextLayout layout;
    if (!buildDomainTextLocked(state, &layout)) return -1;
    const DomainTextRun* run = domainRunById(layout, record.localId);
    if (!run) return -1;
    std::vector<FocusedGlyph> glyphs;
    const int local = buildFocusedGlyphsLocked(state,
                                                const_cast<EditableObjectRecord*>(&record),
                                                &glyphs)
                          ? nearestGlyphBoundary(glyphs, x, y)
                          : run->end - run->start;
    return run->start + std::clamp(local, 0, run->end - run->start);
}

EditableObjectRecord* closestFocusedDomainRecordLocked(PageState& state, float x, float y) {
    EditableTextDomain* domain = focusedDomainLocked(state);
    if (!domain) return nullptr;
    EditableObjectRecord* best = nullptr;
    float bestDistance = INFINITY;
    for (uint64_t id : domain->objectIds) {
        EditableObjectRecord* record = textRecordById(state, id);
        if (!record) continue;
        const float dx = x < record->pageLeft ? record->pageLeft - x
                       : x > record->pageRight ? x - record->pageRight : 0.0f;
        const float dy = y < record->pageBottom ? record->pageBottom - y
                       : y > record->pageTop ? y - record->pageTop : 0.0f;
        const float distance = dx * dx + dy * dy;
        if (distance < bestDistance) {
            bestDistance = distance;
            best = record;
            if (distance == 0) break;
        }
    }
    return best;
}

void markTextChangedLocked(PageState& state) {
    state.dirtyFunctions |= (uint32_t{1} << 1);  // PDF.EditStatus.TEXT
    state.stagedEpoch = ++state.document->mutationEpoch;
    state.hasFocus = state.focusedTextObjectId != 0;
}

bool focusedEditorBoundsLocked(PageState& state, Bounds* result) {
    EditableTextDomain* domain = focusedDomainLocked(state);
    if (!domain || !result) return false;
    const Matrix basisToPage{domain->matrix[0], domain->matrix[3],
                             domain->matrix[1], domain->matrix[4],
                             domain->matrix[2], domain->matrix[5]};
    const bool stableFrame = state.textReflowAnchor != nullptr;
    *result = pageBoundsInBasis(
        {stableFrame ? state.textReflowLeft : domain->left,
         stableFrame && state.textReflowHasLayoutBottom
             ? state.textReflowLayoutBottom : domain->bottom,
         stableFrame ? state.textReflowRight : domain->right,
         domain->top}, basisToPage);
    return std::isfinite(result->left) && std::isfinite(result->bottom) &&
           std::isfinite(result->right) && std::isfinite(result->top) &&
           result->right >= result->left && result->top >= result->bottom;
}

void includeCommandDamage(EditUndoState& command, const Bounds& value) {
    if (!command.hasDamage) {
        command.damage = value;
        command.hasDamage = true;
        return;
    }
    include(value, &command.damage.left, &command.damage.bottom,
            &command.damage.right, &command.damage.top);
}

void includeFocusedEditorDamageLocked(EditUndoState& command, PageState& state) {
    Bounds value;
    if (focusedEditorBoundsLocked(state, &value)) includeCommandDamage(command, value);
}

uint32_t dirtyFunctionForPageObject(int type) {
    return uint32_t{1} << (type == FPDF_PAGEOBJ_IMAGE ? 6 : 4);
}

bool focusObjectLocked(PageState& state, FPDF_PAGEOBJECT object) {
    state.focusedTextObjectId = 0;
    state.focusedTextDomain = -1;
    for (const auto& item : state.editableObjects) {
        if (item.type != FPDF_PAGEOBJ_TEXT || item.object != object) continue;
        state.focusedTextObjectId = item.localId;
        for (std::size_t i = 0; i < state.textDomains.size(); ++i) {
            const auto& ids = state.textDomains[i].objectIds;
            if (std::find(ids.begin(), ids.end(), item.localId) != ids.end()) {
                state.focusedTextDomain = static_cast<int>(i);
                break;
            }
        }
        break;
    }
    state.hasFocus = state.focusedTextObjectId != 0 && state.focusedTextDomain >= 0;
    return state.hasFocus;
}

ErrorCode applyEditUndoLocked(EditUndoState& command, bool after) {
    PageState& state = *command.page;
    if (command.kind == EditUndoState::Kind::ObjectPresence) {
        const bool targetPresent = !after;
        const bool currentPresent = !command.ownsDetachedObject;
        if (targetPresent && !currentPresent) {
            const int count = FPDFPage_CountObjects(state.pdf);
            const std::size_t index = static_cast<std::size_t>(
                std::clamp(command.pageObjectIndex, 0, std::max(count, 0)));
            if (!FPDFPage_InsertObjectAtIndex(state.pdf, command.object, index))
                return ErrorCode::General;
            command.ownsDetachedObject = false;
        } else if (!targetPresent && currentPresent) {
            if (!FPDFPage_RemoveObject(state.pdf, command.object)) return ErrorCode::General;
            command.ownsDetachedObject = true;
        }
        if (!FPDFPage_GenerateContent(state.pdf)) return ErrorCode::General;
        invalidateEditableObjects(command.pageHandle, state);
        rebuildLocked(state);
        recognizeTextDomainsLocked(state);
        state.dirtyFunctions |= dirtyFunctionForPageObject(command.objectType);
        state.stagedEpoch = ++state.document->mutationEpoch;
        command.currentlyAfter = after;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    if (command.kind == EditUndoState::Kind::DomainText) {
        // Presence changes are made by pointer. Insertions are ordered by the
        // original page index so undo restores the PDF content-stream z-order.
        std::vector<DomainUndoItem*> insertions;
        for (auto& item : command.domainItems) {
            const bool targetPresent = after ? item.afterPresent : item.beforePresent;
            if (!targetPresent && !item.ownsDetachedObject) {
                if (!removeDomainObject(item, state)) {
                    setLastError(ErrorCode::General);
                    return ErrorCode::General;
                }
                item.ownsDetachedObject = true;
            } else if (targetPresent && item.ownsDetachedObject) {
                insertions.push_back(&item);
            }
        }
        std::sort(insertions.begin(), insertions.end(), [](const auto* lhs, const auto* rhs) {
            return lhs->pageObjectIndex < rhs->pageObjectIndex;
        });
        for (DomainUndoItem* item : insertions) {
            if (!insertDomainObject(*item, state)) {
                setLastError(ErrorCode::General);
                return ErrorCode::General;
            }
            item->ownsDetachedObject = false;
        }
        for (auto& item : command.domainItems) {
            const bool targetPresent = after ? item.afterPresent : item.beforePresent;
            if (!targetPresent) continue;
            const FS_MATRIX& targetMatrix = after ? item.afterMatrix : item.beforeMatrix;
            if (!applyDomainItemText(item, after) ||
                (item.hasMatrix && !FPDFPageObj_SetMatrix(item.object, &targetMatrix)) ||
                !markFormWriteback(item.object, item.formAncestors)) {
                setLastError(ErrorCode::General);
                return ErrorCode::General;
            }
            const bool textUnchanged = item.beforePresent && item.afterPresent &&
                                       !item.afterFont &&
                                       item.beforeText == item.afterText;
            if (after && item.afterFont && !item.afterCharcodes.empty()) {
                state.shapedLogicalText[item.object] = item.afterText;
            } else if (textUnchanged && item.hasBeforeLogicalOverride) {
                state.shapedLogicalText[item.object] = item.beforeLogicalOverride;
            } else if (!after && item.hasBeforeLogicalOverride) {
                state.shapedLogicalText[item.object] = item.beforeLogicalOverride;
            } else {
                state.shapedLogicalText.erase(item.object);
            }
        }
        if (!FPDFPage_GenerateContent(state.pdf)) {
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
        invalidateEditableObjects(command.pageHandle, state);
        rebuildLocked(state);
        recognizeTextDomainsLocked(state);
        focusObjectLocked(state, command.domainFocusObject);
        state.textReflowHasLayoutBottom = after
            ? command.afterHasLayoutBottom : command.beforeHasLayoutBottom;
        state.textReflowLayoutBottom = after
            ? command.afterLayoutBottom : command.beforeLayoutBottom;
        DomainTextLayout layout;
        const int requestedStart = after ? command.afterStart : command.beforeStart;
        const int requestedEnd = after ? command.afterEnd : command.beforeEnd;
        if (state.hasFocus && buildDomainTextLocked(state, &layout)) {
            const int length = static_cast<int>(layout.text.size());
            state.textSelectionStart = std::clamp(requestedStart, 0, length);
            state.textSelectionEnd = std::clamp(requestedEnd,
                                                state.textSelectionStart, length);
        } else {
            state.textSelectionStart = state.textSelectionEnd = 0;
        }
        command.currentlyAfter = after;
        markTextChangedLocked(state);
        includeFocusedEditorDamageLocked(command, state);
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    if (command.kind != EditUndoState::Kind::Text) {
        bool applied = false;
        if (command.kind == EditUndoState::Kind::LayoutMatrices ||
            command.kind == EditUndoState::Kind::ImageMatrix) {
            const auto& matrices = after ? command.afterMatrices : command.beforeMatrices;
            applied = matrices.size() == command.alignedObjects.size();
            for (std::size_t i = 0; applied && i < matrices.size(); ++i) {
                applied = FPDFPageObj_SetMatrix(command.alignedObjects[i], &matrices[i]) != 0;
                if (applied && i < command.alignedFormAncestors.size()) {
                    applied = markFormWriteback(command.alignedObjects[i],
                                                command.alignedFormAncestors[i]);
                }
            }
        } else if (command.kind == EditUndoState::Kind::FontSize) {
            applied = FPDFTextObj_SetFontSize(
                          command.object,
                          after ? command.afterFontSize : command.beforeFontSize) != 0;
        } else if (command.kind == EditUndoState::Kind::FillColor) {
            const uint32_t color = after ? command.afterColor : command.beforeColor;
            applied = FPDFPageObj_SetFillColor(command.object, (color >> 16) & 0xff,
                                               (color >> 8) & 0xff, color & 0xff,
                                               (color >> 24) & 0xff) != 0;
        } else {
            SetTextObjectFont setFont = setTextObjectFontExtension();
            applied = setFont && setFont(command.object,
                                         after ? command.afterFont : command.beforeFont);
        }
        const bool primaryWriteback = command.kind == EditUndoState::Kind::LayoutMatrices ||
                                      command.kind == EditUndoState::Kind::ImageMatrix ||
                                      markFormWriteback(command.object, command.formAncestors);
        if (!applied || !primaryWriteback ||
            !FPDFPage_GenerateContent(state.pdf)) {
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
        invalidateEditableObjects(command.pageHandle, state);
        rebuildLocked(state);
        recognizeTextDomainsLocked(state);
        if (command.kind == EditUndoState::Kind::ImageMatrix) {
            state.dirtyFunctions |= dirtyFunctionForPageObject(command.objectType);
            state.stagedEpoch = ++state.document->mutationEpoch;
        } else {
            state.focusedTextObjectId = 0;
            state.focusedTextDomain = -1;
            for (const auto& item : state.editableObjects) {
                if (item.object == command.object) {
                    state.focusedTextObjectId = item.localId;
                    for (std::size_t i = 0; i < state.textDomains.size(); ++i) {
                        const auto& ids = state.textDomains[i].objectIds;
                        if (std::find(ids.begin(), ids.end(), item.localId) != ids.end()) {
                            state.focusedTextDomain = static_cast<int>(i);
                            break;
                        }
                    }
                    break;
                }
            }
            markTextChangedLocked(state);
        }
        command.currentlyAfter = after;
        includeFocusedEditorDamageLocked(command, state);
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    const bool targetPresent = after ? command.afterPresent : command.beforePresent;
    const std::u16string& targetText = after ? command.afterText : command.beforeText;
    const bool targetDeleted = after && command.afterDeleted;
    const bool currentPresent = command.ownsDetachedObject == false;
    if (targetPresent && !currentPresent) {
        const int count = FPDFPage_CountObjects(state.pdf);
        const std::size_t index = static_cast<std::size_t>(
            std::clamp(command.pageObjectIndex, 0, std::max(count, 0)));
        if (!FPDFPage_InsertObjectAtIndex(state.pdf, command.object, index)) {
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
        command.ownsDetachedObject = false;
    } else if (!targetPresent && currentPresent) {
        if (!FPDFPage_RemoveObject(state.pdf, command.object)) {
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
        command.ownsDetachedObject = true;
    }
    if (targetPresent) {
        FPDF_FONT targetFont = after ? command.afterFont : command.beforeFont;
        SetTextObjectFont setFont = setTextObjectFontExtension();
        if ((targetFont && (!setFont || !setFont(command.object, targetFont))) ||
            !setObjectText(command.object, targetText) ||
            !markFormWriteback(command.object, command.formAncestors)) {
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
    }
    if (!FPDFPage_GenerateContent(state.pdf)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    invalidateEditableObjects(command.pageHandle, state);
    rebuildLocked(state);
    recognizeTextDomainsLocked(state);
    state.focusedTextObjectId = 0;
    state.focusedTextDomain = -1;
    if (targetPresent && !targetDeleted && !targetText.empty()) {
        for (const auto& item : state.editableObjects) {
            if (item.object != command.object) continue;
            state.focusedTextObjectId = item.localId;
            for (std::size_t i = 0; i < state.textDomains.size(); ++i) {
                const auto& ids = state.textDomains[i].objectIds;
                if (std::find(ids.begin(), ids.end(), item.localId) != ids.end()) {
                    state.focusedTextDomain = static_cast<int>(i);
                    break;
                }
            }
            break;
        }
    }
    const int length = static_cast<int>(targetText.size());
    int domainBase = 0;
    DomainTextLayout restoredLayout;
    if (state.focusedTextObjectId && buildDomainTextLocked(state, &restoredLayout)) {
        const DomainTextRun* restoredRun =
            domainRunById(restoredLayout, state.focusedTextObjectId);
        if (restoredRun) domainBase = restoredRun->start;
    }
    state.textSelectionStart = domainBase + std::clamp(
        after ? command.afterStart : command.beforeStart, 0, length);
    state.textSelectionEnd = domainBase + std::clamp(
        after ? command.afterEnd : command.beforeEnd,
        state.textSelectionStart - domainBase, length);
    command.currentlyAfter = after;
    markTextChangedLocked(state);
    includeFocusedEditorDamageLocked(command, state);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

}  // namespace

ErrorCode focusTextAtPoint(Handle page, float x, float y, int, bool* focused) {
    if (!focused || !std::isfinite(x) || !std::isfinite(y)) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    if (!state->textDomainsRecognized) recognizeTextDomainsLocked(*state);
    state->focusedTextDomain = -1;
    state->focusedTextObjectId = 0;
    for (auto item = state->editableObjects.rbegin(); item != state->editableObjects.rend(); ++item) {
        if (item->type != FPDF_PAGEOBJ_TEXT || x < item->pageLeft || x > item->pageRight ||
            y < item->pageBottom || y > item->pageTop) continue;
        state->focusedTextObjectId = item->localId;
        break;
    }
    // The original host first hit-tests the recognized editor envelope, then
    // lets native choose the closest text line. This is observable for Form
    // editors whose container envelope contains blank space between children.
    if (!state->focusedTextObjectId) {
        int nearestDomain = -1;
        float nearestDistance = INFINITY;
        float nearestArea = INFINITY;
        for (std::size_t index = 0; index < state->textDomains.size(); ++index) {
            EditableTextDomain& domain = state->textDomains[index];
            Matrix pageToBasis;
            if (!inverse(orientationMatrix(domain.orientation), &pageToBasis)) continue;
            float bx = 0, by = 0;
            transform(pageToBasis, x, y, &bx, &by);
            const float dx = bx < domain.left ? domain.left - bx
                           : bx > domain.right ? bx - domain.right : 0.0f;
            const float dy = by < domain.bottom ? domain.bottom - by
                           : by > domain.top ? by - domain.top : 0.0f;
            const float distance = dx * dx + dy * dy;
            // Preserve exact-hit ordering.  Only use distance ranking for the
            // original editor's border frame outside the painted rectangle.
            if (distance == 0.0f) {
                const float area = std::max(0.0f, domain.right - domain.left) *
                                   std::max(0.0f, domain.top - domain.bottom);
                // Nested structural results in the original select the
                // deepest editable child. Area is the observable equivalent
                // when parent/child rectangles overlap at a blank point.
                if (nearestDistance != 0.0f || area < nearestArea) {
                    nearestDomain = static_cast<int>(index);
                    nearestDistance = 0.0f;
                    nearestArea = area;
                }
                continue;
            }
            if (dx <= kEditorBorderContentDistance &&
                dy <= kEditorBorderContentDistance &&
                (distance < nearestDistance ||
                 (distance == nearestDistance &&
                  (domain.right - domain.left) * (domain.top - domain.bottom) <
                      nearestArea))) {
                nearestDomain = static_cast<int>(index);
                nearestDistance = distance;
                nearestArea = (domain.right - domain.left) *
                              (domain.top - domain.bottom);
            }
        }
        if (nearestDomain >= 0) {
            state->focusedTextDomain = nearestDomain;
            EditableObjectRecord* nearest = closestFocusedDomainRecordLocked(*state, x, y);
            if (nearest) state->focusedTextObjectId = nearest->localId;
        }
    }
    if (state->focusedTextObjectId) {
        if (state->focusedTextDomain < 0) {
            for (std::size_t i = 0; i < state->textDomains.size(); ++i) {
                const auto& ids = state->textDomains[i].objectIds;
                if (std::find(ids.begin(), ids.end(), state->focusedTextObjectId) != ids.end()) {
                    state->focusedTextDomain = static_cast<int>(i);
                    break;
                }
            }
        }
        EditableObjectRecord* record = focusedRecordLocked(*state);
        const int position = record
            ? std::max(0, domainPositionForObjectPointLocked(*state, *record, x, y)) : 0;
        state->textSelectionStart = state->textSelectionEnd = position;
        state->textSelectionAnchor = position;
        state->textSelectionDragging = true;
    }
    state->hasFocus = state->focusedTextObjectId != 0;
    if (state->textReflowAnchor && state->focusedTextDomain >= 0) {
        bool anchorStillInDomain = false;
        for (uint64_t id : state->textDomains[static_cast<std::size_t>(
                 state->focusedTextDomain)].objectIds) {
            EditableObjectRecord* item = textRecordById(*state, id);
            if (item && item->object == state->textReflowAnchor) {
                anchorStillInDomain = true;
                break;
            }
        }
        if (!anchorStillInDomain) state->textReflowAnchor = nullptr;
    } else if (state->focusedTextDomain < 0) {
        state->textReflowAnchor = nullptr;
    }
    *focused = state->hasFocus;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode moveTextSelectionAtPoint(Handle page, float x, float y, int, bool* handled) {
    if (!handled || !std::isfinite(x) || !std::isfinite(y)) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = closestFocusedDomainRecordLocked(*state, x, y);
    if (!record || !state->textSelectionDragging) {
        *handled = false;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    const int position = domainPositionForObjectPointLocked(*state, *record, x, y);
    if (position < 0) return ErrorCode::NotFound;
    state->textSelectionStart = std::min(state->textSelectionAnchor, position);
    state->textSelectionEnd = std::max(state->textSelectionAnchor, position);
    *handled = true;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode endTextSelectionDrag(Handle page, bool* handled) {
    if (!handled) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    *handled = state->textSelectionDragging && focusedRecordLocked(*state);
    state->textSelectionDragging = false;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextHoverType(Handle page, float x, float y, int* type) {
    if (!type || !std::isfinite(x) || !std::isfinite(y)) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    *type = 0;
    for (const auto& item : state->editableObjects) {
        if (item.type == FPDF_PAGEOBJ_TEXT && x >= item.pageLeft && x <= item.pageRight &&
            y >= item.pageBottom && y <= item.pageTop) {
            *type = 1;
            break;
        }
    }
    state->hasHover = *type != 0;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

namespace {
bool wordUnit(char16_t value) {
    return (value >= u'a' && value <= u'z') || (value >= u'A' && value <= u'Z') ||
           (value >= u'0' && value <= u'9') || value == u'_';
}

ErrorCode selectTextSpanAtPoint(Handle page, float x, float y, bool wholeObject, bool* selected) {
    if (!selected) return ErrorCode::InvalidArgument;
    bool focused = false;
    ErrorCode code = focusTextAtPoint(page, x, y, 0, &focused);
    if (code != ErrorCode::Ok || !focused) {
        if (code == ErrorCode::Ok) *selected = false;
        return code;
    }
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    if (!record) return ErrorCode::NotFound;
    DomainTextLayout layout;
    if (!buildDomainTextLocked(*state, &layout)) return ErrorCode::NotFound;
    const DomainTextRun* run = domainRunById(layout, record->localId);
    if (!run) return ErrorCode::NotFound;
    const std::u16string& text = wholeObject ? objectTextLocked(*state, record->object)
                                             : layout.text;
    if (text.empty()) {
        *selected = false;
        return ErrorCode::Ok;
    }
    int start = wholeObject ? run->start : 0;
    int end = wholeObject ? run->end : static_cast<int>(text.size());
    if (!wholeObject) {
        int point = std::clamp(state->textSelectionAnchor, 0, end);
        if (point == end) --point;
        if (!wordUnit(text[point])) {
            start = point;
            end = point + 1;
        } else {
            start = point;
            end = point + 1;
            while (start > 0 && wordUnit(text[start - 1])) --start;
            while (end < static_cast<int>(text.size()) && wordUnit(text[end])) ++end;
        }
    }
    state->textSelectionStart = start;
    state->textSelectionEnd = end;
    state->textSelectionAnchor = start;
    state->textSelectionDragging = false;
    *selected = true;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}
}  // namespace

ErrorCode selectTextWordAtPoint(Handle page, float x, float y, bool* selected) {
    return selectTextSpanAtPoint(page, x, y, false, selected);
}

ErrorCode selectTextObjectAtPoint(Handle page, float x, float y, bool* selected) {
    return selectTextSpanAtPoint(page, x, y, true, selected);
}

ErrorCode getTextFocusState(Handle page, int* type, int* status, int* count,
                            EditableObjectRecord* geometry) {
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    EditableTextDomain* domain = focusedDomainLocked(*state);
    const bool hasFocus = record && domain;
    if (type) *type = hasFocus ? 1 : 0;
    if (status) *status = hasFocus ? 2 : 0;
    if (count) *count = hasFocus ? 1 : 0;
    if (geometry) {
        if (!hasFocus) {
            setLastError(ErrorCode::NotFound);
            return ErrorCode::NotFound;
        }
        // The original editor decor frames the semantic editor, while style
        // getters still use the clicked TextObject.  Domain bounds live in the
        // orientation basis, so transform all four corners back to page space
        // for GetFocusObjectRect's historical RectF contract.
        *geometry = *record;
        const Matrix basisToPage{domain->matrix[0], domain->matrix[3],
                                 domain->matrix[1], domain->matrix[4],
                                 domain->matrix[2], domain->matrix[5]};
        const bool stableFrame = state->textReflowAnchor != nullptr;
        const Bounds pageBounds = pageBoundsInBasis(
            {stableFrame ? state->textReflowLeft : domain->left,
             stableFrame && state->textReflowHasLayoutBottom
                 ? state->textReflowLayoutBottom : domain->bottom,
             stableFrame ? state->textReflowRight : domain->right,
             domain->top}, basisToPage);
        geometry->pageLeft = pageBounds.left;
        geometry->pageBottom = pageBounds.bottom;
        geometry->pageRight = pageBounds.right;
        geometry->pageTop = pageBounds.top;
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getFocusedText(Handle page, std::u16string* result) {
    if (!result) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    if (!record) {
        setLastError(ErrorCode::NotFound);
        return ErrorCode::NotFound;
    }
    *result = objectTextLocked(*state, record->object);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getFocusedDomainText(Handle page, std::u16string* result) {
    if (!result) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    DomainTextLayout layout;
    if (!buildDomainTextLocked(*state, &layout)) {
        setLastError(ErrorCode::NotFound);
        return ErrorCode::NotFound;
    }
    *result = std::move(layout.text);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextSelection(Handle page, int* start, int* end) {
    if (!start || !end) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!focusedDomainLocked(*state)) return ErrorCode::NotFound;
    *start = state->textSelectionStart;
    *end = state->textSelectionEnd;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode setTextSelection(Handle page, int start, int end) {
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!focusedDomainLocked(*state)) return ErrorCode::NotFound;
    DomainTextLayout layout;
    if (!buildDomainTextLocked(*state, &layout)) return ErrorCode::NotFound;
    const int length = static_cast<int>(layout.text.size());
    if (start < 0 || end < 0 || start > length || end > length) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    if (!isUnicodeTextBoundary(layout.text, start) ||
        !isUnicodeTextBoundary(layout.text, end)) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    state->textSelectionStart = std::min(start, end);
    state->textSelectionEnd = std::max(start, end);
    const DomainTextRun* activeRun = domainRunAtPosition(
        layout, state->textSelectionStart, state->focusedTextObjectId);
    if (activeRun) state->focusedTextObjectId = activeRun->localId;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextRangeAtPoint(Handle page, float x, float y, int* index) {
    if (!index) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = closestFocusedDomainRecordLocked(*state, x, y);
    if (!record) return ErrorCode::NotFound;
    *index = domainPositionForObjectPointLocked(*state, *record, x, y);
    if (*index < 0) return ErrorCode::NotFound;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextCaretGeometry(Handle page, int position, TextCaretGeometry* geometry) {
    if (!geometry) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    DomainTextLayout layout;
    if (!buildDomainTextLocked(*state, &layout)) return ErrorCode::NotFound;
    const int textLength = static_cast<int>(layout.text.size());
    if (position < 0 || position > textLength) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    if (!isUnicodeTextBoundary(layout.text, position)) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    const DomainTextRun* run = domainRunAtPosition(layout, position,
                                                   state->focusedTextObjectId);
    if (!run) return ErrorCode::NotFound;
    EditableObjectRecord* record = textRecordById(*state, run->localId);
    if (!record) return ErrorCode::NotFound;
    const int localPosition = std::clamp(position - run->start, 0, run->end - run->start);
    std::vector<FocusedGlyph> glyphs;
    if (!buildFocusedGlyphsLocked(*state, record, &glyphs)) return ErrorCode::NotFound;
    const FocusedGlyph* selected = &glyphs.back();
    float x = selected->endX, y = selected->endY;
    float fraction = 1.0f;
    for (const auto& glyph : glyphs) {
        if (localPosition >= glyph.utf16Start && localPosition <= glyph.utf16End) {
            selected = &glyph;
            const int span = std::max(1, glyph.utf16End - glyph.utf16Start);
            fraction = static_cast<float>(localPosition - glyph.utf16Start) / span;
            x = glyph.originX + (glyph.endX - glyph.originX) * fraction;
            y = glyph.originY + (glyph.endY - glyph.originY) * fraction;
            break;
        }
    }
    const float nx = -selected->unitY, ny = selected->unitX;
    geometry->bottomX = x + nx * selected->bottomOffset;
    geometry->bottomY = y + ny * selected->bottomOffset;
    geometry->topX = x + nx * selected->topOffset;
    geometry->topY = y + ny * selected->topOffset;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextCharacterGeometry(Handle page, int index, TextCharacterGeometry* geometry) {
    if (!geometry) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    DomainTextLayout layout;
    if (!buildDomainTextLocked(*state, &layout)) return ErrorCode::NotFound;
    if (!isUnicodeTextBoundary(layout.text, index)) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    const DomainTextRun* run = domainRunAtPosition(layout, index);
    if (!run || index == run->end) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    EditableObjectRecord* record = textRecordById(*state, run->localId);
    if (!record) return ErrorCode::NotFound;
    const int localIndex = index - run->start;
    std::vector<FocusedGlyph> glyphs;
    if (!buildFocusedGlyphsLocked(*state, record, &glyphs)) return ErrorCode::NotFound;
    const FocusedGlyph* selected = nullptr;
    for (const auto& glyph : glyphs) {
        if (localIndex >= glyph.utf16Start && localIndex < glyph.utf16End) {
            selected = &glyph;
            break;
        }
    }
    if (!selected) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    const float nx = -selected->unitY, ny = selected->unitX;
    geometry->quad[0] = selected->originX + nx * selected->bottomOffset;
    geometry->quad[1] = selected->originY + ny * selected->bottomOffset;
    geometry->quad[2] = selected->endX + nx * selected->bottomOffset;
    geometry->quad[3] = selected->endY + ny * selected->bottomOffset;
    geometry->quad[4] = selected->endX + nx * selected->topOffset;
    geometry->quad[5] = selected->endY + ny * selected->topOffset;
    geometry->quad[6] = selected->originX + nx * selected->topOffset;
    geometry->quad[7] = selected->originY + ny * selected->topOffset;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextSelectionDirection(Handle page, int* direction) {
    if (!direction) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    DomainTextLayout layout;
    if (!buildDomainTextLocked(*state, &layout)) return ErrorCode::NotFound;
    const int start = std::clamp(state->textSelectionStart, 0,
                                 static_cast<int>(layout.text.size()));
    const int end = std::clamp(state->textSelectionEnd, start,
                               static_cast<int>(layout.text.size()));
    std::u16string sample = layout.text.substr(start, end - start);
    if (sample.empty()) {
        const DomainTextRun* run = domainRunAtPosition(layout, start,
                                                       state->focusedTextObjectId);
        if (run) sample = layout.text.substr(run->start, run->end - run->start);
    }
    TextDirection resolvedDirection = baseTextDirection(sample);
    const DomainTextRun* selectedRun = domainRunAtPosition(
        layout, start, state->focusedTextObjectId);
    if (selectedRun) {
        const std::u16string runText = layout.text.substr(
            selectedRun->start, selectedRun->end - selectedRun->start);
        std::vector<BidiRun> bidiRuns;
        if (!runText.empty() &&
            resolveVisualBidiRuns(runText, baseTextDirection(runText), &bidiRuns)) {
            const int local = std::clamp(start - selectedRun->start, 0,
                                         static_cast<int>(runText.size()) - 1);
            for (const BidiRun& bidi : bidiRuns) {
                if (local >= bidi.logicalStart &&
                    local < bidi.logicalStart + bidi.length) {
                    resolvedDirection = bidi.direction;
                    break;
                }
            }
        }
    }
    *direction = static_cast<int>(resolvedDirection);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getTextSelectionRects(Handle page, std::vector<TextSelectionRect>* result) {
    if (!result) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    DomainTextLayout layout;
    if (!buildDomainTextLocked(*state, &layout)) return ErrorCode::NotFound;
    result->clear();
    const int selectionStart = std::min(state->textSelectionStart,
                                        state->textSelectionEnd);
    const int selectionEnd = std::max(state->textSelectionStart,
                                      state->textSelectionEnd);
    if (selectionStart == selectionEnd) {
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    int previousLine = -1;
    for (const DomainTextRun& run : layout.runs) {
        const int begin = std::max(selectionStart, run.start);
        const int end = std::min(selectionEnd, run.end);
        if (begin >= end) continue;
        EditableObjectRecord* record = textRecordById(*state, run.localId);
        std::vector<FocusedGlyph> glyphs;
        if (!record || !buildFocusedGlyphsLocked(*state, record, &glyphs)) continue;
        const std::u16string runText = layout.text.substr(run.start, run.end - run.start);
        std::vector<BidiRun> bidiRuns;
        if (!resolveVisualBidiRuns(runText, baseTextDirection(runText), &bidiRuns)) {
            bidiRuns.push_back({0, static_cast<int>(runText.size()),
                                baseTextDirection(runText)});
        }
        for (const BidiRun& bidi : bidiRuns) {
            const int bidiBegin = std::max(begin, run.start + bidi.logicalStart);
            const int bidiEnd = std::min(end, run.start + bidi.logicalStart + bidi.length);
            if (bidiBegin >= bidiEnd) continue;
            TextSelectionRect current;
            current.left = current.bottom = INFINITY;
            current.right = current.top = -INFINITY;
            current.direction = static_cast<int>(bidi.direction);
            for (const FocusedGlyph& glyph : glyphs) {
                const int glyphStart = run.start + glyph.utf16Start;
                const int glyphEnd = run.start + glyph.utf16End;
                if (glyphEnd <= bidiBegin || glyphStart >= bidiEnd) continue;
                const float nx = -glyph.unitY, ny = glyph.unitX;
                const float xs[] = {
                    glyph.originX + nx * glyph.bottomOffset,
                    glyph.endX + nx * glyph.bottomOffset,
                    glyph.endX + nx * glyph.topOffset,
                    glyph.originX + nx * glyph.topOffset,
                };
                const float ys[] = {
                    glyph.originY + ny * glyph.bottomOffset,
                    glyph.endY + ny * glyph.bottomOffset,
                    glyph.endY + ny * glyph.topOffset,
                    glyph.originY + ny * glyph.topOffset,
                };
                for (int corner = 0; corner < 4; ++corner) {
                    current.left = std::min(current.left, xs[corner]);
                    current.right = std::max(current.right, xs[corner]);
                    current.bottom = std::min(current.bottom, ys[corner]);
                    current.top = std::max(current.top, ys[corner]);
                }
            }
            if (!std::isfinite(current.left)) continue;
            if (!result->empty() && previousLine == run.lineIndex &&
                result->back().direction == current.direction) {
                TextSelectionRect& previous = result->back();
                const float gap = std::max(0.0f,
                    std::max(current.left, previous.left) -
                    std::min(current.right, previous.right));
                const float height = std::max(previous.top - previous.bottom,
                                              current.top - current.bottom);
                if (gap <= height) {
                    previous.left = std::min(previous.left, current.left);
                    previous.bottom = std::min(previous.bottom, current.bottom);
                    previous.right = std::max(previous.right, current.right);
                    previous.top = std::max(previous.top, current.top);
                    previousLine = run.lineIndex;
                    continue;
                }
            }
            result->push_back(current);
            previousLine = run.lineIndex;
        }
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

namespace {

bool reflowSpace(char16_t value) {
    return value == u' ' || value == u'\t' || value == u'\n' || value == u'\r';
}

struct ReflowStyle {
    FPDF_FONT font = nullptr;
    float fontSize = 0;
    FS_MATRIX matrix{1, 0, 0, 1, 0, 0};
    Matrix parentToPage{};
    std::vector<FPDF_PAGEOBJECT> formAncestors;
    unsigned int fillR = 0, fillG = 0, fillB = 0, fillA = 255;
    unsigned int strokeR = 0, strokeG = 0, strokeB = 0, strokeA = 255;
    FPDF_TEXT_RENDERMODE renderMode = FPDF_TEXTRENDERMODE_UNKNOWN;
    float totalAdvance = 0;
    int measuredCharacters = 0;
    float characterAdvance[128] = {0};
    unsigned int characterSamples[128] = {0};
    float originToLeft = 0;
};

bool sameReflowStyle(const ReflowStyle& lhs, const ReflowStyle& rhs) {
    return lhs.font == rhs.font && std::fabs(lhs.fontSize - rhs.fontSize) <= 0.05f &&
           lhs.formAncestors == rhs.formAncestors &&
           std::fabs(lhs.matrix.a - rhs.matrix.a) <= 0.001f &&
           std::fabs(lhs.matrix.b - rhs.matrix.b) <= 0.001f &&
           std::fabs(lhs.matrix.c - rhs.matrix.c) <= 0.001f &&
           std::fabs(lhs.matrix.d - rhs.matrix.d) <= 0.001f &&
           lhs.fillR == rhs.fillR && lhs.fillG == rhs.fillG &&
           lhs.fillB == rhs.fillB && lhs.fillA == rhs.fillA &&
           lhs.strokeR == rhs.strokeR && lhs.strokeG == rhs.strokeG &&
           lhs.strokeB == rhs.strokeB && lhs.strokeA == rhs.strokeA &&
           lhs.renderMode == rhs.renderMode;
}

struct StyledReflowUnit {
    char16_t value = 0;
    std::size_t style = 0;
};

float styledUnitAdvance(const StyledReflowUnit& unit,
                        const std::vector<ReflowStyle>& styles) {
    const ReflowStyle& style = styles[unit.style];
    if (unit.value < 128 && style.characterSamples[unit.value] != 0) {
        return style.characterAdvance[unit.value] /
               static_cast<float>(style.characterSamples[unit.value]);
    }
    float glyphWidth = 0;
    CanEncodeUnicode canEncode = canEncodeUnicodeExtension();
    if (canEncode && canEncode(style.font, static_cast<uint32_t>(unit.value)) &&
        FPDFFont_GetGlyphWidth(style.font, static_cast<uint32_t>(unit.value),
                               style.fontSize, &glyphWidth) && glyphWidth > 0.01f) {
        return glyphWidth;
    }
    return style.measuredCharacters > 0
        ? style.totalAdvance / static_cast<float>(style.measuredCharacters)
        : std::max(1.0f, style.fontSize * 0.52f);
}

struct ScopedLoadedFonts {
    std::vector<FPDF_FONT> values;
    ~ScopedLoadedFonts() {
        for (FPDF_FONT value : values) FPDFFont_Close(value);
    }
};

std::string reflowFontName(FPDF_FONT font) {
    const size_t length = font ? FPDFFont_GetBaseFontName(font, nullptr, 0) : 0;
    if (!length) return {};
    std::vector<char> buffer(length);
    return FPDFFont_GetBaseFontName(font, buffer.data(), buffer.size()) == length
        ? std::string(buffer.data()) : std::string();
}

std::size_t fallbackStyleFor(char16_t value, std::size_t preferred,
                             std::vector<ReflowStyle>* styles,
                             FPDF_DOCUMENT document,
                             ScopedLoadedFonts* loadedFonts) {
    CanEncodeUnicode canEncode = canEncodeUnicodeExtension();
    if (!styles || !canEncode || preferred >= styles->size() ||
        value == u'\n' || value == u'\r' ||
        canEncode((*styles)[preferred].font, static_cast<uint32_t>(value))) {
        return preferred;
    }
    const ReflowStyle wanted = (*styles)[preferred];
    std::size_t best = preferred;
    float bestScore = std::numeric_limits<float>::max();
    for (std::size_t index = 0; index < styles->size(); ++index) {
        const ReflowStyle& candidate = (*styles)[index];
        if (candidate.formAncestors != wanted.formAncestors ||
            !canEncode(candidate.font, static_cast<uint32_t>(value))) continue;
        const float score = std::fabs(candidate.fontSize - wanted.fontSize) * 10.0f +
            std::fabs(candidate.matrix.a - wanted.matrix.a) +
            std::fabs(candidate.matrix.b - wanted.matrix.b) +
            std::fabs(candidate.matrix.c - wanted.matrix.c) +
            std::fabs(candidate.matrix.d - wanted.matrix.d);
        if (score < bestScore) {
            bestScore = score;
            best = index;
        }
    }
    if (best != preferred) return best;
    FPDF_FONT replacement = loadReplacementFont(
        document, reflowFontName(wanted.font), static_cast<uint32_t>(value));
    if (replacement && loadedFonts) {
        ReflowStyle candidate = wanted;
        candidate.font = replacement;
        styles->push_back(candidate);
        loadedFonts->values.push_back(replacement);
        return styles->size() - 1;
    }
    return best;
}

std::vector<std::vector<StyledReflowUnit>> wrapStyledText(
        const std::vector<StyledReflowUnit>& text,
        const std::vector<ReflowStyle>& styles, float lineWidth) {
    std::vector<std::vector<StyledReflowUnit>> lines;
    std::size_t start = 0;
    while (start < text.size()) {
        float width = 0;
        std::size_t index = start;
        std::size_t lastSpace = text.size();
        bool forcedBreak = false;
        for (; index < text.size(); ++index) {
            if (text[index].value == u'\n' || text[index].value == u'\r') {
                forcedBreak = true;
                break;
            }
            const float advance = styledUnitAdvance(text[index], styles);
            if (index > start && width + advance > lineWidth) break;
            width += advance;
            if (reflowSpace(text[index].value)) lastSpace = index;
        }
        std::size_t cut = index;
        if (!forcedBreak && index < text.size() && lastSpace >= start &&
            lastSpace < index) cut = lastSpace;
        if (cut == start && !forcedBreak) cut = std::min(start + 1, text.size());
        std::size_t visibleEnd = cut;
        while (visibleEnd > start && reflowSpace(text[visibleEnd - 1].value)) --visibleEnd;
        lines.emplace_back(text.begin() + static_cast<std::ptrdiff_t>(start),
                           text.begin() + static_cast<std::ptrdiff_t>(visibleEnd));
        if (lines.back().empty()) {
            lines.back().push_back({u' ', text[start].style});
        }
        start = forcedBreak ? index + 1 : cut;
        while (start < text.size() && text[start].value != u'\n' &&
               text[start].value != u'\r' && reflowSpace(text[start].value)) ++start;
    }
    if (lines.empty()) lines.push_back({{u' ', 0}});
    return lines;
}

ErrorCode tryStyledDomainReflowLocked(Handle page,
                                       const std::shared_ptr<PageState>& state,
                                       const DomainTextLayout& layout,
                                       int selectionStart, int selectionEnd,
                                       const std::u16string& replacement,
                                       bool* handled) {
    *handled = false;
    EditableTextDomain* domain = focusedDomainLocked(*state);
    const bool insertsLineBreak = replacement.find_first_of(u"\r\n") !=
                                  std::u16string::npos;
    // A one-line editor still needs styled reflow when Enter is inserted;
    // storing LF in one PDF text object is normalized to a visible space by
    // PDFium and cannot create the second positioned line used by WPS.
    if (!domain || (domain->lines.size() < 2 && !insertsLineBreak) ||
        layout.runs.empty() ||
        domain->objectIds.size() > 128) {
        return ErrorCode::Ok;
    }
    std::vector<EditableObjectRecord*> records;
    std::vector<std::size_t> recordStyles;
    std::vector<ReflowStyle> styles;
    records.reserve(layout.runs.size());
    recordStyles.reserve(layout.runs.size());
    Matrix pageToBasis;
    if (!inverse(orientationMatrix(domain->orientation), &pageToBasis)) return ErrorCode::Ok;
    for (const auto& line : domain->lines) {
        for (uint64_t id : line.objectIds) {
            EditableObjectRecord* record = textRecordById(*state, id);
            if (!record || record->pageObjectIndex < 0) {
                return ErrorCode::Ok;
            }
            if (record->formAncestors.empty()) {
                if (FPDFPage_GetObject(state->pdf, record->pageObjectIndex) != record->object) {
                    return ErrorCode::Ok;
                }
            } else if (formChildIndex(record->formAncestors.back(), record->object) < 0 ||
                       !insertFormObjectExtension() || !flushFormContentsExtension()) {
                return ErrorCode::Ok;
            }
            ReflowStyle candidate;
            candidate.font = FPDFTextObj_GetFont(record->object);
            if (!candidate.font ||
                !FPDFTextObj_GetFontSize(record->object, &candidate.fontSize) ||
                !FPDFPageObj_GetMatrix(record->object, &candidate.matrix)) {
                return ErrorCode::Ok;
            }
            candidate.formAncestors = record->formAncestors;
            const Matrix local{candidate.matrix.a, candidate.matrix.b,
                               candidate.matrix.c, candidate.matrix.d,
                               candidate.matrix.e, candidate.matrix.f};
            Matrix pageMatrix{record->matrix[0], record->matrix[3], record->matrix[1],
                              record->matrix[4], record->matrix[2], record->matrix[5]};
            Matrix inverseLocal;
            if (!inverse(local, &inverseLocal)) return ErrorCode::Ok;
            candidate.parentToPage = multiply(pageMatrix, inverseLocal);
            (void)FPDFPageObj_GetFillColor(record->object, &candidate.fillR,
                                           &candidate.fillG, &candidate.fillB, &candidate.fillA);
            (void)FPDFPageObj_GetStrokeColor(record->object, &candidate.strokeR,
                                             &candidate.strokeG, &candidate.strokeB,
                                             &candidate.strokeA);
            candidate.renderMode = FPDFTextObj_GetTextRenderMode(record->object);
            auto found = std::find_if(styles.begin(), styles.end(),
                [&](const ReflowStyle& value) { return sameReflowStyle(value, candidate); });
            std::size_t styleIndex = found == styles.end()
                ? styles.size() : static_cast<std::size_t>(found - styles.begin());
            if (found == styles.end()) {
                const Bounds geometry = itemGeometry(*record, domain->orientation);
                float originX = 0, originY = 0;
                transform(pageToBasis, record->matrix[2], record->matrix[5],
                          &originX, &originY);
                candidate.originToLeft = geometry.left - originX;
                styles.push_back(candidate);
            }
            const std::u16string text = objectTextLocked(*state, record->object);
            const Bounds geometry = itemGeometry(*record, domain->orientation);
            if (!text.empty()) {
                styles[styleIndex].totalAdvance += std::max(0.0f,
                    geometry.right - geometry.left);
                styles[styleIndex].measuredCharacters += static_cast<int>(text.size());
                std::vector<FocusedGlyph> glyphs;
                if (buildFocusedGlyphsLocked(*state, record, &glyphs)) {
                    for (const auto& glyph : glyphs) {
                        if (glyph.utf16Start < 0 ||
                            glyph.utf16Start >= static_cast<int>(text.size())) continue;
                        const char16_t value = text[static_cast<std::size_t>(glyph.utf16Start)];
                        const float advance = std::hypot(glyph.endX - glyph.originX,
                                                         glyph.endY - glyph.originY);
                        if (value < 128 && advance > 0.01f) {
                            styles[styleIndex].characterAdvance[value] += advance;
                            ++styles[styleIndex].characterSamples[value];
                        }
                    }
                }
            }
            records.push_back(record);
            recordStyles.push_back(styleIndex);
        }
    }
    if (records.empty() || styles.empty()) return ErrorCode::Ok;
    bool frameMatches = state->textReflowAnchor &&
        std::fabs(state->textReflowOrientation - domain->orientation) <= 0.035f;
    if (frameMatches) {
        frameMatches = std::any_of(records.begin(), records.end(),
            [&](const EditableObjectRecord* value) {
                return value->object == state->textReflowAnchor;
            });
    }
    float maxFontSize = 1;
    for (const auto& style : styles) maxFontSize = std::max(maxFontSize, style.fontSize);
    std::vector<float> steps;
    for (std::size_t i = 1; i < domain->lines.size(); ++i) {
        const float step = domain->lines[i - 1].baseline - domain->lines[i].baseline;
        // A semantic editor may bridge a Form envelope to following page
        // text. That section gap is a collision boundary, not paragraph
        // leading. The original editor keeps the close-line leading when it
        // lays out inserted rows and subsequently moves the following run.
        if (step > 0.1f && step <= maxFontSize * 3.0f) steps.push_back(step);
    }
    const float measuredBaselineStep = median(
        steps, std::max(1.0f, maxFontSize * 1.2f));
    const float reflowLeft = frameMatches ? state->textReflowLeft : domain->left;
    const float reflowRight = frameMatches ? state->textReflowRight : domain->right;
    const float firstBaseline = frameMatches
        ? state->textReflowFirstBaseline : domain->lines.front().baseline;
    const float baselineStep = frameMatches
        ? state->textReflowBaselineStep : measuredBaselineStep;
    if (!frameMatches) {
        state->textReflowAnchor = records.front()->object;
        state->textReflowLeft = reflowLeft;
        state->textReflowRight = reflowRight;
        state->textReflowFirstBaseline = firstBaseline;
        state->textReflowBaselineStep = baselineStep;
        state->textReflowOrientation = domain->orientation;
        state->textReflowHasLayoutBottom = false;
        state->textReflowLayoutBottom = domain->bottom;
        state->textReflowInitialBottom = domain->bottom;
        state->textReflowInitialLineCount = static_cast<int>(domain->lines.size());
    }
    if (selectionStart < 0 || selectionEnd < selectionStart ||
        selectionEnd > static_cast<int>(layout.text.size())) return ErrorCode::OutOfRange;

    std::vector<StyledReflowUnit> styled;
    styled.reserve(layout.text.size() + replacement.size());
    std::size_t previousStyle = 0;
    for (int index = 0; index < static_cast<int>(layout.text.size()); ++index) {
        const DomainTextRun* run = domainRunAtPosition(layout, index);
        if (run) {
            const auto match = std::find_if(layout.runs.begin(), layout.runs.end(),
                [&](const DomainTextRun& value) { return value.localId == run->localId; });
            if (match != layout.runs.end())
                previousStyle = recordStyles[static_cast<std::size_t>(match - layout.runs.begin())];
        }
        styled.push_back({layout.text[static_cast<std::size_t>(index)] == u'\n'
                              ? u' ' : layout.text[static_cast<std::size_t>(index)],
                          previousStyle});
    }
    const std::size_t insertionStyle = selectionStart < static_cast<int>(styled.size())
        ? styled[static_cast<std::size_t>(selectionStart)].style
        : (styled.empty() ? 0 : styled.back().style);
    styled.erase(styled.begin() + selectionStart, styled.begin() + selectionEnd);
    std::vector<StyledReflowUnit> inserted;
    ScopedLoadedFonts loadedFonts;
    inserted.reserve(replacement.size());
    for (char16_t value : replacement)
        inserted.push_back({value, fallbackStyleFor(
            value, insertionStyle, &styles, state->document->pdf, &loadedFonts)});
    // Keep neutral characters inside an RTL insertion with the neighbouring
    // fallback font so UBA and shaping see one phrase rather than independent
    // words separated by an LTR-font space.
    for (std::size_t index = 1; index + 1 < inserted.size(); ++index) {
        const char32_t value = inserted[index].value;
        if (!isStrongLtrCodePoint(value) && !isStrongRtlCodePoint(value) &&
            inserted[index - 1].style == inserted[index + 1].style) {
            inserted[index].style = inserted[index - 1].style;
        }
    }
    styled.insert(styled.begin() + selectionStart, inserted.begin(), inserted.end());
    if (styled.empty()) return ErrorCode::Ok;
    const float lineWidth = reflowRight - reflowLeft;
    if (lineWidth <= 1.0f) return ErrorCode::Ok;
    const auto wrapped = wrapStyledText(styled, styles, lineWidth);

    struct OutputRun {
        std::size_t style = 0;
        std::u16string text;
        std::size_t line = 0;
        float advanceBefore = 0;
    };
    std::vector<OutputRun> outputRuns;
    for (std::size_t lineIndex = 0; lineIndex < wrapped.size(); ++lineIndex) {
        float advance = 0;
        for (const auto& unit : wrapped[lineIndex]) {
            if (outputRuns.empty() || outputRuns.back().line != lineIndex ||
                outputRuns.back().style != unit.style) {
                outputRuns.push_back({unit.style, {}, lineIndex, advance});
            }
            outputRuns.back().text.push_back(unit.value);
            advance += styledUnitAdvance(unit, styles);
        }
    }
    if (outputRuns.empty() || outputRuns.size() > 512) return ErrorCode::Ok;

    auto command = std::make_shared<EditUndoState>();
    command->kind = EditUndoState::Kind::DomainText;
    command->page = state;
    command->pageHandle = page;
    command->beforeStart = selectionStart;
    command->beforeEnd = selectionEnd;
    command->afterStart = command->afterEnd = std::min(
        selectionStart + static_cast<int>(replacement.size()),
        static_cast<int>(styled.size()));
    command->currentlyAfter = false;
    command->beforeHasLayoutBottom = state->textReflowHasLayoutBottom;
    command->beforeLayoutBottom = state->textReflowLayoutBottom;
    includeFocusedEditorDamageLocked(*command, *state);
    const bool hasFormContent = std::any_of(records.begin(), records.end(),
        [](const EditableObjectRecord* value) {
            return value && !value->formAncestors.empty();
        });
    if (hasFormContent) {
        const int overflowLines = std::max(
            0, static_cast<int>(wrapped.size()) - state->textReflowInitialLineCount);
        // Original-SO oracle: once a Form paragraph overflows, its editor frame
        // includes one collision/continuation row beyond the newly wrapped
        // lines. This is observable through native_getTextEditorRect and is
        // independent of the last TextObject's paint bounds.
        const int reservedLines = overflowLines > 0 ? overflowLines + 1 : 0;
        command->afterHasLayoutBottom = reservedLines > 0;
        command->afterLayoutBottom = state->textReflowInitialBottom -
            static_cast<float>(reservedLines) * baselineStep;
    } else {
        command->afterHasLayoutBottom = false;
        command->afterLayoutBottom = domain->bottom;
    }
    int maxPageIndex = -1;
    for (EditableObjectRecord* record : records)
        maxPageIndex = std::max(maxPageIndex, record->pageObjectIndex);

    for (std::size_t i = 0; i < records.size(); ++i) {
        DomainUndoItem item;
        item.object = records[i]->object;
        item.pageObjectIndex = records[i]->pageObjectIndex;
        item.formAncestors = records[i]->formAncestors;
        item.formObjectIndex = records[i]->formAncestors.empty() ? -1 :
            formChildIndex(records[i]->formAncestors.back(), records[i]->object);
        item.beforeText = objectTextLocked(*state, item.object);
        const auto logical = state->shapedLogicalText.find(item.object);
        if (logical != state->shapedLogicalText.end()) {
            item.hasBeforeLogicalOverride = true;
            item.beforeLogicalOverride = logical->second;
        }
        item.beforeFont = FPDFTextObj_GetFont(item.object);
        item.afterPresent = false;
        item.hasMatrix = FPDFPageObj_GetMatrix(item.object, &item.beforeMatrix) != 0;
        item.afterMatrix = item.beforeMatrix;
        command->domainItems.push_back(std::move(item));
    }
    const Matrix basis = orientationMatrix(domain->orientation);
    std::vector<bool> used(records.size(), false);
    int createdCount = 0;
    for (const auto& output : outputRuns) {
        std::size_t selected = records.size();
        for (std::size_t i = 0; i < records.size(); ++i) {
            if (!used[i] && recordStyles[i] == output.style) {
                selected = i;
                break;
            }
        }
        DomainUndoItem* item = nullptr;
        if (selected < records.size()) {
            used[selected] = true;
            item = &command->domainItems[selected];
        } else {
            const ReflowStyle& style = styles[output.style];
            FPDF_PAGEOBJECT created = FPDFPageObj_CreateTextObj(
                state->document->pdf, style.font, style.fontSize);
            if (!created) return ErrorCode::General;
            DomainUndoItem value;
            value.object = created;
            value.beforePresent = false;
            value.afterPresent = true;
            value.ownsDetachedObject = true;
            value.pageObjectIndex = maxPageIndex + 1 + createdCount++;
            value.formAncestors = style.formAncestors;
            value.formObjectIndex = style.formAncestors.empty() ? -1 :
                FPDFFormObj_CountObjects(style.formAncestors.back()) + createdCount - 1;
            value.hasMatrix = true;
            value.afterMatrix = style.matrix;
            const bool initialized =
                FPDFPageObj_SetFillColor(created, style.fillR, style.fillG,
                                         style.fillB, style.fillA) &&
                FPDFPageObj_SetStrokeColor(created, style.strokeR, style.strokeG,
                                           style.strokeB, style.strokeA) &&
                (style.renderMode == FPDF_TEXTRENDERMODE_UNKNOWN ||
                 FPDFTextObj_SetTextRenderMode(created, style.renderMode));
            if (!initialized) {
                FPDFPageObj_Destroy(created);
                return ErrorCode::General;
            }
            command->domainItems.push_back(std::move(value));
            item = &command->domainItems.back();
        }
        const ReflowStyle& style = styles[output.style];
        item->afterPresent = true;
        item->afterText = output.text;
        item->hasMatrix = true;
        item->afterMatrix.a = style.matrix.a;
        item->afterMatrix.b = style.matrix.b;
        item->afterMatrix.c = style.matrix.c;
        item->afterMatrix.d = style.matrix.d;
        bool shapedRtl = false;
        float shapedAdvance = 0;
        if (baseTextDirection(output.text) == TextDirection::Rtl) {
            ShapedTextEncoding shaped;
            if (buildShapedTextEncoding(state->document->pdf,
                    reflowFontName(style.font), output.text, true,
                    style.fontSize, &shaped)) {
                item->afterFont = shaped.font;
                item->ownsAfterFont = true;
                item->afterCharcodes = std::move(shaped.charcodes);
                item->afterPositions = std::move(shaped.positions);
                if (shaped.requiresIndividualPlacement)
                    item->afterGlyphPositions = std::move(shaped.glyphPositions);
                item->afterShapedRunId = nextShapedRunId();
                shapedAdvance = shaped.advance;
                shapedRtl = true;
            }
        }
        const float originX = reflowLeft + output.advanceBefore - style.originToLeft +
            (shapedRtl ? shapedAdvance : 0.0f);
        const float baseline = firstBaseline -
                               static_cast<float>(output.line) * baselineStep;
        float pageX = 0, pageY = 0;
        transform(basis, originX, baseline, &pageX, &pageY);
        Matrix pageToParent;
        if (!inverse(style.parentToPage, &pageToParent)) return ErrorCode::General;
        transform(pageToParent, pageX, pageY,
                  &item->afterMatrix.e, &item->afterMatrix.f);
        if (!command->domainFocusObject) command->domainFocusObject = item->object;
    }
    ErrorCode applied = applyEditUndoLocked(*command, true);
    if (applied != ErrorCode::Ok) return applied;
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    ErrorCode handleError = ErrorCode::Ok;
    const Handle undoHandle = pageBorrow
        ? handles().insert(HandleType::EditUndo, command, pageBorrow.documentOwner,
                           pageBorrow.documentOwner, &handleError)
        : 0;
    if (!undoHandle) return handleError;
    state->editUndoHandles.push_back(undoHandle);
    state->pendingEditUndoHandles.push_back(undoHandle);
    *handled = true;
    return ErrorCode::Ok;
}

}  // namespace

ErrorCode replaceFocusedText(Handle page, const std::u16string& replacement) {
    if (hasUnsupportedSupplementaryInput(replacement)) {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    if (!record) return ErrorCode::NotFound;
    DomainTextLayout domainLayout;
    if (!buildDomainTextLocked(*state, &domainLayout)) return ErrorCode::NotFound;
    const DomainTextRun* focusedRun = domainRunById(domainLayout, record->localId);
    if (!focusedRun) return ErrorCode::NotFound;
    Bounds initialDamage;
    const bool hasInitialDamage = focusedEditorBoundsLocked(*state, &initialDamage);
    const int globalStart = state->textSelectionStart;
    const int globalEnd = state->textSelectionEnd;
    const DomainTextRun* selectionStartRun = domainRunAtPosition(
        domainLayout, globalStart, record->localId);
    const DomainTextRun* selectionEndRun = globalEnd > globalStart
        ? domainRunAtPosition(domainLayout, globalEnd - 1)
        : selectionStartRun;
    if (!selectionStartRun || !selectionEndRun) return ErrorCode::OutOfRange;
    const bool crossesLine = selectionStartRun->lineIndex != selectionEndRun->lineIndex;
    const bool growsText = static_cast<int>(replacement.size()) > globalEnd - globalStart;
    const bool insertsLineBreak = replacement.find_first_of(u"\r\n") !=
                                  std::u16string::npos;
    const EditableTextDomain* focusedDomain = focusedDomainLocked(*state);
    const bool atomizedLeaderDomain = focusedDomain && std::any_of(
        focusedDomain->lines.begin(), focusedDomain->lines.end(),
        [](const EditableTextLine& line) { return line.objectIds.size() >= 24; });
    const bool textualLeaderDomain = std::any_of(
        domainLayout.runs.begin(), domainLayout.runs.end(),
        [&](const DomainTextRun& run) {
            return std::count(
                domainLayout.text.begin() + run.start,
                domainLayout.text.begin() + run.end, u'.') >= 8;
        });
    const bool leaderHeavyDomain = atomizedLeaderDomain || textualLeaderDomain;
    if (leaderHeavyDomain && insertsLineBreak) {
        // Whole-domain reflow is destructive for custom-encoded TOC rows.
        // Wait for a raw-charcode line split instead of corrupting the page.
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    // Reopened shaped runs may be represented by several positioned paint
    // atoms. Any edit on such a page must first pass through group-aware
    // reflow so shrinking a cluster cannot leave visual attachments behind.
    // Leader/table rows often contain many custom-encoded paint atoms. A
    // normal insertion must stay local; flattening and rewriting the complete
    // domain turns their original charcodes into visibly wrong Unicode glyphs.
    // Explicit line breaks still require positioned-line reflow.
    if (crossesLine || insertsLineBreak ||
        (growsText && !leaderHeavyDomain) ||
        !state->persistentShapedRunIds.empty()) {
        bool reflowed = false;
        ErrorCode reflowCode = tryStyledDomainReflowLocked(
            page, state, domainLayout, globalStart, globalEnd, replacement, &reflowed);
        if (reflowCode != ErrorCode::Ok || reflowed) return reflowCode;
    }
    if (selectionStartRun->localId != selectionEndRun->localId) {
        // Synthetic newlines are editor separators rather than PDF characters.
        // A cross-line replacement removes the selected portions from every
        // participating paint object atomically. WT5b subsequently redistributes
        // the surviving text to close the visual line gap.
        auto command = std::make_shared<EditUndoState>();
        command->kind = EditUndoState::Kind::DomainText;
        command->page = state;
        command->pageHandle = page;
        if (hasInitialDamage) includeCommandDamage(*command, initialDamage);
        command->beforeStart = globalStart;
        command->beforeEnd = globalEnd;
        command->currentlyAfter = true;
        const auto first = std::find_if(domainLayout.runs.begin(), domainLayout.runs.end(),
            [&](const DomainTextRun& run) { return run.localId == selectionStartRun->localId; });
        const auto last = std::find_if(domainLayout.runs.begin(), domainLayout.runs.end(),
            [&](const DomainTextRun& run) { return run.localId == selectionEndRun->localId; });
        if (first == domainLayout.runs.end() || last == domainLayout.runs.end() || first > last)
            return ErrorCode::NotFound;
        int firstLocalCaret = 0;
        for (auto run = first; run <= last; ++run) {
            EditableObjectRecord* affected = textRecordById(*state, run->localId);
            if (!affected) return ErrorCode::NotFound;
            DomainUndoItem item;
            item.object = affected->object;
            item.formAncestors = affected->formAncestors;
            std::vector<FPDF_PAGEOBJECT> liveAncestors;
            if (findFormAncestors(state->pdf, item.object, &liveAncestors))
                item.formAncestors = std::move(liveAncestors);
            if (!item.formAncestors.empty() && !markFormChildDirtyExtension()) {
                setLastError(ErrorCode::Unsupported);
                return ErrorCode::Unsupported;
            }
            item.pageObjectIndex = affected->pageObjectIndex;
            item.beforeText = objectTextLocked(*state, item.object);
            const int localStart = std::clamp(globalStart - run->start, 0,
                                              static_cast<int>(item.beforeText.size()));
            const int localEnd = std::clamp(globalEnd - run->start, localStart,
                                            static_cast<int>(item.beforeText.size()));
            item.afterText = item.beforeText.substr(0, static_cast<std::size_t>(localStart));
            if (run == first) item.afterText.append(replacement);
            item.afterText.append(item.beforeText.substr(static_cast<std::size_t>(localEnd)));
            if (run == first)
                firstLocalCaret = localStart + static_cast<int>(replacement.size());
            if (item.afterText.empty()) {
                FPDF_PAGEOBJECT top = item.pageObjectIndex >= 0
                    ? FPDFPage_GetObject(state->pdf, item.pageObjectIndex) : nullptr;
                if (top == item.object) {
                    item.afterPresent = false;
                } else {
                    item.afterText = u" ";
                    item.afterDeleted = true;
                }
            }
            command->domainItems.push_back(std::move(item));
        }

        // Apply every text delta before touching page ownership. If a private
        // Form writeback or object removal fails, restore all objects before
        // returning so callers never observe half an editor mutation.
        std::vector<DomainUndoItem*> removed;
        auto rollback = [&]() {
            std::sort(removed.begin(), removed.end(), [](const auto* lhs, const auto* rhs) {
                return lhs->pageObjectIndex < rhs->pageObjectIndex;
            });
            for (DomainUndoItem* item : removed) {
                const int count = FPDFPage_CountObjects(state->pdf);
                const std::size_t index = static_cast<std::size_t>(
                    std::clamp(item->pageObjectIndex, 0, std::max(count, 0)));
                if (FPDFPage_InsertObjectAtIndex(state->pdf, item->object, index))
                    item->ownsDetachedObject = false;
            }
            for (auto& item : command->domainItems) {
                if (!item.ownsDetachedObject) {
                    (void)setObjectText(item.object, item.beforeText);
                    (void)markFormWriteback(item.object, item.formAncestors);
                }
            }
            (void)FPDFPage_GenerateContent(state->pdf);
        };
        for (auto& item : command->domainItems) {
            if (!item.afterPresent) continue;
            if (!setObjectText(item.object, item.afterText) ||
                !markFormWriteback(item.object, item.formAncestors)) {
                rollback();
                setLastError(ErrorCode::General);
                return ErrorCode::General;
            }
        }
        for (auto& item : command->domainItems) {
            if (item.afterPresent) continue;
            if (!FPDFPage_RemoveObject(state->pdf, item.object)) {
                rollback();
                setLastError(ErrorCode::General);
                return ErrorCode::General;
            }
            item.ownsDetachedObject = true;
            removed.push_back(&item);
        }
        if (!FPDFPage_GenerateContent(state->pdf)) {
            rollback();
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
        command->domainFocusObject = command->domainItems.front().object;
        if (!command->domainItems.front().afterPresent ||
            command->domainItems.front().afterDeleted) {
            command->domainFocusObject = nullptr;
            for (auto& item : command->domainItems) {
                if (item.afterPresent && !item.afterDeleted) {
                    command->domainFocusObject = item.object;
                    break;
                }
            }
        }
        invalidateEditableObjects(page, *state);
        rebuildLocked(*state);
        recognizeTextDomainsLocked(*state);
        focusObjectLocked(*state, command->domainFocusObject);
        DomainTextLayout afterLayout;
        int afterCaret = 0;
        if (state->hasFocus && buildDomainTextLocked(*state, &afterLayout)) {
            const DomainTextRun* afterRun = domainRunById(afterLayout,
                                                          state->focusedTextObjectId);
            if (afterRun) afterCaret = afterRun->start +
                (command->domainFocusObject == command->domainItems.front().object
                     ? firstLocalCaret : 0);
            afterCaret = std::clamp(afterCaret, 0, static_cast<int>(afterLayout.text.size()));
        }
        state->textSelectionStart = state->textSelectionEnd = afterCaret;
        command->afterStart = command->afterEnd = afterCaret;
        markTextChangedLocked(*state);
        includeFocusedEditorDamageLocked(*command, *state);
        const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
        ErrorCode handleError = ErrorCode::Ok;
        const Handle undoHandle = pageBorrow
            ? handles().insert(HandleType::EditUndo, command, pageBorrow.documentOwner,
                               pageBorrow.documentOwner, &handleError)
            : 0;
        if (!undoHandle) {
            // The applied mutation remains valid. Fully removed objects can be
            // destroyed because no undo command can reinsert them.
            setLastError(handleError);
            return handleError;
        }
        state->editUndoHandles.push_back(undoHandle);
        state->pendingEditUndoHandles.push_back(undoHandle);
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    if (globalStart < focusedRun->start || globalEnd > focusedRun->end) {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    FPDF_PAGEOBJECT editedObject = record->object;
    const int pageObjectIndex = record->pageObjectIndex;
    std::vector<FPDF_PAGEOBJECT> formAncestors = record->formAncestors;
    // Reconstruct from the current PDFium tree after a Form detachment. This
    // avoids keeping a stale nested-holder chain across page cache rebuilds.
    std::vector<FPDF_PAGEOBJECT> liveAncestors;
    if (findFormAncestors(state->pdf, editedObject, &liveAncestors)) {
        formAncestors = std::move(liveAncestors);
    }
    if (!formAncestors.empty() && !markFormChildDirtyExtension()) {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    const std::u16string beforeText = objectTextLocked(*state, editedObject);
    std::u16string text = beforeText;
    const int beforeStart = globalStart - focusedRun->start;
    const int beforeEnd = globalEnd - focusedRun->start;
    const int start = std::clamp(beforeStart, 0, static_cast<int>(text.size()));
    const int end = std::clamp(beforeEnd, start, static_cast<int>(text.size()));
    text.replace(static_cast<std::size_t>(start), static_cast<std::size_t>(end - start), replacement);
    const bool usesTextLeader =
        std::count(beforeText.begin(), beforeText.end(), u'.') >= 8;
    FPDF_FONT beforeFont = FPDFTextObj_GetFont(editedObject);
    FPDF_FONT safeAfterFont = nullptr;
    if (!text.empty() && usesTextLeader) {
        // WPS replaces fonts whose decoded Unicode cannot be written back by
        // the source subset. TeX TOC fonts commonly map the space charcode to
        // a visible ß on SetText. Use a document-owned standard serif face for
        // the one changed leader row; untouched rows keep their raw encoding.
        const std::string fontName = reflowFontName(beforeFont);
        const bool bold = fontName.find("Bold") != std::string::npos ||
                          fontName.find("bold") != std::string::npos;
        safeAfterFont = FPDFText_LoadStandardFont(
            state->document->pdf, bold ? "Times-Bold" : "Times-Roman");
        SetTextObjectFont setFont = setTextObjectFontExtension();
        if (!safeAfterFont || !setFont || !setFont(editedObject, safeAfterFont)) {
            if (safeAfterFont) FPDFFont_Close(safeAfterFont);
            setLastError(ErrorCode::Unsupported);
            return ErrorCode::Unsupported;
        }
    }
    const uint64_t focusedId = state->focusedTextObjectId;
    bool afterPresent = true;
    bool afterDeleted = false;
    if (text.empty()) {
        FPDF_PAGEOBJECT top = pageObjectIndex >= 0
            ? FPDFPage_GetObject(state->pdf, pageObjectIndex) : nullptr;
        if (top == editedObject) {
            if (!FPDFPage_RemoveObject(state->pdf, editedObject)) {
                setLastError(ErrorCode::General);
                return ErrorCode::General;
            }
            afterPresent = false;
        } else {
            // PDFium rejects an empty string and has no public Form insertion
            // API for undoing FPDFFormObj_RemoveObject. Keep the original child
            // as an invisible space carrier so undo/redo preserves its matrix,
            // font, z-order, and Form instance ownership.
            text = u" ";
            afterDeleted = true;
            if (!setObjectText(editedObject, text)) {
                setLastError(ErrorCode::General);
                return ErrorCode::General;
            }
        }
        state->focusedTextObjectId = 0;
        state->focusedTextDomain = -1;
    } else {
        if (!setObjectText(editedObject, text)) {
            if (safeAfterFont) {
                SetTextObjectFont setFont = setTextObjectFontExtension();
                if (setFont && beforeFont) (void)setFont(editedObject, beforeFont);
                FPDFFont_Close(safeAfterFont);
            }
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
    }
    if (afterPresent && !markFormWriteback(editedObject, formAncestors)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    if (!FPDFPage_GenerateContent(state->pdf)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    markTextChangedLocked(*state);
    invalidateEditableObjects(page, *state);
    rebuildLocked(*state);
    recognizeTextDomainsLocked(*state);
    if (!text.empty() && !afterDeleted) {
        state->focusedTextObjectId = focusedId;
        EditableObjectRecord* refreshed = focusedRecordLocked(*state);
        if (refreshed) {
            for (std::size_t i = 0; i < state->textDomains.size(); ++i) {
                const auto& ids = state->textDomains[i].objectIds;
                if (std::find(ids.begin(), ids.end(), focusedId) != ids.end()) {
                    state->focusedTextDomain = static_cast<int>(i);
                    break;
                }
            }
        }
        state->textSelectionStart = state->textSelectionEnd = focusedRun->start +
            start + static_cast<int>(replacement.size());
        state->hasFocus = refreshed != nullptr;
    }
    auto command = std::make_shared<EditUndoState>();
    command->page = state;
    command->pageHandle = page;
    if (hasInitialDamage) includeCommandDamage(*command, initialDamage);
    command->object = editedObject;
    command->formAncestors = formAncestors;
    command->pageObjectIndex = pageObjectIndex;
    command->beforeText = beforeText;
    command->afterText = text;
    command->beforeStart = beforeStart;
    command->beforeEnd = beforeEnd;
    command->afterStart = command->afterEnd = start + static_cast<int>(replacement.size());
    command->beforePresent = true;
    command->afterPresent = afterPresent;
    command->afterDeleted = afterDeleted;
    // Restoring the source subset font and then SetText(beforeText) would
    // recreate its lossy Unicode-to-charcode mapping during undo. Keep the
    // safely encodable font on both sides until raw charcode snapshots exist.
    command->beforeFont = safeAfterFont;
    command->afterFont = safeAfterFont;
    command->ownsTextAfterFont = safeAfterFont != nullptr;
    command->currentlyAfter = true;
    command->ownsDetachedObject = !afterPresent;
    includeFocusedEditorDamageLocked(*command, *state);
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    ErrorCode handleError = ErrorCode::Ok;
    const Handle undoHandle = pageBorrow
        ? handles().insert(HandleType::EditUndo, command, pageBorrow.documentOwner,
                           pageBorrow.documentOwner,
                           &handleError)
        : 0;
    if (!undoHandle) {
        // Keep an applied deletion valid even if command allocation fails.
        // Ownership must be finalized because no later undo can reinsert it.
        command.reset();
        setLastError(handleError);
        return handleError;
    }
    state->editUndoHandles.push_back(undoHandle);
    state->pendingEditUndoHandles.push_back(undoHandle);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

namespace {
ErrorCode finishStyleMutationLocked(Handle page,
                                    const std::shared_ptr<PageState>& state,
                                    FPDF_PAGEOBJECT object,
                                    const std::vector<FPDF_PAGEOBJECT>& ancestors,
                                    const std::shared_ptr<EditUndoState>& command) {
    includeFocusedEditorDamageLocked(*command, *state);
    if (!markFormWriteback(object, ancestors) || !FPDFPage_GenerateContent(state->pdf)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    const int selectionStart = state->textSelectionStart;
    const int selectionEnd = state->textSelectionEnd;
    markTextChangedLocked(*state);
    invalidateEditableObjects(page, *state);
    rebuildLocked(*state);
    recognizeTextDomainsLocked(*state);
    state->focusedTextObjectId = 0;
    state->focusedTextDomain = -1;
    for (const auto& item : state->editableObjects) {
        if (item.object != object) continue;
        state->focusedTextObjectId = item.localId;
        for (std::size_t i = 0; i < state->textDomains.size(); ++i) {
            const auto& ids = state->textDomains[i].objectIds;
            if (std::find(ids.begin(), ids.end(), item.localId) != ids.end()) {
                state->focusedTextDomain = static_cast<int>(i);
                break;
            }
        }
        break;
    }
    state->textSelectionStart = selectionStart;
    state->textSelectionEnd = selectionEnd;
    includeFocusedEditorDamageLocked(*command, *state);
    const BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    ErrorCode handleError = ErrorCode::Ok;
    const Handle undoHandle = pageBorrow
        ? handles().insert(HandleType::EditUndo, command, pageBorrow.documentOwner,
                           pageBorrow.documentOwner,
                           &handleError)
        : 0;
    if (!undoHandle) {
        setLastError(handleError);
        return handleError;
    }
    state->editUndoHandles.push_back(undoHandle);
    state->pendingEditUndoHandles.push_back(undoHandle);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}
}  // namespace

ErrorCode getFocusedTextFontSize(Handle page, float* result) {
    if (!result) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    if (!record || !FPDFTextObj_GetFontSize(record->object, result)) {
        setLastError(ErrorCode::NotFound);
        return ErrorCode::NotFound;
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode setFocusedTextFontSize(Handle page, float size) {
    if (!std::isfinite(size) || size <= 0 || size > 1000) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    if (!record) return ErrorCode::NotFound;
    float before = 0;
    if (!FPDFTextObj_GetFontSize(record->object, &before) ||
        !FPDFTextObj_SetFontSize(record->object, size)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    std::vector<FPDF_PAGEOBJECT> ancestors = record->formAncestors;
    std::vector<FPDF_PAGEOBJECT> liveAncestors;
    if (findFormAncestors(state->pdf, record->object, &liveAncestors)) {
        ancestors = std::move(liveAncestors);
    }
    auto command = std::make_shared<EditUndoState>();
    command->kind = EditUndoState::Kind::FontSize;
    command->page = state;
    command->pageHandle = page;
    command->object = record->object;
    command->formAncestors = ancestors;
    command->beforeFontSize = before;
    command->afterFontSize = size;
    return finishStyleMutationLocked(page, state, record->object, ancestors, command);
}

ErrorCode getFocusedTextColor(Handle page, uint32_t* argb) {
    if (!argb) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    unsigned int r = 0, g = 0, b = 0, a = 0;
    if (!record || !FPDFPageObj_GetFillColor(record->object, &r, &g, &b, &a)) {
        setLastError(ErrorCode::NotFound);
        return ErrorCode::NotFound;
    }
    *argb = ((a & 0xff) << 24) | ((r & 0xff) << 16) | ((g & 0xff) << 8) | (b & 0xff);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode setFocusedTextColor(Handle page, uint32_t argb) {
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    if (!record) return ErrorCode::NotFound;
    unsigned int r = 0, g = 0, b = 0, a = 0;
    if (!FPDFPageObj_GetFillColor(record->object, &r, &g, &b, &a) ||
        !FPDFPageObj_SetFillColor(record->object, (argb >> 16) & 0xff,
                                  (argb >> 8) & 0xff, argb & 0xff,
                                  (argb >> 24) & 0xff)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    const uint32_t before = ((a & 0xff) << 24) | ((r & 0xff) << 16) |
                            ((g & 0xff) << 8) | (b & 0xff);
    std::vector<FPDF_PAGEOBJECT> ancestors = record->formAncestors;
    std::vector<FPDF_PAGEOBJECT> liveAncestors;
    if (findFormAncestors(state->pdf, record->object, &liveAncestors)) {
        ancestors = std::move(liveAncestors);
    }
    auto command = std::make_shared<EditUndoState>();
    command->kind = EditUndoState::Kind::FillColor;
    command->page = state;
    command->pageHandle = page;
    command->object = record->object;
    command->formAncestors = ancestors;
    command->beforeColor = before;
    command->afterColor = argb;
    return finishStyleMutationLocked(page, state, record->object, ancestors, command);
}

ErrorCode getFocusedTextFontName(Handle page, std::string* result) {
    if (!result) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    FPDF_FONT font = record ? FPDFTextObj_GetFont(record->object) : nullptr;
    const size_t length = font ? FPDFFont_GetBaseFontName(font, nullptr, 0) : 0;
    if (!length) {
        setLastError(ErrorCode::NotFound);
        return ErrorCode::NotFound;
    }
    std::vector<char> buffer(length);
    if (FPDFFont_GetBaseFontName(font, buffer.data(), buffer.size()) != length) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    result->assign(buffer.data());
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode setFocusedTextFontName(Handle page, const std::string& name) {
    if (name.empty()) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    EditableObjectRecord* record = focusedRecordLocked(*state);
    if (!record) return ErrorCode::NotFound;
    FPDF_FONT before = FPDFTextObj_GetFont(record->object);
    const size_t currentLength = before ? FPDFFont_GetBaseFontName(before, nullptr, 0) : 0;
    std::vector<char> current(currentLength);
    if (currentLength && FPDFFont_GetBaseFontName(before, current.data(), current.size()) ==
                             currentLength && name == current.data()) {
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    SetTextObjectFont setFont = setTextObjectFontExtension();
    if (!setFont) {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    FPDF_FONT loaded = FPDFText_LoadStandardFont(state->document->pdf, name.c_str());
    if (!loaded) {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    const bool changed = setFont(record->object, loaded) != 0;
    FPDF_FONT after = changed ? FPDFTextObj_GetFont(record->object) : nullptr;
    FPDFFont_Close(loaded);
    if (!changed || !after) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    std::vector<FPDF_PAGEOBJECT> ancestors = record->formAncestors;
    std::vector<FPDF_PAGEOBJECT> liveAncestors;
    if (findFormAncestors(state->pdf, record->object, &liveAncestors)) {
        ancestors = std::move(liveAncestors);
    }
    auto command = std::make_shared<EditUndoState>();
    command->kind = EditUndoState::Kind::Font;
    command->page = state;
    command->pageHandle = page;
    command->object = record->object;
    command->formAncestors = ancestors;
    command->beforeFont = before;
    command->afterFont = after;
    return finishStyleMutationLocked(page, state, record->object, ancestors, command);
}

ErrorCode getFocusedTextBold(Handle page, bool* result) {
    if (!result) return ErrorCode::InvalidArgument;
    std::string name;
    const ErrorCode code = getFocusedTextFontName(page, &name);
    if (code != ErrorCode::Ok) return code;
    const std::size_t subset = name.find('+');
    if (subset != std::string::npos) name.erase(0, subset + 1);
    *result = name.find("Bold") != std::string::npos;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode setFocusedTextBold(Handle page, bool bold) {
    std::string name;
    ErrorCode code = getFocusedTextFontName(page, &name);
    if (code != ErrorCode::Ok) return code;
    const std::size_t subset = name.find('+');
    if (subset != std::string::npos) name.erase(0, subset + 1);
    const bool italic = name.find("Italic") != std::string::npos ||
                        name.find("Oblique") != std::string::npos;
    std::string target;
    if (name.find("Courier") != std::string::npos) {
        target = bold ? (italic ? "Courier-BoldOblique" : "Courier-Bold")
                      : (italic ? "Courier-Oblique" : "Courier");
    } else if (name.find("Times") != std::string::npos) {
        target = bold ? (italic ? "Times-BoldItalic" : "Times-Bold")
                      : (italic ? "Times-Italic" : "Times-Roman");
    } else if (name.find("Helvetica") != std::string::npos ||
               name.find("Arial") != std::string::npos) {
        target = bold ? (italic ? "Helvetica-BoldOblique" : "Helvetica-Bold")
                      : (italic ? "Helvetica-Oblique" : "Helvetica");
    } else {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    return setFocusedTextFontName(page, target);
}

namespace {

struct AlignmentLine {
    std::vector<EditableObjectRecord*> records;
    float left = 0;
    float right = 0;
    float bottom = 0;
    float top = 0;
};

bool focusedAlignmentLinesLocked(PageState& state, EditableTextDomain** domainResult,
                                 std::vector<AlignmentLine>* lines) {
    if (!domainResult || !lines || state.focusedTextDomain < 0 ||
        state.focusedTextDomain >= static_cast<int>(state.textDomains.size())) return false;
    EditableTextDomain& domain = state.textDomains[state.focusedTextDomain];
    Matrix pageToBasis;
    if (!inverse(orientationMatrix(domain.orientation), &pageToBasis)) return false;
    lines->clear();
    for (uint64_t id : domain.objectIds) {
        auto found = std::find_if(state.editableObjects.begin(), state.editableObjects.end(),
            [&](const EditableObjectRecord& item) { return item.localId == id; });
        if (found == state.editableObjects.end() || found->type != FPDF_PAGEOBJ_TEXT) continue;
        const Bounds bounds = textBoundsInBasis(*found, pageToBasis);
        const float center = (bounds.bottom + bounds.top) * 0.5f;
        AlignmentLine* line = nullptr;
        for (auto& candidate : *lines) {
            const float candidateCenter = (candidate.bottom + candidate.top) * 0.5f;
            const float tolerance = 0.5f * std::max(bounds.top - bounds.bottom,
                                                   candidate.top - candidate.bottom);
            if (std::fabs(center - candidateCenter) <= tolerance) {
                line = &candidate;
                break;
            }
        }
        if (!line) {
            lines->push_back({{}, bounds.left, bounds.right, bounds.bottom, bounds.top});
            line = &lines->back();
        } else {
            include(bounds, &line->left, &line->bottom, &line->right, &line->top);
        }
        line->records.push_back(&*found);
    }
    *domainResult = &domain;
    std::sort(lines->begin(), lines->end(), [](const AlignmentLine& lhs,
                                              const AlignmentLine& rhs) {
        return (lhs.bottom + lhs.top) > (rhs.bottom + rhs.top);
    });
    return !lines->empty();
}

bool appendTranslatedObject(EditUndoState& command, EditableObjectRecord& record,
                            float pageDx, float pageDy) {
    FS_MATRIX before{1, 0, 0, 1, 0, 0};
    if (!FPDFPageObj_GetMatrix(record.object, &before)) return false;
    const Matrix local{before.a, before.b, before.c, before.d, before.e, before.f};
    const Matrix combined{record.matrix[0], record.matrix[3], record.matrix[1],
                          record.matrix[4], record.matrix[2], record.matrix[5]};
    Matrix inverseLocal;
    Matrix inverseParent;
    if (!inverse(local, &inverseLocal) ||
        !inverse(multiply(combined, inverseLocal), &inverseParent)) return false;
    const float localDx = inverseParent.a * pageDx + inverseParent.c * pageDy;
    const float localDy = inverseParent.b * pageDx + inverseParent.d * pageDy;
    FS_MATRIX after = before;
    after.e += localDx;
    after.f += localDy;
    if (!FPDFPageObj_SetMatrix(record.object, &after) ||
        !markFormWriteback(record.object, record.formAncestors)) return false;
    command.alignedObjects.push_back(record.object);
    command.alignedFormAncestors.push_back(record.formAncestors);
    command.beforeMatrices.push_back(before);
    command.afterMatrices.push_back(after);
    return true;
}

int inferAlignment(const EditableTextDomain& domain,
                   const std::vector<AlignmentLine>& lines) {
    if (lines.size() < 2) return 0;
    const float tolerance = 1.0f;
    bool left = true, middle = true, right = true;
    const float domainCenter = (domain.left + domain.right) * 0.5f;
    for (const auto& line : lines) {
        left = left && std::fabs(line.left - domain.left) <= tolerance;
        middle = middle && std::fabs((line.left + line.right) * 0.5f - domainCenter) <= tolerance;
        right = right && std::fabs(line.right - domain.right) <= tolerance;
    }
    if (middle && !left) return 1;
    if (right && !left) return 2;
    return 0;
}

}  // namespace

ErrorCode getFocusedTextAlignment(Handle page, int* result) {
    if (!result) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    if (!state->textDomainsRecognized) recognizeTextDomainsLocked(*state);
    EditableTextDomain* domain = nullptr;
    std::vector<AlignmentLine> lines;
    if (!focusedAlignmentLinesLocked(*state, &domain, &lines)) return ErrorCode::NotFound;
    *result = inferAlignment(*domain, lines);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode setFocusedTextAlignment(Handle page, int alignment) {
    if (alignment < 0 || alignment > 4) return ErrorCode::InvalidArgument;
    if (alignment > 2) return ErrorCode::Unsupported;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    if (!state->textDomainsRecognized) recognizeTextDomainsLocked(*state);
    EditableObjectRecord* focused = focusedRecordLocked(*state);
    EditableTextDomain* domain = nullptr;
    std::vector<AlignmentLine> lines;
    if (!focused || !focusedAlignmentLinesLocked(*state, &domain, &lines)) {
        return ErrorCode::NotFound;
    }
    if (inferAlignment(*domain, lines) == alignment) {
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    auto command = std::make_shared<EditUndoState>();
    command->kind = EditUndoState::Kind::LayoutMatrices;
    command->page = state;
    command->pageHandle = page;
    command->object = focused->object;
    command->formAncestors = focused->formAncestors;
    const Matrix basis = orientationMatrix(domain->orientation);
    for (const auto& line : lines) {
        const float width = line.right - line.left;
        float targetLeft = domain->left;
        if (alignment == 1) targetLeft = (domain->left + domain->right - width) * 0.5f;
        if (alignment == 2) targetLeft = domain->right - width;
        const float delta = targetLeft - line.left;
        if (std::fabs(delta) <= 1e-4f) continue;
        const float pageDx = basis.a * delta;
        const float pageDy = basis.b * delta;
        for (EditableObjectRecord* record : line.records) {
            if (!appendTranslatedObject(*command, *record, pageDx, pageDy))
                return ErrorCode::General;
        }
    }
    if (command->alignedObjects.empty()) {
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    return finishStyleMutationLocked(page, state, focused->object,
                                     focused->formAncestors, command);
}

ErrorCode getFocusedTextLeading(Handle page, float* result) {
    if (!result) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    if (!state->textDomainsRecognized) recognizeTextDomainsLocked(*state);
    EditableTextDomain* domain = nullptr;
    std::vector<AlignmentLine> lines;
    if (!focusedAlignmentLinesLocked(*state, &domain, &lines)) return ErrorCode::NotFound;
    if (lines.size() < 2) {
        *result = 1.0f;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    float gapTotal = 0;
    for (std::size_t i = 1; i < lines.size(); ++i) {
        gapTotal += ((lines[i - 1].bottom + lines[i - 1].top) -
                     (lines[i].bottom + lines[i].top)) * 0.5f;
    }
    std::vector<float> sizes;
    for (const auto& line : lines) {
        for (const EditableObjectRecord* record : line.records) {
            float size = 0;
            if (FPDFTextObj_GetFontSize(record->object, &size) && size > 0) sizes.push_back(size);
        }
    }
    if (sizes.empty()) return ErrorCode::General;
    std::sort(sizes.begin(), sizes.end());
    const float referenceSize = sizes[sizes.size() / 2];
    *result = gapTotal / static_cast<float>(lines.size() - 1) / referenceSize;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode setFocusedTextLeading(Handle page, float leading) {
    if (!std::isfinite(leading) || leading <= 0.0f) return ErrorCode::InvalidArgument;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->editableInventoryValid) rebuildLocked(*state);
    if (!state->textDomainsRecognized) recognizeTextDomainsLocked(*state);
    EditableObjectRecord* focused = focusedRecordLocked(*state);
    EditableTextDomain* domain = nullptr;
    std::vector<AlignmentLine> lines;
    if (!focused || !focusedAlignmentLinesLocked(*state, &domain, &lines))
        return ErrorCode::NotFound;
    if (lines.size() < 2) {
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    std::vector<float> sizes;
    for (const auto& line : lines) {
        for (const EditableObjectRecord* record : line.records) {
            float size = 0;
            if (FPDFTextObj_GetFontSize(record->object, &size) && size > 0) sizes.push_back(size);
        }
    }
    if (sizes.empty()) return ErrorCode::General;
    std::sort(sizes.begin(), sizes.end());
    const float targetGap = leading * sizes[sizes.size() / 2];
    const float firstCenter = (lines.front().bottom + lines.front().top) * 0.5f;
    const Matrix basis = orientationMatrix(domain->orientation);
    auto command = std::make_shared<EditUndoState>();
    command->kind = EditUndoState::Kind::LayoutMatrices;
    command->page = state;
    command->pageHandle = page;
    command->object = focused->object;
    command->formAncestors = focused->formAncestors;
    for (std::size_t i = 1; i < lines.size(); ++i) {
        const float currentCenter = (lines[i].bottom + lines[i].top) * 0.5f;
        const float targetCenter = firstCenter - static_cast<float>(i) * targetGap;
        const float delta = targetCenter - currentCenter;
        if (std::fabs(delta) <= 1e-4f) continue;
        const float pageDx = basis.c * delta;
        const float pageDy = basis.d * delta;
        for (EditableObjectRecord* record : lines[i].records) {
            if (!appendTranslatedObject(*command, *record, pageDx, pageDy))
                return ErrorCode::General;
        }
    }
    if (command->alignedObjects.empty()) {
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    return finishStyleMutationLocked(page, state, focused->object,
                                     focused->formAncestors, command);
}

ErrorCode handleTextKeyDown(Handle page, int keyCode, bool* handled) {
    if (!handled) return ErrorCode::InvalidArgument;
    int start = 0, end = 0;
    ErrorCode code = getTextSelection(page, &start, &end);
    if (code != ErrorCode::Ok) return code;
    std::u16string text;
    code = getFocusedDomainText(page, &text);
    if (code != ErrorCode::Ok) return code;
    const bool backspace = keyCode == 8 || keyCode == 67;
    const bool forwardDelete = keyCode == 46 || keyCode == 112;
    if (!backspace && !forwardDelete) {
        *handled = false;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    if (start == end) {
        if (backspace && start > 0) start = previousUnicodeTextBoundary(text, start);
        else if (forwardDelete && end < static_cast<int>(text.size()))
            end = nextUnicodeTextBoundary(text, end);
        else {
            *handled = true;
            setLastError(ErrorCode::Ok);
            return ErrorCode::Ok;
        }
        code = setTextSelection(page, start, end);
        if (code != ErrorCode::Ok) return code;
    }
    *handled = replaceFocusedText(page, {}) == ErrorCode::Ok;
    return *handled ? ErrorCode::Ok : lastError();
}

ErrorCode takePendingEditUndo(Handle page, Handle* result) {
    if (!result) return ErrorCode::InvalidArgument;
    *result = 0;
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    while (!state->pendingEditUndoHandles.empty()) {
        const Handle candidate = state->pendingEditUndoHandles.front();
        state->pendingEditUndoHandles.erase(state->pendingEditUndoHandles.begin());
        if (handles().isValid(candidate, HandleType::EditUndo)) {
            *result = candidate;
            setLastError(ErrorCode::Ok);
            return ErrorCode::Ok;
        }
    }
    setLastError(ErrorCode::NotFound);
    return ErrorCode::NotFound;
}

ErrorCode getEditCommandDamage(Handle command, EditDamageRect* result) {
    if (!result) return ErrorCode::InvalidArgument;
    BorrowedHandle borrowed = handles().borrow(command, HandleType::EditUndo);
    if (!borrowed) return lastError();
    auto state = borrowed.as<EditUndoState>();
    if (!state) return ErrorCode::WrongType;
    std::lock_guard<std::mutex> lock(state->page->document->mutex);
    if (!state->hasDamage) {
        setLastError(ErrorCode::NotFound);
        return ErrorCode::NotFound;
    }
    result->left = state->damage.left;
    result->bottom = state->damage.bottom;
    result->right = state->damage.right;
    result->top = state->damage.top;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode undoEditCommand(Handle command) {
    BorrowedHandle borrowed = handles().borrow(command, HandleType::EditUndo);
    if (!borrowed) return borrowed.error;
    auto state = borrowed.as<EditUndoState>();
    std::lock_guard<std::mutex> lock(state->page->document->mutex);
    if (!state->currentlyAfter) {
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    return applyEditUndoLocked(*state, false);
}

ErrorCode redoEditCommand(Handle command) {
    BorrowedHandle borrowed = handles().borrow(command, HandleType::EditUndo);
    if (!borrowed) return borrowed.error;
    auto state = borrowed.as<EditUndoState>();
    std::lock_guard<std::mutex> lock(state->page->document->mutex);
    if (state->currentlyAfter) {
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    return applyEditUndoLocked(*state, true);
}

ErrorCode deleteEditCommand(Handle command) {
    return handles().invalidate(command, HandleType::EditUndo);
}

}  // namespace ksp::runtime
