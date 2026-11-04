// Rust stub for DocWriter (auto-generated)
// Original: cn/wps/moffice/writer/io/writer/doc/DocWriter.java
use crate::*;

pub struct DocWriter {
    // TODO: fields
}

impl DocWriter {
    pub fn new() -> Self { todo!() }
    pub fn getEncryptedOleStreamWriter(&self) -> EncryptedOleStreamWriter { todo!() }
    pub fn getProviderInfo(&self) -> ProviderInfo { todo!() }
    pub fn getDocument(&self) -> TextDocument { todo!() }
    pub fn isAutoSave(&self) -> bool { todo!() }
    pub fn getWPFDocument(&self) -> HWPFDocument { todo!() }
    pub fn getFfnList(&self) -> ArrayList { todo!() }
    pub fn getRevisionAuthorList(&self) -> ArrayList { todo!() }
    pub fn getStylesHandler(&self) -> StylesHandler { todo!() }
    pub fn getStyleIndexMap(&self) -> Map { todo!() }
    pub fn getInlineShapeMap(&self) -> Map { todo!() }
    pub fn write(&self) { todo!() }
    pub fn filterData(&self) { todo!() }
    pub fn createEmptyHWPFDocument(&self) -> HWPFDocument { todo!() }
    pub fn clearHWPFDocument(&self, doc: HWPFDocument) { todo!() }
    pub fn createExporter(&self) { todo!() }
    pub fn getDocumentExporter(&self) -> Box<dyn IWdDocumentExporter> { todo!() }
    pub fn write_1(&self, hwpfDocument: HWPFDocument, provider: ProviderInfo, itemPath: File, propsPath: File, hasOle: bool) { todo!() }
    pub fn writeSttbRgtplc(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn writeSttbListNames(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn exportFontTable(&self) { todo!() }
    pub fn toFfnArray(infos: ArrayList) -> Vec<Ffn> { todo!() }
    pub fn toFfn(info: FontInfo) -> Ffn { todo!() }
    pub fn toPanoseBytes(info: FontInfo) -> Vec<i8> { todo!() }
    pub fn writeFib(&self, hwpfDocument: HWPFDocument, provider: ProviderInfo, mainStream: HWPFOutputStream, tableStream: HWPFOutputStream, fcMin: i32, fcMac: i32) { todo!() }
    pub fn getFib(&self) -> FileInformationBlock { todo!() }
    pub fn writeComplexFileTable(&self, docStream: HWPFOutputStream, tableStream: HWPFOutputStream) { todo!() }
    pub fn writeCHPBinTable(&self, tableStream: HWPFOutputStream, fcMin: i32) { todo!() }
    pub fn writePAPBinTable(&self, tableStream: HWPFOutputStream, fcMin: i32) { todo!() }
    pub fn writeSectionTable(&self, docSys: HWPFFileSystem, tableStream: HWPFOutputStream, fcMin: i32) { todo!() }
    pub fn writeSttbfAssoc(&self, docSys: HWPFFileSystem, tableStream: HWPFOutputStream) { todo!() }
    pub fn writeDgg(&self, docStream: HWPFOutputStream, tableStream: HWPFOutputStream, tableOffset: i32) { todo!() }
    pub fn writeDrawing(&self, mainStream: HWPFOutputStream, tableStream: HWPFOutputStream) { todo!() }
    pub fn exportShapeAnchor(&self, subdocument: i32) { todo!() }
    pub fn writeBookmarks(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn writeComments(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn writeFootnote(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn writeEndnote(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn exportRevisionMarkAuthorTable(&self) { todo!() }
    pub fn exportRsidTable(&self) { todo!() }
    pub fn exportMetadata(&self) { todo!() }
    pub fn writeRevisions(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn writeTextbox(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn writeHeaderTextbox(&self, tableStream: HWPFOutputStream) { todo!() }
    pub fn getLastSubDocType(&self, doc: TextDocument) -> i32 { todo!() }
    pub fn getLength(doc: Box<dyn Document>) -> i32 { todo!() }
    pub fn stopAutoWrite(&self, textDocument: TextDocument) { todo!() }
    pub fn checkAutoWrite(&self) { todo!() }
    pub fn openAndSaveAll(sdcardDir: String) { todo!() }
    pub fn createLog(logFilePath: String) -> PrintStream { todo!() }
    pub fn listValidDocFiles(samplesDir: File) -> Vec<File> { todo!() }
    pub fn accept(&self, dir: File, name: String) -> bool { todo!() }
    pub fn createIOListener() -> Box<dyn IOListener> { todo!() }
    pub fn onFinish(&self) { todo!() }
    pub fn onError(&self, resultCode: i32, msg: Box<dyn std::any::Any + Send + Sync>) { todo!() }
    pub fn onFinishDumpObjects(&self) { todo!() }
    pub fn onLoadParas(&self, cp: i32) { todo!() }
    pub fn onFirstLock(&self) { todo!() }
    pub fn onFirstUnLock(&self) { todo!() }
    pub fn onHtmlOpenError(&self) { todo!() }
    pub fn onFinishVerifyWritePassword(&self) { todo!() }
    pub fn beginLoadOnlineSecurityDoc(&self) { todo!() }
    pub fn openAndSave(log: PrintStream, path: String, savePath: String) { todo!() }
    pub fn initFfnList(&self) { todo!() }
    pub fn exportAsArray(&self) -> Vec<i8> { todo!() }
}
