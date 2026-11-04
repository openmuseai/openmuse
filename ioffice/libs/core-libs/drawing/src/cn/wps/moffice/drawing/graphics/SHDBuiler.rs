#![allow(non_snake_case, dead_code)]

use super::SHD::SHD;

// Java: public class SHDBuiler extends SHD
pub struct SHDBuiler {
    pub base: SHD,
}

// Java: public class SHDBuiler extends SHD
impl SHDBuiler {
    // Java: SHDBuiler()
    pub fn new() -> Self {
      // Java: super();
      SHDBuiler { base: SHD::new() }
    }

    // Java: SHDBuiler(int crBack)
    pub fn new_with_back(crBack: i32) -> Self {
      // Java: super(crBack);
      SHDBuiler { base: SHD::with_back(crBack) }
    }

    // Java: SHDBuiler(int crFore, int crBack, int ipat)
    pub fn new_with_all(crFore: i32, crBack: i32, ipat: i32) -> Self {
      // Java: super(crFore, crBack, ipat);
      SHDBuiler { base: SHD::with_all(crFore, crBack, ipat) }
    }

    // Java: SHDBuiler(SHD shd)
    pub fn new_from_shd(shd: &SHD) -> Self {
      // Java: super(shd.colorFore, shd.colorBack, shd.ipat);
      SHDBuiler { base: SHD::with_all(shd.colorFore, shd.colorBack, shd.ipat) }
    }

    // Java: void setColorFore(int color)
    pub fn setColorFore(&mut self, color: i32) {
      // Java: this.colorFore = color;
      self.base.colorFore = color;
    }

    // Java: void setColorBack(int color)
    pub fn setColorBack(&mut self, color: i32) {
      // Java: this.colorBack = color;
      self.base.colorBack = color;
    }

    // Java: void setPattern(int pattern)
    pub fn setPattern(&mut self, pattern: i32) {
      // Java: this.ipat = pattern;
      self.base.ipat = pattern;
    }

    // Java: void setValue(SHD shd)
    pub fn setValue(&mut self, shd: &SHD) {
      // Java: if (shd != null)
      {
         // Java: this.colorBack = shd.colorBack;
         self.base.colorBack = shd.colorBack;
         // Java: this.colorFore = shd.colorFore;
         self.base.colorFore = shd.colorFore;
         // Java: this.ipat = shd.ipat;
         self.base.ipat = shd.ipat;
      }
    }

}
