use super::NumFmtStyle::NumFmtStyle;

// Java: public class WordFmtStyle extends NumFmtStyle
pub struct WordFmtStyle {
    pub(crate) base: NumFmtStyle,
    // Java field: boolean mHasRTLFlag
    pub(crate) mHasRTLFlag: bool,
    // Java field: short mDefEastAsiaLID
    pub(crate) mDefEastAsiaLID: i16,
    // Java field: short mDefLatinLID
    pub(crate) mDefLatinLID: i16,
    // Java field: short mDefBidiLID
    pub(crate) mDefBidiLID: i16,
}

// Java: public class WordFmtStyle extends NumFmtStyle
impl WordFmtStyle {
    // Java: WordFmtStyle()
    pub fn new() -> Self {
      // Java: super();
      WordFmtStyle {
          base: NumFmtStyle::new(),
          mHasRTLFlag: false,
          mDefEastAsiaLID: 0,
          mDefLatinLID: 0,
          mDefBidiLID: 0,
      }
    }

}
