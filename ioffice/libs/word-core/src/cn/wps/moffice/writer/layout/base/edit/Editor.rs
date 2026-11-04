// Rust stub for Editor (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/edit/Editor.java
use crate::*;

pub struct Editor {
    // TODO: fields
}

impl Editor {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn getEditLayoutPage(&self) -> i32 { todo!() }
    pub fn flow(&self, rect: Rect, layoutState: PageViewLayoutState) -> bool { todo!() }
    pub fn flowEditNormal(&self, rect: Rect, pageViewLayoutState: PageViewLayoutState) -> bool { todo!() }
    pub fn updateRefCPs(&self) { todo!() }
    pub fn offsetLayoutPages(&self, rect: Rect) { todo!() }
    pub fn isWebMode(&self) -> bool { todo!() }
    pub fn findSpaceBefore(&self, cp: i32) -> bool { todo!() }
    pub fn isTextBoxOfTextFrame(&self, page: i32) -> bool { todo!() }
    pub fn checkInsertTextLine(&self) -> bool { todo!() }
    pub fn checkRemoveTextLine(&self) -> bool { todo!() }
    pub fn checkReplaceTextLine(&self) -> bool { todo!() }
    pub fn checkRevisionReplaceTextLine(&self) -> bool { todo!() }
    pub fn checkRevisionRemoveTextLine(&self) -> bool { todo!() }
    pub fn needQuickFlowFromPrevLine(&self, textLine: i32, mainEvent: Event) -> bool { todo!() }
    pub fn checkForFirstLine(&self) -> bool { todo!() }
    pub fn checkForPagination(&self) -> bool { todo!() }
    pub fn hasPaginationProp(&self) -> bool { todo!() }
    pub fn isFirstParaOfPage(&self) -> bool { todo!() }
    pub fn isLayoutPageStartAtCP0(&self) -> bool { todo!() }
    pub fn findLine(&self, event: Event) -> bool { todo!() }
    pub fn isCellStart(&self, document: Box<dyn Document>, cp: i32) -> bool { todo!() }
}
