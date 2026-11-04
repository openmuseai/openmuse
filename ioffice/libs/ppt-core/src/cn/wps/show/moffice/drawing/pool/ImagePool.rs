// Rust stub for ImagePool (auto-generated)
// Original: cn/wps/show/moffice/drawing/pool/ImagePool.java
use crate::*;

pub struct ImagePool {
    // TODO: fields
}

impl ImagePool {
    pub fn new() -> Self { todo!() }
    pub fn pool() -> ImagePool { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn destroy(&self) { todo!() }
    pub fn setPoolConfig(&self, poolConfig: Box<dyn IPoolConfig>) { todo!() }
    pub fn poolConfig(&self) -> Box<dyn IPoolConfig> { todo!() }
    pub fn getPictureInfo(&self, path: String) -> PictureInfo { todo!() }
    pub fn getCache(&self, picInfo: PictureInfo, displayWidth: i32, displayHeight: i32) -> Box<dyn Bitmap> { todo!() }
    pub fn getCache_1(&self, picInfo: PictureInfo, displayWidth: i32, displayHeight: i32, samplePolicy: CalcInSamplePolicies::ICalcInSamplePolicy) -> Box<dyn Bitmap> { todo!() }
    pub fn createBitmap(&self, picInfo: PictureInfo, displayWidth: i32, displayHeight: i32) -> Box<dyn Bitmap> { todo!() }
    pub fn createBitmap_1(&self, picInfo: PictureInfo, displayWidth: i32, displayHeight: i32, samplePolicy: CalcInSamplePolicies::ICalcInSamplePolicy) -> Box<dyn Bitmap> { todo!() }
    pub fn createRegionBitmap(&self, pictureInfo: PictureInfo, region: Rect, regionExpectedWidth: i32, regionExpectedHeight: i32) -> Box<dyn Bitmap> { todo!() }
    pub fn loadBitmap(&self, picInfo: PictureInfo, displayWidth: i32, displayHeight: i32) -> Box<dyn Bitmap> { todo!() }
    pub fn loadBitmap_1(&self, picInfo: PictureInfo, displayWidth: i32, displayHeight: i32, samplePolicy: CalcInSamplePolicies::ICalcInSamplePolicy) -> Box<dyn Bitmap> { todo!() }
    pub fn getRegionCache(&self, pictureInfo: PictureInfo, region: Rect, regionExpectedWidth: i32, regionExpectedHeight: i32) -> Box<dyn Bitmap> { todo!() }
    pub fn loadRegionBitmap(&self, pictureInfo: PictureInfo, region: Rect, regionExpectedWidth: i32, regionExpectedHeight: i32) -> Box<dyn Bitmap> { todo!() }
    pub fn getOrLoadThumbBitmap(&self, picInfo: PictureInfo, displayWidth: i32, displayHeight: i32) -> Box<dyn Bitmap> { todo!() }
    pub fn getBitmapCache(&self, width: i32, height: i32, config: Bitmap::Config) -> Box<dyn Bitmap> { todo!() }
    pub fn reuseBitmapCache(&self, bitmap: Box<dyn Bitmap>) { todo!() }
    pub fn transGifBitmap(&self, pictureInfo: PictureInfo, bitmap: Box<dyn Bitmap>) -> Box<dyn Bitmap> { todo!() }
    pub fn getMetaDrawer(&self) -> MetaFileDrawer { todo!() }
}
