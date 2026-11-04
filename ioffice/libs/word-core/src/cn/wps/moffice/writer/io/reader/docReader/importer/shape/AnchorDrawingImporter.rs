// Rust stub for AnchorDrawingImporter (auto-generated)
// Original: cn/wps/moffice/writer/io/reader/docReader/importer/shape/AnchorDrawingImporter.java
use crate::*;

pub struct AnchorDrawingImporter {
    // TODO: fields
}

impl AnchorDrawingImporter {
    pub fn new() -> Self { todo!() }
    pub fn onImportBKShape(&self, spContainer: EscherContainerRecord, bStoreContainer: EscherContainerRecord) -> Shape { todo!() }
    pub fn importProperty(&self, spContainer: EscherContainerRecord, bStoreContainer: EscherContainerRecord, shape: Shape, fbtSp: EscherSpRecord) { todo!() }
    pub fn correctFillSolidColor(&self, shape: Shape, fbtOpt: EscherOptRecord) { todo!() }
    pub fn onImportBackGroundShapeFlag(&self, fbtSp: EscherSpRecord, shape: Shape) { todo!() }
    pub fn onImportGroupShape(&self, spgrContainer: EscherContainerRecord, bStoreContainer: EscherContainerRecord, parentShape: GroupShape) -> Shape { todo!() }
    pub fn addShapeToCore(&self, parentShape: GroupShape, drawingContainer: Box<dyn IDrawingContainer>, shape: Shape) { todo!() }
    pub fn correctTextbox(&self, shape: Shape) { todo!() }
    pub fn getTextboxNodeIndex(&self, textboxNode: PLCTextboxText::TextboxTextNode) -> Integer { todo!() }
    pub fn changeGroupShapeToShape(&self, groupShape: GroupShape) -> Shape { todo!() }
    pub fn onImportOneShape(&self, spContainer: EscherContainerRecord, bStoreContainer: EscherContainerRecord, parentShape: GroupShape) -> Shape { todo!() }
    pub fn onImportChildShape(&self, spContainer: EscherContainerRecord, bStoreContainer: EscherContainerRecord, groupShape: GroupShape) { todo!() }
    pub fn setCoordBounds(&self, spContainer: EscherContainerRecord, groupShape: GroupShape) { todo!() }
    pub fn onImport(&self, spContainer: EscherContainerRecord, bStoreContainer: EscherContainerRecord, shape: Shape) { todo!() }
    pub fn onImportImage(&self, bStoreContainer: EscherContainerRecord, fbtOpt: EscherOptRecord, fbtUDefProp: EscherUDefPropRecord, shape: Shape) { todo!() }
    pub fn onImportShapeFlag(&self, fbtSp: EscherSpRecord, shape: Shape) { todo!() }
    pub fn onImportChildAnchor(&self, spContainer: EscherContainerRecord, shape: Shape) { todo!() }
    pub fn onImportTextBoxProp(&self, fbtSp: EscherSpRecord, fbtOpt: EscherOptRecord, fbtUDefProp: EscherUDefPropRecord, shape: Shape) { todo!() }
    pub fn setDefaultProps(&self, fbtOpt: EscherOptRecord, textFrame: TextFrame) { todo!() }
    pub fn importRelTxbxDoc(&self, fbtSp: EscherSpRecord) { todo!() }
    pub fn onImportAdditionalData(&self, fbtUDefProp: EscherUDefPropRecord, shape: Shape) { todo!() }
}
