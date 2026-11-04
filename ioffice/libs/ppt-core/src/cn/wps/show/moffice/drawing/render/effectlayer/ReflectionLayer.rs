// Rust stub for ReflectionLayer (auto-generated)
// Original: cn/wps/show/moffice/drawing/render/effectlayer/ReflectionLayer.java
use crate::*;

pub struct ReflectionLayer {
    // TODO: fields
}

impl ReflectionLayer {
    pub fn new() -> Self { todo!() }
    pub fn support(shape: Box<dyn RenderModel>, env: ShapeEnv) -> bool { todo!() }
    pub fn onDraw(&self) { todo!() }
    pub fn checkCanApplyOrigin(&self, textExist: bool) -> bool { todo!() }
    pub fn onBegin(&self) -> bool { todo!() }
    pub fn adjustScale(&self, sx: f32, sy: f32, dstRct: RectF) { todo!() }
    pub fn drawReflection(&self) { todo!() }
    pub fn setReflectionMatrix(&self, bounds: RectF) { todo!() }
    pub fn drawReflectionBitmap(&self) { todo!() }
    pub fn getReflectionBitmap(&self, blurBitmap: Box<dyn Bitmap>, paint: Paint, bounds: RectF) -> Box<dyn Bitmap> { todo!() }
    pub fn getGradientShader(&self, reflection: Reflection, color: i32, bounds: RectF) -> Shader { todo!() }
    pub fn getIndexForYAxisMinValue(&self, boundPts: Vec<f32>) -> i32 { todo!() }
    pub fn getIndexForYAxisMaxValue(&self, boundPts: Vec<f32>) -> i32 { todo!() }
    pub fn blurBitmap(&self, bitmap: Box<dyn Bitmap>, radius: f32, canReuseInBitmap: bool) -> Box<dyn Bitmap> { todo!() }
    pub fn rsBlurBitmap(bitmap: Box<dyn Bitmap>, radius: f32, canReuseInBitmap: bool, canUseShapeBitmapPool: bool) -> Box<dyn Bitmap> { todo!() }
    pub fn fastBlur(sentBitmap: Box<dyn Bitmap>, radius: i32, canReuseInBitmap: bool, canUseShapeBitmapPool: bool) -> Box<dyn Bitmap> { todo!() }
}
