#include "PdfRuntime.h"

#include <fpdf_edit.h>
#include <fpdf_text.h>

#include <algorithm>
#include <atomic>
#include <new>
#ifdef __ANDROID__
#include <android/log.h>
#endif
#include <dlfcn.h>

namespace ksp::runtime {

namespace {
using SeparateSharedForms = FPDF_BOOL (*)(FPDF_PAGE);

// libkwopdf stores this value in the initialized module singleton (+0x168),
// rather than in PDFDocument. The module exists for the complete lifetime of
// the rebuild library, so the original setter's initialized branch is always
// available here.
std::atomic<int> gSectionRecognizeAlgorithmGeneration{1};

SeparateSharedForms separateSharedFormsExtension() {
#ifdef __EMSCRIPTEN__
    return &KSPPage_SeparateSharedForms;
#else
    static void* module = dlopen("libpdfium.so", RTLD_NOW);
    static auto function = reinterpret_cast<SeparateSharedForms>(
        dlsym(module ? module : RTLD_DEFAULT, "KSPPage_SeparateSharedForms"));
    return function;
#endif
}
}  // namespace

bool setSectionRecognizeAlgorithmGeneration(int generation) {
    gSectionRecognizeAlgorithmGeneration.store(generation,
                                                std::memory_order_relaxed);
    setLastError(ErrorCode::Ok);
    return true;
}

int sectionRecognizeAlgorithmGeneration() {
    setLastError(ErrorCode::Ok);
    return gSectionRecognizeAlgorithmGeneration.load(std::memory_order_relaxed);
}

ErrorCode getRecognitionContextDiagnostics(
    Handle document, RecognitionContextDiagnostics* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    const auto& context = state->recognitionContext;
    result->invocationCount = context.invocationCount;
    result->uniquePageCount = static_cast<int>(context.firstSeenPages.size());
    result->lastPageIndex = context.lastPageIndex;
    result->lastDomainCount = context.lastDomainCount;
    result->lastAlgorithmGeneration = context.lastAlgorithmGeneration;
    result->cachedPageCount = static_cast<int>(context.pages.size());
    int cachedDomainCount = 0;
    for (const auto& entry : context.pages) {
        if (entry.second) {
            cachedDomainCount += static_cast<int>(entry.second->projections.size());
        }
    }
    result->cachedDomainCount = cachedDomainCount;
    result->replacementCount = context.replacementCount;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getRecognitionProcessorDiagnostics(
    Handle document, RecognitionProcessorDiagnostics* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    const auto& context = state->recognitionContext;
    result->commitCount = context.processorCommitCount;
    result->recognizedPageCount = static_cast<int>(context.pages.size());
    result->formulaPageCount = context.processorFormulaPageCount;
    result->formulaDomainCount = context.processorFormulaDomainCount;
    result->formulaSpanCount = context.processorFormulaSpanCount;
    result->compoundFormulaCount = context.processorCompoundFormulaCount;
    result->maxFormulaSpanCount = context.processorMaxFormulaSpanCount;
    result->lastPageIndex = context.lastPageIndex;
    result->algorithmGeneration = context.lastAlgorithmGeneration;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getRecognitionPageDiagnostics(
    Handle document, int pageIndex, RecognitionPageDiagnostics* result) {
    if (!result || pageIndex < 0) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    const auto found = state->recognitionContext.pages.find(pageIndex);
    if (found == state->recognitionContext.pages.end() || !found->second) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    *result = {};
    result->domainCount = static_cast<int>(found->second->domains.size());
    result->nodeCount = static_cast<int>(found->second->hierarchy.size());
    for (const EditableTextDomainNode& node : found->second->hierarchy) {
        switch (node.kind) {
            case EditableTextDomainKind::Section: ++result->sectionCount; break;
            case EditableTextDomainKind::Paragraph:
            case EditableTextDomainKind::ReferenceLabel:
            case EditableTextDomainKind::ReferenceBody:
            case EditableTextDomainKind::FormParagraph:
                ++result->paragraphCount;
                break;
            case EditableTextDomainKind::Line: ++result->lineCount; break;
            case EditableTextDomainKind::Span: ++result->spanCount; break;
            case EditableTextDomainKind::Formula: ++result->formulaCount; break;
            case EditableTextDomainKind::Table: ++result->tableCount; break;
            case EditableTextDomainKind::TableRow: ++result->tableRowCount; break;
            case EditableTextDomainKind::TableCell: ++result->tableCellCount; break;
        }
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getRecognitionDomainDiagnostics(
    Handle document, int pageIndex,
    std::vector<RecognitionDomainDiagnostics>* result) {
    if (!result || pageIndex < 0) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    const auto found = state->recognitionContext.pages.find(pageIndex);
    if (found == state->recognitionContext.pages.end() || !found->second) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    result->clear();
    result->reserve(found->second->domains.size());
    for (const EditableTextDomain& domain : found->second->domains) {
        RecognitionDomainDiagnostics item;
        item.kind = static_cast<int>(domain.kind);
        item.lineCount = static_cast<int>(domain.lines.size());
        item.objectCount = static_cast<int>(domain.objectIds.size());
        for (const EditableTextLine& line : domain.lines) {
            item.spanCount += static_cast<int>(line.objectIds.size());
        }
        result->push_back(item);
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

DocumentState::~DocumentState() {
    if (pdf) FPDF_CloseDocument(pdf);
}

PageState::~PageState() {
    if (pdf) {
        std::lock_guard<std::mutex> lock(document->mutex);
        FPDF_ClosePage(pdf);
    }
}

ErrorCode mapPdfiumOpenError(unsigned long error) {
    switch (error) {
        case FPDF_ERR_SUCCESS: return ErrorCode::Ok;
        case FPDF_ERR_FORMAT: return ErrorCode::FileFormat;
        case FPDF_ERR_PASSWORD: return ErrorCode::Password;
        case FPDF_ERR_SECURITY: return ErrorCode::SecurityHandler;
        case FPDF_ERR_FILE: return ErrorCode::Io;
        default: return ErrorCode::General;
    }
}

std::shared_ptr<DocumentState> borrowDocument(Handle document, ErrorCode* error) {
    BorrowedHandle value = handles().borrow(document, HandleType::Document);
    if (error) *error = value.error;
    return value ? value.as<DocumentState>() : nullptr;
}

std::shared_ptr<PageState> borrowPage(Handle page, ErrorCode* error,
                                      Handle expectedDocument) {
    BorrowedHandle value = handles().borrow(page, HandleType::Page, expectedDocument);
    if (error) *error = value.error;
    return value ? value.as<PageState>() : nullptr;
}

TextPageState::~TextPageState() {
    if (pdf) {
        std::lock_guard<std::mutex> lock(page->document->mutex);
        FPDFText_ClosePage(pdf);
    }
}

std::shared_ptr<TextPageState> borrowTextPage(Handle textPage, ErrorCode* error,
                                              Handle expectedPage) {
    BorrowedHandle value = handles().borrow(textPage, HandleType::TextPage, 0, expectedPage);
    if (error) *error = value.error;
    return value ? value.as<TextPageState>() : nullptr;
}

ErrorCode createTextPage(Handle page, Handle* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    BorrowedHandle pageBorrow = handles().borrow(page, HandleType::Page);
    if (!pageBorrow) return pageBorrow.error;
    auto pageState = pageBorrow.as<PageState>();
    std::unique_lock<std::mutex> lock(pageState->document->mutex);
    Handle existing = handles().findChild(page, HandleType::TextPage);
    if (existing) {
        *result = existing;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    FPDF_TEXTPAGE tp = FPDFText_LoadPage(pageState->pdf);
    if (!tp) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    auto textState = std::make_shared<TextPageState>(tp, pageState);
    ErrorCode code = ErrorCode::Ok;
    Handle handle = handles().insert(HandleType::TextPage, textState,
                                     pageBorrow.documentOwner, page, &code);
    if (!handle) {
        // The parent may have been invalidated after borrow. Close under the
        // already-held document mutex, then disarm the RAII wrapper.
        FPDFText_ClosePage(tp);
        textState->pdf = nullptr;
        setLastError(code);
        return code;
    }
    *result = handle;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode closeTextPage(Handle textPage) {
    ErrorCode code = handles().invalidate(textPage, HandleType::TextPage);
    setLastError(code);
    return code;
}

ErrorCode reloadTextPage(Handle page) {
    // In-place FPDF_TEXTPAGE rebuild: token stays stable, Java keeps its handle.
    const Handle textPage = handles().findChild(page, HandleType::TextPage);
    if (!textPage) {
        ErrorCode pageError;
        if (!borrowPage(page, &pageError)) return pageError;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    ErrorCode code;
    auto state = borrowTextPage(textPage, &code, page);
    if (!state) return code;
    auto pageState = state->page;  // parent page is already validated by borrow
    std::lock_guard<std::mutex> lock(pageState->document->mutex);
    FPDF_TEXTPAGE fresh = FPDFText_LoadPage(pageState->pdf);
    if (!fresh) {
        setLastError(ErrorCode::General);  // keep the old, still readable TextPage
        return ErrorCode::General;
    }
    FPDF_TEXTPAGE old = state->pdf;
    state->pdf = fresh;
    FPDFText_ClosePage(old);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}


ErrorCode openDocument(const std::string& path, Handle* result, const char* password) {
    if (!result || path.empty()) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    FPDF_DOCUMENT pdf = FPDF_LoadDocument(path.c_str(), password);
    const ErrorCode openCode = pdf ? ErrorCode::Ok : mapPdfiumOpenError(FPDF_GetLastError());
    if (!pdf && openCode != ErrorCode::Password) {
        setLastError(openCode);
        return openCode;
    }
    ErrorCode code = ErrorCode::Ok;
    Handle handle = handles().insert(HandleType::Document,
                                     std::make_shared<DocumentState>(pdf, path), 0, 0, &code);
    if (!handle) return code;
    *result = handle;
    setLastError(openCode);
    return openCode;
}

ErrorCode openDocumentBytes(const uint8_t* bytes, std::size_t size,
                            Handle* result, const char* password) {
    if (!result || !bytes || size == 0 || size > (512ULL << 20)) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    std::shared_ptr<DocumentState> state;
    try {
        state = std::make_shared<DocumentState>(nullptr, std::string{});
        state->backingBytes.assign(bytes, bytes + size);
    } catch (const std::bad_alloc&) {
        setLastError(ErrorCode::Io);
        return ErrorCode::Io;
    }
    state->pdf = FPDF_LoadMemDocument64(state->backingBytes.data(),
                                         state->backingBytes.size(), password);
    const ErrorCode openCode = state->pdf ? ErrorCode::Ok :
        mapPdfiumOpenError(FPDF_GetLastError());
    if (!state->pdf && openCode != ErrorCode::Password) {
        setLastError(openCode);
        return openCode;
    }
    ErrorCode code = ErrorCode::Ok;
    const Handle handle = handles().insert(HandleType::Document, state, 0, 0, &code);
    if (!handle) return code;
    *result = handle;
    setLastError(openCode);
    return openCode;
}

ErrorCode closeDocument(Handle document) {
    return handles().invalidateDocumentTree(document);
}

ErrorCode reopenWithPassword(Handle document, const std::string& password) {
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    if (state->pdf) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    FPDF_DOCUMENT reopened = state->backingBytes.empty()
        ? FPDF_LoadDocument(state->path.c_str(), password.c_str())
        : FPDF_LoadMemDocument64(state->backingBytes.data(),
                                  state->backingBytes.size(), password.c_str());
    if (!reopened) {
        code = mapPdfiumOpenError(FPDF_GetLastError());
        setLastError(code);
        return code;
    }
    state->pdf = reopened;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode closeDocumentParser(Handle document) {
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    code = handles().invalidateDocumentChildren(document);
    if (code != ErrorCode::Ok) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    if (!state->pdf) {
        setLastError(ErrorCode::InvalidHandle);
        return ErrorCode::InvalidHandle;
    }
    FPDF_CloseDocument(state->pdf);
    state->pdf = nullptr;
    state->recognitionContext.clear();
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode reopenDocument(Handle document, const std::string& path) {
    if (path.empty()) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    if (state->pdf) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    FPDF_DOCUMENT reopened = FPDF_LoadDocument(path.c_str(), nullptr);
    if (!reopened) {
        code = mapPdfiumOpenError(FPDF_GetLastError());
        setLastError(code);
        return code;
    }
    state->pdf = reopened;
    state->backingBytes.clear();
    state->path = path;
    state->recognitionContext.clear();
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getPageCount(Handle document, int* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    if (!state->pdf) {
        setLastError(ErrorCode::Password);
        return ErrorCode::Password;
    }
    *result = FPDF_GetPageCount(state->pdf);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode loadPage(Handle document, int index, Handle* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    FPDF_PAGE pdfPage = nullptr;
    {
        std::lock_guard<std::mutex> lock(state->mutex);
        if (!state->pdf) {
            setLastError(ErrorCode::Password);
            return ErrorCode::Password;
        }
        const int count = FPDF_GetPageCount(state->pdf);
        if (index < 0 || index >= count) {
            setLastError(ErrorCode::OutOfRange);
            return ErrorCode::OutOfRange;
        }
        pdfPage = FPDF_LoadPage(state->pdf, index);
        if (!pdfPage) {
#ifdef __ANDROID__
            __android_log_print(ANDROID_LOG_ERROR, "KSPEDIT",
                                "FPDF_LoadPage failed index=%d count=%d pdfError=%lu",
                                index, count, FPDF_GetLastError());
#endif
            setLastError(ErrorCode::General);
            return ErrorCode::General;
        }
        auto pending = std::find(state->pendingSharedFormSeparation.begin(),
                                 state->pendingSharedFormSeparation.end(), index);
        if (pending != state->pendingSharedFormSeparation.end()) {
            SeparateSharedForms separate = separateSharedFormsExtension();
            if (!separate || !separate(pdfPage) || !FPDFPage_GenerateContent(pdfPage)) {
                FPDF_ClosePage(pdfPage);
                setLastError(ErrorCode::General);
                return ErrorCode::General;
            }
            state->pendingSharedFormSeparation.erase(pending);
            ++state->mutationEpoch;
        }
    }
    auto page = std::make_shared<PageState>(pdfPage, state, index);
    Handle pageHandle = handles().insert(HandleType::Page, page, document, document, &code);
    if (!pageHandle) return code;
    *result = pageHandle;
    return ErrorCode::Ok;
}

ErrorCode closePage(Handle page) {
    return handles().invalidatePageTree(page);
}

ErrorCode getPageSize(Handle page, float* width, float* height) {
    if (!width || !height) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    *width = FPDF_GetPageWidthF(state->pdf);
    *height = FPDF_GetPageHeightF(state->pdf);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getPageSizeAt(Handle document, int index, float* width, float* height) {
    Handle page = 0;
    ErrorCode code = loadPage(document, index, &page);
    if (code != ErrorCode::Ok) return code;
    code = getPageSize(page, width, height);
    const ErrorCode closeCode = closePage(page);
    return code == ErrorCode::Ok ? closeCode : code;
}

ErrorCode getFormFill(Handle document, Handle* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    ErrorCode code;
    auto doc = borrowDocument(document, &code);
    if (!doc) return code;
    std::lock_guard<std::mutex> lock(doc->mutex);
    Handle existing = handles().findChild(document, HandleType::FormFill);
    if (existing) {
        *result = existing;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    auto form = std::make_shared<FormFillState>();
    form->document = document;
    Handle value = handles().insert(HandleType::FormFill, form, document,
                                    document, &code);
    if (!value) return code;
    *result = value;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getFormFillEditor(Handle formFill, Handle* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    BorrowedHandle borrowed = handles().borrow(formFill, HandleType::FormFill);
    if (!borrowed) return borrowed.error;
    auto form = borrowed.as<FormFillState>();
    std::lock_guard<std::mutex> lock(form->mutex);
    if (form->editor && handles().isValid(form->editor, HandleType::TextEditor)) {
        *result = form->editor;
        setLastError(ErrorCode::Ok);
        return ErrorCode::Ok;
    }
    auto editor = std::make_shared<TextEditorState>();
    editor->form = form;
    ErrorCode code;
    form->editor = handles().insert(HandleType::TextEditor, editor,
                                    borrowed.documentOwner, formFill, &code);
    if (!form->editor) return code;
    *result = form->editor;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode bindFormFillPage(Handle formFill, Handle page) {
    BorrowedHandle formBorrow = handles().borrow(formFill, HandleType::FormFill);
    if (!formBorrow) return formBorrow.error;
    BorrowedHandle pageBorrow = handles().borrow(
        page, HandleType::Page, formBorrow.documentOwner);
    if (!pageBorrow) return pageBorrow.error;
    auto form = formBorrow.as<FormFillState>();
    std::lock_guard<std::mutex> lock(form->mutex);
    form->currentPage = page;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode resolveTextEditorPage(Handle editor, Handle* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    *result = 0;
    BorrowedHandle borrowed = handles().borrow(editor, HandleType::TextEditor);
    if (!borrowed) return borrowed.error;
    auto state = borrowed.as<TextEditorState>();
    auto form = state->form.lock();
    if (!form) {
        setLastError(ErrorCode::StaleHandle);
        return ErrorCode::StaleHandle;
    }
    std::lock_guard<std::mutex> lock(form->mutex);
    BorrowedHandle page = handles().borrow(
        form->currentPage, HandleType::Page, borrowed.documentOwner);
    if (!page) return page.error;
    *result = form->currentPage;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

}  // namespace ksp::runtime
