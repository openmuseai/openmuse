// Rust stub for ITypoShape (auto-generated)
// Original: cn/wps/show/moffice/drawing/render/ITypoShape.java
use crate::*;

pub struct ITypoShape {
    // TODO: fields
}

impl ITypoShape {
    pub fn new() -> Self { todo!() }
    pub fn getShapeRect(&self) -> RectF { todo!() }
    pub fn getShapeRenderRect(&self) -> RectF { todo!() }
    pub fn getShape(&self) -> Shape { todo!() }
    pub fn getChildCount(&self) -> i32 { todo!() }
    pub fn getChild(&self, var1: i32) -> ITypoShape { todo!() }
    pub fn getRenderModel(&self) -> Box<dyn RenderModel> { todo!() }
    pub fn render(&self, var1: ShapeEnv) { todo!() }
    pub fn isHidden(&self) -> bool { todo!() }
    pub fn isModularGroupDefaultPic(&self) -> bool { todo!() }
    pub fn getTextRenderModel(&self) -> Box<dyn ITextRenderModel> { todo!() }
}
