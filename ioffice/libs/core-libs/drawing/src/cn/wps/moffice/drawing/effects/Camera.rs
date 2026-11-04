#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports)]

use super::super::PropBase::PropBase;
use std::sync::Arc;
use std::any::Any;

pub type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class Camera extends PropBase
pub struct Camera {
    pub(crate) base: PropBase,
}

// Java: public class Camera extends PropBase
impl Camera {
    // Java: Camera()
    pub fn new() -> Self {
      // Java: super();
      Camera { base: PropBase::new() }
    }

    // Java: void setFov(float fov)
    pub fn setFov(&mut self, fov: f32) {
      // Java: this.mProperty.setFloat(392, fov);
      self.base.getProperty_mut().setFloat(392, fov);
    }

    // Java: float getFov()
    pub fn getFov(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(392, 0.0F);
      self.base.getProperty_mut().getFloat_default(392, 0.0_f32)
    }

    // Java: Float getFovObj()
    pub fn getFovObj(&mut self) -> Option<f32> {
      // Java: Object val = this.mProperty.getValue(392);
      let val = self.base.getValue(392);
      // Java: return val != null ? (Float)val : null;
      val.and_then(|v| v.downcast_ref::<f32>().copied())
    }

    // Java: void setPresetCamera(int prst)
    pub fn setPresetCamera(&mut self, prst: i32) {
      // Java: this.mProperty.setInt(394, prst);
      self.base.getProperty_mut().setInt(394, prst);
    }

    // Java: int getPresetCamera()
    pub fn getPresetCamera(&mut self) -> i32 {
      // Java: return this.mProperty.getInt(394, 0);
      self.base.getProperty().getInt_default(394, 0)
    }

    // Java: Integer getPresetCameraObj()
    pub fn getPresetCameraObj(&mut self) -> Option<i32> {
      // Java: Object val = this.mProperty.getValue(394);
      let val = self.base.getValue(394);
      // Java: return val != null ? (Integer)val : null;
      val.and_then(|v| v.downcast_ref::<i32>().copied())
    }

    // Java: void setZoom(float zoom)
    pub fn setZoom(&mut self, zoom: f32) {
      // Java: this.mProperty.setFloat(393, zoom);
      self.base.getProperty_mut().setFloat(393, zoom);
    }

    // Java: float getZoom()
    pub fn getZoom(&mut self) -> f32 {
      // Java: return this.mProperty.getFloat(393, 1.0F);
      self.base.getProperty().getFloat_default(393, 1.0_f32)
    }

    // Java: Float getZoomObj()
    pub fn getZoomObj(&mut self) -> Option<f32> {
      // Java: Object val = this.mProperty.getValue(393);
      let val = self.base.getValue(393);
      // Java: return val != null ? (Float)val : null;
      val.and_then(|v| v.downcast_ref::<f32>().copied())
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
