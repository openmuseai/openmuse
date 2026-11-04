#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.PropBase
use crate::cn::wps::moffice::drawing::PropBase::PropBase;
use super::TileFlipMode::TileFlipMode;

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class TileInfo extends PropBase
#[derive(Clone)]
pub struct TileInfo {
    pub base: PropBase,
}

// Java: public class TileInfo extends PropBase
impl TileInfo {
    // Java: TileInfo()
    pub fn new() -> Self {
      // Java: super();
      TileInfo { base: PropBase::new() }
    }

    // Java: float getOffsetX()
    pub fn getOffsetX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(508, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(508, 0.0_f32)
    }

    // Java: void setOffsetX(float offsetX)
    pub fn setOffsetX(&mut self, offsetX: f32) {
      // Java: this.mProperty.setFloat(508, offsetX);
      self.base.mProperty.as_mut().unwrap().setFloat(508, offsetX);
    }

    // Java: float getOffsetY()
    pub fn getOffsetY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(509, 0.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(509, 0.0_f32)
    }

    // Java: void setOffsetY(float offsetY)
    pub fn setOffsetY(&mut self, offsetY: f32) {
      // Java: this.mProperty.setFloat(509, offsetY);
      self.base.mProperty.as_mut().unwrap().setFloat(509, offsetY);
    }

    // Java: float getScaleX()
    pub fn getScaleX(&self) -> f32 {
      // Java: return this.mProperty.getFloat(510, 1.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(510, 1.0_f32)
    }

    // Java: void setScaleX(float scaleX)
    pub fn setScaleX(&mut self, scaleX: f32) {
      // Java: this.mProperty.setFloat(510, scaleX);
      self.base.mProperty.as_mut().unwrap().setFloat(510, scaleX);
    }

    // Java: float getScaleY()
    pub fn getScaleY(&self) -> f32 {
      // Java: return this.mProperty.getFloat(511, 1.0F);
      self.base.mProperty.as_ref().unwrap().base.getFloat_default(511, 1.0_f32)
    }

    // Java: void setScaleY(float scaleY)
    pub fn setScaleY(&mut self, scaleY: f32) {
      // Java: this.mProperty.setFloat(511, scaleY);
      self.base.mProperty.as_mut().unwrap().setFloat(511, scaleY);
    }

    // Java: int getAlign()
    pub fn getAlign(&self) -> i32 {
      // Java: return this.mProperty.getInt(512, 7);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(512, 7)
    }

    // Java: void setAlign(int align)
    pub fn setAlign(&mut self, align: i32) {
      // Java: this.mProperty.setInt(512, align);
      self.base.mProperty.as_mut().unwrap().setInt(512, align);
    }

    // Java: TileFlipMode getFlipMode()
    pub fn getFlipMode(&self) -> Option<TileFlipMode> {
      // Java: Object object = this.mProperty.getValue(513);
      let object = self.base.mProperty.as_ref().unwrap().base.getValue(513);
      // Java: return object == null ? null : (TileFlipMode)object;
      object.and_then(|arc| arc.downcast_ref::<TileFlipMode>().cloned())
    }

    // Java: void setFlipMode(TileFlipMode fileMode)
    pub fn setFlipMode(&mut self, fileMode: TileFlipMode) {
      // Java: this.mProperty.setValue(513, fileMode);
      self.base.mProperty.as_mut().unwrap().setValue(513, Some(Arc::new(fileMode) as Arc<dyn Any + Send + Sync>));
    }

}
