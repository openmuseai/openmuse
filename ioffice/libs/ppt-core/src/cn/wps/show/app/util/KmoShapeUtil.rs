// Rust stub for KmoShapeUtil (auto-generated)
// Original: cn/wps/show/app/util/KmoShapeUtil.java
use crate::*;

pub struct KmoShapeUtil {
    // TODO: fields
}

impl KmoShapeUtil {
    pub fn new() -> Self { todo!() }
    pub fn calcShapeRect(shape: KmoShowShape, rcShape: RectF) { todo!() }
    pub fn calcShapeRotation(shape: KmoShowShape, containSelf: bool) -> i32 { todo!() }
    pub fn calcShapeFlip(shape: KmoShowShape, flip: Vec<i32>, containSelf: bool) { todo!() }
    pub fn calcShape2TopMatrix(shape: KmoShowShape, matrix: Matrix, rotate: bool) -> RectF { todo!() }
    pub fn calcShapeMatrix(matrix: Matrix, rcShape: RectF, rotation: f32, flipH: bool, flipV: bool) { todo!() }
    pub fn calcChildRelativeRect(group: KmoShowShape, rcGroup: RectF, rcChild: RectF, childRotation: f32) { todo!() }
}
