// Java: import android.graphics.Bitmap
use super::FontFill::Bitmap;

// Java: public class PatternAttr
#[derive(Clone)]
pub struct PatternAttr {
    // Java field: int foreColor
    pub(crate) foreColor: i32,
    // Java field: int bgColor
    pub(crate) bgColor: i32,
    // Java field: int pattType
    pub(crate) pattType: i32,
    // Java field: Bitmap patternBmp
    pub(crate) patternBmp: Option<Bitmap>,
}

// Java: public class PatternAttr
impl PatternAttr {
    // Java: PatternAttr(int foreColor, int bgColor, int pattType)
    pub fn new_with_type(foreColor: i32, bgColor: i32, pattType: i32) -> Self {
      // Java: super();
      // Java: this.foreColor = foreColor;
      // Java: this.bgColor = bgColor;
      // Java: this.pattType = pattType;
      PatternAttr {
          foreColor,
          bgColor,
          pattType,
          patternBmp: None,
      }
    }

    // Java: PatternAttr(int foreColor, int bgColor, Bitmap bmp)
    pub fn new_with_bmp(foreColor: i32, bgColor: i32, bmp: Bitmap) -> Self {
      // Java: super();
      // Java: this.foreColor = foreColor;
      // Java: this.bgColor = bgColor;
      // Java: this.patternBmp = bmp;
      PatternAttr {
          foreColor,
          bgColor,
          pattType: 0,
          patternBmp: Some(bmp),
      }
    }

    // Java: int getForeColor()
    pub fn getForeColor(&self) -> i32 {
      // Java: return this.foreColor;
      self.foreColor
    }

    // Java: int getBgColor()
    pub fn getBgColor(&self) -> i32 {
      // Java: return this.bgColor;
      self.bgColor
    }

    // Java: int getPattType()
    pub fn getPattType(&self) -> i32 {
      // Java: return this.pattType;
      self.pattType
    }

    // Java: Bitmap getPatternBmp()
    pub fn getPatternBmp(&self) -> Option<&Bitmap> {
      // Java: return this.patternBmp;
      self.patternBmp.as_ref()
    }

    // Java: boolean equals(PatternAttr pattFill)
    pub fn equals(&self, pattFill: Option<&PatternAttr>) -> bool {
      // Java: if (pattFill == null)
      let pattFill = match pattFill {
          None =>
      {
         // Java: return false;
         return false;
      }
      // Java: else
          Some(p) => p,
      };
      {
         // Java: return this.foreColor == pattFill.foreColor && this.bgColor == pattFill.bgColor && this.pattType == pattFill.pattType;
         self.foreColor == pattFill.foreColor && self.bgColor == pattFill.bgColor && self.pattType == pattFill.pattType
      }
    }

}
