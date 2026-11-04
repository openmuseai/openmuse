// Rust stub for FitTextHandler (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/line/align/FitTextHandler.java
use crate::*;

pub struct FitTextHandler {
    // TODO: fields
}

impl FitTextHandler {
    pub fn new() -> Self { todo!() }
    pub fn reuseClean(&self) -> bool { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn setSubLineEnv(&self, subLineEnv: SubLineEnv, layoutState: LayoutState) { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn calcExtends(&self) { todo!() }
    pub fn calcTotal(&self) { todo!() }
    pub fn adjustToSameStyle(&self) { todo!() }
    pub fn isWidthInFirstPos(&self, run: Graphs) -> bool { todo!() }
    pub fn isInitFontSpaceZero(&self, run: Graphs) -> bool { todo!() }
    pub fn addTotal(&self, run: Graphs, isLast: bool) { todo!() }
    pub fn calcGraphTotalSpace(&self, run: Graphs, isLast: bool) -> f32 { todo!() }
    pub fn calcFontSapce(&self, run: Graphs, checkInitZero: bool) -> i32 { todo!() }
    pub fn fitTextPrepare(&self, extendWidth: f32) { todo!() }
    pub fn setSpaceScale(&self, extendWidth: f32, adjWidth: f32, spaceTotal: f32) { todo!() }
    pub fn setExtendSpace(&self, extendWidth: f32) { todo!() }
    pub fn setCharScale(&self, extraScale: f32, pureWidth: f32) -> f32 { todo!() }
    pub fn onAddRun(&self, run: Graphs) { todo!() }
    pub fn onAddBidiRun(&self, run: Graphs, isRTL: bool) { todo!() }
    pub fn calcLineLeftAndRight(&self, line: Box<dyn ITypoRect>) -> bool { todo!() }
    pub fn adjustRun(&self, run: Graphs) { todo!() }
    pub fn extendOrCompress(&self, run: Graphs, isLastRun: bool) { todo!() }
    pub fn changeFontStyle(&self, fontSpace: i32, charScale: f32, run: Graphs) { todo!() }
    pub fn searchFontStyleInCache(&self, oriFontStyle: TypoFontStyle, fontSpace: i32, charScale: f32) -> TypoFontStyle { todo!() }
    pub fn adjustLine(&self, line: Box<dyn ITypoRect>) { todo!() }
    pub fn adjustLineForFitText(&self, line: Box<dyn ITypoRect>) { todo!() }
    pub fn adjustBidiLine(&self, line: Box<dyn ITypoRect>) { todo!() }
    pub fn adjustBidiLineForFitText(&self, line: Box<dyn ITypoRect>) { todo!() }
    pub fn zeroWidth(&self, run: Graphs) { todo!() }
    pub fn adjustLineForLeftAlign(&self, line: Box<dyn ITypoRect>, isBidi: bool) { todo!() }
    pub fn adjustLineForRightAlign(&self, line: Box<dyn ITypoRect>, isBidi: bool) { todo!() }
    pub fn adjustLineForCenter(&self, line: Box<dyn ITypoRect>, isBidi: bool) { todo!() }
    pub fn getAlignment(&self) -> i32 { todo!() }
}
