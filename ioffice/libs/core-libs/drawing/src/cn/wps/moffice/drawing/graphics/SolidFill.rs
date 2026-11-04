#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.color.ColorWithTransform
// use super::super::color::ColorWithTransform::ColorWithTransform; // color module disabled
#[derive(Clone)]
pub struct ColorWithTransform;
use super::FillBase::FillBase;

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class SolidFill extends FillBase
pub struct SolidFill {
    pub base: FillBase,
}

// Java: public class SolidFill extends FillBase
impl SolidFill {
    // Java: SolidFill()
    pub fn new() -> Self {
      // Java: super(0);
      SolidFill { base: FillBase::new_with_fill_type(0) }
    }

    // Java: SolidFill(int color)
    pub fn new_with_color(color: i32) -> Self {
      // Java: super(0);
      let mut sf = SolidFill { base: FillBase::new_with_fill_type(0) };
      // Java: this.setColor(color);
      sf.base.setColor(color);
      sf
    }

    // Java: boolean isSimpleColorFill()
    pub fn isSimpleColorFill(&mut self) -> bool {
      // Java: return this.getProperty().size() == 3 && this.contains(463) && this.contains(477) && this.contains(468);
      self.base.base.getProperty().base.size() == 3
          && self.base.base.contains(463)
          && self.base.base.contains(477)
          && self.base.base.contains(468)
    }

    // Java: void setColorWithTransform(ColorWithTransform colorWithTransform)
    pub fn setColorWithTransform(&mut self, colorWithTransform: Element) {
      // Java: this.mProperty.setValue(516, colorWithTransform);
      self.base.base.mProperty.as_mut().unwrap().setValue(516, colorWithTransform);
    }

    // Java: ColorWithTransform getColorWithTransform()
    pub fn getColorWithTransform(&self) -> Element {
      // Java: Object obj = this.mProperty.getValue(516);
      // Java: return obj == null ? null : (ColorWithTransform)obj;
      self.base.base.mProperty.as_ref().unwrap().base.getValue(516)
    }

}
