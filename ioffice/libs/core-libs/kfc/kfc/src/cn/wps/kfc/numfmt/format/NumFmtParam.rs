#![allow(non_snake_case)]
#![allow(dead_code)]

// Java: public class NumFmtParam
#[derive(Clone)]
pub struct NumFmtParam {
    // Java field: int mMaxLength
    pub mMaxLength: i32,
    // Java field: boolean isUse1904
    pub isUse1904: bool,
}

// Java: public class NumFmtParam
impl NumFmtParam {
    // Java: NumFmtParam()
    pub fn new() -> Self {
      // Java: super();
      NumFmtParam {
          mMaxLength: 0,
          isUse1904: false,
      }
    }

}
