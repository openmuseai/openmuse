#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports)]

use super::super::PropBase::PropBase;
use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class LightRig extends PropBase
pub struct LightRig {
    pub(crate) base: PropBase,
}

// Java: public class LightRig extends PropBase
impl LightRig {
    // Java: LightRig()
    pub fn new() -> Self {
      // Java: super();
      LightRig { base: PropBase::new() }
    }

    // Java: void setLightDir(int dir)
    pub fn setLightDir(&mut self, dir: i32) {
      // Java: this.mProperty.setInt(399, dir);
      self.base.getProperty_mut().setInt(399, dir);
    }

    // Java: int getLightDir()
    pub fn getLightDir(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(399, 6);
      self.base.getProperty().getInt_default(399, 6)
    }

    // Java: void setLightPreset(int prst)
    pub fn setLightPreset(&mut self, prst: i32) {
      // Java: this.mProperty.setInt(400, prst);
      self.base.getProperty_mut().setInt(400, prst);
    }

    // Java: int getLightPreset()
    pub fn getLightPreset(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(400, 1);
      self.base.getProperty().getInt_default(400, 1)
    }

    // Java: void setRot(Rotation rot)
    pub fn setRot(&mut self, rot: Element) {
      // Java: this.mProperty.setValue(391, rot);
      self.base.setValue(391, rot);
    }

    // Java: Rotation getRot()
    pub fn getRot(&mut self) -> Element {
      // Java: Object val = this.mProperty.getValue(391);
      let val = self.base.getValue(391);
      // Java: return val != null ? (Rotation)val : null;
      val
    }

}
