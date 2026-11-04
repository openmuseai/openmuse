#pragma once

#include "PdfRuntime.h"

#include <cstdint>

namespace ksp::runtime {

// Values are fixed by cn.wps.moffice.pdf.core.PDF.EditStatus.
constexpr int kEditStatusNone = 0;
constexpr int kEditStatusClipPath = 8;
constexpr int kInsertStatusNone = 0;
constexpr int kInsertStatusTable = 26;

ErrorCode setEditStatus(Handle document, int status);
ErrorCode getEditStatus(Handle document, int* status);
ErrorCode setInsertStatus(Handle document, int status);
ErrorCode getInsertStatus(Handle document, int* status);

ErrorCode syncEditPage(Handle document, Handle page, bool forSave);
ErrorCode clearEditPageData(Handle document, Handle page);
ErrorCode clearEditFocus(Handle document, Handle page);
ErrorCode clearHoverHandle(Handle document, Handle page);
ErrorCode backupEditPage(Handle document, Handle page, int editStatus);
ErrorCode separateSharedDataInEditPage(Handle document, int pageIndex);

ErrorCode regeneratePageContent(Handle page);
ErrorCode isPageModified(Handle page, bool* modified);
ErrorCode isPageFunctionModified(Handle page, int editStatus, bool* modified);

// Mutation entry point for the object-edit work items. It centralizes dirty,
// cache and focus state so text/image/path implementations cannot diverge.
ErrorCode stagePageMutation(Handle page, int editStatus, bool focus, bool hover);

struct EditDiagnostics {
    uint32_t dirtyFunctions = 0;
    uint64_t stagedEpoch = 0;
    uint64_t committedEpoch = 0;
    uint64_t textCacheEpoch = 0;
    bool hasBackup = false;
    bool hasFocus = false;
    bool hasHover = false;
};
ErrorCode getEditDiagnostics(Handle page, EditDiagnostics* result);

}  // namespace ksp::runtime
