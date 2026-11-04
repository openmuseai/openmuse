// Rust stub for RenderModelOverride (auto-generated)
// Original: cn/wps/show/render/RenderModelOverride.java
use crate::*;

pub struct RenderModelOverride {
    // TODO: fields
}

impl RenderModelOverride {
    pub fn new() -> Self { todo!() }
    pub fn setOrigin(&self, origin: Box<dyn RenderModel>) { todo!() }
    pub fn setOverlayFill(&self, overlayFill: bool) { todo!() }
    pub fn setFillColor(&self, rgb: i32) { todo!() }
    pub fn setFillBitmap(&self, gifFrame: Box<dyn Bitmap>) { todo!() }
    pub fn setFillOn(&self, on: bool) { todo!() }
    pub fn setStrokeOn(&self, on: bool) { todo!() }
    pub fn hasFillColor(&self) -> bool { todo!() }
    pub fn hasStrokeColor(&self) -> bool { todo!() }
    pub fn setStrokeColor(&self, rgb: i32) { todo!() }
    pub fn isFillRotationWithShape(&self) -> bool { todo!() }
    pub fn isFlipH(&self) -> bool { todo!() }
    pub fn isFlipV(&self) -> bool { todo!() }
    pub fn getRotation(&self) -> f32 { todo!() }
    pub fn getAbsoluteRotation(&self) -> f32 { todo!() }
    pub fn getGraphicsPaths(&self, width: f32, height: f32) -> Vec<GraphicsPath> { todo!() }
    pub fn _createFillColor(&self) -> FillBase { todo!() }
    pub fn _createFillBlip(&self) -> FillBase { todo!() }
    pub fn _setFillColor(&self, rgb: i32) { todo!() }
    pub fn _setFillBlip(&self, bmp: Box<dyn Bitmap>) { todo!() }
    pub fn getFill(&self) -> FillBase { todo!() }
    pub fn getLineProperty(&self) -> LineProperty { todo!() }
    pub fn getPicture(&self) -> Picture { todo!() }
    pub fn getReflection(&self) -> Reflection { todo!() }
    pub fn getObject3d(&self) -> Object3D { todo!() }
    pub fn getGlow(&self) -> Glow { todo!() }
    pub fn getSoftEdge(&self) -> SoftEdge { todo!() }
    pub fn getShadow(&self) -> Shadow { todo!() }
    pub fn getGeoText(&self) -> GeoText { todo!() }
    pub fn getOutRect(&self, rectF: RectF) -> RectF { todo!() }
    pub fn getGRF(&self) -> GRF { todo!() }
    pub fn hasInk(&self) -> bool { todo!() }
    pub fn getInk(&self) -> Ink { todo!() }
    pub fn getShapePos(&self) -> Box<dyn IPos> { todo!() }
    pub fn getDiagramDrawingML(&self) -> Box<dyn IDiagram> { todo!() }
    pub fn getShapeType(&self) -> i32 { todo!() }
    pub fn getHidden(&self) -> bool { todo!() }
    pub fn getChart(&self) -> Shape { todo!() }
    pub fn getTextObj(&self) -> Text { todo!() }
    pub fn getShapeID(&self) -> i32 { todo!() }
    pub fn getTextRect(&self, w: f32, h: f32) -> RectF { todo!() }
    pub fn getNormalOutRect(&self, rectF: RectF) -> RectF { todo!() }
}
