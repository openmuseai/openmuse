// Rust stub for PictureImporter (auto-generated)
// Original: cn/wps/moffice/writer/io/reader/docReader/importer/shape/PictureImporter.java
use crate::*;

pub struct PictureImporter {
    // TODO: fields
}

impl PictureImporter {
    pub fn new() -> Self { todo!() }
    pub fn importPictureInfo(stream: DocumentInputStream, bstore: EscherContainerRecord, blipID: i32, shape: Shape) -> EscherBSERecord { todo!() }
    pub fn getShape(&self) -> Shape { todo!() }
    pub fn getType(&self) -> i32 { todo!() }
    pub fn isEndNode(&self) -> bool { todo!() }
    pub fn setMediaID(&self, mediaID: i32) { todo!() }
    pub fn dispose() { todo!() }
    pub fn isHandled(&self) -> bool { todo!() }
    pub fn setHandled(&self) { todo!() }
    pub fn dumpBlip(&self, baseInfo: PictureLoader::DefaultDumpBlipHandler::BlipBaseInfo) -> i32 { todo!() }
    pub fn doDumpBlip(blipInfo: BlipDocInfo, rootDir: String) -> i32 { todo!() }
    pub fn handleBreakDownPic(blipInfo: BlipDocInfo) -> i32 { todo!() }
    pub fn getNewPicName(extension: String) -> String { todo!() }
    pub fn pauseThread(&self) { todo!() }
    pub fn wakeUpThread(&self) { todo!() }
}
