// Rust stub for DrawingPositionTool (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/drawing/DrawingPositionTool.java
use crate::*;

pub struct DrawingPositionTool {
    // TODO: fields
}

impl DrawingPositionTool {
    pub fn new() -> Self { todo!() }
    pub fn calcNormalizedPageCoreOffset(page: TypoPage, rect: Box<dyn ITypoRect>) { todo!() }
    pub fn getColPageDrawingOffsetResult(curPage: TypoPage, destPage: TypoPage) -> i64 { todo!() }
    pub fn getColPageDrawingOffsetX(offsetResult: i64) -> i32 { todo!() }
    pub fn getColPageDrawingOffsetY(offsetResult: i64) -> i32 { todo!() }
    pub fn isWrap(shapePos: Box<dyn IPos>) -> bool { todo!() }
    pub fn isWrapTableOrTextFrame(drawing: i32, snapshot: TypoSnapshot) -> bool { todo!() }
    pub fn isHoriPosOfLineContainerPage(relX: i32) -> bool { todo!() }
    pub fn isVertPosOfLineContainerPage(relY: i32) -> bool { todo!() }
    pub fn isHoriPosOfLineContainerPage_1(pos: Box<dyn IPos>) -> bool { todo!() }
    pub fn isVertPosOfLineContainerPage_1(pos: Box<dyn IPos>) -> bool { todo!() }
    pub fn normalizeRelX(pos: Box<dyn IPos>, floatsPage: TypoPage, globalPageIdx: i32) -> i32 { todo!() }
    pub fn normalizeRelY(pos: Box<dyn IPos>, floatsPage: TypoPage, globalPageIdx: i32, isBefore2013Mode: bool) -> i32 { todo!() }
    pub fn shouldAdjustAlignRectWithExcessHeaderFooter(relY: i32, isWrapTblOrTextframe: bool, alignPage: TypoPage, floatsPage: TypoPage) -> bool { todo!() }
    pub fn getRelInOutSidePageVertOffset(pos: Box<dyn IPos>, globalPageIdx: i32, floatsPage: TypoPage) -> i32 { todo!() }
    pub fn offsetRelInOutSidePageDrawingInLayoutPage(drawing: TypoDrawing, globalPageIdx: i32, floatsPage: TypoPage) { todo!() }
    pub fn normalizeAlignH(alignH: i32, globalPageIdx: i32) -> i32 { todo!() }
    pub fn normalizeAlignV(relY: i32, alignV: i32, globalPageIdx: i32) -> i32 { todo!() }
    pub fn getDrawingLeftOfPageCore(alignH: i32, globalPageIdx: i32, shapeRect: Box<dyn ITypoRect>, alignRect: Box<dyn ITypoRect>) -> i32 { todo!() }
    pub fn getDrawingTopOfPageCore(relY: i32, alignV: i32, globalPageIdx: i32, shapeRect: Box<dyn ITypoRect>, alignRect: Box<dyn ITypoRect>) -> i32 { todo!() }
    pub fn offsetRotateDrawing(left: i32, top: i32, drawing: TypoDrawing) { todo!() }
    pub fn shouldCorrectDrawing(drawing: TypoDrawing) -> bool { todo!() }
    pub fn offsetTypoRectX(rect: Box<dyn ITypoRect>, deltaX: i32) { todo!() }
    pub fn offsetTypoRectY(rect: Box<dyn ITypoRect>, deltaY: i32) { todo!() }
    pub fn correctWrapDrawing(rect: Box<dyn ITypoRect>, limitRect: Box<dyn ITypoRect>, needCorrectHori: bool, needCorrectVert: bool) -> bool { todo!() }
    pub fn shouldOffsetDrawingUsingRenderRect(drawing: TypoDrawing, alignH: i32, alignV: i32) -> bool { todo!() }
    pub fn shouldReverseShapeRectLeft(drawing: TypoDrawing, relX: i32, floatsPage: TypoPage) -> bool { todo!() }
    pub fn shouldOffsetDrawingUsingRenderRect_1(drawing: TypoDrawing) -> bool { todo!() }
    pub fn updateRelParaQuasiFloatsVertPos(drawings: WIntArrayList, mainState: MainState) { todo!() }
}
