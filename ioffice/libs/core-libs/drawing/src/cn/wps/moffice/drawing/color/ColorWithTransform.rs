#![allow(non_snake_case, dead_code)]

// Java: import cn.wps.moffice.drawing.PropBase
use super::super::PropBase::PropBase;

use std::sync::Arc;
use std::any::Any;

type Element = Option<Arc<dyn Any + Send + Sync>>;

// Java: public class ColorWithTransform extends PropBase
pub struct ColorWithTransform {
    pub base: PropBase,
}

// Java: public class ColorWithTransform extends PropBase
impl ColorWithTransform {
    // Java: ColorWithTransform()
    pub fn new() -> Self {
      // Java: super();
      ColorWithTransform { base: PropBase::new() }
    }

    // Java: boolean isThreeRGB()
    pub fn isThreeRGB(&self) -> bool {
      // Java: return this.mProperty.getBoolean(974, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(974, false)
    }

    // Java: void setThreeRGB(boolean threeRGB)
    pub fn setThreeRGB(&mut self, threeRGB: bool) {
      // Java: this.mProperty.setBoolean(974, threeRGB);
      self.base.mProperty.as_mut().unwrap().setBoolean(974, threeRGB);
    }

    // Java: boolean isHexRGB()
    pub fn isHexRGB(&self) -> bool {
      // Java: return this.mProperty.getBoolean(975, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(975, false)
    }

    // Java: void setHexRGB(boolean hexRGB)
    pub fn setHexRGB(&mut self, hexRGB: bool) {
      // Java: this.mProperty.setBoolean(975, hexRGB);
      self.base.mProperty.as_mut().unwrap().setBoolean(975, hexRGB);
    }

    // Java: boolean isHSL()
    pub fn isHSL(&self) -> bool {
      // Java: return this.mProperty.getBoolean(976, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(976, false)
    }

    // Java: void setHSL(boolean HSL)
    pub fn setHSL(&mut self, HSL: bool) {
      // Java: this.mProperty.setBoolean(976, HSL);
      self.base.mProperty.as_mut().unwrap().setBoolean(976, HSL);
    }

    // Java: boolean isSysClr()
    pub fn isSysClr(&self) -> bool {
      // Java: return this.mProperty.getBoolean(980, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(980, false)
    }

    // Java: void setSysClr(boolean sysClr)
    pub fn setSysClr(&mut self, sysClr: bool) {
      // Java: this.mProperty.setBoolean(977, sysClr);
      self.base.mProperty.as_mut().unwrap().setBoolean(977, sysClr);
    }

    // Java: boolean isPrstClr()
    pub fn isPrstClr(&self) -> bool {
      // Java: return this.mProperty.getBoolean(978, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(978, false)
    }

    // Java: void setPrstClr(boolean prstClr)
    pub fn setPrstClr(&mut self, prstClr: bool) {
      // Java: this.mProperty.setBoolean(978, prstClr);
      self.base.mProperty.as_mut().unwrap().setBoolean(978, prstClr);
    }

    // Java: boolean isThemeClr()
    pub fn isThemeClr(&self) -> bool {
      // Java: return this.mProperty.getBoolean(975, false);
      self.base.mProperty.as_ref().unwrap().base.getBoolean_default(975, false)
    }

    // Java: void setThemeClr(boolean themeClr)
    pub fn setThemeClr(&mut self, themeClr: bool) {
      // Java: this.mProperty.setBoolean(979, themeClr);
      self.base.mProperty.as_mut().unwrap().setBoolean(979, themeClr);
    }

    // Java: ClrTransform getColorTransform()
    pub fn getColorTransform(&self) -> Element {
      // Java: Object val = this.mProperty.getValue(981);
      // Java: return null != val && val instanceof ClrTransform ? (ClrTransform)val : null;
      self.base.mProperty.as_ref().unwrap().base.getValue(981)
    }

    // Java: void setColorTransform(ClrTransform colorTransform)
    pub fn setColorTransform(&mut self, colorTransform: Element) {
      // Java: this.mProperty.setValue(981, colorTransform);
      self.base.mProperty.as_mut().unwrap().setValue(981, colorTransform);
    }

    // Java: int getSrgbClr()
    pub fn getSrgbClr(&self) -> i32 {
      // Java: return this.mProperty.getInt(980, 0);
      self.base.mProperty.as_ref().unwrap().base.getInt_default(980, 0)
    }

    // Java: void setSrgbClr(int srgbClr)
    pub fn setSrgbClr(&mut self, srgbClr: i32) {
      // Java: this.mProperty.setInt(980, srgbClr);
      self.base.mProperty.as_mut().unwrap().setInt(980, srgbClr);
    }

}
