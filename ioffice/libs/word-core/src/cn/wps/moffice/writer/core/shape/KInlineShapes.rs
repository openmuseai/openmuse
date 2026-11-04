// Rust stub for KInlineShapes (auto-generated)
// Original: cn/wps/moffice/writer/core/shape/KInlineShapes.java
use crate::*;

pub struct KInlineShapes {
    // TODO: fields
}

impl KInlineShapes {
    pub fn new() -> Self { todo!() }
    pub fn addPicture(&self, filePath: String, linkToFile: bool, saveWithDocument: bool, range: KRange) -> KShape { todo!() }
    pub fn addPicture_1(&self, filePath: String, linkToFile: bool, saveWithDocument: bool, range: KRange, maxWidth: f32, maxHeight: f32) -> KShape { todo!() }
    pub fn addPicture_2(&self, filePath: String, range: KRange, width: f32, height: f32) -> KShape { todo!() }
    pub fn addPicture_3(&self, range: KRange, rectF: RectF, mediaID: i32) -> KShape { todo!() }
    pub fn copyPicture(&self, srcShape: Shape, filePath: String, range: KRange) -> KShape { todo!() }
    pub fn getPicInfo(&self, filePath: String, range: KRange, rectF: RectF) -> i32 { todo!() }
    pub fn addOLEObject(&self, classType: String, fileName: String, iconFileName: String, link2File: bool, displayAsIcon: bool, range: KRange) -> KShape { todo!() }
    pub fn createPictureShape(&self, document: Box<dyn Document>, iconFileName: String, displayAsIcon: bool) -> Shape { todo!() }
    pub fn count(&self) -> i32 { todo!() }
    pub fn item(&self, index: i32) -> KShape { todo!() }
    pub fn isProtectOn(&self) -> bool { todo!() }
    pub fn isThrowProtectedException(&self) -> bool { todo!() }
}
