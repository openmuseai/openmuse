// Rust stub for FilterForSpaces (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/line/charSpace/FilterForSpaces.java
use crate::*;

pub struct FilterForSpaces {
    // TODO: fields
}

impl FilterForSpaces {
    pub fn new() -> Self { todo!() }
    pub fn getTransTabIndexs(&self) -> WIntArrayList { todo!() }
    pub fn getAlignDotIndex(&self) -> i32 { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn handleForTrans(&self, cur: Graphs) -> bool { todo!() }
    pub fn setCompactWidth(&self, graphs: Graphs, fontSpace: i32) { todo!() }
    pub fn isLetterChar(&self, graphs: Graphs, firstOrLast: bool) -> bool { todo!() }
    pub fn isLetterGraphs(&self, graphs: Graphs) -> bool { todo!() }
    pub fn isBetweenLetters(&self, prev: Graphs, cur: Graphs, next: Graphs) -> bool { todo!() }
    pub fn canTansTabToSpace(&self) -> bool { todo!() }
    pub fn isTOCFieldPara(&self, document: Box<dyn Document>, paraStart: i32) -> bool { todo!() }
}
