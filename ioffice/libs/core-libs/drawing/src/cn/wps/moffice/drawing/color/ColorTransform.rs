#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports)]

use super::IColorTransform::IColorTransform;

// Java: public class ColorTransform implements IColorTransform
pub struct ColorTransform {
    // Java field: int mType
    pub(crate) mType: i32,
    // Java field: double mVale
    pub(crate) mVale: f64,
}

// Java: public class ColorTransform implements IColorTransform
impl ColorTransform {
    // Java: ColorTransform(int type, double val)
    pub fn new(r#type: i32, val: f64) -> Self {
      // Java: super();
      // Java: this.mType = type;
      // Java: this.mVale = val;
      ColorTransform { mType: r#type, mVale: val }
    }

    // Java: int type()
    pub fn r#type(&self) -> i32 {
      // Java: return this.mType;
      self.mType
    }

    // Java: double value()
    pub fn value(&self) -> f64 {
      // Java: return this.mVale;
      self.mVale
    }

}

impl IColorTransform for ColorTransform {
    fn r#type(&self) -> i32 { self.mType }
    fn value(&self) -> f64 { self.mVale }
}
