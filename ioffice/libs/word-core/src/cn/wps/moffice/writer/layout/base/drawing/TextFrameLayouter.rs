// Rust stub for TextFrameLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/drawing/TextFrameLayouter.java
use crate::*;

pub struct TextFrameLayouter {
    // TODO: fields
}

impl TextFrameLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn onStart(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn onEnd(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getDrawing(&self) -> i32 { todo!() }
    pub fn drawingHasInPage(&self) -> bool { todo!() }
    pub fn isDropCap(&self) -> bool { todo!() }
    pub fn getFloatContainerPage(&self, stateEnvoy: IStateEnvoy) -> TypoPage { todo!() }
    pub fn isEmptyTextFrame(&self, drawing: i32) -> bool { todo!() }
    pub fn afterFlow(&self, frame: TypoTextFrame) { todo!() }
    pub fn flow(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getDrawing_1(&self, document: Box<dyn Document>, range: i64, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn doGetDrawing(&self, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn updateRelPageOrRelMarginDrawings(&self, textBox: i32, textframeOffsetX: i32, textframeOffsetY: i32, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getTextboxLayouterState(&self, stateEnvoy: IStateEnvoy) -> TextboxLayoutState { todo!() }
    pub fn createTextboxLayouter(&self, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn flowTextBox(&self, drawing: TypoDrawing, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn getTextFlow(&self, frameTextFlow: FrameTextFlow) -> i32 { todo!() }
    pub fn initPageEnv(&self, textBoxPageEnv: PageEnv, oldTxtBoxRt: Rect, stateEnvoy: IStateEnvoy) { todo!() }
    pub fn correctAutoHeight(&self, typoTextbox: TypoTextBox) { todo!() }
    pub fn correctMinHeight(&self, typoTextbox: TypoTextBox, minHeight: i32) { todo!() }
    pub fn correctAutoWidth(&self, typoTextbox: TypoTextBox, stateEnvoy: IStateEnvoy, isDropCap: bool) { todo!() }
    pub fn getLineWidth(&self, brc: BRC) -> f32 { todo!() }
    pub fn getFrameRect(&self, property: PropertySet, textBoxPageEnv: PageEnv, stateEnvoy: IStateEnvoy) -> Rect { todo!() }
    pub fn frameIsAutoWidth(width: i32) -> bool { todo!() }
    pub fn frameIsAutoHeight(height: WHeightAbs) -> bool { todo!() }
    pub fn getFramePos(&self, property: PropertySet) -> Box<dyn IPos> { todo!() }
    pub fn findFrameEnd(&self, stateEnvoy: IStateEnvoy) -> i32 { todo!() }
    pub fn isInSameFrame(&self, prop1: PropertySet, prop2: PropertySet) -> bool { todo!() }
    pub fn equals(prop1: PropertySet, prop2: PropertySet, pair: Pair) -> bool { todo!() }
    pub fn cp2fc(&self, document: Box<dyn Document>, cp: i32) -> i32 { todo!() }
}

pub trait IStateEnvoy {}
