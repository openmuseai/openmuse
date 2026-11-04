#pragma once

#include "HandleRegistry.h"

#include <fpdfview.h>

#include <memory>
#include <cstddef>
#include <mutex>
#include <string>
#include <cstdint>
#include <vector>
#include <unordered_map>

namespace ksp::runtime {

struct RecognitionPageSnapshot;

struct DocumentState {
    DocumentState(FPDF_DOCUMENT value, std::string sourcePath)
        : pdf(value), path(std::move(sourcePath)) {}
    ~DocumentState();

    FPDF_DOCUMENT pdf = nullptr;
    std::string path;
    // PDFium borrows memory input for the entire document lifetime.
    std::vector<uint8_t> backingBytes;
    std::mutex mutex;
    int editStatus = 0;
    int insertStatus = 0;
    uint64_t mutationEpoch = 0;
    // SectionRecognize creates a short-lived page analyzer, then commits its
    // result into a longer-lived document edit session. Keep only stable
    // observations here: never retain PageState, PDFium object, or editor
    // pointers across Java page-wrapper recycling.
    struct RecognitionContext {
        uint64_t invocationCount = 0;
        uint64_t replacementCount = 0;
        std::vector<int> firstSeenPages;
        // The original consumer owns a page-keyed tree of copied recognition
        // records.  Store immutable, pointer-free copies at Document lifetime;
        // a Java PDFPage wrapper may be recycled without dropping them.
        std::unordered_map<int, std::shared_ptr<const RecognitionPageSnapshot>> pages;
        int lastPageIndex = -1;
        int lastDomainCount = 0;
        int lastAlgorithmGeneration = 1;
        // Aggregated, pointer-free processor state. It is committed only after
        // a successful recognize pass and recomputed when a page snapshot is
        // replaced, so retrying one page cannot double-count its evidence.
        uint64_t processorCommitCount = 0;
        int processorFormulaPageCount = 0;
        int processorFormulaDomainCount = 0;
        int processorFormulaSpanCount = 0;
        int processorCompoundFormulaCount = 0;
        int processorMaxFormulaSpanCount = 0;

        void clear() {
            invocationCount = 0;
            replacementCount = 0;
            firstSeenPages.clear();
            pages.clear();
            lastPageIndex = -1;
            lastDomainCount = 0;
            lastAlgorithmGeneration = 1;
            processorCommitCount = 0;
            processorFormulaPageCount = 0;
            processorFormulaDomainCount = 0;
            processorFormulaSpanCount = 0;
            processorCompoundFormulaCount = 0;
            processorMaxFormulaSpanCount = 0;
        }
    } recognitionContext;
    // Page indexes requested through separateSharedDataInEditPage. Separation
    // must run on the live editing page, rather than a throwaway PDFium page.
    std::vector<int> pendingSharedFormSeparation;
};

struct TextPageState;  // defined below

struct FormFillState {
    Handle document = 0;
    Handle currentPage = 0;
    Handle editor = 0;
    std::mutex mutex;
};

struct TextEditorState {
    std::weak_ptr<FormFillState> form;
};

struct EditableObjectRecord {
    uint64_t localId = 0;
    FPDF_PAGEOBJECT object = nullptr;  // borrowed; never crosses the JNI boundary
    // Borrowed outermost-to-innermost Form chain. Runtime mutations use it to
    // dirty each private Form stream through the fixed-version ExtensionAPI.
    std::vector<FPDF_PAGEOBJECT> formAncestors;
    int pageObjectIndex = -1;
    int type = 0;
    float left = 0;
    float top = 0;
    float right = 0;
    float bottom = 0;
    float pageLeft = 0;
    float pageTop = 0;
    float pageRight = 0;
    float pageBottom = 0;
    // Analyzer grouping envelope. For a text object inside a Form XObject
    // this is the containing Form's page-space bounds; output geometry still
    // comes from the text quad itself.
    float groupLeft = 0;
    float groupTop = 0;
    float groupRight = 0;
    float groupBottom = 0;
    // Android Matrix order: a,c,e,b,d,f,0,0,1.
    float matrix[9] = {1, 0, 0, 0, 1, 0, 0, 0, 1};
};

struct EditableTextLine {
    std::vector<uint64_t> objectIds;
    float left = 0;
    float top = 0;
    float right = 0;
    float bottom = 0;
    float baseline = 0;
    float medianHeight = 0;
};

enum class EditableTextDomainKind : uint8_t {
    Section,
    Paragraph,
    Line,
    Span,
    Formula,
    Table,
    TableRow,
    TableCell,
    ReferenceLabel,
    ReferenceBody,
    FormParagraph,
};

struct EditableTextDomainNode {
    uint64_t id = 0;
    uint64_t parentId = 0;
    EditableTextDomainKind kind = EditableTextDomainKind::Section;
    int editableDomainIndex = -1;
    std::vector<uint64_t> children;
};

struct EditableTextDomain {
    uint64_t nodeId = 0;
    uint64_t parentNodeId = 0;
    EditableTextDomainKind kind = EditableTextDomainKind::Paragraph;
    bool editableLeaf = true;
    std::vector<uint64_t> objectIds;
    std::vector<EditableTextLine> lines;
    float left = 0;
    float top = 0;
    float right = 0;
    float bottom = 0;
    float groupLeft = 0;
    float groupTop = 0;
    float groupRight = 0;
    float groupBottom = 0;
    float orientation = 0;
    float matrix[9] = {1, 0, 0, 0, 1, 0, 0, 0, 1};
};

enum class TextEditorProjectionStage : uint8_t {
    TextBlock,
    TextRun,
    Span,
};

// Public editor geometry is a view over one writable domain. Multiple
// structural nodes may project to the same domain without acquiring ownership
// of its PageObjects.
struct TextEditorProjection {
    uint64_t sourceNodeId = 0;
    int targetDomainIndex = -1;
    EditableTextDomainKind kind = EditableTextDomainKind::Paragraph;
    TextEditorProjectionStage stage = TextEditorProjectionStage::TextBlock;
    float left = 0;
    float top = 0;
    float right = 0;
    float bottom = 0;
    float matrix[9] = {1, 0, 0, 0, 1, 0, 0, 0, 1};
};

// Stable input handed from the one-call page analyzer to the persistent
// document consumer.  Public JNI editors remain a projection owned by the live
// PageState; this snapshot deliberately contains no PDFium or registry handle.
struct RecognitionPageSnapshot {
    int pageIndex = -1;
    int algorithmGeneration = 1;
    uint64_t sourceMutationEpoch = 0;
    uint64_t sourceInventoryEpoch = 0;
    std::vector<EditableTextDomain> domains;
    std::vector<EditableTextDomainNode> hierarchy;
    std::vector<TextEditorProjection> projections;
};

// Harness-only structural counters. They make the persistent recognition tree
// observable without exposing PDFium pointers or mutable runtime objects.
struct RecognitionPageDiagnostics {
    int domainCount = 0;
    int nodeCount = 0;
    int sectionCount = 0;
    int paragraphCount = 0;
    int lineCount = 0;
    int spanCount = 0;
    int formulaCount = 0;
    int tableCount = 0;
    int tableRowCount = 0;
    int tableCellCount = 0;
};

struct RecognitionDomainDiagnostics {
    int kind = 0;
    int lineCount = 0;
    int objectCount = 0;
    int spanCount = 0;
};

struct PageState {
    PageState(FPDF_PAGE value, std::shared_ptr<DocumentState> parent, int pageIndex)
        : pdf(value), document(std::move(parent)), index(pageIndex) {}
    ~PageState();

    FPDF_PAGE pdf = nullptr;
    std::shared_ptr<DocumentState> document;
    int index = -1;
    uint32_t dirtyFunctions = 0;
    uint32_t backupDirtyFunctions = 0;
    uint64_t stagedEpoch = 0;
    uint64_t committedEpoch = 0;
    uint64_t textCacheEpoch = 0;
    bool hasEditBackup = false;
    bool hasFocus = false;
    bool hasHover = false;
    bool editableInventoryValid = false;
    uint64_t editableInventoryEpoch = 0;
    std::vector<EditableObjectRecord> editableObjects;
    std::unordered_map<uint64_t, Handle> editableObjectHandles;
    // Logical UTF-16 text for runtime-created shaped objects. PDFium exposes
    // these objects in visual order before save/reopen; the editor contract is
    // always logical order.
    std::unordered_map<FPDF_PAGEOBJECT, std::u16string> shapedLogicalText;
    std::unordered_map<FPDF_PAGEOBJECT, uint32_t> persistentShapedRunIds;
    bool textDomainsRecognized = false;
    std::vector<EditableTextDomain> textDomains;
    // Structural nodes are separate from the public writable editor list.
    // This mirrors the original LrSect -> LrParagraph/LrFormula hierarchy
    // without assigning one PageObject to more than one writable owner.
    std::vector<EditableTextDomainNode> textDomainHierarchy;
    std::vector<TextEditorProjection> textEditorProjections;
    int focusedTextDomain = -1;
    uint64_t focusedTextObjectId = 0;
    int textSelectionStart = 0;
    int textSelectionEnd = 0;
    int textSelectionAnchor = 0;
    bool textSelectionDragging = false;
    // Original editor keeps its paragraph frame stable while incremental IME
    // commits rebuild the underlying PageObjects.
    FPDF_PAGEOBJECT textReflowAnchor = nullptr;
    float textReflowLeft = 0;
    float textReflowRight = 0;
    float textReflowFirstBaseline = 0;
    float textReflowBaselineStep = 0;
    float textReflowOrientation = 0;
    // The original native editor exposes a layout frame separately from the
    // union of painted glyph bounds. Form-backed paragraphs reserve another
    // leading row when overflow reaches the following page object.
    bool textReflowHasLayoutBottom = false;
    float textReflowLayoutBottom = 0;
    float textReflowInitialBottom = 0;
    int textReflowInitialLineCount = 0;
    // Opaque commands mirror PDFFormFill.onAddEditUndo -> PDFMixEditCommand.
    // Their registry parent is Document, while the payload retains this page,
    // so a transaction survives recycling the transient Java PDFPage token.
    std::vector<Handle> editUndoHandles;
    std::vector<Handle> pendingEditUndoHandles;
    std::size_t editBackupUndoDepth = 0;
};

struct PageObjectState {
    std::shared_ptr<PageState> page;
    uint64_t localId = 0;
    uint64_t inventoryEpoch = 0;
    int type = 0;
};

ErrorCode openDocument(const std::string& path, Handle* result,
                       const char* password = nullptr);
ErrorCode openDocumentBytes(const uint8_t* bytes, std::size_t size,
                            Handle* result, const char* password = nullptr);
ErrorCode closeDocument(Handle document);
ErrorCode reopenWithPassword(Handle document, const std::string& password);
ErrorCode closeDocumentParser(Handle document);
ErrorCode reopenDocument(Handle document, const std::string& path);
ErrorCode getPageCount(Handle document, int* result);
ErrorCode loadPage(Handle document, int index, Handle* result);
ErrorCode closePage(Handle page);
ErrorCode getPageSize(Handle page, float* width, float* height);
ErrorCode getPageSizeAt(Handle document, int index, float* width, float* height);
// The original Set/GetSectionRecognizeAlgGen pair addresses module state and
// has no document-handle argument in its JNI descriptor.
bool setSectionRecognizeAlgorithmGeneration(int generation);
int sectionRecognizeAlgorithmGeneration();
struct RecognitionContextDiagnostics {
    uint64_t invocationCount = 0;
    int uniquePageCount = 0;
    int lastPageIndex = -1;
    int lastDomainCount = 0;
    int lastAlgorithmGeneration = 1;
    int cachedPageCount = 0;
    int cachedDomainCount = 0;
    uint64_t replacementCount = 0;
};
struct RecognitionProcessorDiagnostics {
    uint64_t commitCount = 0;
    int recognizedPageCount = 0;
    int formulaPageCount = 0;
    int formulaDomainCount = 0;
    int formulaSpanCount = 0;
    int compoundFormulaCount = 0;
    int maxFormulaSpanCount = 0;
    int lastPageIndex = -1;
    int algorithmGeneration = 1;
};
ErrorCode getRecognitionContextDiagnostics(
    Handle document, RecognitionContextDiagnostics* result);
ErrorCode getRecognitionProcessorDiagnostics(
    Handle document, RecognitionProcessorDiagnostics* result);
ErrorCode getRecognitionPageDiagnostics(
    Handle document, int pageIndex, RecognitionPageDiagnostics* result);
ErrorCode getRecognitionDomainDiagnostics(
    Handle document, int pageIndex,
    std::vector<RecognitionDomainDiagnostics>* result);
ErrorCode getFormFill(Handle document, Handle* result);
ErrorCode getFormFillEditor(Handle formFill, Handle* result);
ErrorCode bindFormFillPage(Handle formFill, Handle page);
ErrorCode resolveTextEditorPage(Handle editor, Handle* result);

// TextPage: one live token per page, owner = page handle. Paired with the page's
// lifetime: closing a page invalidates its TextPage first (see PdfRuntime.cpp).
struct TextPageState {
    TextPageState(FPDF_TEXTPAGE value, std::shared_ptr<PageState> parent)
        : pdf(value), page(std::move(parent)) {}
    ~TextPageState();

    FPDF_TEXTPAGE pdf = nullptr;
    std::shared_ptr<PageState> page;
};

ErrorCode createTextPage(Handle page, Handle* result);
ErrorCode closeTextPage(Handle textPage);
ErrorCode reloadTextPage(Handle page);  // in-place swap, token unchanged
std::shared_ptr<TextPageState> borrowTextPage(Handle textPage, ErrorCode* error = nullptr,
                                              Handle expectedPage = 0);

std::shared_ptr<DocumentState> borrowDocument(Handle document, ErrorCode* error = nullptr);
std::shared_ptr<PageState> borrowPage(Handle page, ErrorCode* error = nullptr,
                                      Handle expectedDocument = 0);

ErrorCode mapPdfiumOpenError(unsigned long error);

}  // namespace ksp::runtime
