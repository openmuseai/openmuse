// Rust stub for DocumentImporter (auto-generated)
// Original: cn/wps/moffice/writer/io/reader/docReader/importer/DocumentImporter.java
use crate::*;

pub struct DocumentImporter {
    // TODO: fields
}

impl DocumentImporter {
    pub fn new() -> Self { todo!() }
    pub fn getTextDocument(&self) -> TextDocument { todo!() }
    pub fn getHWPFDocument(&self) -> HWPFDocument { todo!() }
    pub fn getIOListener(&self) -> Box<dyn IOListener> { todo!() }
    pub fn getDlProvider(&self) -> Box<dyn IDLProvider> { todo!() }
    pub fn getPLCFieldImporter(&self) -> PLCFieldImporter { todo!() }
    pub fn getStyleIndexMap(&self) -> StyleIdHashMap { todo!() }
    pub fn getOLEInfoList(&self) -> ArrayList { todo!() }
    pub fn getStyles(&self) -> KStyles { todo!() }
    pub fn importTextboxInShape(&self, spid: i32) { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn onImport(&self) { todo!() }
    pub fn dumpAllPictures(&self) { todo!() }
    pub fn dumpVbaMacro(&self) { todo!() }
    pub fn begin(&self) { todo!() }
    pub fn onImportDop(&self) { todo!() }
    pub fn onImportMetaData(&self) { todo!() }
    pub fn onImportSttbfAssoc(&self) { todo!() }
    pub fn onImportFontTable(&self) { todo!() }
    pub fn onImportRsid(&self) { todo!() }
    pub fn onImportNumbering(&self) { todo!() }
    pub fn onImportBackground(&self) { todo!() }
    pub fn onImportSections(&self) { todo!() }
    pub fn onImportDocuments(&self) { todo!() }
    pub fn onImportCustomXml(&self) { todo!() }
    pub fn onImportUnknownData(&self) { todo!() }
    pub fn onImportBookmarks(&self) { todo!() }
    pub fn getHeaderStartCP(&self) -> i32 { todo!() }
    pub fn onImportWpsCustomData(&self) { todo!() }
    pub fn end(&self) { todo!() }
    pub fn waitDumpBlipThreadFinish(&self) { todo!() }
    pub fn hasFootnote(&self) -> bool { todo!() }
    pub fn hasHeader(&self) -> bool { todo!() }
    pub fn hasComment(&self) -> bool { todo!() }
    pub fn hasEndnote(&self) -> bool { todo!() }
    pub fn hasTextbox(&self) -> bool { todo!() }
    pub fn hasHeaderTextbox(&self) -> bool { todo!() }
    pub fn readStwUser(&self) { todo!() }
    pub fn initDefaultProp(&self) { todo!() }
    pub fn getDefaultStyleProp(&self) -> PropertySet { todo!() }
    pub fn getMainDocumentLoadCp(&self) -> i32 { todo!() }
    pub fn getHeaderDocumentLoadCp(&self) -> i32 { todo!() }
    pub fn getCommentDocumentLoadCp(&self) -> i32 { todo!() }
    pub fn getEndnotDocumentLoadCp(&self) -> i32 { todo!() }
    pub fn getTextBoxDocumentLoadCp(&self) -> i32 { todo!() }
    pub fn getHeaderTextBoxDocumentLoadCp(&self) -> i32 { todo!() }
    pub fn getFootnoteDocumentLoadCp(&self) -> i32 { todo!() }
    pub fn getHeaderDocumentImporter(&self) -> SubDocumentImporter { todo!() }
    pub fn getFootnoteDocumentImporter(&self) -> SubDocumentImporter { todo!() }
    pub fn getEndnoteDocumentImporter(&self) -> SubDocumentImporter { todo!() }
    pub fn getMainDocumentImporter(&self) -> SubDocumentImporter { todo!() }
    pub fn getCommentDocumentImporter(&self) -> SubDocumentImporter { todo!() }
    pub fn getTextboxDocumentImporter(&self) -> SubDocumentImporter { todo!() }
    pub fn getHeaderTextboxDocumentImporter(&self) -> SubDocumentImporter { todo!() }
}
