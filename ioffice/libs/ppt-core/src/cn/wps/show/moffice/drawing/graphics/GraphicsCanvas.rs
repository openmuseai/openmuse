// Rust stub for GraphicsCanvas (auto-generated)
// Original: cn/wps/show/moffice/drawing/graphics/GraphicsCanvas.java
use crate::*;

pub struct GraphicsCanvas {
    // TODO: fields
}

impl GraphicsCanvas {
    pub fn new() -> Self { todo!() }
    pub fn getmMediaLib(&self) -> Box<dyn IMediaLib> { todo!() }
    pub fn setMediaLib(&self, mediaLib: Box<dyn IMediaLib>) { todo!() }
    pub fn getGraphicspath(&self) -> GraphicsPath { todo!() }
    pub fn getGraphicsFillTool(&self) -> GraphicsFillTool { todo!() }
    pub fn setCanvas(&self, canvas: Canvas) { todo!() }
    pub fn setCanvas_1(&self, canvas: Canvas, minStrokeWidth: f32) { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn init(&self, rotWithShape: bool, canDistortion: bool, multithreading: bool) { todo!() }
    pub fn setShadow(&self, shadow: Shadow) { todo!() }
    pub fn setGlow(&self, glow: Glow) { todo!() }
    pub fn setFill(&self, fill: FillBase) { todo!() }
    pub fn setStroke(&self, stroke: LineProperty) { todo!() }
    pub fn setFlip(&self, b: bool, b1: bool) { todo!() }
    pub fn draw(&self, path: Path, rectF: RectF, fillType: i32) { todo!() }
    pub fn drawPicture(&self, pic: Picture, x: f32, y: f32, width: f32, height: f32, rotation: f32) { todo!() }
    pub fn drawPath(&self, path: GraphicsPath, rotation: f32, rectF: RectF, bCache: bool) { todo!() }
    pub fn drawPath_1(&self, path: GraphicsPath, rotation: f32, rectF: RectF) { todo!() }
    pub fn clipPath(&self, o: Box<dyn std::any::Any + Send + Sync>) { todo!() }
    pub fn drawBitmap(&self, o: Box<dyn std::any::Any + Send + Sync>, o1: Box<dyn std::any::Any + Send + Sync>, o2: Box<dyn std::any::Any + Send + Sync>, o3: Box<dyn std::any::Any + Send + Sync>) { todo!() }
    pub fn drawRect(&self, o: Box<dyn std::any::Any + Send + Sync>, o1: Box<dyn std::any::Any + Send + Sync>) { todo!() }
    pub fn drawPath_2(&self, path: GraphicsPath, rotation: f32) { todo!() }
    pub fn getException(&self) -> Box<dyn IGraphicsException> { todo!() }
    pub fn setException(&self, exception: Box<dyn IGraphicsException>) { todo!() }
    pub fn save(&self) { todo!() }
    pub fn restore(&self) { todo!() }
    pub fn translate(&self, dx: f32, dy: f32) { todo!() }
    pub fn scale(&self, sx: f32, sy: f32) { todo!() }
    pub fn scale_1(&self, sx: f32, sy: f32, px: f32, py: f32) { todo!() }
    pub fn rotate(&self, degrees: f32) { todo!() }
    pub fn rotate_1(&self, degrees: f32, px: f32, py: f32) { todo!() }
    pub fn getCanvas(&self) -> Box<dyn std::any::Any + Send + Sync> { todo!() }
    pub fn isInstractRender(&self) -> bool { todo!() }
    pub fn concat(&self, values: Vec<f32>) { todo!() }
    pub fn skew(&self, sx: f32, sy: f32) { todo!() }
    pub fn setRoatation(&self, rot: f32) { todo!() }
    pub fn setPatternScale(&self, patternScale: f32) { todo!() }
    pub fn getPatternScale(&self) -> f32 { todo!() }
    pub fn setVisibleRect(&self, rectF: RectF) { todo!() }
    pub fn getMediaLib(&self) -> Box<dyn IMediaLib> { todo!() }
    pub fn setRenderEvent(&self, renderEvent: IRenderEvent) { todo!() }
    pub fn renderEvent(&self) -> IRenderEvent { todo!() }
    pub fn getGraphicsFlags(&self) -> GraphicsFlags { todo!() }
    pub fn setPrintMode(&self, b: bool) { todo!() }
    pub fn getPrintMode(&self) -> bool { todo!() }
    pub fn setFastDraw(&self, b: bool) { todo!() }
    pub fn setDrawHigh(&self, b: bool) { todo!() }
    pub fn isFastDraw(&self) -> bool { todo!() }
    pub fn setDelayLoadBitmap(&self, b: bool) { todo!() }
    pub fn delayLoadBitmap(&self) -> bool { todo!() }
    pub fn setUseLowQualityBitmap(&self, b: bool) { todo!() }
    pub fn useLowQualityBitmap(&self) -> bool { todo!() }
    pub fn setFillNotScaleWithDenisty(&self, b: bool) { todo!() }
    pub fn isFillNotScaleWithDenisty(&self) -> bool { todo!() }
    pub fn setDrawSlideThumb(&self, b: bool) { todo!() }
    pub fn isDrawSlideThumb(&self) -> bool { todo!() }
    pub fn setDraw3DFilter(&self, filter: Box<dyn IDraw3DFilter>) { todo!() }
    pub fn getDraw3DFilter(&self) -> Box<dyn IDraw3DFilter> { todo!() }
}
