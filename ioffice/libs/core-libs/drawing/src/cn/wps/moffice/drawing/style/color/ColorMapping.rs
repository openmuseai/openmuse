#![allow(non_snake_case, dead_code)]

use super::IColorMapping::IColorMapping;

// Java: public class ColorMapping implements IColorMapping
pub struct ColorMapping {
    // Java field: int[] mClrMap
    pub(crate) mClrMap: Vec<i32>,
}

// Java: public class ColorMapping implements IColorMapping
impl ColorMapping {
    // Java: ColorMapping()
    pub fn new() -> Self {
      // Java: super();
      let mut mClrMap = vec![0i32; 17];
      // Java: for(int i = 0; i < this.mClrMap.length; this.mClrMap[i] = i++)
      for i in 0..17 {
          mClrMap[i] = i as i32;
      }
      ColorMapping { mClrMap }
    }

    // Java: void setMapColor(int schemeFrom, int schemeTo)
    pub fn setMapColor(&mut self, schemeFrom: i32, schemeTo: i32) {
      // Java: this.mClrMap[schemeFrom] = schemeTo;
      self.mClrMap[schemeFrom as usize] = schemeTo;
    }

    // Java: int mapColor(int schemeClr)
    pub fn mapColor(&self, schemeClr: i32) -> i32 {
      // Java: return this.mClrMap[schemeClr];
      self.mClrMap[schemeClr as usize]
    }

}

impl IColorMapping for ColorMapping {
    fn mapColor(&self, var1: i32) -> i32 {
        self.mapColor(var1)
    }
}
