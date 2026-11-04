// Rust stub for ShapeEnv (auto-generated)
// Original: cn/wps/show/moffice/drawing/render/ShapeEnv.java
use crate::*;

pub struct ShapeEnv {
    // TODO: fields
}

impl ShapeEnv {
    pub fn new() -> Self { todo!() }
    pub fn clone(&self) -> ShapeEnv { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn setRenderMode(&self, mode: i32) { todo!() }
    pub fn renderMode(&self) -> i32 { todo!() }
    pub fn getRectF(&self) -> RectF { todo!() }
    pub fn getShape(&self) -> Box<dyn RenderModel> { todo!() }
    pub fn setVisibleRectF(&self, rectF: RectF) { todo!() }
    pub fn getVisibleRectF(&self) -> RectF { todo!() }
    pub fn setCanvas(&self, canvas: Canvas) { todo!() }
    pub fn getCanvas(&self) -> Canvas { todo!() }
    pub fn setGraphics(&self, graphics: Box<dyn IGraphics>) { todo!() }
    pub fn getGraphics(&self) -> Box<dyn IGraphics> { todo!() }
    pub fn setRIDMap(&self, idMap: DrawingRIDMap) { todo!() }
    pub fn getRIDMap(&self) -> DrawingRIDMap { todo!() }
    pub fn setCanDistortion(&self, canDistortion: bool) { todo!() }
    pub fn canDistortion(&self) -> bool { todo!() }
    pub fn setMultithread(&self, multithread: bool) { todo!() }
    pub fn getMultithread(&self) -> bool { todo!() }
    pub fn setRenderEvent(&self, renderEvent: IRenderEvent) { todo!() }
    pub fn getRenderEvent(&self) -> IRenderEvent { todo!() }
    pub fn setTextRender(&self, render: IShapeRender) { todo!() }
    pub fn getTextRender(&self) -> IShapeRender { todo!() }
    pub fn setDecorRender(&self, render: IShapeRender) { todo!() }
    pub fn getDecorRender(&self) -> IShapeRender { todo!() }
    pub fn getRotate(&self) -> f32 { todo!() }
    pub fn setDrawClipBack(&self, bBack: bool) { todo!() }
    pub fn getDrawClipBack(&self) -> bool { todo!() }
    pub fn setTypoShape(&self, typoShape: ITypoShape) { todo!() }
    pub fn getTypoShape(&self) -> ITypoShape { todo!() }
    pub fn setDrawBalloon(&self, balloon: bool) { todo!() }
    pub fn beginTask(&self, raiseExption: bool) { todo!() }
    pub fn endTask(&self) { todo!() }
    pub fn interrup(&self) { todo!() }
    pub fn lastIsInterrup(&self) -> bool { todo!() }
    pub fn checkInterrup(&self) -> bool { todo!() }
    pub fn setDrawEffectsLayer(&self, b: bool) { todo!() }
    pub fn isDrawEffectsLayer(&self) -> bool { todo!() }
    pub fn setDrawShadow(&self, b: bool) { todo!() }
    pub fn isDrawShadow(&self) -> bool { todo!() }
    pub fn isDrawShape3d(&self) -> bool { todo!() }
    pub fn setDrawShape3d(&self, b: bool) { todo!() }
    pub fn setTextDrawn(&self, textDrawn: bool) { todo!() }
    pub fn getTextDrawn(&self) -> bool { todo!() }
    pub fn isDrawModularDefaultPic(&self) -> bool { todo!() }
    pub fn setDrawModularDefaultPic(&self, mDrawModularDefaultPic: bool) { todo!() }
    pub fn getShapeMatrixInfo(&self) -> ShapeMatrixInfo { todo!() }
    pub fn push(&self, matrix: ShapeMatrix) { todo!() }
    pub fn pop(&self) { todo!() }
    pub fn getShapeMatrix(&self) -> List { todo!() }
    pub fn calcRotation(&self) -> f32 { todo!() }
    pub fn calcFlipH(&self) -> bool { todo!() }
    pub fn calcFlipV(&self) -> bool { todo!() }
    pub fn clear(&self) { todo!() }
    pub fn obtain() -> ShapeMatrix { todo!() }
    pub fn recycle(&self) { todo!() }
    pub fn clear_1(&self) { todo!() }
}
