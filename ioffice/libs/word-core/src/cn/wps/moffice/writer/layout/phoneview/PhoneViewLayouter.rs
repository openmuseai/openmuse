// Rust stub for PhoneViewLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/phoneview/PhoneViewLayouter.java
use crate::*;

pub struct PhoneViewLayouter {
    // TODO: fields
}

impl PhoneViewLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn createLayoutState(&self) -> LayoutState { todo!() }
    pub fn initLayouters(&self) { todo!() }
    pub fn isInTextFrame(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn flowTextFrame(&self, layoutState: LayoutState) { todo!() }
    pub fn initTextFrameLayouter(&self) { todo!() }
    pub fn initTableLayouter(&self) { todo!() }
    pub fn initDrawingLayouter(&self) { todo!() }
    pub fn initFootEndNoteLayouter(&self) { todo!() }
    pub fn getPageHeight(&self) -> i32 { todo!() }
    pub fn finishPage(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn finishPage_1(&self, textLine: i32, layoutState: LayoutState) { todo!() }
    pub fn clearTypoDocument(&self) { todo!() }
    pub fn clearEmbedCache(&self) { todo!() }
    pub fn onModifyFirst(&self, pageIndex: i32, recycleAfter: bool) { todo!() }
    pub fn clearInvalidatePages(&self, pageId: i32) { todo!() }
    pub fn flowPage(&self) -> TypoLayoutPage { todo!() }
    pub fn printDebugInfo(&self) { todo!() }
    pub fn postFlowPage(&self, rangeStartCP: i32, isFirstPage: bool) -> TypoLayoutPage { todo!() }
    pub fn flowRemainEmbedBalloon(&self, endLayoutPage: TypoLayoutPage, layoutState: PhoneViewLayoutState) { todo!() }
    pub fn adjustPage(&self, cachePage: i32, startCP: i32) { todo!() }
    pub fn getLastLineBottom(&self, page: TypoLayoutPage) -> i32 { todo!() }
    pub fn adjustEmbedBalloonLine(&self, page: TypoLayoutPage) { todo!() }
    pub fn mvLinesTo(&self, srcPage: i32, desPage: i32, maxCoreHeight: i32) -> i32 { todo!() }
    pub fn adjustFoldTable(&self, page: TypoLayoutPage, totalExtents: i32) -> i32 { todo!() }
    pub fn adjustLines(&self, page: TypoLayoutPage, totalExtents: i32) { todo!() }
    pub fn hasOnlyBreak(&self, result: TypoLayoutPage) -> bool { todo!() }
    pub fn onlyParagraphBreakInPage(&self, page: TypoLayoutPage) -> bool { todo!() }
    pub fn isTypoPagePosListEmpty(&self) -> bool { todo!() }
    pub fn flowCP(&self, cp: i32, endCp: i32, docType: i32) { todo!() }
    pub fn flowToPage(&self, pageIndex: i32) -> i32 { todo!() }
    pub fn flowToCP(&self, cp: i32, needPrepare: bool) { todo!() }
    pub fn prepareFlowFromDocumentBegin(&self) -> bool { todo!() }
    pub fn initGraphBuffForLoading(&self) { todo!() }
    pub fn prepareFlowInScrollDirection(&self) -> bool { todo!() }
    pub fn prepareFlowToBegin(&self) -> bool { todo!() }
    pub fn prepareFlowToDocumentEnd(&self) -> bool { todo!() }
    pub fn _prepareFlowToDocumentEnd(&self) -> bool { todo!() }
    pub fn prepareToJumpToLastView(&self) -> bool { todo!() }
    pub fn ensureValidEndPage(&self, rect: Rect) -> bool { todo!() }
    pub fn doAfterFlowLine(&self, layoutState: LayoutState) { todo!() }
    pub fn flowLineBalloonItem(&self, curLine: i32, layoutState: PhoneViewLayoutState) { todo!() }
    pub fn addBalloonToEmbedLine(&self, curLine: i32, balloonPage: i32) -> i32 { todo!() }
    pub fn isComplete(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn preFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn flowEmbedBalloonCache(&self, layoutState: PhoneViewLayoutState) { todo!() }
    pub fn postFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn onScroll(&self, scrollTop: i32, needNodifyListener: bool) { todo!() }
    pub fn prepareGraphs(&self, layoutState: LayoutState) { todo!() }
    pub fn processEvents(&self, layoutRect: Rect) { todo!() }
    pub fn setMaxCPExForPagePos(&self) { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn getType(&self) -> i32 { todo!() }
    pub fn prepareEmbeds(&self, wordAlingFont: i32, layoutState: LayoutState) { todo!() }
    pub fn locateEmbedCommentScroll(&self, embedCommentCp: i32) -> LocateResult { todo!() }
    pub fn calEmbedBalloonFontSize(&self, startCP: i32, mainDocument: Box<dyn Document>) { todo!() }
    pub fn calAverageFontSize(&self, document: Box<dyn Document>, start: i32, end: i32) -> i32 { todo!() }
    pub fn isParaBreak(&self, document: Box<dyn Document>, cp: i32) -> bool { todo!() }
    pub fn needReflowLine(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn getLastValidLineInPage(&self, line: i32, lineType: i32, snapshot: TypoSnapshot) -> i32 { todo!() }
}
