// Rust stub for ShapeReaderTool (auto-generated)
// Original: cn/wps/moffice/writer/io/reader/docReader/tools/ShapeReaderTool.java
use crate::*;

pub struct ShapeReaderTool {
    // TODO: fields
}

impl ShapeReaderTool {
    pub fn new() -> Self { todo!() }
    pub fn getShapePos(picf: PICF, pos: Box<dyn IPos>) -> Box<dyn IPos> { todo!() }
    pub fn getShapePos_1(diskDoc: HWPFDocument, fspaNode: GenericPropertyNode, pos: Box<dyn IPos>) -> Box<dyn IPos> { todo!() }
    pub fn getGRF(flag: i32, shape: Shape) -> GRF { todo!() }
    pub fn convertGeoAdjust(shape: Shape) { todo!() }
    pub fn adjustGeoAdjust(shape: Shape) { todo!() }
    pub fn getString(content: Vec<i8>, charSet: String) -> String { todo!() }
    pub fn filterValidChar(str: String) -> String { todo!() }
    pub fn writeToFile(data: Vec<i8>, fileName: String, rootDir: String) -> String { todo!() }
    pub fn setShapeRectChange(pos: Box<dyn IPos>, rectF: RectF) -> bool { todo!() }
}
