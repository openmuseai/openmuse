#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports)]

use super::super::PropBase::PropBase;
use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Bevel extends PropBase
pub struct Bevel {
    pub(crate) base: PropBase,
}

// Java: public class Bevel extends PropBase
impl Bevel {
    // Java: Bevel()
    pub fn new() -> Self {
      // Java: super();
      Bevel { base: PropBase::new() }
    }

    // Java: void setHeight(float h)
    pub fn setHeight(&mut self, h: f32) {
      // Java: this.mProperty.setFloat(379, h);
      self.base.getProperty_mut().setFloat(379, h);
    }

    // Java: float getHeight()
    pub fn getHeight(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(379, 6.0F);
      self.base.getProperty_mut().getBase().getFloat_default(379, 6.0_f32)
    }

    // Java: Float getHeightObj()
    pub fn getHeightObj(&mut self) -> Option<f32> {
      // Java: Object val = this.mProperty.getValue(379);
      let val = self.base.getProperty_mut().getValue(379);
      // Java: return val != null ? (Float)val : null;
      val.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setWidth(float w)
    pub fn setWidth(&mut self, w: f32) {
      // Java: this.mProperty.setFloat(380, w);
      self.base.getProperty_mut().setFloat(380, w);
    }

    // Java: float getWidth()
    pub fn getWidth(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(380, 6.0F);
      self.base.getProperty_mut().getBase().getFloat_default(380, 6.0_f32)
    }

    // Java: Float getWidthObj()
    pub fn getWidthObj(&mut self) -> Option<f32> {
      // Java: Object val = this.mProperty.getValue(380);
      let val = self.base.getProperty_mut().getValue(380);
      // Java: return val != null ? (Float)val : null;
      val.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setPreset(int prst)
    pub fn setPreset(&mut self, prst: i32) {
      // Java: this.mProperty.setInt(381, prst);
      self.base.getProperty_mut().setInt(381, prst);
    }

    // Java: int getPreset()
    pub fn getPreset(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(381, 0);
      self.base.getProperty_mut().getBase().getInt_default(381, 0)
    }

}
