// Rust stub for EffectLayerRender (auto-generated)
// Original: cn/wps/show/moffice/drawing/render/effectlayer/EffectLayerRender.java
use crate::*;

pub struct EffectLayerRender {
    // TODO: fields
}

impl EffectLayerRender {
    pub fn new() -> Self { todo!() }
    pub fn clear() { todo!() }
    pub fn renderShapeEffects(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> bool { todo!() }
    pub fn renderReflection(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> bool { todo!() }
    pub fn render3DReflectionEffects(model: ModelBase, env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> bool { todo!() }
    pub fn renderGroupEffects(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> bool { todo!() }
    pub fn renderGroupReflectioin(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> bool { todo!() }
    pub fn renderContainerEffects(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>, r#type: ContainerEffectsLayer::ContainerType) -> bool { todo!() }
    pub fn renderTextLayerEffects(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> bool { todo!() }
    pub fn createTextRenderLayer(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> TextRenderLayer { todo!() }
    pub fn renderOuterShadowEffects(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> bool { todo!() }
    pub fn render3DShapeEffects(model: ModelBase, env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) -> bool { todo!() }
    pub fn getBound(env: ShapeEnv, typoShape: ITypoShape) -> RectF { todo!() }
    pub fn pushOneMatrix(env: ShapeEnv, typoShape: ITypoShape) { todo!() }
    pub fn getCacheBoundRegion() -> EffectsRegion { todo!() }
    pub fn reuseBoundRegion(region: EffectsRegion) { todo!() }
    pub fn allocateOrNew(tClass: Box<dyn std::any::Any + Send + Sync>) ->  { todo!() }
    pub fn reuseLayer(val: EffectLayerBase) -> bool { todo!() }
}
