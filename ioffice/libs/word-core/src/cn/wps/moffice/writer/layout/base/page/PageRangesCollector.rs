// Rust stub for PageRangesCollector (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/page/PageRangesCollector.java
use crate::*;

pub struct PageRangesCollector {
    // TODO: fields
}

impl PageRangesCollector {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn collectRanges(&self, doc: Box<dyn Document>, page: TypoPage, start: i32, end: i32, r#ref: i32, collectRangeForFirstRow: bool, collectRangeForLastRow: bool, snapshot: TypoSnapshot, drawingLayers: TypoDrawingLayers) { todo!() }
    pub fn collectRangesForBreakCell(&self, doc: Box<dyn Document>, page: TypoPage, snapshot: TypoSnapshot, drawingLayers: TypoDrawingLayers) { todo!() }
    pub fn collectRanges_1(&self, doc: Box<dyn Document>, pRow: i32, isFirstLine: bool, isLastLine: bool, page: TypoPage, snapshot: TypoSnapshot) { todo!() }
    pub fn collectRanges_2(&self, srcPage: i32, collectMainRange: bool, page: TypoPage, snapshot: TypoSnapshot) { todo!() }
    pub fn addLine(&self, line: i32) { todo!() }
    pub fn commitMainRange(&self, page: TypoPage, snapshot: TypoSnapshot) { todo!() }
    pub fn getFirstLineRef(&self, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn getFirstStartCP(&self, snapshot: TypoSnapshot) -> i32 { todo!() }
    pub fn getLastEndCP(&self, snapshot: TypoSnapshot) -> i32 { todo!() }
}
