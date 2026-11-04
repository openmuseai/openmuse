// Rust stub for GraphBuff (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/graph/GraphBuff.java
use crate::*;

pub struct GraphBuff {
    // TODO: fields
}

impl GraphBuff {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn setMaxGraphSize(&self, var1: i32) { todo!() }
    pub fn clear(&self, layoutState: LayoutState) { todo!() }
    pub fn setRecommendCPRef(&self, cpRef: i32) { todo!() }
    pub fn getRecommendCPRef(&self) -> i32 { todo!() }
    pub fn doClear(&self, needClearDrawings: bool, layoutState: LayoutState) { todo!() }
    pub fn doCleanGraphReader(&self) { todo!() }
    pub fn prepareGraphs(&self, wordAlignFont: i32, layoutState: LayoutState) -> bool { todo!() }
    pub fn doPrepareGraphs(&self, var1: i32, var2: LayoutState) { todo!() }
    pub fn setCalcGraphHeight(&self, var1: bool) { todo!() }
    pub fn setIsInMath(&self, var1: bool, var2: LayoutState) { todo!() }
    pub fn addBidi(&self, graphIndex: i32, ch: u16, layoutState: LayoutState) { todo!() }
    pub fn setIgnoreHpsAndVAlignRun(&self, var1: bool) { todo!() }
    pub fn setIgnoreCapsAndSmallCaps(&self, var1: bool) { todo!() }
    pub fn resetForInit(&self) { todo!() }
    pub fn ensureGraphsForEmbed(&self) { todo!() }
    pub fn getArgsForEmbedSize(&self) -> i32 { todo!() }
    pub fn getEmbedSize(&self) -> i32 { todo!() }
    pub fn getGraphsForEmbed(&self, index: i32) -> Graphs { todo!() }
    pub fn getArgsForEmbed(&self, index: i32) -> Box<dyn std::any::Any + Send + Sync> { todo!() }
    pub fn removeEmbedArgsAt(&self, curEmbedIndex: i32) { todo!() }
    pub fn getCP(&self, graphIndex: i32) -> i32 { todo!() }
    pub fn addFunctionSepField(&self, var1: i32) { todo!() }
    pub fn setIgnoreLeading(&self, var1: bool) { todo!() }
    pub fn setMathContent(&self, var1: bool) { todo!() }
    pub fn initFilter(&self, var1: Box<dyn Document>, var2: i32, var3: i32, var4: LayoutState) { todo!() }
}
