// Rust stub for DrawingLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/drawing/DrawingLayouter.java
use crate::*;

pub struct DrawingLayouter {
    // TODO: fields
}

impl DrawingLayouter {
    pub fn new() -> Self { todo!() }
    pub fn onFlowWrapDrawing(&self, stateEnvoy: IStateEnvoy, anchorCP: i32) { todo!() }
    pub fn adjustTextFrame(&self, drawing: TypoDrawing, paraTop: i32, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn adjustDropCap(&self, drawing: TypoDrawing, stateEnvoy: IStateEnvoy, shapeRect: RectF, alignRect: Box<dyn ITypoRect>, relX: i32, relY: i32, alignX: i32, alignY: i32) { todo!() }
    pub fn adjustDropCapHorizontal(&self, drawing: TypoDrawing, stateEnvoy: IStateEnvoy, shapeRect: RectF, alignRect: Box<dyn ITypoRect>, relX: i32, relY: i32, alignX: i32, alignY: i32) { todo!() }
    pub fn adjustDropCapVertical(&self, drawing: TypoDrawing, stateEnvoy: IStateEnvoy, shapeRect: RectF, alignRect: Box<dyn ITypoRect>, relX: i32, relY: i32, alignX: i32, alignY: i32) { todo!() }
    pub fn offsetMarginToEdge(&self, drawing: TypoDrawing, alignRect: Box<dyn ITypoRect>, maxOffset: i32, offsetOpposite: bool, isHorizontal: bool) { todo!() }
    pub fn adjustWrapTable(&self, drawing: TypoDrawing, tableInfo: TableInfo, paraTop: i32, needCutTable: bool, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getTablePos(&self, tableInfo: TableInfo) -> Box<dyn IPos> { todo!() }
    pub fn correctForCellMargin(&self, drawing: TypoDrawing, tableInfo: TableInfo, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn onStart(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn onEnd(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn correctIfWrapTableOutOfPage(&self, drawing: TypoDrawing, pos: Box<dyn IPos>, needCutTable: bool, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn correctIfWrapShapeOutOfPage(&self, drawing: TypoDrawing, pos: Box<dyn IPos>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn adjustFloatRect(&self, drawing: TypoDrawing, anchorLine: i32, paraLine: i32, relX: i32, relY: i32, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn doOffsetDrawing(&self, drawing: TypoDrawing, relX: i32, relY: i32, anchorLine: i32, shapeRect: Box<dyn ITypoRect>, alignRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcAffectRect(&self, drawing: TypoDrawing, anchorLine: i32, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn doCalcAffectRect(&self, drawing: TypoDrawing, anchorLine: i32, layoutRect: TypoRectCoreImpl, affectRect: TypoRectCoreImpl, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn getAlignRectRelColumnHoriOffset(&self, drawing: TypoDrawing, anchorLine: i32, paraLine: i32, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn calcLimitRect(&self, pos: Box<dyn IPos>, outRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcAlignRect(&self, drawing: TypoDrawing, outRect: Box<dyn ITypoRect>, anchorLine: i32, paraLine: i32, paraTop: i32, relX: i32, relY: i32, needCalcLeftAndRight: bool, needCalcTopAndBottom: bool, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn doCalcAlignRectLeftAndRight(&self, drawing: TypoDrawing, relX: i32, anchorLine: i32, paraLine: i32, paraTop: i32, outRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcAlignRectLeftAndRightWithPage(&self, drawing: TypoDrawing, anchorLine: i32, relX: i32, floatsPage: TypoPage, outRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcAlignRectTopAndBottomWithPage(&self, drawing: TypoDrawing, anchorLine: i32, relY: i32, floatsPage: TypoPage, outRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn doCalcAlignRectTopAndBottom(&self, drawing: TypoDrawing, relY: i32, anchorLine: i32, paraLine: i32, paraTop: i32, outRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn calcTextBoxFitRect(&self, outRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn flowChildShapes(&self, drawing: TypoDrawing, parentLeft: i32, parentTop: i32, dw: f32, dh: f32, parentRot: f32, rtParentCoord: RectF, parentRenderRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn createTextboxLayouter(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getTextBoxLayoutState(&self) -> TextboxLayoutState { todo!() }
    pub fn flowTextBox(&self, drawing: TypoDrawing, shape: Shape, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn vAlignTextBox(&self, typoTextbox: TypoTextBox, vTextAnchor: i32) { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn getAnchorHoriPos(&self, drawing: TypoDrawing, anchorLine: i32, needRelFloatsPage: bool, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn getEmbedContainerPage(&self, stateEnvoy: IStateEnvoy) -> TypoPage { todo!() }
    pub fn getFloatContainerPage(&self, stateEnvoy: IStateEnvoy) -> TypoPage { todo!() }
    pub fn getLineContainerPage(&self, drawing: TypoDrawing, anchorLine: i32, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn getHoriAlignPage(&self, drawing: TypoDrawing, anchorLine: i32, relX: i32, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn getVertAlignPage(&self, drawing: TypoDrawing, anchorLine: i32, relY: i32, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn addDrawings(&self, nodeList: KPList, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getEmbed(&self, shape: Shape, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn getFloatDrawing(&self, floatsPage: TypoPage, shape: Shape, cp: i32, stateEnvoy: IStateEnvoy) -> TypoDrawing { todo!() }
    pub fn doAddFloatCache(&self, floatsCache: KList, floatsPage: TypoPage, shape: Shape, anchorCP: i32, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn needFlow(&self, shape: Shape) -> bool { todo!() }
    pub fn changeShapePos(&self, cloneShape: Shape) { todo!() }
    pub fn doPreFlowDrawing(&self, drawing: TypoDrawing, shape: Shape, relX: i32, relY: i32, isEmbed: bool, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn doPreFlowEmbedDrawing(&self, drawing: TypoDrawing, shape: Shape, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn setDrawingShapeRect(&self, drawing: TypoDrawing, shape: Shape, relX: i32, relY: i32, needCalcSize: bool, isEmbed: bool, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn setDrawingRenderRect(&self, drawing: TypoDrawing, shape: Shape, outRect: Box<dyn ITypoRect>, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn setDrawingLayoutRect(&self, drawing: TypoDrawing, renderRect: Box<dyn ITypoRect>) { todo!() }
    pub fn addDrawingToPage(&self, drawing: TypoDrawing, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn filterFlowDrawing(&self, drawingItem: DrawingCacheItem, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn getFilterFloatCache(&self, row: i32, maxAnchorCP: i32, stateEnvoy: IStateEnvoy) -> KList { todo!() }
    pub fn doPostFlowDrawings(&self, row: i32, maxAnchorCP: i32, needAddToPage: bool, needFindAnchorLines: bool, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn flowBeforeCP(&self, maxAnchorCP: i32, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn flowForTableRow(&self, row: i32, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn doFlow(&self, row: i32, maxAnchorCP: i32, stateEnvoy: IStateEnvoy) -> bool { todo!() }
    pub fn getPageSetup(&self, stateEnvoy: IStateEnvoy) -> PageSetup { todo!() }
}

pub trait IStateEnvoy {}
