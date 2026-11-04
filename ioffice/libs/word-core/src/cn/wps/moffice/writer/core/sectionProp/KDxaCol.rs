// Rust stub for KDxaCol (auto-generated)
// Original: cn/wps/moffice/writer/core/sectionProp/KDxaCol.java
use crate::*;

pub struct KDxaCol {
    // TODO: fields
}

impl KDxaCol {
    pub fn new() -> Self { todo!() }
    pub fn setFixedDxaColumns(&self, selType: SelectionType, originRange: KRange, column: i32, fToLeft: Boolean) -> bool { todo!() }
    pub fn setFixedDxaColumns_1(&self, node: PLCSection::SectionNode, column: i32, fToLeft: Boolean) -> bool { todo!() }
    pub fn addSectionIfNeed(&self, selType: SelectionType, originRange: KRange, column: i32, fToLeft: Boolean) { todo!() }
    pub fn needAddSection(&self, cp: i32, column: i32, fToLeft: Boolean, fStart: bool) -> bool { todo!() }
    pub fn notifyResizingSectionTables(&self, node: PLCSection::SectionNode) { todo!() }
    pub fn setFixedDxaColumns_2(&self, selType: SelectionType, originRange: KRange, column: i32) -> bool { todo!() }
    pub fn setFixedDxaColumns_3(&self, selType: SelectionType, originRange: KRange, fToLeft: bool) -> bool { todo!() }
    pub fn setFixedDxaColumns_4(&self, node: PLCSection::SectionNode, fToLeft: bool) -> bool { todo!() }
    pub fn setFixedDxaColumns_5(&self, node: PLCSection::SectionNode, column: i32) -> bool { todo!() }
    pub fn addSection(&self, cp: i32, sectionBreak: i32) { todo!() }
    pub fn getDxaColumns(&self, cp: i32) -> Vec<DxaColumn> { todo!() }
    pub fn getMinFixedColumnWdith(&self, cp: i32) -> f32 { todo!() }
    pub fn getMainDocument(&self) -> Box<dyn Document> { todo!() }
    pub fn getTextDocument(&self) -> TextDocument { todo!() }
}
