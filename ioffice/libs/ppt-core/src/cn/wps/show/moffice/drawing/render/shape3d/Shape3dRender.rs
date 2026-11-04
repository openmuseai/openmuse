// Rust stub for Shape3dRender (auto-generated)
// Original: cn/wps/show/moffice/drawing/render/shape3d/Shape3dRender.java
use crate::*;

pub struct Shape3dRender {
    // TODO: fields
}

impl Shape3dRender {
    pub fn new() -> Self { todo!() }
    pub fn support(shape: Box<dyn RenderModel>, env: ShapeEnv) -> bool { todo!() }
    pub fn hasShape3d(shape: Box<dyn RenderModel>) -> bool { todo!() }
    pub fn render(env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) { todo!() }
    pub fn getTimeLog(&self) -> String { todo!() }
    pub fn init(&self, env: ShapeEnv, baseDrawing: Box<dyn IBaseDrawing>) { todo!() }
    pub fn drawEffectsForGroup(&self) { todo!() }
    pub fn draw(&self, canvas: Canvas) { todo!() }
    pub fn replaceForParentEffect(&self, parent: Box<dyn RenderModel>) { todo!() }
    pub fn restoreSelfEffect(&self) { todo!() }
    pub fn drawEffect(&self, foureDrawOriginIgnore: bool) { todo!() }
    pub fn draw3DShape(&self, canvas: Canvas) { todo!() }
    pub fn drawEffectBound(&self, canvas: Canvas) { todo!() }
    pub fn dispose(&self) { todo!() }
    pub fn getTexture(&self) -> Box<dyn ITexture> { todo!() }
    pub fn getRegionInfo(&self) -> Box<dyn I3DShapeRegion> { todo!() }
    pub fn getHullPath(&self) -> Box<dyn IHullPath> { todo!() }
    pub fn getCore2DDrawing(&self) -> Box<dyn ICore2DDrawing> { todo!() }
    pub fn hasAlpha(&self) -> bool { todo!() }
    pub fn isCancasCompatible(&self) -> bool { todo!() }
    pub fn drawFillAndStroke(&self, destCanvas: Canvas) { todo!() }
    pub fn setDraw3DFilter(&self, filter: Box<dyn IDraw3DFilter>) { todo!() }
    pub fn getTextRect(&self, rect: RectF) -> bool { todo!() }
    pub fn saveCanvasScale(&self, canvas: Canvas) { todo!() }
    pub fn resetCanvasScale(&self) { todo!() }
    pub fn getDraw3DFilter(&self) -> Box<dyn IDraw3DFilter> { todo!() }
    pub fn setLightFilterInfo(&self, faceNoraml: Vector3f) { todo!() }
    pub fn enableLightTransInner(&self, enable: bool) { todo!() }
    pub fn isLightTransInnerEnable(&self) -> bool { todo!() }
    pub fn transLightColor(&self, color: i32) -> i32 { todo!() }
    pub fn getLightColorFilter(&self) -> Vec<f32> { todo!() }
    pub fn isLineStyle(&self) -> bool { todo!() }
    pub fn isKeepShapeGradFill(&self) -> bool { todo!() }
    pub fn needLightForMainFace(&self) -> bool { todo!() }
    pub fn checkCanLightColorFilterInner(&self) -> bool { todo!() }
    pub fn drawText(&self, canvas: Canvas, setDrawTagOnly: bool) { todo!() }
    pub fn checkUseOpengl(&self) -> bool { todo!() }
    pub fn addTag(&self, tag: i32) { todo!() }
    pub fn removeTag(&self, tag: i32) { todo!() }
    pub fn checktTag(&self, tag: i32) -> bool { todo!() }
}
