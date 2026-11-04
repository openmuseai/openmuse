#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.PropBase
use super::super::PropBase::PropBase;

use std::ops::{Deref, DerefMut};

// Java: public class Effect extends PropBase
pub struct Effect {
    pub base: PropBase,
}

impl DerefMut for Effect {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.base
    }
}

impl Deref for Effect {
    type Target = PropBase;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class Effect extends PropBase
impl Effect {
    // Java: Effect()
    pub fn new() -> Self {
      // Java: super();
      // LLM JavaLine Aggregation: super() creates parent PropBase, merged into struct init
      Effect { base: PropBase::new() }
    }

    // Java: Effect clone()
    pub fn clone(&mut self) -> Effect {
      // Java: return (Effect)super.clone();
      Effect { base: self.base.clone() }
    }

    // Java: void setBlurRadius(float radius)
    pub fn setBlurRadius(&mut self, radius: f32) {
      // Java: this.mProperty.setFloat(142, radius);
      self.base.mProperty.as_mut().unwrap().setFloat(142, radius);
    }

    // Java: float getBlurRadius()
    pub fn getBlurRadius(&self) -> f32 {
      // Java: return this.mProperty.getFloat(142, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(142, 0.0_f32)
    }

}
