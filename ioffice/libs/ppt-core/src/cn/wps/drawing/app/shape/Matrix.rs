// Rust stub for Matrix (auto-generated)
// Original: cn/wps/drawing/app/shape/Matrix.java
use crate::*;

pub struct Matrix {
    // TODO: fields
}

impl Matrix {
    pub fn new() -> Self { todo!() }
    pub fn constructWithCopy(A: Vec<Vec<f64>>) -> Matrix { todo!() }
    pub fn copy(&self) -> Matrix { todo!() }
    pub fn clone(&self) -> Box<dyn std::any::Any + Send + Sync> { todo!() }
    pub fn getArray(&self) -> Vec<Vec<f64>> { todo!() }
    pub fn getArrayCopy(&self) -> Vec<Vec<f64>> { todo!() }
    pub fn getColumnPackedCopy(&self) -> Vec<f64> { todo!() }
    pub fn getRowPackedCopy(&self) -> Vec<f64> { todo!() }
    pub fn getRowDimension(&self) -> i32 { todo!() }
    pub fn getColumnDimension(&self) -> i32 { todo!() }
    pub fn get(&self, i: i32, j: i32) -> f64 { todo!() }
    pub fn getMatrix(&self, i0: i32, i1: i32, j0: i32, j1: i32) -> Matrix { todo!() }
    pub fn getMatrix_1(&self, r: Vec<i32>, c: Vec<i32>) -> Matrix { todo!() }
    pub fn getMatrix_2(&self, i0: i32, i1: i32, c: Vec<i32>) -> Matrix { todo!() }
    pub fn getMatrix_3(&self, r: Vec<i32>, j0: i32, j1: i32) -> Matrix { todo!() }
    pub fn set(&self, i: i32, j: i32, s: f64) { todo!() }
    pub fn setMatrix(&self, X: Matrix) { todo!() }
    pub fn setMatrix_1(&self, i0: i32, i1: i32, j0: i32, j1: i32, X: Matrix) { todo!() }
    pub fn setMatrix_2(&self, r: Vec<i32>, c: Vec<i32>, X: Matrix) { todo!() }
    pub fn setMatrix_3(&self, r: Vec<i32>, j0: i32, j1: i32, X: Matrix) { todo!() }
    pub fn setMatrix_4(&self, i0: i32, i1: i32, c: Vec<i32>, X: Matrix) { todo!() }
    pub fn transpose(&self) -> Matrix { todo!() }
    pub fn norm1(&self) -> f64 { todo!() }
    pub fn normInf(&self) -> f64 { todo!() }
    pub fn uminus(&self) -> Matrix { todo!() }
    pub fn plus(&self, B: Matrix) -> Matrix { todo!() }
    pub fn plusEquals(&self, B: Matrix) -> Matrix { todo!() }
    pub fn minus(&self, B: Matrix) -> Matrix { todo!() }
    pub fn minusEquals(&self, B: Matrix) -> Matrix { todo!() }
    pub fn arrayTimes(&self, B: Matrix) -> Matrix { todo!() }
    pub fn arrayTimesEquals(&self, B: Matrix) -> Matrix { todo!() }
    pub fn arrayRightDivide(&self, B: Matrix) -> Matrix { todo!() }
    pub fn arrayRightDivideEquals(&self, B: Matrix) -> Matrix { todo!() }
    pub fn arrayLeftDivide(&self, B: Matrix) -> Matrix { todo!() }
    pub fn arrayLeftDivideEquals(&self, B: Matrix) -> Matrix { todo!() }
    pub fn times(&self, s: f64) -> Matrix { todo!() }
    pub fn timesEquals(&self, s: f64) -> Matrix { todo!() }
    pub fn times_1(&self, B: Matrix) -> Matrix { todo!() }
    pub fn trace(&self) -> f64 { todo!() }
    pub fn random(m: i32, n: i32) -> Matrix { todo!() }
    pub fn identity(m: i32, n: i32) -> Matrix { todo!() }
    pub fn checkMatrixDimensions(&self, B: Matrix) { todo!() }
    pub fn reset(&self) { todo!() }
    pub fn preConcat(&self, worldTransform: Matrix) { todo!() }
    pub fn postConcat(&self, other: Matrix) { todo!() }
    pub fn preScale(&self, sx: f32, sy: f32) { todo!() }
    pub fn preTranslate(&self, dx: f32, dy: f32) { todo!() }
    pub fn preRotate(&self, degrees: f32) { todo!() }
    pub fn mapPoint(&self, pts: Vec<f32>) { todo!() }
    pub fn mapPoint_1(&self, x: f32, y: f32) -> PointF { todo!() }
    pub fn mapRadius(&self, radius: f32) -> f32 { todo!() }
    pub fn mapRect(&self, destination: RectF, source: RectF) -> bool { todo!() }
    pub fn transform(&self, x: f64, y: f64) -> PointF { todo!() }
    pub fn length(&self) -> f64 { todo!() }
}
