// Rust stub for ColorInterpreter (auto-generated)
// Original: cn/wps/show/app/util/ColorInterpreter.java
use crate::*;

pub struct ColorInterpreter {
    // TODO: fields
}

impl ColorInterpreter {
    pub fn new() -> Self { todo!() }
    pub fn getHexRGBAValue(color: Color, ctx: ColorInterpretCtx) -> i32 { todo!() }
    pub fn getHexARGBValue(color: Color, ctx: ColorInterpretCtx) -> i32 { todo!() }
    pub fn ScRGBAToARGB(r: f64, g: f64, b: f64, a: f64) -> i32 { todo!() }
    pub fn getR(rgba: i32) -> i32 { todo!() }
    pub fn getG(rgba: i32) -> i32 { todo!() }
    pub fn getB(rgba: i32) -> i32 { todo!() }
    pub fn interpretRGBA(color: Color, ctx: ColorInterpretCtx, rgba: ColorSpace::RGBA) { todo!() }
    pub fn getRGBA(color: Color, rgba: ColorSpace::RGBA, ctx: ColorInterpretCtx) { todo!() }
    pub fn getRGBAEx(color: Color, rgba: ColorSpace::RGBA, ctx: ColorInterpretCtx) { todo!() }
    pub fn getNormalRGBA(color: Color, rgba: ColorSpace::RGBA, ctx: ColorInterpretCtx) { todo!() }
    pub fn applyTransforms(color: Color, rgba: ColorSpace::RGBA) { todo!() }
    pub fn applyTransforms_1(lst: List, rgba: ColorSpace::RGBA) { todo!() }
    pub fn applyTransform(op: i32, colorTransform: ColorTransform, rgba: ColorSpace::RGBA) { todo!() }
}
