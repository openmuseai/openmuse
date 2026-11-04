#include "ProductSession.h"
#include "ProductEditBridge.h"

#include "../runtime/EditRuntime.h"
#include "../runtime/ObjectRuntime.h"
#include "../runtime/PdfiumLifecycle.h"
#include "../runtime/SaveRuntime.h"

#include <fpdf_save.h>
#include <fpdf_doc.h>
#include <fpdf_text.h>
#include <fpdfview.h>

#include <algorithm>
#include <cmath>
#include <cstdlib>
#include <cstring>
#include <limits>
#include <new>
#include <string>
#include <vector>

namespace ksp::product {
namespace {

using runtime::EditableObjectRecord;
using runtime::ErrorCode;
using runtime::Handle;

constexpr std::size_t kMaxSnapshotBytes = 512ULL * 1024 * 1024;
constexpr std::size_t kMaxRasterPixels = 16ULL * 1024 * 1024;
constexpr int kMaxFullRasterDimension = 1 << 20;
constexpr int kMaxPageTextUnits = 1'000'000;
constexpr int kMaxPageLinks = 4096;
constexpr std::size_t kMaxLinkUriBytes = 1024 * 1024;

int code(ErrorCode value) { return runtime::asInt(value); }

struct SnapshotWriter {
    FPDF_FILEWRITE base{};
    std::vector<uint8_t> bytes;
    bool limitExceeded = false;

    SnapshotWriter() {
        base.version = 1;
        base.WriteBlock = write;
    }

    static int write(FPDF_FILEWRITE* base, const void* data, unsigned long size) {
        auto* writer = reinterpret_cast<SnapshotWriter*>(base);
        if ((size != 0 && !data) ||
            size > kMaxSnapshotBytes - writer->bytes.size()) {
            writer->limitExceeded = true;
            return 0;
        }
        try {
            const auto* first = static_cast<const uint8_t*>(data);
            if (size != 0) writer->bytes.insert(writer->bytes.end(), first, first + size);
        } catch (const std::bad_alloc&) {
            writer->limitExceeded = true;
            return 0;
        }
        return 1;
    }
};

bool rasterBytes(int width, int height, int capacity, int* required) {
    if (width <= 0 || height <= 0 || capacity < 0) return false;
    const auto pixels = static_cast<std::size_t>(width) * height;
    if (pixels > kMaxRasterPixels ||
        pixels > static_cast<std::size_t>(std::numeric_limits<int>::max() / 4))
        return false;
    *required = static_cast<int>(pixels * 4);
    return capacity >= *required;
}

int renderManagedPage(Handle page, int x, int y, int width, int height,
                      int fullWidth, int fullHeight, uint8_t* rgba,
                      int required) {
    ErrorCode error;
    auto managed = runtime::borrowPage(page, &error);
    if (!managed) return code(error);
    FPDF_BITMAP bitmap = FPDFBitmap_Create(width, height, 1);
    if (!bitmap) return code(ErrorCode::General);
    FPDFBitmap_FillRect(bitmap, 0, 0, width, height, 0xFFFFFFFF);
    FPDF_RenderPageBitmap(bitmap, managed->pdf, -x, -y, fullWidth, fullHeight, 0,
                          FPDF_ANNOT | FPDF_RENDER_NO_SMOOTHTEXT);
    const auto* source = static_cast<const uint8_t*>(FPDFBitmap_GetBuffer(bitmap));
    const int stride = FPDFBitmap_GetStride(bitmap);
    if (!source || stride < width * 4) {
        FPDFBitmap_Destroy(bitmap);
        return code(ErrorCode::General);
    }
    for (int row = 0; row < height; ++row) {
        const uint8_t* sourceRow = source + static_cast<std::size_t>(row) * stride;
        uint8_t* targetRow = rgba + static_cast<std::size_t>(row) * width * 4;
        for (int column = 0; column < width; ++column) {
            const uint8_t* bgra = sourceRow + column * 4;
            uint8_t* output = targetRow + column * 4;
            output[0] = bgra[2]; output[1] = bgra[1];
            output[2] = bgra[0]; output[3] = bgra[3];
        }
    }
    FPDFBitmap_Destroy(bitmap);
    return required;
}

int readManagedPageText(Handle page, KspProductPageText* output) {
    ErrorCode error;
    auto managed = runtime::borrowPage(page, &error);
    if (!managed) return code(error);
    FPDF_TEXTPAGE textPage = FPDFText_LoadPage(managed->pdf);
    if (!textPage) return code(ErrorCode::General);
    struct TextPageCloser {
        FPDF_TEXTPAGE page;
        ~TextPageCloser() { FPDFText_ClosePage(page); }
    } closer{textPage};
    const int count = FPDFText_CountChars(textPage);
    if (count < 0) return code(ErrorCode::General);
    if (count > kMaxPageTextUnits) return code(ErrorCode::OutOfRange);
    try {
        std::vector<KspProductTextChar> chars;
        chars.reserve(static_cast<std::size_t>(count));
        for (int i = 0; i < count; ++i) {
            uint32_t unicode = FPDFText_GetUnicode(textPage, i);
            if (unicode == 0 || unicode > 0x10FFFF ||
                (unicode >= 0xD800 && unicode <= 0xDFFF))
                unicode = 0xFFFD;
            double left = 0, right = 0, bottom = 0, top = 0;
            if (!FPDFText_GetCharBox(textPage, i, &left, &right,
                                      &bottom, &top) ||
                !std::isfinite(left) || !std::isfinite(right) ||
                !std::isfinite(bottom) || !std::isfinite(top) ||
                std::abs(left) > std::numeric_limits<float>::max() ||
                std::abs(right) > std::numeric_limits<float>::max() ||
                std::abs(bottom) > std::numeric_limits<float>::max() ||
                std::abs(top) > std::numeric_limits<float>::max()) {
                left = right = bottom = top = 0;
            }
            const auto addUnit = [&](uint32_t unit) {
                chars.push_back({unit, static_cast<float>(left),
                                 static_cast<float>(bottom),
                                 static_cast<float>(right),
                                 static_cast<float>(top)});
            };
            if (unicode <= 0xFFFF) {
                addUnit(unicode);
            } else {
                unicode -= 0x10000;
                addUnit(0xD800 + (unicode >> 10));
                addUnit(0xDC00 + (unicode & 0x3FF));
            }
            if (chars.size() > static_cast<std::size_t>(kMaxPageTextUnits))
                return code(ErrorCode::OutOfRange);
        }
        if (chars.empty()) return 0;
        auto* allocated = static_cast<KspProductTextChar*>(
            std::malloc(chars.size() * sizeof(KspProductTextChar)));
        if (!allocated) return code(ErrorCode::Io);
        std::memcpy(allocated, chars.data(),
                     chars.size() * sizeof(KspProductTextChar));
        output->chars = allocated;
        output->count = static_cast<int32_t>(chars.size());
        return 0;
    } catch (const std::bad_alloc&) {
        return code(ErrorCode::Io);
    }
}

int readManagedPageLinks(Handle page, Handle document,
                         KspProductPageLinks* output) {
    ErrorCode error;
    auto managedPage = runtime::borrowPage(page, &error);
    if (!managedPage) return code(error);
    auto managedDocument = runtime::borrowDocument(document, &error);
    if (!managedDocument) return code(error);
    try {
        std::vector<KspProductPageLink> links;
        std::vector<uint8_t> uriData;
        int position = 0;
        int scanned = 0;
        FPDF_LINK pdfLink = nullptr;
        while (FPDFLink_Enumerate(managedPage->pdf, &position, &pdfLink)) {
            if (++scanned > kMaxPageLinks) return code(ErrorCode::OutOfRange);
            FS_RECTF rect{};
            if (!pdfLink || !FPDFLink_GetAnnotRect(pdfLink, &rect) ||
                !std::isfinite(rect.left) || !std::isfinite(rect.right) ||
                !std::isfinite(rect.bottom) || !std::isfinite(rect.top))
                continue;
            KspProductPageLink link{};
            link.left = std::min(rect.left, rect.right);
            link.bottom = std::min(rect.bottom, rect.top);
            link.right = std::max(rect.left, rect.right);
            link.top = std::max(rect.bottom, rect.top);
            link.page_index = -1;
            FPDF_DEST dest = FPDFLink_GetDest(managedDocument->pdf, pdfLink);
            FPDF_ACTION action = FPDFLink_GetAction(pdfLink);
            if (!dest && action && FPDFAction_GetType(action) == PDFACTION_GOTO)
                dest = FPDFAction_GetDest(managedDocument->pdf, action);
            if (dest) {
                const int index = FPDFDest_GetDestPageIndex(managedDocument->pdf,
                                                            dest);
                if (index >= 0) {
                    link.kind = 2;
                    link.page_index = index;
                }
            } else if (action && FPDFAction_GetType(action) == PDFACTION_URI) {
                const unsigned long length = FPDFAction_GetURIPath(
                    managedDocument->pdf, action, nullptr, 0);
                if (length > 0 && length <= kMaxLinkUriBytes - uriData.size()) {
                    std::vector<uint8_t> uri(length);
                    if (FPDFAction_GetURIPath(managedDocument->pdf, action,
                                               uri.data(), length) == length) {
                        if (uri.back() == 0) uri.pop_back();
                        link.kind = 1;
                        link.uri_offset = static_cast<uint32_t>(uriData.size());
                        link.uri_length = static_cast<uint32_t>(uri.size());
                        uriData.insert(uriData.end(), uri.begin(), uri.end());
                    }
                }
            }
            links.push_back(link);
        }
        if (!links.empty()) {
            output->links = static_cast<KspProductPageLink*>(
                std::malloc(links.size() * sizeof(KspProductPageLink)));
            if (!output->links) return code(ErrorCode::Io);
            std::memcpy(output->links, links.data(),
                        links.size() * sizeof(KspProductPageLink));
            output->count = static_cast<int32_t>(links.size());
        }
        if (!uriData.empty()) {
            output->uri_data = static_cast<uint8_t*>(std::malloc(uriData.size()));
            if (!output->uri_data) {
                std::free(output->links);
                *output = {};
                return code(ErrorCode::Io);
            }
            std::memcpy(output->uri_data, uriData.data(), uriData.size());
            output->uri_size = static_cast<uint32_t>(uriData.size());
        }
        return 0;
    } catch (const std::bad_alloc&) {
        return code(ErrorCode::Io);
    }
}

KspMacEditSession* sessionForDocument(Handle document, ErrorCode result,
                                      int* error) {
    if (error) *error = code(result);
    if (result != ErrorCode::Ok) {
        if (document) (void)runtime::closeDocument(document);
        return nullptr;
    }
    auto* session = new (std::nothrow) KspMacEditSession;
    if (!session) {
        (void)runtime::closeDocument(document);
        if (error) *error = code(ErrorCode::Io);
        return nullptr;
    }
    session->document = document;
    result = runtime::setEditStatus(document, 1);
    if (result != ErrorCode::Ok) {
        (void)runtime::closeDocument(document);
        delete session;
        if (error) *error = code(result);
        return nullptr;
    }
    return session;
}

void deleteHistoryFrom(KspMacEditSession* session, std::size_t begin) {
    for (std::size_t i = begin; i < session->history.size(); ++i)
        (void)ksp::runtime::deleteEditCommand(session->history[i]);
    session->history.resize(begin);
}

ErrorCode focusDomain(KspMacEditSession* session, int index) {
    EditableObjectRecord geometry;
    ErrorCode result = ksp::runtime::getTextEditorGeometry(
        session->page, index, false, &geometry);
    if (result != ErrorCode::Ok) return result;
    const float bx = (geometry.left + geometry.right) * 0.5f;
    const float by = (geometry.bottom + geometry.top) * 0.5f;
    const float x = geometry.matrix[0] * bx + geometry.matrix[1] * by +
                    geometry.matrix[2];
    const float y = geometry.matrix[3] * bx + geometry.matrix[4] * by +
                    geometry.matrix[5];
    bool focused = false;
    result = ksp::runtime::focusTextAtPoint(session->page, x, y, 0, &focused);
    return result == ErrorCode::Ok && focused ? ErrorCode::Ok : ErrorCode::NotFound;
}

ErrorCode appendPendingUndo(KspMacEditSession* session) {
    Handle command = 0;
    ErrorCode result = ksp::runtime::takePendingEditUndo(session->page, &command);
    if (result != ErrorCode::Ok || !command) return result;
    deleteHistoryFrom(session, session->historyPosition);
    session->history.push_back(command);
    session->historyPosition = session->history.size();
    return ErrorCode::Ok;
}

ErrorCode replaceSelectionLocked(KspMacEditSession* session,
                                 const uint16_t* text, int length) {
    std::u16string replacement;
    if (length > 0) {
        const auto* units = reinterpret_cast<const char16_t*>(text);
        replacement.assign(units, units + length);
    }
    ErrorCode result = ksp::runtime::replaceFocusedText(session->page, replacement);
    return result == ErrorCode::Ok ? appendPendingUndo(session) : result;
}

}  // namespace

KspMacEditSession* openSession(const char* path, int* error) {
    runtime::ensurePdfiumInitialized();
    if (error) *error = code(ErrorCode::InvalidArgument);
    if (!path || !*path) return nullptr;
    Handle document = 0;
    const ErrorCode result = runtime::openDocument(path, &document);
    return sessionForDocument(document, result, error);
}

KspMacEditSession* openSessionBytes(const uint8_t* bytes, uint64_t size,
                                    int* error) {
    runtime::ensurePdfiumInitialized();
    if (error) *error = code(ErrorCode::InvalidArgument);
    if (!bytes || size == 0 || size > std::numeric_limits<std::size_t>::max())
        return nullptr;
    Handle document = 0;
    const ErrorCode result = runtime::openDocumentBytes(
        bytes, static_cast<std::size_t>(size), &document);
    return sessionForDocument(document, result, error);
}

void closeSession(KspMacEditSession* session) {
    if (!session) return;
    {
        std::lock_guard<std::mutex> lock(session->mutex);
        for (Handle command : session->history)
            (void)runtime::deleteEditCommand(command);
        session->history.clear();
        if (session->page) (void)runtime::closePage(session->page);
        if (session->document) (void)runtime::closeDocument(session->document);
        session->page = session->document = 0;
    }
    delete session;
}

int pageCount(KspMacEditSession* session) {
    if (!session) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    int count = 0;
    const ErrorCode result = runtime::getPageCount(session->document, &count);
    return result == ErrorCode::Ok ? count : code(result);
}

int loadEditPage(KspMacEditSession* session, int pageIndex) {
    if (!session) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    // A loaded edit page is a long-lived transaction, as on Android. Re-running
    // SectionRecognize here invalidates focus/object identity after every key.
    if (session->page && session->pageIndex == pageIndex) return 0;
    deleteHistoryFrom(session, 0);
    session->historyPosition = 0;
    if (session->page) {
        const ErrorCode synced = ksp::runtime::syncEditPage(
            session->document, session->page, true);
        if (synced != ErrorCode::Ok) return code(synced);
        (void)ksp::runtime::closePage(session->page);
    }
    session->page = 0;
    ErrorCode result = ksp::runtime::loadPage(session->document, pageIndex,
                                               &session->page);
    if (result == ErrorCode::Ok)
        result = ksp::runtime::recognizeTextDomains(session->page);
    session->pageIndex = result == ErrorCode::Ok ? pageIndex : -1;
    return code(result);
}

int domainCount(KspMacEditSession* session) {
    if (!session) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (!session->page) return code(ErrorCode::InvalidArgument);
    int count = 0;
    const ErrorCode result = ksp::runtime::getTextEditorCount(session->page, &count);
    return result == ErrorCode::Ok ? count : code(result);
}

int domainRect(KspMacEditSession* session, int index,
                             float* left, float* bottom, float* right, float* top) {
    if (!session || !left || !bottom || !right || !top)
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    EditableObjectRecord value;
    const ErrorCode result = ksp::runtime::getTextEditorGeometry(
        session->page, index, false, &value);
    if (result != ErrorCode::Ok) return code(result);
    const float xs[] = {value.left, value.right, value.right, value.left};
    const float ys[] = {value.bottom, value.bottom, value.top, value.top};
    *left = *bottom = 1e30f;
    *right = *top = -1e30f;
    for (int i = 0; i < 4; ++i) {
        const float x = value.matrix[0] * xs[i] + value.matrix[1] * ys[i] +
                        value.matrix[2];
        const float y = value.matrix[3] * xs[i] + value.matrix[4] * ys[i] +
                        value.matrix[5];
        *left = std::min(*left, x); *right = std::max(*right, x);
        *bottom = std::min(*bottom, y); *top = std::max(*top, y);
    }
    return 0;
}

int domainText(KspMacEditSession* session, int index,
                             uint16_t* output, int capacity) {
    if (!session || capacity < 0) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    ErrorCode result = focusDomain(session, index);
    if (result != ErrorCode::Ok) return code(result);
    std::u16string text;
    result = ksp::runtime::getFocusedDomainText(session->page, &text);
    if (result != ErrorCode::Ok) return code(result);
    if (!output || capacity == 0) return static_cast<int>(text.size());
    const int copied = std::min(capacity, static_cast<int>(text.size()));
    std::copy_n(reinterpret_cast<const uint16_t*>(text.data()), copied, output);
    return copied;
}

int focusAt(KspMacEditSession* session, float x, float y) {
    if (!session) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (!session->page) return code(ErrorCode::InvalidArgument);
    bool focused = false;
    const ErrorCode result = ksp::runtime::focusTextAtPoint(
        session->page, x, y, 0, &focused);
    if (result != ErrorCode::Ok) return code(result);
    return focused ? 1 : 0;
}

int focusedText(KspMacEditSession* session,
                              uint16_t* output, int capacity) {
    if (!session || capacity < 0)
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (!session->page) return code(ErrorCode::InvalidArgument);
    std::u16string text;
    const ErrorCode result = ksp::runtime::getFocusedDomainText(session->page, &text);
    if (result != ErrorCode::Ok) return code(result);
    if (!output || capacity == 0) return static_cast<int>(text.size());
    const int copied = std::min(capacity, static_cast<int>(text.size()));
    std::copy_n(reinterpret_cast<const uint16_t*>(text.data()), copied, output);
    return copied;
}

int selection(KspMacEditSession* session, int* start, int* end) {
    if (!session || !start || !end)
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (!session->page) return code(ErrorCode::InvalidArgument);
    return code(ksp::runtime::getTextSelection(session->page, start, end));
}

int setSelection(KspMacEditSession* session, int start, int end) {
    if (!session) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (!session->page) return code(ErrorCode::InvalidArgument);
    return code(ksp::runtime::setTextSelection(session->page, start, end));
}

int caret(KspMacEditSession* session, float* bottomX,
                       float* bottomY, float* topX, float* topY) {
    if (!session || !bottomX || !bottomY || !topX || !topY)
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (!session->page) return code(ErrorCode::InvalidArgument);
    int start = 0, end = 0;
    ErrorCode result = ksp::runtime::getTextSelection(session->page, &start, &end);
    ksp::runtime::TextCaretGeometry caret;
    if (result == ErrorCode::Ok)
        result = ksp::runtime::getTextCaretGeometry(session->page, end, &caret);
    if (result != ErrorCode::Ok) return code(result);
    *bottomX = caret.bottomX; *bottomY = caret.bottomY;
    *topX = caret.topX; *topY = caret.topY;
    return 0;
}

int replaceSelection(KspMacEditSession* session,
                                   const uint16_t* text, int length) {
    if (!session || length < 0 || (length && !text))
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (!session->page) return code(ErrorCode::InvalidArgument);
    return code(replaceSelectionLocked(session, text, length));
}

int pageSize(KspMacEditSession* session, int pageIndex, float* width,
             float* height) {
    if (!session || pageIndex < 0 || !width || !height)
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    const ErrorCode result = session->page && session->pageIndex == pageIndex
        ? runtime::getPageSize(session->page, width, height)
        : runtime::getPageSizeAt(session->document, pageIndex, width, height);
    return code(result);
}

int renderLoadedPage(KspMacEditSession* session, int width, int height,
                     uint8_t* rgba, int capacity) {
    int required = 0;
    if (!session || !rgba || !rasterBytes(width, height, capacity, &required))
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (!session->page) return code(ErrorCode::InvalidArgument);
    return renderManagedPage(session->page, 0, 0, width, height, width, height,
                             rgba, required);
}

int renderPageAt(KspMacEditSession* session, int pageIndex, int width,
                 int height, uint8_t* rgba, int capacity) {
    return renderPageRegion(session, pageIndex, 0, 0, width, height, width,
                            height, rgba, capacity);
}

int renderPageRegion(KspMacEditSession* session, int pageIndex, int x, int y,
                     int width, int height, int fullWidth, int fullHeight,
                     uint8_t* rgba, int capacity) {
    int required = 0;
    if (!session || pageIndex < 0 || !rgba || x < 0 || y < 0 ||
        fullWidth <= 0 || fullHeight <= 0 ||
        fullWidth > kMaxFullRasterDimension ||
        fullHeight > kMaxFullRasterDimension ||
        x > fullWidth || y > fullHeight ||
        width > fullWidth - x || height > fullHeight - y ||
        !rasterBytes(width, height, capacity, &required))
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (session->page && session->pageIndex == pageIndex)
        return renderManagedPage(session->page, x, y, width, height,
                                 fullWidth, fullHeight, rgba, required);
    Handle temporary = 0;
    const ErrorCode opened = runtime::loadPage(session->document, pageIndex,
                                               &temporary);
    if (opened != ErrorCode::Ok) return code(opened);
    const int rendered = renderManagedPage(temporary, x, y, width, height,
                                           fullWidth, fullHeight, rgba,
                                           required);
    (void)runtime::closePage(temporary);
    return rendered;
}

int readPageText(KspMacEditSession* session, int pageIndex,
                 KspProductPageText* output) {
    if (output) {
        output->chars = nullptr;
        output->count = 0;
    }
    if (!session || pageIndex < 0 || !output)
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (session->page && session->pageIndex == pageIndex)
        return readManagedPageText(session->page, output);
    Handle temporary = 0;
    const ErrorCode opened = runtime::loadPage(session->document, pageIndex,
                                                &temporary);
    if (opened != ErrorCode::Ok) return code(opened);
    const int result = readManagedPageText(temporary, output);
    (void)runtime::closePage(temporary);
    return result;
}

void freePageText(KspProductPageText* output) {
    if (!output) return;
    std::free(output->chars);
    output->chars = nullptr;
    output->count = 0;
}

int readPageLinks(KspMacEditSession* session, int pageIndex,
                  KspProductPageLinks* output) {
    if (output) *output = {};
    if (!session || pageIndex < 0 || !output)
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (session->page && session->pageIndex == pageIndex)
        return readManagedPageLinks(session->page, session->document, output);
    Handle temporary = 0;
    const ErrorCode opened = runtime::loadPage(session->document, pageIndex,
                                               &temporary);
    if (opened != ErrorCode::Ok) return code(opened);
    const int result = readManagedPageLinks(temporary, session->document, output);
    (void)runtime::closePage(temporary);
    return result;
}

void freePageLinks(KspProductPageLinks* output) {
    if (!output) return;
    std::free(output->links);
    std::free(output->uri_data);
    *output = {};
}

int replaceDomain(KspMacEditSession* session, int index,
                                const uint16_t* text, int length) {
    if (!session || length < 0 || (length && !text))
        return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    ErrorCode result = focusDomain(session, index);
    if (result != ErrorCode::Ok) return code(result);
    std::u16string before;
    result = ksp::runtime::getFocusedDomainText(session->page, &before);
    if (result == ErrorCode::Ok)
        result = ksp::runtime::setTextSelection(session->page, 0,
                                                static_cast<int>(before.size()));
    if (result != ErrorCode::Ok) return code(result);
    return code(replaceSelectionLocked(session, text, length));
}

int undo(KspMacEditSession* session) {
    if (!session) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (session->historyPosition == 0) return code(ErrorCode::NotFound);
    const ErrorCode result = ksp::runtime::undoEditCommand(
        session->history[session->historyPosition - 1]);
    if (result == ErrorCode::Ok) --session->historyPosition;
    return code(result);
}

int redo(KspMacEditSession* session) {
    if (!session) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (session->historyPosition >= session->history.size())
        return code(ErrorCode::NotFound);
    const ErrorCode result = ksp::runtime::redoEditCommand(
        session->history[session->historyPosition]);
    if (result == ErrorCode::Ok) ++session->historyPosition;
    return code(result);
}

int canUndo(KspMacEditSession* session) {
    if (!session) return 0;
    std::lock_guard<std::mutex> lock(session->mutex);
    return session->historyPosition > 0;
}

int canRedo(KspMacEditSession* session) {
    if (!session) return 0;
    std::lock_guard<std::mutex> lock(session->mutex);
    return session->historyPosition < session->history.size();
}

int saveSession(KspMacEditSession* session, const char* path) {
    if (!session || !path || !*path) return code(ErrorCode::InvalidArgument);
    std::lock_guard<std::mutex> lock(session->mutex);
    if (session->page) {
        ErrorCode result = runtime::syncEditPage(session->document, session->page,
                                                  true);
        if (result != ErrorCode::Ok) return code(result);
    }
    Handle saver = 0;
    ErrorCode result = runtime::createSaveSession(session->document, false,
                                                   &saver);
    bool started = false;
    if (result == ErrorCode::Ok)
        result = runtime::startSaveSession(saver, path, &started);
    if (result == ErrorCode::Ok && started) {
        ErrorCode saveError;
        if (runtime::continueSaveSession(saver, 500, &saveError) != 100)
            result = saveError;
    }
    if (saver) (void)runtime::closeSaveSession(saver);
    return code(result);
}

int exportSessionBytes(KspMacEditSession* session, uint8_t** data,
                       uint64_t* size) {
    if (data) *data = nullptr;
    if (size) *size = 0;
    if (!session || !data || !size) return code(ErrorCode::InvalidArgument);
    try {
        std::lock_guard<std::mutex> lock(session->mutex);
        if (session->page) {
            const ErrorCode synced = runtime::syncEditPage(session->document,
                                                             session->page, true);
            if (synced != ErrorCode::Ok) return code(synced);
        }
        ErrorCode error;
        const auto document = runtime::borrowDocument(session->document, &error);
        if (!document) return code(error);
        SnapshotWriter writer;
        {
            std::lock_guard<std::mutex> documentLock(document->mutex);
            if (!document->pdf) return code(ErrorCode::InvalidHandle);
            const bool saved = FPDF_SaveAsCopy(document->pdf, &writer.base, 0);
            if (writer.limitExceeded) return code(ErrorCode::OutOfRange);
            if (!saved) return code(ErrorCode::Io);
        }
        if (writer.bytes.empty()) return code(ErrorCode::Io);
        auto* allocated = static_cast<uint8_t*>(std::malloc(writer.bytes.size()));
        if (!allocated) return code(ErrorCode::Io);
        std::memcpy(allocated, writer.bytes.data(), writer.bytes.size());
        *data = allocated;
        *size = writer.bytes.size();
        return 0;
    } catch (...) {
        return code(ErrorCode::General);
    }
}

}  // namespace ksp::product
