// Java: import cn.wps.kfc.numfmt.math.TimeCalc
use crate::cn::wps::kfc::numfmt::math::TimeCalc;

// Java: public class FormatContext
pub struct FormatContext {
    // Java field: double mFormatValue
    pub(crate) mFormatValue: f64,
    // Java field: String mFormatText
    pub(crate) mFormatText: Option<String>,
    // Java field: String mInteger
    pub(crate) mInteger: String,
    // Java field: String mDecimal
    pub(crate) mDecimal: String,
    // Java field: TimeCalc.TimeStruct mTime
    pub(crate) mTime: Option<TimeCalc::TimeStruct>,
}

// Java: public class FormatContext
impl FormatContext {
    // Java: FormatContext()
    pub fn new() -> Self {
      // Java: super();
      FormatContext {
          mFormatValue: 0.0,
          mFormatText: None,
          mInteger: String::new(),
          mDecimal: String::new(),
          mTime: None,
      }
    }

    // Java: void clear()
    pub fn clear(&mut self) {
      // Java: this.mFormatValue = (double)0.0F;
      self.mFormatValue = 0.0;
      // Java: this.mFormatText = null;
      self.mFormatText = None;
      // Java: this.mInteger = "";
      self.mInteger = String::new();
      // Java: this.mDecimal = "";
      self.mDecimal = String::new();
      // Java: this.mTime = null;
      self.mTime = None;
    }

    // Java: boolean valueIsText()
    pub fn valueIsText(&self) -> bool {
      // Java: return this.mFormatText != null;
      self.mFormatText.is_some()
    }

    // Java: void valueStrToLocale(NumFmtStyle numFmtStyle)
    pub fn valueStrToLocale(&mut self, num_fmt_style: &super::NumFmtStyle::NumFmtStyle) {
      // Java: if (numFmtStyle.mNumeralShape != 0)
      if num_fmt_style.mNumeralShape != 0
      {
         // Java: this.mInteger = NumConverter.transDigits((String)this.mInteger, 0, this.mInteger.length(), numFmtStyle.mNumeralShape);
         let len = self.mInteger.encode_utf16().count();
         self.mInteger = super::NumConverter::NumConverter::transDigits(&self.mInteger, 0, len, num_fmt_style.mNumeralShape);
         // Java: if (this.mDecimal.length() > 0)
         if !self.mDecimal.is_empty()
         {
            // Java: this.mDecimal = NumConverter.transDigits((String)this.mDecimal, 0, this.mDecimal.length(), numFmtStyle.mNumeralShape);
            let len = self.mDecimal.encode_utf16().count();
            self.mDecimal = super::NumConverter::NumConverter::transDigits(&self.mDecimal, 0, len, num_fmt_style.mNumeralShape);
         }
      }
    }

}
