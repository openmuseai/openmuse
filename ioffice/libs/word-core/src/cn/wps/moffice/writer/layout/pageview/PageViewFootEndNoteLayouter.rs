// Rust stub for PageViewFootEndNoteLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/pageview/PageViewFootEndNoteLayouter.java
use crate::*;

pub struct PageViewFootEndNoteLayouter {
    // TODO: fields
}

impl PageViewFootEndNoteLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn flow(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn flowForTextLine(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn getFootNotes(&self, layoutState: LayoutState) -> TypoFootNotes { todo!() }
    pub fn setFootNotes(&self, pointer: i32, layoutState: LayoutState) { todo!() }
    pub fn collectRanges(&self, layoutState: LayoutState, lastLine: i32) { todo!() }
    pub fn containCP(&self, cp: i32) -> bool { todo!() }
    pub fn flowWithSortedFootnotes(&self, layoutState: LayoutState, lastLine: i32) -> bool { todo!() }
    pub fn breakCache(&self, curItem: FootNoteCacheItem, footEndNoteCache: FootNoteCache, layoutState: LayoutState) { todo!() }
    pub fn getAdjustWrapTableRowBottom(&self, lastLine: i32, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn setWrapTableNoKeepCacheWhenRollBack(&self, lastLine: i32, snapshot: TypoSnapshot) { todo!() }
    pub fn applyLastLineForAffectRect(&self, lastLine: i32, layoutState: LayoutState, snapshot: TypoSnapshot) { todo!() }
    pub fn isLayoutPageOrColumnPage(&self, obj: i32, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn quickFindTextlineFromRooPage(&self, layoutState: LayoutState, document: Box<dyn Document>, cp: i32, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn getMinTop(&self, layoutState: LayoutState) -> i32 { todo!() }
    pub fn getFirstBottomOrderByYOfAnchor(&self, layoutState: LayoutState) -> i32 { todo!() }
    pub fn checkIsLastLineToParentRoot(&self, pLine: i32, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn getAnchorAndCalcBottomForCurPageCore(&self, firstVaildItem: FootNoteCacheItem, layoutState: LayoutState) -> i32 { todo!() }
    pub fn bottomForThisPageCore(&self, anchorLine: i32, layoutState: LayoutState) -> i32 { todo!() }
    pub fn breakFootnotes(&self, layoutState: LayoutState) { todo!() }
    pub fn breakFootNote(&self, footNote: TypoFootEndNote, maxHeight: i32, footEndNoteEnv: FootEndNoteEnv, layoutState: LayoutState) { todo!() }
    pub fn removeCacheByFootEndNotePointer(&self, pointer: i32) { todo!() }
    pub fn rollbackFootnotes(&self, layoutState: LayoutState) { todo!() }
    pub fn isInFixHeightRowOrCantBreakRow(&self, lastLine: i32) -> bool { todo!() }
    pub fn isFirstLine(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn recycleFootNotesPage(&self, layoutState: LayoutState) { todo!() }
    pub fn needFinishPageForFootNotes(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn flowBreakFootNotes(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn flowEndNotes(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn isVanish(&self, refCP: i32) -> bool { todo!() }
    pub fn getFootEndNotePageXPos(&self, layoutState: LayoutState) -> i32 { todo!() }
    pub fn addFootNote(&self, footNotes: TypoFootNotes, footEndNote: TypoFootEndNote, adjustPageHeigh: bool, layoutState: LayoutState) { todo!() }
    pub fn addEndNote(&self, page: TypoPage, footEndNote: TypoFootEndNote, layoutState: LayoutState) { todo!() }
    pub fn getFootNoteTxtRange(&self, refNote: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode) -> Long { todo!() }
    pub fn prepareFlowFootNoteEnv(&self, refNote: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode, footNoteLayoutEnv: LayoutState) -> bool { todo!() }
    pub fn flowFootNote(&self, cpRef: i32, refNote: PLCFootnoteEndnoteRef::FootnoteEndnoteRefNode, atLeastOneLine: bool, footNotes: TypoFootNotes, footNoteLayoutEnv: LayoutState, pageEnv: PageEnv) -> TypoFootEndNote { todo!() }
    pub fn flowFootNote_1(&self, cpRef: i32, atLeastOneLine: bool, footNotes: TypoFootNotes, footNoteLayoutEnv: LayoutState, pageEnv: PageEnv) -> TypoFootEndNote { todo!() }
    pub fn flowTextBox(&self, footNoteLayoutEnv: LayoutState, maxHeight: i32, atLeastOneLine: bool, pageEnv: PageEnv) { todo!() }
    pub fn getDocumentName(&self, doc: Box<dyn Document>) -> String { todo!() }
    pub fn initFootNotesPage(&self, minTop: i32, forceInit: bool, layoutState: LayoutState, isBreak: bool) -> bool { todo!() }
    pub fn createSepFootEndNote(&self, docType: i32, pageEnv: PageEnv, isBreak: bool) -> TypoFootEndNote { todo!() }
    pub fn adjustFootNotesPage(&self, layoutState: LayoutState) { todo!() }
    pub fn adjustMaxLayoutHeight(&self, layoutState: LayoutState) { todo!() }
    pub fn initFlowEndNote(&self, footEndNoteEnv: FootEndNoteEnv) { todo!() }
    pub fn initFlowFootNote(&self, footEndNoteEnv: FootEndNoteEnv) { todo!() }
    pub fn initFootEndNoteTextLayouter(&self) { todo!() }
    pub fn flowFootNoteBeneathText(&self, layoutState: LayoutState) -> bool { todo!() }
    pub fn getEndNoteTxtRange(&self, footnoteEndnoteTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode) -> Long { todo!() }
    pub fn flowEndNote(&self, cpRef: i32, footnoteEndnoteTxtNode: PLCFootnoteEndnoteTxt::FootnoteEndnoteTxtNode, atLeastOneLine: bool, page: TypoPage, endNoteLayoutState: LayoutState, mainLayoutState: LayoutState) -> TypoFootEndNote { todo!() }
    pub fn flowEndNote_1(&self, cpRef: i32, atLeastOneLine: bool, page: TypoPage, endNoteLayoutState: LayoutState, mainLayoutState: LayoutState) -> TypoFootEndNote { todo!() }
    pub fn initEnv(&self, footEndNoteEnv: FootEndNoteEnv) { todo!() }
}
