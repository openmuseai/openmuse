// Rust stub for DocFieldCollector (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/line/docfield/DocFieldCollector.java
use crate::*;

pub struct DocFieldCollector {
    // TODO: fields
}

impl DocFieldCollector {
    pub fn new() -> Self { todo!() }
    pub fn log(s: String) { todo!() }
    pub fn init(&self, textLine: TypoTextLine, layoutState: LayoutState) { todo!() }
    pub fn addRun(&self, run: Graphs, runLeft: i32, runRight: i32, revisionAuthor: i32, hyperlinkId: i32) { todo!() }
    pub fn doAddRun(&self, run: Graphs, runLeft: i32, runRight: i32, revisionAuthor: i32, hyperlinkId: i32) { todo!() }
    pub fn processTrailSpace(&self) { todo!() }
    pub fn commit(&self) { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn reuseClean(&self) -> bool { todo!() }
    pub fn collectForNormalGraphs(&self, graphs: Graphs, isRTL: bool, result: Box<dyn IMergeCollectResult>, startCP: i32, endCp: i32) -> bool { todo!() }
    pub fn collectDocFieldInGraph(&self, graphs: Graphs, isRTL: bool) { todo!() }
    pub fn collectForLeaderHiddend(&self, graphs: Graphs, result: Box<dyn IMergeCollectResult>, starCp: i32) { todo!() }
    pub fn collectDdfInGap(&self, graphs: Graphs, result: Box<dyn IMergeCollectResult>, collectStart: i32, collectEnd: i32) { todo!() }
    pub fn addFlagToLastGraph(&self, graphs: Graphs, it: IMergeCollectResult::IIterator) -> bool { todo!() }
    pub fn parseOne(&self, flags: DfdFlagsInGraphs, it: IMergeCollectResult::IIterator, graphs: Graphs, isRTL: bool, startCP: i32, endCp: i32) { todo!() }
}

pub struct PLCFieldIterator { /* TODO */ }
