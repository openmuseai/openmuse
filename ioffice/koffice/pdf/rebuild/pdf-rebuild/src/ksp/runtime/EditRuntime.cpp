#include "EditRuntime.h"
#include "ObjectRuntime.h"

#include <fpdf_edit.h>

#include <algorithm>
#include <dlfcn.h>
#include <vector>

namespace ksp::runtime {
namespace {

using SeparateSharedForms = FPDF_BOOL (*)(FPDF_PAGE);

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

bool validEditStatus(int status) {
    return status >= kEditStatusNone && status <= kEditStatusClipPath;
}

bool validInsertStatus(int status) {
    return status >= kInsertStatusNone && status <= kInsertStatusTable;
}

uint32_t functionBit(int status) {
    return status > kEditStatusNone && status <= kEditStatusClipPath
               ? (uint32_t{1} << static_cast<unsigned>(status)) : 0;
}

std::shared_ptr<PageState> borrowOwnedPage(Handle document, Handle page,
                                           ErrorCode* error) {
    auto documentState = borrowDocument(document, error);
    if (!documentState) return nullptr;
    return borrowPage(page, error, document);
}

}  // namespace

ErrorCode setEditStatus(Handle document, int status) {
    if (!validEditStatus(status)) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    state->editStatus = status;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getEditStatus(Handle document, int* status) {
    if (!status) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    *status = state->editStatus;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode setInsertStatus(Handle document, int status) {
    if (!validInsertStatus(status)) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    state->insertStatus = status;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getInsertStatus(Handle document, int* status) {
    if (!status) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    *status = state->insertStatus;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode stagePageMutation(Handle page, int editStatus, bool focus, bool hover) {
    const uint32_t bit = functionBit(editStatus);
    if (!bit) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    state->dirtyFunctions |= bit;
    state->stagedEpoch = ++state->document->mutationEpoch;
    state->hasFocus = focus;
    state->hasHover = hover;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode backupEditPage(Handle document, Handle page, int editStatus) {
    if (!validEditStatus(editStatus) || editStatus == kEditStatusNone) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowOwnedPage(document, page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    state->backupDirtyFunctions = state->dirtyFunctions;
    state->editBackupUndoDepth = state->editUndoHandles.size();
    state->hasEditBackup = true;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode syncEditPage(Handle document, Handle page, bool) {
    ErrorCode code;
    auto state = borrowOwnedPage(document, page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->pdf || !FPDFPage_GenerateContent(state->pdf)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    state->committedEpoch = state->stagedEpoch;
    if (state->dirtyFunctions & functionBit(1)) ++state->textCacheEpoch;
    state->dirtyFunctions = 0;
    state->hasEditBackup = false;
    state->editBackupUndoDepth = state->editUndoHandles.size();
    state->hasFocus = false;
    state->hasHover = false;
    state->focusedTextDomain = -1;
    state->focusedTextObjectId = 0;
    state->textSelectionStart = state->textSelectionEnd = 0;
    invalidateEditableObjects(page, *state);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode clearEditPageData(Handle document, Handle page) {
    ErrorCode code;
    auto state = borrowOwnedPage(document, page, &code);
    if (!state) return code;
    std::vector<Handle> rollback;
    {
        std::lock_guard<std::mutex> lock(state->document->mutex);
        const std::size_t depth = std::min(state->editBackupUndoDepth,
                                           state->editUndoHandles.size());
        rollback.assign(state->editUndoHandles.begin() + depth,
                        state->editUndoHandles.end());
    }
    for (auto item = rollback.rbegin(); item != rollback.rend(); ++item)
        (void)undoEditCommand(*item);
    for (Handle command : rollback) (void)deleteEditCommand(command);
    std::lock_guard<std::mutex> lock(state->document->mutex);
    const std::size_t depth = std::min(state->editBackupUndoDepth,
                                       state->editUndoHandles.size());
    state->editUndoHandles.resize(depth);
    state->pendingEditUndoHandles.erase(
        std::remove_if(state->pendingEditUndoHandles.begin(),
                       state->pendingEditUndoHandles.end(), [&](Handle value) {
                           return std::find(rollback.begin(), rollback.end(), value) != rollback.end();
                       }),
        state->pendingEditUndoHandles.end());
    state->dirtyFunctions = 0;
    state->hasEditBackup = false;
    state->hasFocus = false;
    state->hasHover = false;
    state->focusedTextDomain = -1;
    state->focusedTextObjectId = 0;
    state->textSelectionStart = state->textSelectionEnd = 0;
    state->stagedEpoch = state->committedEpoch;
    ++state->textCacheEpoch;
    invalidateEditableObjects(page, *state);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode clearEditFocus(Handle document, Handle page) {
    ErrorCode code;
    auto state = borrowOwnedPage(document, page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    state->hasFocus = false;
    state->focusedTextDomain = -1;
    state->focusedTextObjectId = 0;
    state->textSelectionStart = state->textSelectionEnd = 0;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode clearHoverHandle(Handle document, Handle page) {
    ErrorCode code;
    auto state = borrowOwnedPage(document, page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    state->hasHover = false;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode separateSharedDataInEditPage(Handle document, int pageIndex) {
    ErrorCode code;
    auto state = borrowDocument(document, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->mutex);
    if (!state->pdf || pageIndex < 0 || pageIndex >= FPDF_GetPageCount(state->pdf)) {
        setLastError(ErrorCode::OutOfRange);
        return ErrorCode::OutOfRange;
    }
    SeparateSharedForms separate = separateSharedFormsExtension();
    if (!separate) {
        setLastError(ErrorCode::Unsupported);
        return ErrorCode::Unsupported;
    }
    // PDFium's Form instance and its generated resources belong to the live
    // editor page. Defer until loadPage() owns that page for the full edit/save
    // lifetime; a temporary FPDF_LoadPage loses the detached resource graph on
    // Android when it closes.
    if (std::find(state->pendingSharedFormSeparation.begin(),
                  state->pendingSharedFormSeparation.end(), pageIndex) ==
        state->pendingSharedFormSeparation.end()) {
        state->pendingSharedFormSeparation.push_back(pageIndex);
    }
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode regeneratePageContent(Handle page) {
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    if (!state->pdf || !FPDFPage_GenerateContent(state->pdf)) {
        setLastError(ErrorCode::General);
        return ErrorCode::General;
    }
    invalidateEditableObjects(page, *state);
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode isPageModified(Handle page, bool* modified) {
    if (!modified) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    *modified = state->dirtyFunctions != 0;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode isPageFunctionModified(Handle page, int editStatus, bool* modified) {
    if (!modified) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    const uint32_t bit = functionBit(editStatus);
    if (!bit) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    *modified = (state->dirtyFunctions & bit) != 0;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

ErrorCode getEditDiagnostics(Handle page, EditDiagnostics* result) {
    if (!result) {
        setLastError(ErrorCode::InvalidArgument);
        return ErrorCode::InvalidArgument;
    }
    ErrorCode code;
    auto state = borrowPage(page, &code);
    if (!state) return code;
    std::lock_guard<std::mutex> lock(state->document->mutex);
    result->dirtyFunctions = state->dirtyFunctions;
    result->stagedEpoch = state->stagedEpoch;
    result->committedEpoch = state->committedEpoch;
    result->textCacheEpoch = state->textCacheEpoch;
    result->hasBackup = state->hasEditBackup;
    result->hasFocus = state->hasFocus;
    result->hasHover = state->hasHover;
    setLastError(ErrorCode::Ok);
    return ErrorCode::Ok;
}

}  // namespace ksp::runtime
