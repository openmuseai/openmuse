// Rust stub for InsertDocument (auto-generated)
// Original: cn/wps/moffice/writer/insert/InsertDocument.java
use crate::*;

pub struct InsertDocument {
    // TODO: fields
}

impl InsertDocument {
    pub fn new() -> Self { todo!() }
    pub fn Insert(&self, src: Box<dyn Document>, dest: Box<dyn Document>) { todo!() }
    pub fn Insert_1(&self, src: Box<dyn Document>, start: i32, end: i32, dest: Box<dyn Document>, cp: i32) { todo!() }
    pub fn filterRowEndCellEnd(&self, papxProp: PropertySet) -> PropertySet { todo!() }
    pub fn Insert2(&self, src: Box<dyn Document>, start: i32, end: i32, dest: Box<dyn Document>, cp: i32) { todo!() }
    pub fn dealWithTable(&self, start: i32, cp: i32) -> bool { todo!() }
    pub fn insertPointHasComment(&self, destPapx: IPapxTable::Entry, cp: i32) -> bool { todo!() }
    pub fn isNullDocument2(&self, document: Box<dyn Document>) -> bool { todo!() }
    pub fn isNullDocument(&self, document: Box<dyn Document>) -> bool { todo!() }
    pub fn insert(&self, chpx: IChpxTable::Entry) { todo!() }
    pub fn getRunPropertyOfTextInsert(&self) -> PropertySet { todo!() }
    pub fn insertText(&self) { todo!() }
    pub fn removeRevisionProp(&self, prop: PropertySet) -> PropertySet { todo!() }
    pub fn addStyle(&self, styleId: i32) { todo!() }
    pub fn addTableStyle(&self, styleId: i32) { todo!() }
    pub fn insertParagraph(&self, anchor: u16, cp: i32) { todo!() }
    pub fn getRunPropOfParagraph(&self, cp: i32, document: Box<dyn Document>) -> PropertySet { todo!() }
    pub fn getTableAdjustProperty(&self, cp: i32, srcProp: MutablePropertySet) -> bool { todo!() }
    pub fn createDefTable(&self, srcDefTable: DefTable) -> DefTable { todo!() }
    pub fn getAutoNumAdjustProperty(&self, prop: PropertySet) -> PropertySet { todo!() }
    pub fn getAdjustParaProperty(&self, prop: PropertySet) -> PropertySet { todo!() }
    pub fn insertShape(&self, cp: i32, anchor: u16) { todo!() }
    pub fn insertField(&self, cp: i32, anchor: u16) { todo!() }
}
