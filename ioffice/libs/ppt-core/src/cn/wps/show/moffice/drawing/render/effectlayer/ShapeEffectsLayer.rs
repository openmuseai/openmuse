// Rust stub for ShapeEffectsLayer (auto-generated)
// Original: cn/wps/show/moffice/drawing/render/effectlayer/ShapeEffectsLayer.java
use crate::*;

pub struct ShapeEffectsLayer {
    // TODO: fields
}

impl ShapeEffectsLayer {
    pub fn new() -> Self { todo!() }
    pub fn support(shape: Box<dyn RenderModel>, env: ShapeEnv) -> bool { todo!() }
    pub fn onDraw(&self) { todo!() }
    pub fn draw_innnerShadowFlow_sameScaled(&self) { todo!() }
    pub fn draw_innerShadowFlow_diffScaled(&self) { todo!() }
    pub fn draw_outerShadowFlow_sameScaled(&self) { todo!() }
    pub fn applyTextOrigin(&self, textLayer: EffectLayerBase) { todo!() }
    pub fn draw_outerShadowFlow_diffScaled(&self) { todo!() }
    pub fn blendToContainerForLayer(&self, baseOp: EffectLayerBase::BaseOperation, firstBlendBitmap: Box<dyn Bitmap>, firstMode: Xfermode, secondBaseOp: EffectLayerBase::BaseOperation, secondBlendBitmap: Box<dyn Bitmap>, secondMode: Xfermode) { todo!() }
    pub fn onBegin(&self) -> bool { todo!() }
    pub fn softEdge(&self, srcBitmap: Box<dyn Bitmap>) { todo!() }
    pub fn createErosionMask(&self, srcBitmap: Box<dyn Bitmap>) -> Box<dyn Bitmap> { todo!() }
    pub fn shrinkCircleSpan(&self, scrBitmap: Box<dyn Bitmap>, radius: i32) -> Box<dyn Bitmap> { todo!() }
    pub fn generateStdCirclePane(&self, circlePane: ArrayList, radius: i32) { todo!() }
    pub fn createGlow(&self, srcBitmap: Box<dyn Bitmap>) -> Box<dyn Bitmap> { todo!() }
    pub fn maximizeOpacity(&self, srcBimatp: Box<dyn Bitmap>) { todo!() }
    pub fn overLayInnerShadowTo(&self, srcBitmap: Box<dyn Bitmap>) { todo!() }
    pub fn createInnerShadow(&self, srcBitmap: Box<dyn Bitmap>) -> Box<dyn Bitmap> { todo!() }
    pub fn invertBitmap(&self, bitmap: Box<dyn Bitmap>) -> Box<dyn Bitmap> { todo!() }
    pub fn translateInnerShadow(&self, shadowBitmap: Box<dyn Bitmap>, paint: Paint, dx: f32, dy: f32, offsetX: i32, offsetY: i32, blurRadius: f32) -> Box<dyn Bitmap> { todo!() }
}
