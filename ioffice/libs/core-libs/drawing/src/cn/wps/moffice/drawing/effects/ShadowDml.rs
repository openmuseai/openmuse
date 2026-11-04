#![allow(non_snake_case, dead_code)]

use std::sync::Arc;
use std::any::Any;

use super::Shadow::Shadow;
use super::ShadowPos::ShadowPos;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class ShadowDml extends Shadow
pub struct ShadowDml {
    pub base: Shadow,
}

// Java: public class ShadowDml extends Shadow
impl ShadowDml {
    // Java: ShadowDml()
    pub fn new() -> Self {
      // Java: super();
      let mut sdml = ShadowDml { base: Shadow::new() };
      // Java: this.setDml(true);
      sdml.base.setDml(true);
      sdml
    }

    // Java: void setShadowPrstType(int type)
    pub fn setShadowPrstType(&mut self, r#type: i32) {
      // Java: this.mProperty.setInt(208, type);
      if let Some(ref mut prop) = self.base.base.base.mProperty {
          prop.setInt(208, r#type);
      }
    }

    // Java: Integer getShadowPrstTypeObj()
    pub fn getShadowPrstTypeObj(&self) -> Option<i32> {
      // Java: Object object = this.mProperty.getValue(208);
      // Java: return object == null ? null : (Integer)object;
      if let Some(ref prop) = self.base.base.base.mProperty {
          prop.base.getValue(208).and_then(|arc| arc.downcast_ref::<i32>().copied())
      } else {
          None
      }
    }

    // Java: void setShadowDir(float dir)
    pub fn setShadowDir(&mut self, dir: f32) {
      // Java: this.mProperty.setFloat(203, dir);
      if let Some(ref mut prop) = self.base.base.base.mProperty {
          prop.setFloat(203, dir);
      }
    }

    // Java: float getShadowDir()
    pub fn getShadowDir(&self) -> f32 {
      // Java: return this.mProperty.getFloat(203, 0.0F);
      if let Some(ref prop) = self.base.base.base.mProperty {
          prop.base.getFloat_default(203, 0.0_f32)
      } else {
          0.0_f32
      }
    }

    // Java: void setShadowDist(float dist)
    pub fn setShadowDist(&mut self, dist: f32) {
      // Java: this.mProperty.setFloat(204, dist);
      if let Some(ref mut prop) = self.base.base.base.mProperty {
          prop.setFloat(204, dist);
      }
    }

    // Java: float getShadowDist()
    pub fn getShadowDist(&self) -> f32 {
      // Java: return this.mProperty.getFloat(204, 0.0F);
      if let Some(ref prop) = self.base.base.base.mProperty {
          prop.base.getFloat_default(204, 0.0_f32)
      } else {
          0.0_f32
      }
    }

    // Java: void setShadowAlgn(int algn)
    pub fn setShadowAlgn(&mut self, algn: i32) {
      // Java: this.mProperty.setInt(206, algn);
      if let Some(ref mut prop) = self.base.base.base.mProperty {
          prop.setInt(206, algn);
      }
    }

    // Java: int getShadowAlgn()
    pub fn getShadowAlgn(&self) -> i32 {
      // Java: return this.mProperty.getInt(206, 0);
      if let Some(ref prop) = self.base.base.base.mProperty {
          prop.base.getInt_default(206, 0)
      } else {
          0
      }
    }

    // Java: void setShadowPos(ShadowPos pos)
    pub fn setShadowPos(&mut self, pos: ShadowPos) {
      // Java: this.mProperty.setValue(207, pos);
      if let Some(ref mut prop) = self.base.base.base.mProperty {
          prop.setValue(207, Some(Arc::new(pos) as Arc<dyn Any + Send + Sync>));
      }
    }

    // Java: ShadowPos getShadowPos()
    pub fn getShadowPos(&self) -> Option<ShadowPos> {
      // Java: Object pos = this.mProperty.getValue(207);
      // Java: return pos != null ? (ShadowPos)pos : null;
      if let Some(ref prop) = self.base.base.base.mProperty {
          prop.base.getValue(207).and_then(|arc| arc.downcast_ref::<ShadowPos>().cloned())
      } else {
          None
      }
    }

}
