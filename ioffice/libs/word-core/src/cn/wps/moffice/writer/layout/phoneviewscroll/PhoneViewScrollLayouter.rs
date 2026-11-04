// Rust stub for PhoneViewScrollLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/phoneviewscroll/PhoneViewScrollLayouter.java
use crate::*;

pub struct PhoneViewScrollLayouter {
    // TODO: fields
}

impl PhoneViewScrollLayouter {
    pub fn new() -> Self { todo!() }
    pub fn initLayouters(&self) { todo!() }
    pub fn isInTextFrame(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn flowTextFrame(&self, layoutState: LayoutState) { todo!() }
    pub fn initTextFrameLayouter(&self) { todo!() }
    pub fn needReflowLine(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn getLastValidLineInPage(&self, line: i32, lineType: i32, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn initTableLayouter(&self) { todo!() }
    pub fn initDrawingLayouter(&self) { todo!() }
    pub fn initFootEndNoteLayouter(&self) { todo!() }
    pub fn finishPage(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn prepareGraphs(&self, layoutState: LayoutState) { todo!() }
    pub fn getType(&self) -> i32 { todo!() }
    pub fn prepareEmbeds(&self, wordAlingFont: i32, layoutState: LayoutState) { todo!() }
    pub fn locateEmbedCommentScroll(&self, embedCommentCp: i32) -> LocateResult { todo!() }
    pub fn createLayoutState(&self) -> LayoutState { todo!() }
    pub fn doFlowLine(&self, layoutState: LayoutState) { todo!() }
    pub fn doBeforeFlowLine(&self, layoutState: LayoutState) { todo!() }
    pub fn doAfterFlowLine(&self, layoutState: LayoutState) { todo!() }
    pub fn postFlow(&self, layoutState: LayoutState) { todo!() }
    pub fn getBeginFlowCP(&self, para: IPapxTable::Entry, flowCp: i32) -> i32 { todo!() }
    pub fn collectCardList(&self, state: PhoneViewScrollState) { todo!() }
    pub fn onIoLoadFinish(&self, hasAudioComment: bool) { todo!() }
    pub fn initCardPaper(&self, state: LayoutState) { todo!() }
}
