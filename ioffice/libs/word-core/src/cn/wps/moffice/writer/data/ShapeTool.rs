// Rust stub for ShapeTool (auto-generated)
// Original: cn/wps/moffice/writer/data/ShapeTool.java
use crate::*;

pub struct ShapeTool {
    // TODO: fields
}

impl ShapeTool {
    pub fn new() -> Self { todo!() }
    pub fn isShape(document: Box<dyn Document>, cp: i32) -> bool { todo!() }
    pub fn isShape_1(document: Box<dyn Document>, frag: IChpxTable::Entry) -> bool { todo!() }
    pub fn position(document: Box<dyn Document>, shape: Shape) -> i32 { todo!() }
    pub fn findShape(document: Box<dyn Document>, cp: i32, isInline: bool) -> Shape { todo!() }
    pub fn findInk(shape: Shape) -> Ink { todo!() }
    pub fn findShape_1(document: Box<dyn Document>, cp: i32) -> Shape { todo!() }
    pub fn findShape_2(document: Box<dyn Document>, anchor: PLCShapeAnchor::ShapeAnchor) -> Shape { todo!() }
    pub fn findShapeByImmutableShape(shape: Shape) -> Shape { todo!() }
    pub fn isShapeTriple(triple: PLCField::FieldTriple) -> bool { todo!() }
}
