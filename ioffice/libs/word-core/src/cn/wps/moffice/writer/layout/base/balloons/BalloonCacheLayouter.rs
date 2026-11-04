// Rust stub for BalloonCacheLayouter (auto-generated)
// Original: cn/wps/moffice/writer/layout/base/balloons/BalloonCacheLayouter.java
use crate::*;

pub struct BalloonCacheLayouter {
    // TODO: fields
}

impl BalloonCacheLayouter {
    pub fn new() -> Self { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn hascomments(&self) -> bool { todo!() }
    pub fn makeBalloonCache(&self, pPage: i32) { todo!() }
    pub fn isPhoneReadMode(&self) -> bool { todo!() }
    pub fn quickMakeBalloonCache(&self, document: Box<dyn Document>, startCP: i32, endCP: i32, newLines: Box<dyn IIntList>) { todo!() }
    pub fn updateCache(&self, document: Box<dyn Document>, start: i32, end: i32, revisions: WIntArrayList, comments: WIntArrayList, newLines: Box<dyn IIntList>) { todo!() }
    pub fn getNodeType(revisionType: i32) -> i32 { todo!() }
    pub fn init(&self) { todo!() }
    pub fn revision(&self) { todo!() }
    pub fn addRevision(&self, document: Box<dyn Document>, startRef: i32, authorId: i32, anchor: i32, ballonCacheType: i32) { todo!() }
    pub fn addTableChangeRevision(&self, document: Box<dyn Document>, startRef: i32, authorId: i32, anchor: i32, level: i32, ballonCacheType: i32) { todo!() }
    pub fn updateItemPos(&self, item: i32, anchor: i32) { todo!() }
    pub fn updateItemPos_1(&self, item: i32, anchor: i32, data: Box<dyn std::any::Any + Send + Sync>) { todo!() }
    pub fn offsetItemPos(&self, offset: i32, offsetYPos: bool, document: Box<dyn Document>, start: i32, end: i32) { todo!() }
    pub fn removeAfterCP(&self, document: Box<dyn Document>, cp: i32) { todo!() }
}
