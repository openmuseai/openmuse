// Rust stub for DgCanvas (auto-generated)
// Original: cn/wps/drawing/app/DgCanvas.java
use crate::*;

pub struct DgCanvas {
    // TODO: fields
}

impl DgCanvas {
    pub fn new() -> Self { todo!() }
    pub fn addEmptyShape(&self, r#type: i32) -> i32 { todo!() }
    pub fn addPic(&self, picMediaId: i32, top: i32, left: i32, width: i32, height: i32) -> i32 { todo!() }
    pub fn addPic_1(&self, picMediaId: i32, rotation: i32, top: i32, left: i32, width: i32, height: i32) -> i32 { todo!() }
    pub fn addInk(&self, inkId: i32, top: i32, left: i32, width: i32, height: i32) -> i32 { todo!() }
    pub fn replacePicture2PlaceHolder(&self, picMediaId: i32, phId: i32, left: f32, right: f32, top: f32, bottom: f32) -> i32 { todo!() }
    pub fn addPresetShape(&self, isTextBox: bool, presetType: i32, fillColor: i32, lineColor: i32, lineWidth: i32, left: i32, top: i32, width: i32, height: i32, text: String, fontColor: i32, fontSize: i32, flipH: bool, flipV: bool) -> i32 { todo!() }
    pub fn addTable(&self, styleId: String, colCount: i32, rowCount: i32, left: i32, top: i32, width: i32, height: i32) -> i32 { todo!() }
    pub fn addConnector(&self, connectorType: i32, fillColor: i32, lineColor: i32, lineWidth: i32, left: i32, top: i32, width: i32, height: i32, hasBeginArrow: bool, hasEndArrow: bool, hasFlipH: bool, hasFlipV: bool) -> i32 { todo!() }
    pub fn addChart(&self, left: i32, top: i32, width: i32, height: i32) -> i32 { todo!() }
    pub fn addCurveLine(&self, points: List, bounds: Rect, fillColor: i32, lineColor: i32, lineWidth: i32, isQuad: bool) -> i32 { todo!() }
    pub fn getPoint(&self, x: f32, y: f32) -> AdjPoint2D { todo!() }
    pub fn copyOf(&self, canvas: DgCanvas) { todo!() }
}
