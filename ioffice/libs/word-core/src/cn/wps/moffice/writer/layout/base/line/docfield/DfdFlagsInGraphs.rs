// Rust stub for DfdFlagsInGraphs (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/line/docfield/DfdFlagsInGraphs.java
use crate::*;

pub struct DfdFlagsInGraphs {
    // TODO: fields
}

impl DfdFlagsInGraphs {
    pub fn new() -> Self { todo!() }
    pub fn newObject(&self) -> Iterator { todo!() }
    pub fn isAutoSize(&self) -> bool { todo!() }
    pub fn passivateObject(&self, obj: Iterator) { todo!() }
    pub fn add(&self, beingIndex: i32, endIndex: i32, pos: i32, width: i32, flag: i32) -> Item { todo!() }
    pub fn size(&self) -> i32 { todo!() }
    pub fn item(&self, index: i32) -> Item { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn last(&self) -> Item { todo!() }
    pub fn toString_function toString() { [native code] }(&self) -> String { todo!() }
    pub fn createDfdFlagsIterator(graphs: Graphs) -> Iterator { todo!() }
    pub fn init(&self, beginIndex: i32, endIndex: i32, pos: i32, width: i32, flag: i32) { todo!() }
    pub fn getFlag(&self) -> i32 { todo!() }
    pub fn changeFlag(&self, flag: i32) { todo!() }
    pub fn getBeginIndex(&self) -> i32 { todo!() }
    pub fn getEndIndex(&self) -> i32 { todo!() }
    pub fn isZeroField(&self) -> bool { todo!() }
    pub fn getPos(&self) -> i32 { todo!() }
    pub fn getWidth(&self) -> i32 { todo!() }
    pub fn getEndPos(&self) -> i32 { todo!() }
    pub fn toString_function toString() { [native code] }1(&self) -> String { todo!() }
    pub fn init_1(&self, graphs: Graphs) -> Iterator { todo!() }
    pub fn getFlags(&self) -> DfdFlagsInGraphs { todo!() }
    pub fn recycle(&self) { todo!() }
    pub fn getGraphs(&self) -> Graphs { todo!() }
    pub fn getDfdFlag(&self) -> i32 { todo!() }
    pub fn itBegin(&self) -> Graphs { todo!() }
    pub fn hasNext(&self) -> bool { todo!() }
    pub fn next(&self) -> Graphs { todo!() }
    pub fn itRestore(&self) { todo!() }
    pub fn changeToDocField(&self, index: i32) { todo!() }
}
