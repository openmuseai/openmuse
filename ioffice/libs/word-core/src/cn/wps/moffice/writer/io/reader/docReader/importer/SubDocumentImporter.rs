// Rust stub for SubDocumentImporter (auto-generated)
// Original: cn/wps/moffice/writer/io/reader/docReader/importer/SubDocumentImporter.java
use crate::*;

pub struct SubDocumentImporter {
    // TODO: fields
}

impl SubDocumentImporter {
    pub fn new() -> Self { todo!() }
    pub fn getHWPFDocument(&self) -> HWPFDocument { todo!() }
    pub fn getParent(&self) -> DocumentImporter { todo!() }
    pub fn getSubDocument(&self) -> Box<dyn Document> { todo!() }
    pub fn getIOListener(&self) -> Box<dyn IOListener> { todo!() }
    pub fn getStyleIndexMap(&self) -> StyleIdHashMap { todo!() }
    pub fn getDlProvider(&self) -> Box<dyn IDLProvider> { todo!() }
    pub fn getStyles(&self) -> KStyles { todo!() }
    pub fn begin(&self) { todo!() }
    pub fn end(&self) { todo!() }
    pub fn onImport(&self) { todo!() }
    pub fn onImportContext(&self) { todo!() }
    pub fn createContextImporter(&self) { todo!() }
    pub fn initContextImporter(&self) { todo!() }
    pub fn getStartGCP(&self) -> i32 { todo!() }
    pub fn getEndGCP(&self) -> i32 { todo!() }
    pub fn onImportField(&self) { todo!() }
    pub fn onImportPLCs(&self) { todo!() }
}

pub struct Position { /* TODO */ }
