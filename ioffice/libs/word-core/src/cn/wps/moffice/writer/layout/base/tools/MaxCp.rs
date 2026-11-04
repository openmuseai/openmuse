// Rust stub for MaxCp (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/tools/MaxCp.java
use crate::*;

pub struct MaxCp {
    // TODO: fields
}

impl MaxCp {
    pub fn new() -> Self { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn waitIoCp(&self, cp: i32, env: LayoutState, textDocument: TextDocument, locksHelper: LocksHelper) -> i32 { todo!() }
    pub fn getMaxCP(doc: Box<dyn Document>, chpxTable: Box<dyn IChpxTable>, max: MaxCp, lastPara: IPapxTable::Entry) { todo!() }
    pub fn isValidParaEndForLayout(doc: Box<dyn Document>, curPara: IPapxTable::Entry, chpxTable: Box<dyn IChpxTable>, paraEndCp: i32, maxCp: MaxCp) -> bool { todo!() }
    pub fn isVanish(chpxTable: Box<dyn IChpxTable>, paraBreakCp: i32) -> bool { todo!() }
    pub fn isInTable(paraProp: PropertySet, doc: Box<dyn Document>, cp: i32, maxCp: MaxCp) -> bool { todo!() }
    pub fn isInTextFrame(paraProp: PropertySet) -> bool { todo!() }
}
