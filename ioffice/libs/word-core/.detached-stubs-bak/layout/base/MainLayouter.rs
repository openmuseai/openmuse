// Rust stub for MainLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/MainLayouter.java
use crate::*;

pub struct MainLayouter {
    // TODO: fields
}

impl MainLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn flowPage(&self) -> TypoLayoutPage { todo!() }
    pub fn waitRangeEnd(&self, maxCp: i32) -> i32 { todo!() }
    pub fn acceptFirstCache(&self, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn flowCP(&self, var1: i32, var2: i32, var3: i32) { todo!() }
    pub fn flowToPage(&self, var1: i32) -> i32 { todo!() }
    pub fn flowDocument(&self, maxPageCount: i32) { todo!() }
    pub fn flowDocument_1(&self, maxPageCount: i32, interrupt: Box<dyn IInterruptState>) { todo!() }
    pub fn collectDrawingsForTrace(snapshot: TypoSnapshot) { todo!() }
    pub fn prepareFlowFromDocumentBegin(&self) -> bool { todo!() }
    pub fn prepareFlowInScrollDirection(&self) -> bool { todo!() }
    pub fn prepareFlowToDocumentEnd(&self) -> bool { todo!() }
    pub fn prepareToJumpToLastView(&self) -> bool { todo!() }
    pub fn changePageSize(&self) { todo!() }
    pub fn ensureValidEndPage(&self, var1: Rect) -> bool { todo!() }
    pub fn hasFinishIoLoading(&self) -> bool { todo!() }
    pub fn getCurPageTop(&self) -> i32 { todo!() }
    pub fn getPageHeight(&self) -> i32 { todo!() }
    pub fn swapToLocal(&self, sync: bool) { todo!() }
    pub fn swapFromLocal(&self) -> bool { todo!() }
    pub fn clearRevisionCache(&self) { todo!() }
    pub fn onScroll(&self, scrollTop: i32) { todo!() }
    pub fn onScroll_1(&self, scrollTop: i32, needNodifyListener: bool) { todo!() }
    pub fn clearTypoDocument(&self) { todo!() }
    pub fn clearForStructure(&self) { todo!() }
    pub fn postFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn isInTable(paraProp: PropertySet) -> bool { todo!() }
    pub fn processEvents(&self, layoutRect: Rect) { todo!() }
    pub fn getBeginFlowCP(&self, para: IPapxTable::Entry) -> i32 { todo!() }
    pub fn getBeginFlowCP_1(&self, para: IPapxTable::Entry, flowCp: i32) -> i32 { todo!() }
    pub fn allowCutFlow(&self, para: IPapxTable::Entry, flowCp: i32) -> bool { todo!() }
    pub fn hasAntherChar(&self, ch: u16) -> bool { todo!() }
    pub fn needSkip(&self, para: IPapxTable::Entry, paraStart: i32) -> bool { todo!() }
    pub fn isRevisionDelPara(&self, prepParaEndCP: i32) -> bool { todo!() }
    pub fn onNewRangeEnd(&self, lastRangeEnd: i32, rangeEnd: i32) { todo!() }
    pub fn calcMaxValidRangeEnd(&self, curCP: i32) -> i32 { todo!() }
    pub fn waitToCp(&self, waitCp: i32) -> i32 { todo!() }
    pub fn initGraphBuffForLoading(&self) { todo!() }
    pub fn rollbackLines(&self, layoutState: LayoutState) { todo!() }
    pub fn reflowComments(&self, hasAudioComment: bool) -> bool { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn resetForInit(&self) { todo!() }
    pub fn getType(&self) -> i32 { todo!() }
    pub fn doOnIoLoadFinish(&self) { todo!() }
    pub fn onIoLoadFinish(&self, hasAudioComment: bool) { todo!() }
    pub fn hasFindIOError(&self) -> bool { todo!() }
    pub fn onAfterFirstRender(&self) { todo!() }
    pub fn isFirstLineTableHeader(&self, line: i32) -> bool { todo!() }
    pub fn isNullOrFirstLineTableHeader(&self, line: i32) -> bool { todo!() }
    pub fn getLastValidLineInPage(&self, line: i32, lineType: i32, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn getRedundantDrawingLimitCP(&self, lastVisibleLine: i32, drawing: i32, anchorCP: i32, layoutState: LayoutState) -> i32 { todo!() }
    pub fn isRedundantDrawing(&self, lastVisibleLine: i32, drawing: i32, anchorCP: i32) -> bool { todo!() }
    pub fn shouldInvalidateRedundantWrapDrawing(&self, lastVisibleLine: i32, drawing: i32, anchorCP: i32, limitCP: i32, layoutState: LayoutState) -> bool { todo!() }
    pub fn hasVMergeCells(&self, rowInfo: RowInfo) -> bool { todo!() }
    pub fn getLayoutNotInCellDrawingLimitCP(&self, wrapDrawingCP: i32) -> i32 { todo!() }
    pub fn onModified(&self, changeRect: Rect) { todo!() }
    pub fn getDocEnv(&self) -> DocumentEnv { todo!() }
    pub fn getViewEnv(&self) -> ViewEnv { todo!() }
    pub fn isComplete(&self) -> bool { todo!() }
    pub fn onSetTextDocument(&self) { todo!() }
    pub fn clearInvalidatePages(&self, pageId: i32) { todo!() }
    pub fn invalidCardState(&self) { todo!() }
    pub fn makeBalloonCaches(&self) { todo!() }
    pub fn commitMemTag(pointer: i32, layoutPage: i32, snapshot: TypoSnapshot) { todo!() }
    pub fn loadStartStateFromLastVersion(&self, stateSnapshot: StateSnapshot, pLayoutPage: i32, pools: Pools) { todo!() }
    pub fn locateEmbedCommentScroll(&self, embedCommentCp: i32) -> LocateResult { todo!() }
    pub fn initGraphBuff(&self, layoutState: LayoutState) { todo!() }
    pub fn initGraphBuff_1(&self, layoutState: LayoutState, endCp: i32) { todo!() }
    pub fn getFitInfo(&self) -> PhoneViewFitInfo { todo!() }
    pub fn getLineBreakAnalyzer(&self) -> LineBreakAnalyzer { todo!() }
    pub fn isInAnalyzer(&self) -> bool { todo!() }
    pub fn onPage(&self, index: i32, pPage: i32, snapshot: TypoSnapshot) -> bool { todo!() }
}
