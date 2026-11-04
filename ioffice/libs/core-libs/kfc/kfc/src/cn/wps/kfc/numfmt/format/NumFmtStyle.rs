// Java: public class NumFmtStyle
pub struct NumFmtStyle {
    // Java field: boolean mHasPercent
    pub(crate) mHasPercent: bool,
    // Java field: boolean mHasThousSep
    pub(crate) mHasThousSep: bool,
    // Java field: boolean mIsRMBScale
    pub(crate) mIsRMBScale: bool,
    // Java field: short mDBNumber
    pub(crate) mDBNumber: i16,
    // Java field: boolean mHasNCL
    pub(crate) mHasNCL: bool,
    // Java field: byte mNumeralShape
    pub(crate) mNumeralShape: i8,
    // Java field: byte mCalendarType
    pub(crate) mCalendarType: i8,
    // Java field: short mLocaleCodeID
    pub(crate) mLocaleCodeID: i16,
    // Java field: int mMaxLength
    pub(crate) mMaxLength: i32,
    // Java field: boolean mUse1904
    pub(crate) mUse1904: bool,
}

// Java: public class NumFmtStyle
impl NumFmtStyle {
    // Java: NumFmtStyle()
    pub fn new() -> Self {
      // Java: super();
      NumFmtStyle {
          mHasPercent: false,
          mHasThousSep: false,
          mIsRMBScale: false,
          mDBNumber: 0,
          mHasNCL: false,
          mNumeralShape: 0,
          mCalendarType: 0,
          mLocaleCodeID: 0,
          mMaxLength: 0,
          mUse1904: false,
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mHasPercent = false;
      self.mHasPercent = false;
      // Java: this.mHasThousSep = false;
      self.mHasThousSep = false;
      // Java: this.mIsRMBScale = false;
      self.mIsRMBScale = false;
      // Java: this.mHasNCL = false;
      self.mHasNCL = false;
      // Java: this.mDBNumber = 0;
      self.mDBNumber = 0;
      // Java: this.mNumeralShape = 0;
      self.mNumeralShape = 0;
      // Java: this.mCalendarType = 0;
      self.mCalendarType = 0;
      // Java: this.mLocaleCodeID = 0;
      self.mLocaleCodeID = 0;
      // Java: this.mMaxLength = 0;
      self.mMaxLength = 0;
      // Java: this.mUse1904 = false;
      self.mUse1904 = false;
    }

}
