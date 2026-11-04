// Rust stub for ShapeTool (auto-generated)
// Original: cn/wps/show/app/shape/ShapeTool.java
use crate::*;

pub struct ShapeTool {
    // TODO: fields
}

impl ShapeTool {
    pub fn new() -> Self { todo!() }
    pub fn getPenKitName(brand: String) -> String { todo!() }
    pub fn setShape2Bitmap(shape2Bitmap: Box<dyn IShape2Bitmap>) { todo!() }
    pub fn getShape2Bitmap() -> Box<dyn IShape2Bitmap> { todo!() }
    pub fn getSmoothLine(pts: Vec<f32>) -> Vec<f32> { todo!() }
    pub fn getSmoothPath(pts: Vec<f32>) -> Path { todo!() }
    pub fn getGraphicsPath(pts: Vec<f32>, isSmooth: bool, isClose: bool) -> GraphicsPath { todo!() }
    pub fn getSmoothLineLst(pts: Vec<f32>) -> List { todo!() }
    pub fn scalePts(pts: Vec<f32>, scale: f32) { todo!() }
    pub fn getLinesBounds(pts: Vec<f32>) -> RectF { todo!() }
    pub fn getQuadsBounds(pts: Vec<f32>) -> RectF { todo!() }
    pub fn getFitRect(path: String, slideWidth: i32, slideHeight: i32) -> Rect { todo!() }
    pub fn getFitRect_1(rect: Rect, slideWidth: i32, slideHeight: i32) -> Rect { todo!() }
    pub fn getDefaultOleFile(rootDir: String, fileExt: String) -> String { todo!() }
    pub fn getDefaultPenkit(rootPath: String) -> String { todo!() }
    pub fn isHuaweiOrHonorPenkitData(penKitFileName: String) -> bool { todo!() }
    pub fn getDefaultPenkitIcon(rootPath: String) -> String { todo!() }
    pub fn addMediaData(pres: KmoPresentation, mediaPath: String) -> i32 { todo!() }
    pub fn addPictureData(pres: KmoPresentation, path: String, rotation: i32, left: i32, top: i32, width: i32, height: i32) -> i32 { todo!() }
    pub fn addOleObject(slide: Box<dyn ISlide>, filePath: String, picPath: String, shapeId: i32, oleName: String, progId: String, isNew: bool) { todo!() }
}
