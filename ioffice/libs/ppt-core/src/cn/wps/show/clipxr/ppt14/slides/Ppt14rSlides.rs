// Rust stub for Ppt14rSlides (auto-generated)
// Original: cn/wps/show/clipxr/ppt14/slides/Ppt14rSlides.java
use crate::*;

pub struct Ppt14rSlides {
    // TODO: fields
}

impl Ppt14rSlides {
    pub fn new() -> Self { todo!() }
    pub fn initialize() { todo!() }
    pub fn getSlideIdByFileName(fileName: String) -> i64 { todo!() }
    pub fn onSlideIds(&self, rids: List) { todo!() }
    pub fn onSlideMasterIds(&self, rids: List) { todo!() }
    pub fn open(&self) { todo!() }
    pub fn getMasterInTargetPres(&self, slideMaster: KmoSlideMaster) -> KmoSlideMaster { todo!() }
    pub fn getLayoutInTargetPres(&self, master: KmoSlideMaster, layoutName: String) -> KmoSlideLayout { todo!() }
    pub fn getLayoutName(&self, layoutRel: OpcrRelationship) -> String { todo!() }
    pub fn getMasterReader(&self, slideReader: PptxrSlide) -> Ppt14rMaster { todo!() }
}
