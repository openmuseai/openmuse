#![allow(non_snake_case)]
use super::SchemeClr::java_hash;

// Java: public class Cxn
#[derive(Default)]
pub struct Cxn { pub mAng: Option<i32>, pub mX: Option<i32>, pub mY: Option<i32> }
impl Cxn {
    // Java: implicit Cxn()
    pub fn new() -> Self { Self::default() }
    // Java: public void setXY(String x, String y)
    pub fn setXY(&mut self, x: &str, y: &str) { self.mX = Some(java_hash(x)); self.mY = Some(java_hash(y)); }
    // Java: public void setAng(String ang)
    pub fn setAng(&mut self, ang: &str) { self.mAng = Some(java_hash(ang)); }
}
