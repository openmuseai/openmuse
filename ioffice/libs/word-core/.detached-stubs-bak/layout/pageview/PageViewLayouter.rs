// Rust stub for PageViewLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/pageview/PageViewLayouter.java
use crate::*;

pub struct PageViewLayouter {
    // TODO: fields
}

impl PageViewLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn preFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn checkAndSetMirrorMaringOffset(&self, pageEnv: PageEnv, newPage: TypoLayoutPage, pHeaderFooterPage: i32) -> bool { todo!() }
    pub fn postFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn flowPage(&self) -> TypoLayoutPage { todo!() }
    pub fn changePageSize(&self) { todo!() }
    pub fn ensureValidEndPage(&self, rect: Rect) -> bool { todo!() }
    pub fn initLayouters(&self) { todo!() }
    pub fn initTableLayouter(&self) { todo!() }
    pub fn initDrawingLayouter(&self) { todo!() }
    pub fn initFootEndNoteLayouter(&self) { todo!() }
    pub fn createLayoutState(&self) -> LayoutState { todo!() }
    pub fn finishPage(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn needFlowRestFootnote(&self, docType: i32) -> bool { todo!() }
    pub fn flowCP(&self, cp: i32, endCp: i32, docType: i32) { todo!() }
    pub fn hasFlowCP(&self, layoutState: LayoutState, cp: i32) -> bool { todo!() }
    pub fn prepareToJumpToLastView(&self) -> bool { todo!() }
    pub fn jumpForGrid(&self, service: ScrollMemoryService) -> bool { todo!() }
    pub fn flowTextFrame(&self, layoutState: LayoutState) { todo!() }
    pub fn flowToPage(&self, pageIndex: i32) -> i32 { todo!() }
    pub fn rollbackLine(&self, textLine: i32, layoutState: LayoutState) { todo!() }
    pub fn restartSectionLayouter(&self, layoutState: LayoutState) { todo!() }
    pub fn rollbackLines(&self, layoutState: LayoutState) { todo!() }
    pub fn onNewRangeEnd(&self, lastRangeEnd: i32, rangeEnd: i32) { todo!() }
    pub fn isComplete(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn doAfterFlowLine(&self, layoutState: LayoutState) { todo!() }
    pub fn getFootEndNoteLayouter(&self) -> FootEndNoteLayouter { todo!() }
    pub fn prepareFlowFromDocumentBegin(&self) -> bool { todo!() }
    pub fn prepareFlowInScrollDirection(&self) -> bool { todo!() }
    pub fn prepareFlowToDocumentEnd(&self) -> bool { todo!() }
    pub fn acceptFirstCache(&self, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn onModifyFirst(&self, pageIndex: i32, recycleAfter: bool) { todo!() }
    pub fn processEvents(&self, layoutRect: Rect) { todo!() }
    pub fn getEditor(&self) { todo!() }
    pub fn flowTextLine(&self, layoutState: LayoutState) { todo!() }
    pub fn doFlowLine(&self, layoutState: LayoutState) { todo!() }
    pub fn processPageBreakBeforePara(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn flowColumns(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn clearTypoDocument(&self) { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn resetForInit(&self) { todo!() }
    pub fn getType(&self) -> i32 { todo!() }
    pub fn doOnIoLoadFinish(&self) { todo!() }
    pub fn onAfterFirstRender(&self) { todo!() }
    pub fn clearFootNoteRefCPs(&self, layoutState: LayoutState) { todo!() }
    pub fn clearDrawing(&self, layoutState: LayoutState) { todo!() }
    pub fn makeBalloonCaches(&self) { todo!() }
    pub fn flowDocument(&self, maxPageCount: i32, interrupt: Box<dyn IInterruptState>) { todo!() }
    pub fn swapToLocal(&self, sync: bool) { todo!() }
    pub fn swapFromLocal(&self) -> bool { todo!() }
    pub fn rollBackDropCap(&self, layoutState: LayoutState) -> bool { todo!() }
}
