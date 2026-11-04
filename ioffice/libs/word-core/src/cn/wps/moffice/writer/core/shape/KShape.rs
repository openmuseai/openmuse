// Rust stub for KShape (auto-generated)
// Original: cn/wps/moffice/writer/core/shape/KShape.java
use crate::*;

pub struct KShape {
    // TODO: fields
}

impl KShape {
    pub fn new() -> Self { todo!() }
    pub fn setShape(&self, shape: Shape) { todo!() }
    pub fn getShape(&self) -> Shape { todo!() }
    pub fn setInlineShape(&self, inlineShape: Shape) { todo!() }
    pub fn getInlineShape(&self) -> Shape { todo!() }
    pub fn offset(&self, offsetX: f32, offsetY: f32) { todo!() }
    pub fn isInlineShape(&self) -> bool { todo!() }
    pub fn hasDiagram(&self) -> bool { todo!() }
    pub fn isPicture(&self) -> bool { todo!() }
    pub fn isTextBox(&self) -> bool { todo!() }
    pub fn getTextBoxCpStart(&self) -> i32 { todo!() }
    pub fn getTextBoxVisitableCpStart(&self) -> i32 { todo!() }
    pub fn getTextBoxCpEnd(&self) -> i32 { todo!() }
    pub fn getTextBoxRange(&self) -> KRange { todo!() }
    pub fn setTypoRect(&self, rectF: RectF) { todo!() }
    pub fn getTypoRect(&self) -> RectF { todo!() }
    pub fn setDecription(&self, description: String) { todo!() }
    pub fn isDiagram(&self) -> bool { todo!() }
    pub fn isChartShape(&self) -> bool { todo!() }
    pub fn canAddTextBox(&self) -> bool { todo!() }
    pub fn addTextBox(&self) -> Text { todo!() }
    pub fn hasInk(&self) -> bool { todo!() }
    pub fn hasChart(&self) -> bool { todo!() }
    pub fn isNormalShape(&self) -> bool { todo!() }
    pub fn isGeoText(&self) -> bool { todo!() }
    pub fn isLinkedTextBox(&self) -> bool { todo!() }
    pub fn isCanvas(&self) -> bool { todo!() }
    pub fn canRotate(&self) -> bool { todo!() }
    pub fn notifyDelEvent(&self) { todo!() }
    pub fn replacePic(&self, picPath: String) -> bool { todo!() }
    pub fn getOleFilePath(&self) -> String { todo!() }
    pub fn getOle(&self) -> OLE { todo!() }
    pub fn isValidOleShape(&self) -> bool { todo!() }
    pub fn isPenkit(&self) -> bool { todo!() }
    pub fn isPenkitForSpecBrand(&self, brand: String) -> bool { todo!() }
    pub fn isPenkit_1(ole: OLE, document: Box<dyn Document>) -> bool { todo!() }
    pub fn isPenkitForSpecBrand_1(brand: String, ole: OLE, document: Box<dyn Document>) -> bool { todo!() }
    pub fn isHuaweiOrHonorPenkitData(penKitFileName: String) -> bool { todo!() }
    pub fn getPenKitFileName(ole: OLE, document: Box<dyn Document>) -> String { todo!() }
    pub fn getOleFile(&self) -> String { todo!() }
    pub fn updateOle(&self, rectF: RectF, fileName: String, iconFileName: String) -> bool { todo!() }
    pub fn isGroupShape(&self) -> bool { todo!() }
    pub fn getGroupItem(&self) -> KGroupShape { todo!() }
    pub fn isProtectOn(&self) -> bool { todo!() }
    pub fn isThrowProtectedException(&self) -> bool { todo!() }
    pub fn isVisible(&self) -> bool { todo!() }
    pub fn setVisible(&self, bVisible: bool) { todo!() }
    pub fn getName(&self) -> String { todo!() }
    pub fn setName(&self, name: String) { todo!() }
    pub fn getAlternativeText(&self) -> String { todo!() }
    pub fn setAlternativeText(&self, text: String) { todo!() }
    pub fn getRelativeHorizontalPosition(&self) -> i32 { todo!() }
    pub fn setRelativeHorizontalPosition(&self, rel: i32) { todo!() }
    pub fn getRelativeVerticalPosition(&self) -> i32 { todo!() }
    pub fn setRelativeVerticalPosition(&self, rel: i32) { todo!() }
    pub fn getTextFrame(&self) -> KTextFrame { todo!() }
    pub fn getFill(&self) -> KFill { todo!() }
    pub fn getLine(&self) -> KLine { todo!() }
    pub fn setIsAdd(&self, isAdd: bool) { todo!() }
    pub fn isAdd(&self) -> bool { todo!() }
    pub fn getPos(&self) -> RectF { todo!() }
    pub fn setPos(&self, pos: RectF) { todo!() }
    pub fn getScaleV(&self) -> f32 { todo!() }
    pub fn setScaleV(&self, scale: f32) { todo!() }
    pub fn getScaleH(&self) -> f32 { todo!() }
    pub fn setScaleH(&self, scale: f32) { todo!() }
    pub fn execute(&self) { todo!() }
    pub fn unexecute(&self) { todo!() }
}

pub struct CmdReplaceMediaId { /* TODO */ }
