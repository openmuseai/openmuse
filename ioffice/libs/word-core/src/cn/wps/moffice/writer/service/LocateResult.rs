// Rust stub for LocateResult (auto-generated)
// Original: cn/wps/moffice/writer/service/LocateResult.java
use crate::*;

pub struct LocateResult {
    // TODO: fields
}

impl LocateResult {
    pub fn new() -> Self { todo!() }
    pub fn transToRender(&self, scale: f32, pageGrid: Box<dyn IPageGrid>) { todo!() }
    pub fn updateForPageGrid(&self, destGrid: Box<dyn IPageGrid>) { todo!() }
    pub fn clone(&self) -> LocateResult { todo!() }
    pub fn setRunRect(&self, runRect: Rect) { todo!() }
    pub fn setInGraphRect(&self, graphRect: Rect) { todo!() }
    pub fn setDrawRect(&self, drawRect: Rect) { todo!() }
    pub fn setLineRect(&self, lineRect: Rect) { todo!() }
    pub fn setCellRect(&self, cellRect: Rect) { todo!() }
    pub fn finishLocate(&self, pLayoutPage: i32, posVisitor: Box<dyn IPosVisitor>) { todo!() }
    pub fn getRunRect(&self) -> Rect { todo!() }
    pub fn getGraphRect(&self) -> Rect { todo!() }
    pub fn getDrawRect(&self) -> Rect { todo!() }
    pub fn getDrawRectF(&self) -> RectF { todo!() }
    pub fn getLineRect(&self) -> Rect { todo!() }
    pub fn getCellRect(&self) -> Rect { todo!() }
    pub fn getCellRectF(&self) -> RectF { todo!() }
    pub fn getLayoutPageRect(&self) -> Rect { todo!() }
    pub fn getX(&self) -> i32 { todo!() }
    pub fn getY(&self) -> i32 { todo!() }
    pub fn getBottom(&self) -> i32 { todo!() }
    pub fn getWidth(&self) -> i32 { todo!() }
    pub fn getHeight(&self) -> i32 { todo!() }
    pub fn getRunLeft(&self, top: bool) -> i32 { todo!() }
    pub fn getRunTop(&self, left: bool) -> i32 { todo!() }
    pub fn getRunRight(&self, top: bool) -> i32 { todo!() }
    pub fn getRunBottom(&self, left: bool) -> i32 { todo!() }
    pub fn getRunWidth(&self) -> i32 { todo!() }
    pub fn getRunHeight(&self) -> i32 { todo!() }
    pub fn getLineLeft(&self, top: bool) -> i32 { todo!() }
    pub fn getLineTop(&self, left: bool) -> i32 { todo!() }
    pub fn getLineRight(&self, top: bool) -> i32 { todo!() }
    pub fn getLineBottom(&self, left: bool) -> i32 { todo!() }
    pub fn getLineWidth(&self) -> i32 { todo!() }
    pub fn getLineHeight(&self) -> i32 { todo!() }
    pub fn getLayoutPage(&self) -> i32 { todo!() }
    pub fn isInCell(&self) -> bool { todo!() }
    pub fn merge(&self, merge: LocateResult) { todo!() }
    pub fn union(&self, srcRest: Rect, dstRect: Rect) { todo!() }
    pub fn getInRunRect(&self) -> Rect { todo!() }
    pub fn getInGraphRect(&self) -> Rect { todo!() }
    pub fn getInLineRect(&self) -> Rect { todo!() }
    pub fn getInLayoutPageRect(&self) -> Rect { todo!() }
    pub fn getPageGrid(&self) -> Box<dyn IPageGrid> { todo!() }
    pub fn getInDrawRect(&self) -> Rect { todo!() }
    pub fn getInCellRect(&self) -> Rect { todo!() }
    pub fn copy(&self) -> LocateResult { todo!() }
    pub fn toString_function toString() { [native code] }(&self) -> String { todo!() }
    pub fn expandBottom(&self, bottom: i32) { todo!() }
    pub fn expandTop(&self, top: i32) { todo!() }
    pub fn getLayoutRealX(&self) -> i32 { todo!() }
    pub fn hasLayoutRealX(&self) -> bool { todo!() }
    pub fn setLayoutRealX(&self, x: i32) { todo!() }
    pub fn setInColumns(&self) { todo!() }
    pub fn isInColumns(&self) -> bool { todo!() }
    pub fn setCellEndCP(&self, cellEndCP: i32) { todo!() }
    pub fn getCellEndCP(&self) -> i32 { todo!() }
    pub fn setCellLevel(&self, cellLevel: i32) { todo!() }
    pub fn getCellLevel(&self) -> i32 { todo!() }
    pub fn isVisible(&self) -> bool { todo!() }
    pub fn setRTL(&self, rtl: bool) { todo!() }
    pub fn isRTL(&self) -> bool { todo!() }
    pub fn getTextFlow(&self) -> i32 { todo!() }
    pub fn setTextDir(&self, textDir: i32) { todo!() }
    pub fn setLine(&self, line: i32) { todo!() }
    pub fn getLine(&self) -> i32 { todo!() }
    pub fn getTextLine(&self, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn getLineStartCP(&self, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn getLineEndCP(&self, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn inSameTypoPage(&self, locateResult: LocateResult, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn setGrpSel(&self, isInGrpSel: bool) { todo!() }
    pub fn isInGrpSel(&self) -> bool { todo!() }
    pub fn setInnerExternal(&self, external: f32) { todo!() }
    pub fn getExternal(&self) -> f32 { todo!() }
}
