// Java field: short FORMAT_INVALID
pub(crate) const FORMAT_INVALID: i16 = -1;
// Java field: short FORMAT_UNKNOWN
pub(crate) const FORMAT_UNKNOWN: i16 = 0;
// Java field: short FORMAT_GENERAL
pub(crate) const FORMAT_GENERAL: i16 = 1;
// Java field: short FORMAT_NUMBERIC
pub(crate) const FORMAT_NUMBERIC: i16 = 512;
// Java field: short FORMAT_CURRENCY
pub(crate) const FORMAT_CURRENCY: i16 = 513;
// Java field: short FORMAT_PERCENT
pub(crate) const FORMAT_PERCENT: i16 = 4;
// Java field: short FORMAT_FRACTION
pub(crate) const FORMAT_FRACTION: i16 = 8;
// Java field: short FORMAT_SCIENCE
pub(crate) const FORMAT_SCIENCE: i16 = 16;
// Java field: short FORMAT_DATE_TIME
pub(crate) const FORMAT_DATE_TIME: i16 = 8192;
// Java field: short FORMAT_DATE
pub(crate) const FORMAT_DATE: i16 = 8193;
// Java field: short FORMAT_TIME
pub(crate) const FORMAT_TIME: i16 = 8194;
// Java field: short FORMAT_TEXT
pub(crate) const FORMAT_TEXT: i16 = 64;
// Java field: short FORMAT_UNSURE
pub(crate) const FORMAT_UNSURE: i16 = 128;

// Java: public class NumFmtType
pub struct NumFmtType {
}

// Java: public class NumFmtType
impl NumFmtType {
    // Java: NumFmtType()
    pub fn new() -> Self {
      // Java: super();
      NumFmtType {}
    }

    // Java: static boolean isTextFormat(short type)
    pub fn isTextFormat(fmt_type: i16) -> bool {
      // Java: return type == 64 || type == 1 || type == 128 || type == 0;
      fmt_type == 64 || fmt_type == 1 || fmt_type == 128 || fmt_type == 0
    }

}
