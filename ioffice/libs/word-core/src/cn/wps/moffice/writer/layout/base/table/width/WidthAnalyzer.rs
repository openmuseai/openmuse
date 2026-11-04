// Rust stub for WidthAnalyzer (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/table/width/WidthAnalyzer.java
use crate::*;

pub struct WidthAnalyzer {
    // TODO: fields
}

impl WidthAnalyzer {
    pub fn new() -> Self { todo!() }
    pub fn reuseClean(&self) -> bool { todo!() }
    pub fn reuseInit(&self) { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn getKerningHandler(&self) { todo!() }
    pub fn getPunctSpaceHandler(&self) { todo!() }
    pub fn getFastGraph(&self) -> Box<dyn IFastGraph> { todo!() }
    pub fn setFastGraph(&self, fastGraph: Box<dyn IFastGraph>) { todo!() }
    pub fn getWidthReceiver(&self) -> Box<dyn IWidthReceiver> { todo!() }
    pub fn setWidthReceiver(&self, autoInfo: Box<dyn IWidthReceiver>) { todo!() }
    pub fn doAnalyze(&self, state: LayoutState) { todo!() }
    pub fn checkLongestForTablHead(&self) { todo!() }
    pub fn getGraphBdr(&self, run: Graphs) -> i32 { todo!() }
    pub fn addRun(&self, run: Graphs) -> bool { todo!() }
    pub fn calCharSpace(&self, prevRun: Graphs, run: Graphs) -> i32 { todo!() }
    pub fn calcAutoSpace(&self, prevRun: Graphs, run: Graphs) -> i32 { todo!() }
    pub fn calcTablWidth(&self, run: Graphs) -> i32 { todo!() }
    pub fn addPunt(&self, run: Graphs) { todo!() }
    pub fn isPreNoramlText(&self) -> bool { todo!() }
    pub fn calctLayoutIndent(&self, firstRun: Graphs) { todo!() }
    pub fn isPartOfWord(&self, graphs: Graphs) -> bool { todo!() }
    pub fn isPartOfComplexGraphs(&self, graphs: Graphs) -> bool { todo!() }
    pub fn getChar(&self, graphs: Graphs) -> u16 { todo!() }
    pub fn toGraphsString(layoutState: LayoutState, graphIndex: i32, len: i32) -> String { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn setParaRange(&self, range: i64) { todo!() }
    pub fn addAutoNum(&self, w: i32, endIndex: i32) { todo!() }
    pub fn addAutoNumForBdr(&self, w: i32) { todo!() }
    pub fn setLeftRightIndent(&self, left: i32, right: i32) { todo!() }
    pub fn commit(&self, runToCheckMaxChar: Graphs) { todo!() }
    pub fn addTailSpace(&self, spaceWidth: i32) { todo!() }
    pub fn takeTailSpace(&self) -> i32 { todo!() }
    pub fn addAutoWidth(&self, width: i32) { todo!() }
    pub fn commit_1(&self) { todo!() }
    pub fn commitEmbed(&self, graphs: Graphs) { todo!() }
    pub fn isAdding(&self) -> bool { todo!() }
    pub fn getLastAddState(&self) -> LastAddState { todo!() }
    pub fn addPart(&self, graphs: Graphs, kerningAdd: i32) { todo!() }
    pub fn addEndPunt(&self, graphs: Graphs, startIndex: i32, endIndex: i32, widtAdjust: i32) { todo!() }
    pub fn addHeadPunt(&self, graphs: Graphs, widtAdjust: i32) { todo!() }
    pub fn getWordRange(&self) -> i64 { todo!() }
    pub fn getParaRange(&self) -> i64 { todo!() }
    pub fn isContainsAutoNum(&self) -> bool { todo!() }
    pub fn getWordText(&self) -> String { todo!() }
    pub fn isFirtLineForMaxWord(&self) -> bool { todo!() }
    pub fn getMinWidth(&self) -> i32 { todo!() }
    pub fn getMaxWidth(&self) -> i32 { todo!() }
    pub fn getAutoWidthTotal(&self) -> i32 { todo!() }
    pub fn toString_function toString() { [native code] }(&self) -> String { todo!() }
}
