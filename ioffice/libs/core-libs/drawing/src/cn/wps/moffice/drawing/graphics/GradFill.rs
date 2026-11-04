#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.graphics.RectF
use core_libs_util::graphics::RectF::RectF;

use super::FillBase::FillBase;

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java field: int LINEARTYPE_HORI
pub const LINEARTYPE_HORI: i32 = 0;
// Java field: int LINEARTYPE_VERT
pub const LINEARTYPE_VERT: i32 = 1;
// Java field: int LINEARTYPE_OBLIQUE_TOP
pub const LINEARTYPE_OBLIQUE_TOP: i32 = 2;
// Java field: int LINEARTYPE_OBLIQUE_BOTTOM
pub const LINEARTYPE_OBLIQUE_BOTTOM: i32 = 3;

// Java: public class GradFill extends FillBase
pub struct GradFill {
    pub base: FillBase,
}

// Java: public class GradFill extends FillBase
impl GradFill {
    // Java: GradFill()
    pub fn new() -> Self {
      // Java: super(4);
      GradFill { base: FillBase::new_with_fill_type(4) }
    }

    // Java: float getAngle()
    pub fn getAngle(&self) -> f32 {
      // Java: return this.mProperty.getFloat(479, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(479, 0.0_f32)
    }

    // Java: void setAngle(float angle)
    pub fn setAngle(&mut self, angle: f32) {
      // Java: this.mProperty.setFloat(479, angle);
      self.base.base.mProperty.as_mut().unwrap().setFloat(479, angle);
    }

    // Java: float getFocus()
    pub fn getFocus(&self) -> f32 {
      // Java: return this.mProperty.getFloat(480, 0.0F);
      self.base.base.mProperty.as_ref().unwrap().base.getFloat_default(480, 0.0_f32)
    }

    // Java: void setFocus(float focus)
    pub fn setFocus(&mut self, focus: f32) {
      // Java: this.mProperty.setFloat(480, focus);
      self.base.base.mProperty.as_mut().unwrap().setFloat(480, focus);
    }

    // Java: void setGradStopList(GradStopList list)
    pub fn setGradStopList(&mut self, list: Element) {
      // Java: this.mProperty.setValue(483, list);
      self.base.base.mProperty.as_mut().unwrap().setValue(483, list);
    }

    // Java: GradStopList getGradStopList()
    pub fn getGradStopList(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(483);
      // Java: return object == null ? null : (GradStopList)object;
      self.base.base.mProperty.as_ref().unwrap().base.getValue(483)
    }

    // Java: void setTileRect(RectF rectF)
    pub fn setTileRect(&mut self, rectF: Element) {
      // Java: this.mProperty.setValue(475, rectF);
      self.base.base.mProperty.as_mut().unwrap().setValue(475, rectF);
    }

    // Java: RectF getTileRect()
    pub fn getTileRect(&self) -> Element {
      // Java: Object object = this.mProperty.getValue(475);
      // Java: return object == null ? null : (RectF)object;
      self.base.base.mProperty.as_ref().unwrap().base.getValue(475)
    }

    // Java: int getLinearType()
    pub fn getLinearType(&self) -> i32 {
      // Java: float angle = (this.getAngle() + 180.0F) % 180.0F;
      let angle = (self.getAngle() + 180.0_f32) % 180.0_f32;
      // Java: if (angle < 45.0F)
      if angle < 45.0_f32 {
         // Java: return 1;
         return 1;
      }
      // Java: else if (angle < 90.0F)
      else if angle < 90.0_f32 {
         // Java: return 3;
         return 3;
      }
      // Java: else
      {
         // Java: return angle < 135.0F ? 0 : 2;
         if angle < 135.0_f32 { 0 } else { 2 }
      }
    }

    // Java: int getFillShadeType()
    pub fn getFillShadeType(&self) -> i32 {
      // Java: return this.mProperty.getInt(491, 3);
      self.base.base.mProperty.as_ref().unwrap().base.getInt_default(491, 3)
    }

    // Java: void setFillShadeType(int fillShadeType)
    pub fn setFillShadeType(&mut self, fillShadeType: i32) {
      // Java: this.mProperty.setInt(491, fillShadeType);
      self.base.base.mProperty.as_mut().unwrap().setInt(491, fillShadeType);
    }

}
