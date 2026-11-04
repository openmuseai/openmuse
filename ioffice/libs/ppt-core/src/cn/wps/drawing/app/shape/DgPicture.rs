// Rust stub for DgPicture (auto-generated)
// Original: cn/wps/drawing/app/shape/DgPicture.java
use crate::*;

pub struct DgPicture {
    // TODO: fields
}

impl DgPicture {
    pub fn new() -> Self { todo!() }
    pub fn find(&self, spId: i32) -> DgPicture { todo!() }
    pub fn getPicMediaId(&self) -> i32 { todo!() }
    pub fn getFill(&self) -> Fill { todo!() }
    pub fn changePicture(&self, mediaId: i32) { todo!() }
    pub fn putModularGroupPicture(&self, mediaId: i32) { todo!() }
    pub fn changePicture_1(&self, mediaId: i32, width: i32, height: i32) { todo!() }
    pub fn changePicture_2(&self, mediaId: i32, left: f32, right: f32, top: f32, bottom: f32) { todo!() }
    pub fn getSrcRect(&self) -> RectF { todo!() }
    pub fn setSrcRect(&self, srcRect: RectF) { todo!() }
    pub fn copyBlipEffectsFrom(&self, srcShape: Shape) { todo!() }
    pub fn copySrcRectFrom(&self, srcShape: Shape) { todo!() }
    pub fn getClipType(&self) -> i32 { todo!() }
    pub fn setPrstGeom(&self, r#type: i32) { todo!() }
    pub fn setCustGeom(&self, pts: Vec<f32>, bounds: RectF, bSmooth: bool) { todo!() }
    pub fn setCustGeom_1(&self, paths: Vec<GraphicsPath>, bounds: RectF) { todo!() }
    pub fn getPoint(&self, x: i32, y: i32) -> AdjPoint2D { todo!() }
    pub fn getCoord(&self, v: i32) -> AdjCoordinate { todo!() }
    pub fn getAngle(&self, v: f32) -> AdjAngle { todo!() }
}
