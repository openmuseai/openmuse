#![allow(non_snake_case, non_upper_case_globals, dead_code, unused_imports, unused_variables)]

use super::super::PropBase::PropBase;

// Java: public class ClrTransform extends PropBase
pub struct ClrTransform {
    pub(crate) base: PropBase,
}

// Java: public class ClrTransform extends PropBase
impl ClrTransform {
    // Java: ClrTransform()
    pub fn new() -> Self {
      // Java: super();
      ClrTransform { base: PropBase::new() }
    }

    // Java: Float getTint()
    pub fn getTint(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(982) ? this.mProperty.getFloat(982) : null;
      if self.base.contains(982) { Some(self.base.getProperty().getFloat(982)) } else { None }
    }

    // Java: void setTint(Float tint)
    pub fn setTint(&mut self, tint: Option<f32>) {
      // Java: if (null != tint)
      if let Some(val) = tint
      {
         // Java: this.mProperty.setFloat(982, tint);
         self.base.getProperty_mut().setFloat(982, val);
      }
    }

    // Java: Float getShade()
    pub fn getShade(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(983) ? this.mProperty.getFloat(983) : null;
      if self.base.contains(983) { Some(self.base.getProperty().getFloat(983)) } else { None }
    }

    // Java: void setShade(Float shade)
    pub fn setShade(&mut self, shade: Option<f32>) {
      // Java: if (null != shade)
      if let Some(val) = shade
      {
         // Java: this.mProperty.setFloat(983, shade);
         self.base.getProperty_mut().setFloat(983, val);
      }
    }

    // Java: Float getComp()
    pub fn getComp(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(984) ? this.mProperty.getFloat(984) : null;
      if self.base.contains(984) { Some(self.base.getProperty().getFloat(984)) } else { None }
    }

    // Java: void setComp(Float comp)
    pub fn setComp(&mut self, comp: Option<f32>) {
      // Java: if (null != comp)
      if let Some(val) = comp
      {
         // Java: this.mProperty.setFloat(984, comp);
         self.base.getProperty_mut().setFloat(984, val);
      }
    }

    // Java: Float getInv()
    pub fn getInv(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(985) ? this.mProperty.getFloat(985) : null;
      if self.base.contains(985) { Some(self.base.getProperty().getFloat(985)) } else { None }
    }

    // Java: void setInv(Float val)
    pub fn setInv(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(985, val);
         self.base.getProperty_mut().setFloat(985, v);
      }
    }

    // Java: Float getGray()
    pub fn getGray(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(986) ? this.mProperty.getFloat(986) : null;
      if self.base.contains(986) { Some(self.base.getProperty().getFloat(986)) } else { None }
    }

    // Java: void setGray(Float val)
    pub fn setGray(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(986, val);
         self.base.getProperty_mut().setFloat(986, v);
      }
    }

    // Java: Float getAlpha()
    pub fn getAlpha(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(987) ? this.mProperty.getFloat(987) : null;
      if self.base.contains(987) { Some(self.base.getProperty().getFloat(987)) } else { None }
    }

    // Java: void setAlpha(Float val)
    pub fn setAlpha(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(987, val);
         self.base.getProperty_mut().setFloat(987, v);
      }
    }

    // Java: Float getAlphaOff()
    pub fn getAlphaOff(& mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(988) ? this.mProperty.getFloat(988) : null;
      if self.base.contains(988) { Some(self.base.getProperty().getFloat(988)) } else { None }
    }

    // Java: void setAlphaOff(Float val)
    pub fn setAlphaOff(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(988, val);
         self.base.getProperty_mut().setFloat(988, v);
      }
    }

    // Java: Float getAlphaMod()
    pub fn getAlphaMod(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(989) ? this.mProperty.getFloat(989) : null;
      if self.base.contains(989) { Some(self.base.getProperty().getFloat(989)) } else { None }
    }

    // Java: void setAlphaMod(Float val)
    pub fn setAlphaMod(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(989, val);
         self.base.getProperty_mut().setFloat(989, v);
      }
    }

    // Java: Float getRed()
    pub fn getRed(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(990) ? this.mProperty.getFloat(990) : null;
      if self.base.contains(990) { Some(self.base.getProperty().getFloat(990)) } else { None }
    }

    // Java: void setRed(Float val)
    pub fn setRed(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(990, val);
         self.base.getProperty_mut().setFloat(990, v);
      }
    }

    // Java: Float getRedOff()
    pub fn getRedOff(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(991) ? this.mProperty.getFloat(991) : null;
      if self.base.contains(991) { Some(self.base.getProperty().getFloat(991)) } else { None }
    }

    // Java: void setRedOff(Float val)
    pub fn setRedOff(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(991, val);
         self.base.getProperty_mut().setFloat(991, v);
      }
    }

    // Java: Float getRedMod()
    pub fn getRedMod(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(992) ? this.mProperty.getFloat(992) : null;
      if self.base.contains(992) { Some(self.base.getProperty().getFloat(992)) } else { None }
    }

    // Java: void setRedMod(Float val)
    pub fn setRedMod(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(992, val);
         self.base.getProperty_mut().setFloat(992, v);
      }
    }

    // Java: Float getGreen()
    pub fn getGreen(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(993) ? this.mProperty.getFloat(993) : null;
      if self.base.contains(993) { Some(self.base.getProperty().getFloat(993)) } else { None }
    }

    // Java: void setGreen(Float val)
    pub fn setGreen(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(993, val);
         self.base.getProperty_mut().setFloat(993, v);
      }
    }

    // Java: Float getGreenOff()
    pub fn getGreenOff(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(994) ? this.mProperty.getFloat(994) : null;
      if self.base.contains(994) { Some(self.base.getProperty().getFloat(994)) } else { None }
    }

    // Java: void setGreenOff(Float val)
    pub fn setGreenOff(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(994, val);
         self.base.getProperty_mut().setFloat(994, v);
      }
    }

    // Java: Float getGreenMod()
    pub fn getGreenMod(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(995) ? this.mProperty.getFloat(995) : null;
      if self.base.contains(995) { Some(self.base.getProperty().getFloat(995)) } else { None }
    }

    // Java: void setGreenMod(Float val)
    pub fn setGreenMod(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(995, val);
         self.base.getProperty_mut().setFloat(995, v);
      }
    }

    // Java: Float getBlue()
    pub fn getBlue(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(996) ? this.mProperty.getFloat(996) : null;
      if self.base.contains(996) { Some(self.base.getProperty().getFloat(996)) } else { None }
    }

    // Java: void setBlue(Float val)
    pub fn setBlue(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(996, val);
         self.base.getProperty_mut().setFloat(996, v);
      }
    }

    // Java: Float getBlueOff()
    pub fn getBlueOff(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(997) ? this.mProperty.getFloat(997) : null;
      if self.base.contains(997) { Some(self.base.getProperty().getFloat(997)) } else { None }
    }

    // Java: void setBlueOff(Float val)
    pub fn setBlueOff(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(997, val);
         self.base.getProperty_mut().setFloat(997, v);
      }
    }

    // Java: Float getBlueMod()
    pub fn getBlueMod(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(998) ? this.mProperty.getFloat(998) : null;
      if self.base.contains(998) { Some(self.base.getProperty().getFloat(998)) } else { None }
    }

    // Java: void setBlueMod(Float val)
    pub fn setBlueMod(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(998, val);
         self.base.getProperty_mut().setFloat(998, v);
      }
    }

    // Java: Integer getHue()
    pub fn getHue(&mut self) -> Option<i32> {
      // Java: return this.mProperty.contains(999) ? this.mProperty.getInt(999) : null;
      if self.base.contains(999) { Some(self.base.getProperty().getInt(999)) } else { None }
    }

    // Java: void setHue(Integer val)
    pub fn setHue(&mut self, val: Option<i32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setInt(999, val);
         self.base.getProperty_mut().setInt(999, v);
      }
    }

    // Java: Integer getHueOff()
    pub fn getHueOff(&mut self) -> Option<i32> {
      // Java: return this.mProperty.contains(1000) ? this.mProperty.getInt(1000) : null;
      if self.base.contains(1000) { Some(self.base.getProperty().getInt(1000)) } else { None }
    }

    // Java: void setHueOff(Integer val)
    pub fn setHueOff(&mut self, val: Option<i32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setInt(1000, val);
         self.base.getProperty_mut().setInt(1000, v);
      }
    }

    // Java: Float getHueMod()
    pub fn getHueMod(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1001) ? this.mProperty.getFloat(1001) : null;
      if self.base.contains(1001) { Some(self.base.getProperty().getFloat(1001)) } else { None }
    }

    // Java: void setHueMod(Float val)
    pub fn setHueMod(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1001, val);
         self.base.getProperty_mut().setFloat(1001, v);
      }
    }

    // Java: Float getSat()
    pub fn getSat(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1002) ? this.mProperty.getFloat(1002) : null;
      if self.base.contains(1002) { Some(self.base.getProperty().getFloat(1002)) } else { None }
    }

    // Java: void setSat(Float val)
    pub fn setSat(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1002, val);
         self.base.getProperty_mut().setFloat(1002, v);
      }
    }

    // Java: Float getSatOff()
    pub fn getSatOff(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1003) ? this.mProperty.getFloat(1003) : null;
      if self.base.contains(1003) { Some(self.base.getProperty().getFloat(1003)) } else { None }
    }

    // Java: void setSatOff(Float val)
    pub fn setSatOff(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1003, val);
         self.base.getProperty_mut().setFloat(1003, v);
      }
    }

    // Java: Float getSatMod()
    pub fn getSatMod(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1004) ? this.mProperty.getFloat(1004) : null;
      if self.base.contains(1004) { Some(self.base.getProperty().getFloat(1004)) } else { None }
    }

    // Java: void setSatMod(Float val)
    pub fn setSatMod(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1004, val);
         self.base.getProperty_mut().setFloat(1004, v);
      }
    }

    // Java: Float getLum()
    pub fn getLum(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1005) ? this.mProperty.getFloat(1005) : null;
      if self.base.contains(1005) { Some(self.base.getProperty().getFloat(1005)) } else { None }
    }

    // Java: void setLum(Float val)
    pub fn setLum(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1005, val);
         self.base.getProperty_mut().setFloat(1005, v);
      }
    }

    // Java: Float getLumOff()
    pub fn getLumOff(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1006) ? this.mProperty.getFloat(1006) : null;
      if self.base.contains(1006) { Some(self.base.getProperty().getFloat(1006)) } else { None }
    }

    // Java: void setLumOff(Float val)
    pub fn setLumOff(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1006, val);
         self.base.getProperty_mut().setFloat(1006, v);
      }
    }

    // Java: Float getLumMod()
    pub fn getLumMod(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1007) ? this.mProperty.getFloat(1007) : null;
      if self.base.contains(1007) { Some(self.base.getProperty().getFloat(1007)) } else { None }
    }

    // Java: void setLumMod(Float val)
    pub fn setLumMod(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1007, val);
         self.base.getProperty_mut().setFloat(1007, v);
      }
    }

    // Java: Float getGamma()
    pub fn getGamma(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1008) ? this.mProperty.getFloat(1008) : null;
      if self.base.contains(1008) { Some(self.base.getProperty().getFloat(1008)) } else { None }
    }

    // Java: void setGamma(Float val)
    pub fn setGamma(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1008, val);
         self.base.getProperty_mut().setFloat(1008, v);
      }
    }

    // Java: Float getInvGamma()
    pub fn getInvGamma(&mut self) -> Option<f32> {
      // Java: return this.mProperty.contains(1009) ? this.mProperty.getFloat(1009) : null;
      if self.base.contains(1009) { Some(self.base.getProperty().getFloat(1009)) } else { None }
    }

    // Java: void setInvGamma(Float val)
    pub fn setInvGamma(&mut self, val: Option<f32>) {
      // Java: if (null != val)
      if let Some(v) = val
      {
         // Java: this.mProperty.setFloat(1009, val);
         self.base.getProperty_mut().setFloat(1009, v);
      }
    }

}
