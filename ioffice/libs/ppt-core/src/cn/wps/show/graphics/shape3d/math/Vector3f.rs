// Rust stub for Vector3f (auto-generated)
// Original: cn/wps/show/graphics/shape3d/math/Vector3f.java
use crate::*;

pub struct Vector3f {
    // TODO: fields
}

impl Vector3f {
    pub fn new() -> Self { todo!() }
    pub fn zero(&self) { todo!() }
    pub fn isZero(&self) -> bool { todo!() }
    pub fn notZero(&self) -> bool { todo!() }
    pub fn createArray(size: i32) -> Vec<Vector3f> { todo!() }
    pub fn set(&self, x: f32, y: f32, z: f32) -> Vector3f { todo!() }
    pub fn set_1(&self, x: f32, y: f32) -> Vector3f { todo!() }
    pub fn set_2(&self, v: Vector3f) -> Vector3f { todo!() }
    pub fn set_3(&self, f: f32) -> Vector3f { todo!() }
    pub fn module(&self) -> f32 { todo!() }
    pub fn sub(&self, v: Vector3f) -> Vector3f { todo!() }
    pub fn subZ(&self, z: f32) -> Vector3f { todo!() }
    pub fn sub_1(&self, x: f32, y: f32, z: f32) -> Vector3f { todo!() }
    pub fn add(&self, v: Vector3f) -> Vector3f { todo!() }
    pub fn add_1(&self, x: f32, y: f32, z: f32) -> Vector3f { todo!() }
    pub fn add_2(&self, x: f32, y: f32) -> Vector3f { todo!() }
    pub fn scale(&self, s: f32) -> Vector3f { todo!() }
    pub fn scale_1(&self, xs: f32, ys: f32) -> Vector3f { todo!() }
    pub fn normalize(&self) -> Vector3f { todo!() }
    pub fn normalize_1(&self, dest: Vector3f) { todo!() }
    pub fn dotProduct(&self, v: Vector3f) -> f32 { todo!() }
    pub fn crossProduct(&self, v: Vector3f) -> Vector3f { todo!() }
    pub fn multiply(sp: Vector3f, op: Vector3f, ep: Vector3f) -> f32 { todo!() }
    pub fn toString_function toString() { [native code] }(&self) -> String { todo!() }
    pub fn length(p1: Vector3f, p2: Vector3f) -> f32 { todo!() }
    pub fn equals(&self, v: Vector3f) -> bool { todo!() }
    pub fn clearPool() { todo!() }
    pub fn obtain() -> Vector3f { todo!() }
    pub fn recycle(&self) { todo!() }
}
