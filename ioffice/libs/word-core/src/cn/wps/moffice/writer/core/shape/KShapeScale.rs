// Rust stub for KShapeScale (auto-generated)
// Original: cn/wps/moffice/writer/core/shape/KShapeScale.java
use crate::*;

pub struct KShapeScale {
    // TODO: fields
}

impl KShapeScale {
    pub fn new() -> Self { todo!() }
    pub fn beginScale(&self, coreShape: KShape, hitPos: HitPos, x: f32, y: f32) -> bool { todo!() }
    pub fn resizeTableInShape(&self, shape: Shape) { todo!() }
    pub fn ScaleRect(&self, rectF: RectF, shapePos: ShapePos) { todo!() }
    pub fn adjustRectFlip(&self, rectF: RectF, bHori: bool, bJustRect: bool) { todo!() }
    pub fn adjustHitPos(&self, bHori: bool) { todo!() }
    pub fn getCenterPoint(&self, rect: RectF, width: f32, height: f32, rot: f32, hitPos: HitPos) -> PointF { todo!() }
    pub fn getHitPoint(&self, oldRect: RectF, newRect: RectF, oldHit: HitPos, newHit: HitPos, oldPoint: PointF, rot: i32) -> PointF { todo!() }
    pub fn getScale(&self, oldWidth: f32, oldHeight: f32, newWidth: f32, newHeight: f32, bRatio: bool) -> PointF { todo!() }
    pub fn adjustFlip(&self, rectF: RectF, bJustRect: bool) -> bool { todo!() }
    pub fn adjustRect(&self, from: PointF, to: PointF, needRatio: bool) { todo!() }
    pub fn adjustInit(&self, rectF: RectF, oldHitPos: HitPos, newWidth: f32, newHeight: f32, x: f32, y: f32) { todo!() }
    pub fn getRotHitPos(&self, hitPos: HitPos, curRot: i32, rot: i32) -> HitPos { todo!() }
    pub fn getRotField(&self, rot: f32) -> i32 { todo!() }
    pub fn getNotFlipRotation(&self, rot: f32, flipH: bool, flipV: bool) -> f32 { todo!() }
    pub fn adjustOtherShapes(&self) { todo!() }
    pub fn adjustFlipH(&self, grf: GRF, rectF: RectF, bFlip: bool, hitPos: HitPos) { todo!() }
    pub fn adjustFlipV(&self, grf: GRF, rectF: RectF, bFlip: bool, hitPos: HitPos) { todo!() }
    pub fn adjustOtherRect(&self, rectF: RectF, width: f32, height: f32, hitPos: HitPos) { todo!() }
    pub fn scale(&self, coreShape: KShape, from: PointF, to: PointF, needRatio: bool) -> bool { todo!() }
    pub fn endScale(&self, x: f32, y: f32) -> bool { todo!() }
    pub fn submit(&self) { todo!() }
    pub fn getCenterPoint_1(rect: RectF, width: f32, height: f32, rot: f32, hitPos: HitPos, outPoint: PointF) { todo!() }
}
