use super::NumFmtParam::NumFmtParam;

// Java: public class WordFmtParam extends NumFmtParam
pub struct WordFmtParam {
    pub(crate) base: NumFmtParam,
    // Java field: boolean mHasRTLFlag
    pub(crate) mHasRTLFlag: bool,
    // Java field: short mDefEastAsiaLID
    pub(crate) mDefEastAsiaLID: i16,
    // Java field: short mDefLatinLID
    pub(crate) mDefLatinLID: i16,
    // Java field: short mDefBidiLID
    pub(crate) mDefBidiLID: i16,
}

// Java: public class WordFmtParam extends NumFmtParam
impl WordFmtParam {
    // Java: WordFmtParam()
    pub fn new() -> Self {
      // Java: super();
      WordFmtParam {
          base: NumFmtParam::new(),
          mHasRTLFlag: false,
          mDefEastAsiaLID: 0,
          mDefLatinLID: 0,
          mDefBidiLID: 0,
      }
    }

}
