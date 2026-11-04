// Java: import android.graphics.Bitmap
use util::graphics::Bitmap;

// Java: public class PatternAttr
pub struct PatternAttr {
    // Java field: int foreColor
    pub(crate) foreColor: i32,
    // Java field: int bgColor
    pub(crate) bgColor: i32,
    // Java field: int pattType
    pub(crate) pattType: i32,
    // Java field: Bitmap patternBmp
    pub(crate) patternBmp: Arc<Mutex<Bitmap>>,
}

// Java: public class PatternAttr
impl PatternAttr {
    // Java: PatternAttr(int foreColor, int bgColor, int pattType)
    pub fn PatternAttr(&self, int foreColor, int bgColor, int pattType) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.foreColor = foreColor;
      // TODO_RUST_CODE
      // Java: this.bgColor = bgColor;
      // TODO_RUST_CODE
      // Java: this.pattType = pattType;
      // TODO_RUST_CODE
    }

    // Java: PatternAttr(int foreColor, int bgColor, Bitmap bmp)
    pub fn PatternAttr(&self, int foreColor, int bgColor, Bitmap bmp) {
      // Java: super();
      // TODO_RUST_CODE
      // Java: this.foreColor = foreColor;
      // TODO_RUST_CODE
      // Java: this.bgColor = bgColor;
      // TODO_RUST_CODE
      // Java: this.patternBmp = bmp;
      // TODO_RUST_CODE
    }

    // Java: int getForeColor()
    pub fn getForeColor(&self) -> i32 {
      // Java: return this.foreColor;
      // TODO_RUST_CODE
    }

    // Java: int getBgColor()
    pub fn getBgColor(&self) -> i32 {
      // Java: return this.bgColor;
      // TODO_RUST_CODE
    }

    // Java: int getPattType()
    pub fn getPattType(&self) -> i32 {
      // Java: return this.pattType;
      // TODO_RUST_CODE
    }

    // Java: Bitmap getPatternBmp()
    pub fn getPatternBmp(&self) -> Bitmap {
      // Java: return this.patternBmp;
      // TODO_RUST_CODE
    }

    // Java: boolean equals(PatternAttr pattFill)
    pub fn equals(&self, PatternAttr pattFill) -> bool {
      // Java: if (pattFill == null)
        // TODO_RUST_CODE
      {
         // Java: return false;
         // TODO_RUST_CODE
      }
      // Java: else
        // TODO_RUST_CODE
      {
         // Java: return this.foreColor == pattFill.foreColor && this.bgColor == pattFill.bgColor && this.pattType == pattFill.pattType;
         // TODO_RUST_CODE
      }
    }

}
