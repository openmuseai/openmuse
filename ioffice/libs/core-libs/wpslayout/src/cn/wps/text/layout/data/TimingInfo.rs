// Java field: byte NONE
pub(crate) const NONE: i8 = 0;
// Java field: byte FONTSIZE
pub(crate) const FONTSIZE: i8 = 1;
// Java field: byte FONTCOLOR
pub(crate) const FONTCOLOR: i8 = 2;
// Java field: byte UNDERLINE
pub(crate) const UNDERLINE: i8 = 4;
// Java field: byte BOLD
pub(crate) const BOLD: i8 = 8;
// Java field: byte STRIKETHROUGH
pub(crate) const STRIKETHROUGH: i8 = 16;
// Java field: byte ITALIC
pub(crate) const ITALIC: i8 = 32;
// Java field: byte DYNAMIC_DIGIT
pub(crate) const DYNAMIC_DIGIT: i8 = 64;

// Java: public class TimingInfo
pub struct TimingInfo {
    // Java field: int startPara
    pub(crate) startPara: i32,
    // Java field: int endPara
    pub(crate) endPara: i32,
    // Java field: int startOffset
    pub(crate) startOffset: i32,
    // Java field: int endOffset
    pub(crate) endOffset: i32,
    // Java field: byte alterType
    pub(crate) alterType: i8,
    // Java field: float fontsizeScale
    pub(crate) fontsizeScale: f32,
    // Java field: int fontcolor
    pub(crate) fontcolor: i32,
    // Java field: boolean underline
    pub(crate) underline: bool,
    // Java field: boolean bold
    pub(crate) bold: bool,
    // Java field: boolean strikethrough
    pub(crate) strikethrough: bool,
    // Java field: boolean italic
    pub(crate) italic: bool,
    // Java field: double dynamicDigitFraction
    pub(crate) dynamicDigitFraction: f64,
    // Java field: double dynamicDigitDest
    pub(crate) dynamicDigitDest: f64,
    // Java field: double mDynamicDigitSrc
    pub(crate) mDynamicDigitSrc: f64,
    // Java field: boolean mDynamicDigitIsSrcAbs
    pub(crate) mDynamicDigitIsSrcAbs: bool,
    // Java field: boolean mDynamicDigitIsDestAbs
    pub(crate) mDynamicDigitIsDestAbs: bool,
    // Java field: int mDynamicDigitAnimMode
    pub(crate) mDynamicDigitAnimMode: i32,
    // Java field: boolean mDynamicDigitOptimizeRate
    pub(crate) mDynamicDigitOptimizeRate: bool,
}

// Java: public class TimingInfo
impl TimingInfo {
    // Java: TimingInfo()
    pub fn new() -> Self {
      // Java: super();
      TimingInfo {
          startPara: 0, endPara: 0, startOffset: 0, endOffset: 0,
          alterType: 0, fontsizeScale: 0.0, fontcolor: 0,
          underline: false, bold: false, strikethrough: false, italic: false,
          dynamicDigitFraction: 0.0, dynamicDigitDest: 0.0, mDynamicDigitSrc: 0.0,
          mDynamicDigitIsSrcAbs: false, mDynamicDigitIsDestAbs: false,
          mDynamicDigitAnimMode: 0, mDynamicDigitOptimizeRate: false,
      }
    }

    // Java: int getStartPara()
    pub fn getStartPara(&self) -> i32 {
      // Java: return this.startPara;
      self.startPara
    }

    // Java: void setStartPara(int startPara)
    pub fn setStartPara(&mut self, startPara: i32) {
      // Java: this.startPara = startPara;
      self.startPara = startPara;
    }

    // Java: float getFontsizeScale()
    pub fn getFontsizeScale(&self) -> f32 {
      // Java: return this.fontsizeScale;
      self.fontsizeScale
    }

    // Java: void setFontsizeScale(float fontsizeScale)
    pub fn setFontsizeScale(&mut self, fontsizeScale: f32) {
      // Java: this.fontsizeScale = fontsizeScale;
      self.fontsizeScale = fontsizeScale;
    }

    // Java: double getDynamicDigitFraction()
    pub fn getDynamicDigitFraction(&self) -> f64 {
      // Java: return this.dynamicDigitFraction;
      self.dynamicDigitFraction
    }

    // Java: void setDynamicDigitFraction(double fraction)
    pub fn setDynamicDigitFraction(&mut self, fraction: f64) {
      // Java: this.dynamicDigitFraction = fraction;
      self.dynamicDigitFraction = fraction;
    }

    // Java: double getDynamicDigitDest()
    pub fn getDynamicDigitDest(&self) -> f64 {
      // Java: return this.dynamicDigitDest;
      self.dynamicDigitDest
    }

    // Java: void setDynamicDigitDest(double dest)
    pub fn setDynamicDigitDest(&mut self, dest: f64) {
      // Java: this.dynamicDigitDest = dest;
      self.dynamicDigitDest = dest;
    }

    // Java: double getDynamicDigitSrc()
    pub fn getDynamicDigitSrc(&self) -> f64 {
      // Java: return this.mDynamicDigitSrc;
      self.mDynamicDigitSrc
    }

    // Java: void setDynamicDigitSrc(double dynamicDigitSrc)
    pub fn setDynamicDigitSrc(&mut self, dynamicDigitSrc: f64) {
      // Java: this.mDynamicDigitSrc = dynamicDigitSrc;
      self.mDynamicDigitSrc = dynamicDigitSrc;
    }

    // Java: boolean isDynamicDigitIsSrcAbs()
    pub fn isDynamicDigitIsSrcAbs(&self) -> bool {
      // Java: return this.mDynamicDigitIsSrcAbs;
      self.mDynamicDigitIsSrcAbs
    }

    // Java: void setDynamicDigitIsSrcAbs(boolean dynamicDigitIsSrcAbs)
    pub fn setDynamicDigitIsSrcAbs(&mut self, dynamicDigitIsSrcAbs: bool) {
      // Java: this.mDynamicDigitIsSrcAbs = dynamicDigitIsSrcAbs;
      self.mDynamicDigitIsSrcAbs = dynamicDigitIsSrcAbs;
    }

    // Java: boolean isDynamicDigitIsDestAbs()
    pub fn isDynamicDigitIsDestAbs(&self) -> bool {
      // Java: return this.mDynamicDigitIsDestAbs;
      self.mDynamicDigitIsDestAbs
    }

    // Java: void setDynamicDigitIsDestAbs(boolean dynamicDigitIsDestAbs)
    pub fn setDynamicDigitIsDestAbs(&mut self, dynamicDigitIsDestAbs: bool) {
      // Java: this.mDynamicDigitIsDestAbs = dynamicDigitIsDestAbs;
      self.mDynamicDigitIsDestAbs = dynamicDigitIsDestAbs;
    }

    // Java: int getDynamicDigitAnimMode()
    pub fn getDynamicDigitAnimMode(&self) -> i32 {
      // Java: return this.mDynamicDigitAnimMode;
      self.mDynamicDigitAnimMode
    }

    // Java: void setDynamicDigitAnimMode(int dynamicDigitAnimMode)
    pub fn setDynamicDigitAnimMode(&mut self, dynamicDigitAnimMode: i32) {
      // Java: this.mDynamicDigitAnimMode = dynamicDigitAnimMode;
      self.mDynamicDigitAnimMode = dynamicDigitAnimMode;
    }

    // Java: boolean isDynamicDigitOptimizeRate()
    pub fn isDynamicDigitOptimizeRate(&self) -> bool {
      // Java: return this.mDynamicDigitOptimizeRate;
      self.mDynamicDigitOptimizeRate
    }

    // Java: void setDynamicDigitOptimizeRate(boolean mDynamicDigitOptimizeRate)
    pub fn setDynamicDigitOptimizeRate(&mut self, mDynamicDigitOptimizeRate: bool) {
      // Java: this.mDynamicDigitOptimizeRate = mDynamicDigitOptimizeRate;
      self.mDynamicDigitOptimizeRate = mDynamicDigitOptimizeRate;
    }

    // Java: int getEndPara()
    pub fn getEndPara(&self) -> i32 {
      // Java: return this.endPara;
      self.endPara
    }

    // Java: void setEndPara(int endPara)
    pub fn setEndPara(&mut self, endPara: i32) {
      // Java: this.endPara = endPara;
      self.endPara = endPara;
    }

    // Java: int getStartOffset()
    pub fn getStartOffset(&self) -> i32 {
      // Java: return this.startOffset;
      self.startOffset
    }

    // Java: void setStartOffset(int startOffset)
    pub fn setStartOffset(&mut self, startOffset: i32) {
      // Java: this.startOffset = startOffset;
      self.startOffset = startOffset;
    }

    // Java: int getEndOffset()
    pub fn getEndOffset(&self) -> i32 {
      // Java: return this.endOffset;
      self.endOffset
    }

    // Java: void setEndOffset(int endOffset)
    pub fn setEndOffset(&mut self, endOffset: i32) {
      // Java: this.endOffset = endOffset;
      self.endOffset = endOffset;
    }

    // Java: int getFontcolor()
    pub fn getFontcolor(&self) -> i32 {
      // Java: return this.fontcolor;
      self.fontcolor
    }

    // Java: void setFontcolor(int fontcolor)
    pub fn setFontcolor(&mut self, fontcolor: i32) {
      // Java: this.fontcolor = fontcolor;
      self.fontcolor = fontcolor;
    }

    // Java: boolean isUnderline()
    pub fn isUnderline(&self) -> bool {
      // Java: return this.underline;
      self.underline
    }

    // Java: void setUnderline(boolean underline)
    pub fn setUnderline(&mut self, underline: bool) {
      // Java: this.underline = underline;
      self.underline = underline;
    }

    // Java: boolean isBold()
    pub fn isBold(&self) -> bool {
      // Java: return this.bold;
      self.bold
    }

    // Java: void setBold(boolean bold)
    pub fn setBold(&mut self, bold: bool) {
      // Java: this.bold = bold;
      self.bold = bold;
    }

    // Java: boolean isStrikethrough()
    pub fn isStrikethrough(&self) -> bool {
      // Java: return this.strikethrough;
      self.strikethrough
    }

    // Java: void setStrikethrough(boolean strikethrough)
    pub fn setStrikethrough(&mut self, strikethrough: bool) {
      // Java: this.strikethrough = strikethrough;
      self.strikethrough = strikethrough;
    }

    // Java: boolean isItalic()
    pub fn isItalic(&self) -> bool {
      // Java: return this.italic;
      self.italic
    }

    // Java: void setItalic(boolean italic)
    pub fn setItalic(&mut self, italic: bool) {
      // Java: this.italic = italic;
      self.italic = italic;
    }

    // Java: byte getAlterType()
    pub fn getAlterType(&self) -> i8 {
      // Java: return this.alterType;
      self.alterType
    }

    // Java: void setAlterType(byte alterType)
    pub fn setAlterType(&mut self, alterType: i8) {
      // Java: this.alterType = alterType;
      self.alterType = alterType;
    }

}
