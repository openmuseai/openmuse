#pragma once

#include "PdfRuntime.h"

namespace ksp::runtime {

ErrorCode analyzeEditableObjects(Handle page);
ErrorCode recognizeTextDomains(Handle page);
ErrorCode getTextEditorCount(Handle page, int* result);
ErrorCode getTextEditorGeometry(Handle page, int index, bool reverse,
                                EditableObjectRecord* result);
ErrorCode getTextEditorIds(Handle page, std::vector<uint64_t>* result);
struct TextObjectDiagnostic {
    uint64_t localId = 0;
    float pageLeft = 0;
    float pageBottom = 0;
    float pageRight = 0;
    float pageTop = 0;
    float groupLeft = 0;
    float groupBottom = 0;
    float groupRight = 0;
    float groupTop = 0;
    float fontSize = 0;
    std::string fontName;
    uint32_t fillColor = 0;
    int renderMode = -1;
    int formDepth = 0;
    float matrix[9] = {1, 0, 0, 0, 1, 0, 0, 0, 1};
    std::u16string text;
};
ErrorCode getTextObjectDiagnostics(Handle page,
                                   std::vector<TextObjectDiagnostic>* result);
ErrorCode getPageObjectHandles(Handle page, int type, std::vector<Handle>* result);
ErrorCode getPageObjectRect(Handle page, Handle object, int expectedType,
                            EditableObjectRecord* result);
ErrorCode getPageObjectAtPoint(Handle page, int expectedType, float x, float y,
                               Handle* result);
ErrorCode getPageObjectIndex(Handle page, Handle object, int expectedType, int* result);
ErrorCode removePageObject(Handle page, Handle object, int expectedType);
ErrorCode resizeImageObject(Handle page, Handle image, float left, float bottom,
                            float right, float top);
ErrorCode rotateImageObjectClockwise(Handle page, Handle image);
ErrorCode reverseImageObjectHorizontal(Handle page, Handle image);
ErrorCode focusTextAtPoint(Handle page, float x, float y, int flags, bool* focused);
ErrorCode moveTextSelectionAtPoint(Handle page, float x, float y, int flags, bool* handled);
ErrorCode endTextSelectionDrag(Handle page, bool* handled);
ErrorCode getTextHoverType(Handle page, float x, float y, int* type);
ErrorCode selectTextWordAtPoint(Handle page, float x, float y, bool* selected);
ErrorCode selectTextObjectAtPoint(Handle page, float x, float y, bool* selected);
ErrorCode getTextFocusState(Handle page, int* type, int* status, int* count,
                            EditableObjectRecord* geometry);
ErrorCode getFocusedText(Handle page, std::u16string* result);
ErrorCode getFocusedDomainText(Handle page, std::u16string* result);
ErrorCode getTextSelection(Handle page, int* start, int* end);
ErrorCode setTextSelection(Handle page, int start, int end);
ErrorCode getTextRangeAtPoint(Handle page, float x, float y, int* index);
struct TextCharacterGeometry {
    float quad[8] = {0};  // baseline start/end followed by top end/start in page space
};
struct TextCaretGeometry {
    float bottomX = 0;
    float bottomY = 0;
    float topX = 0;
    float topY = 0;
};
ErrorCode getTextCharacterGeometry(Handle page, int index, TextCharacterGeometry* geometry);
ErrorCode getTextCaretGeometry(Handle page, int position, TextCaretGeometry* geometry);
struct TextSelectionRect {
    float left = 0;
    float bottom = 0;
    float right = 0;
    float top = 0;
    int direction = 0;  // PDFPageEditor.LocateRect: 0 LTR, 1 RTL
};
ErrorCode getTextSelectionRects(Handle page, std::vector<TextSelectionRect>* result);
ErrorCode getTextSelectionDirection(Handle page, int* direction);
ErrorCode replaceFocusedText(Handle page, const std::u16string& replacement);
ErrorCode getFocusedTextFontSize(Handle page, float* result);
ErrorCode setFocusedTextFontSize(Handle page, float size);
ErrorCode getFocusedTextColor(Handle page, uint32_t* argb);
ErrorCode setFocusedTextColor(Handle page, uint32_t argb);
ErrorCode getFocusedTextFontName(Handle page, std::string* result);
ErrorCode setFocusedTextFontName(Handle page, const std::string& name);
ErrorCode getFocusedTextBold(Handle page, bool* result);
ErrorCode setFocusedTextBold(Handle page, bool bold);
ErrorCode getFocusedTextAlignment(Handle page, int* result);
ErrorCode setFocusedTextAlignment(Handle page, int alignment);
ErrorCode getFocusedTextLeading(Handle page, float* result);
ErrorCode setFocusedTextLeading(Handle page, float leading);
ErrorCode handleTextKeyDown(Handle page, int keyCode, bool* handled);
ErrorCode takePendingEditUndo(Handle page, Handle* result);
struct EditDamageRect {
    float left = 0;
    float bottom = 0;
    float right = 0;
    float top = 0;
};
ErrorCode getEditCommandDamage(Handle command, EditDamageRect* result);
ErrorCode undoEditCommand(Handle command);
ErrorCode redoEditCommand(Handle command);
ErrorCode deleteEditCommand(Handle command);
void invalidateEditableObjects(Handle page, PageState& state);

}  // namespace ksp::runtime
