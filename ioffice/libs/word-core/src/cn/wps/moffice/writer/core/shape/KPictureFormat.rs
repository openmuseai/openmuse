// Rust stub for KPictureFormat (auto-generated)
// Original: cn/wps/moffice/writer/core/shape/KPictureFormat.java
use crate::*;

pub struct KPictureFormat {
    // TODO: fields
}

impl KPictureFormat {
    pub fn new() -> Self { todo!() }
    pub fn enterClip(&self, coreShape: KShape) -> bool { todo!() }
    pub fn beginClip(&self, hitPos: HitPos, x: f32, y: f32, listener: IClipTransactListener) -> bool { todo!() }
    pub fn getCenterPoint(&self, rect: RectF, width: f32, height: f32, rot: f32) -> PointF { todo!() }
    pub fn getScale(&self, oldWidth: f32, oldHeight: f32, newWidth: f32, newHeight: f32) -> PointF { todo!() }
    pub fn adjustRectSize(&self, x: f32, y: f32) -> RectF { todo!() }
    pub fn adjustPicLeft(&self, picture: Picture, rectF: RectF) { todo!() }
    pub fn adjustPicRight(&self, picture: Picture, rectF: RectF) { todo!() }
    pub fn adjustPicTop(&self, picture: Picture, rectF: RectF) { todo!() }
    pub fn adjustPicBottom(&self, picture: Picture, rectF: RectF) { todo!() }
    pub fn adustPicRect(&self, shape: Shape, rectF: RectF, grf: GRF) { todo!() }
    pub fn clip(&self, x: f32, y: f32) -> bool { todo!() }
    pub fn clip_locked(&self, x: f32, y: f32) -> bool { todo!() }
    pub fn getMoveOffset(&self, x: f32, y: f32, rot: f32, grf: GRF) -> PointF { todo!() }
    pub fn endClip(&self) -> bool { todo!() }
    pub fn exitClip(&self) -> bool { todo!() }
    pub fn getPictureInfo(&self, coreShape: KShape) -> PictureInfo { todo!() }
    pub fn getPictureInfo_1(&self) -> PictureInfo { todo!() }
    pub fn getPicPath(&self, coreShape: KShape) -> String { todo!() }
    pub fn CropLeft(&self, crop: f32) -> bool { todo!() }
    pub fn CropRight(&self, crop: f32) -> bool { todo!() }
    pub fn CropTop(&self, crop: f32) -> bool { todo!() }
    pub fn CropBottom(&self, crop: f32) -> bool { todo!() }
    pub fn isCliping(&self) -> bool { todo!() }
    pub fn recordCrop(&self) { todo!() }
    pub fn compareShapeRect(&self, ori: RectF, dest: RectF) -> bool { todo!() }
    pub fn submitCrop(&self) { todo!() }
    pub fn getTransparentBackground(&self) -> bool { todo!() }
    pub fn setTransparentBackground(&self, bTrans: bool) { todo!() }
    pub fn setTransparencyColor(&self, color: i32) { todo!() }
    pub fn getTransparencyColor(&self) -> i32 { todo!() }
    pub fn isThrowProtectedException(&self, textDocument: TextDocument) -> bool { todo!() }
}

pub trait IClipTransactListener {}
