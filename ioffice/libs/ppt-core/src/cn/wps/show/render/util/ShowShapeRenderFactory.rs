// Rust stub for ShowShapeRenderFactory (auto-generated)
// Original: cn/wps/show/render/util/ShowShapeRenderFactory.java
use crate::*;

pub struct ShowShapeRenderFactory {
    // TODO: fields
}

impl ShowShapeRenderFactory {
    pub fn new() -> Self { todo!() }
    pub fn CreateShowShapeRender(typoShape: ShowTypoShape, shapeEnv: ShowShapeEnv) -> IShowShapeRender { todo!() }
    pub fn CreateShowShapeRender_1(shapeBase: KmoShowShape, shapeCtx: Box<dyn ISlide>, canvas: Canvas, interpretCtx: InterpretCtx, drawFlags: i32) -> IShowShapeRender { todo!() }
    pub fn CreateShowShapeRender_2(shapeBase: KmoShowShape, interpretCtx: InterpretCtx, shapeEnv: ShowShapeEnv, geometry: RectF) -> IShowShapeRender { todo!() }
    pub fn CreateGraphicFrameRender(typoShape: ShowTypoShape, shapeEnv: ShapeEnv, shapeContext: Box<dyn ISlide>, drawFlags: i32) -> IShowShapeRender { todo!() }
    pub fn CreateShapeRender(typoShape: ShowTypoShape, shapeEnv: ShapeEnv, drawFlags: i32) -> IShowShapeRender { todo!() }
    pub fn createShapeEnv(canvas: Canvas, shapeContext: Box<dyn ISlide>, drawFlags: i32) -> ShowShapeEnv { todo!() }
    pub fn borrowShapeEnv(canvas: Canvas, slideRenderContext: SlideRenderContext) -> ShowShapeEnv { todo!() }
    pub fn returnShapeEnv(env: ShowShapeEnv) { todo!() }
    pub fn borrowTypobackground(slide: Box<dyn ISlide>, background: Background, ctx: InterpretCtx, width: f32, height: f32) -> ShowTypoBackground { todo!() }
    pub fn returnTypobackground(typoBackground: ShowTypoBackground) { todo!() }
    pub fn borrowTableFillRender(fill: Fill, interpretCtx: InterpretCtx, textRect: RectF, offset: PointF, env: ShapeEnv) -> TableFillRender { todo!() }
    pub fn returnTableFillRender(tableFillRender: TableFillRender) { todo!() }
    pub fn borrowBackgroundRender(bg: ShowTypoBackground, shapeEnv: ShowShapeEnv) -> BackgroundShapeRender { todo!() }
    pub fn returnBackgroundRender(backgroundShapeRender: BackgroundShapeRender) { todo!() }
}
