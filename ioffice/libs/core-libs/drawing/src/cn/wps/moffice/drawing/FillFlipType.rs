#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java field: int FLIP_NONE
pub(crate) const FLIP_NONE: i32 = 0;
// Java field: int FLIP_H
pub(crate) const FLIP_H: i32 = 1;
// Java field: int FLIP_V
pub(crate) const FLIP_V: i32 = 16;
// Java field: int FLIP_H_V
pub(crate) const FLIP_H_V: i32 = 17;

// Java: public class FillFlipType
pub struct FillFlipType {
}

// Java: public class FillFlipType
impl FillFlipType {
    // Java: FillFlipType()
    pub fn new() -> Self {
      // Java: super();
      FillFlipType {}
    }

    // Java: static boolean isFlipH(int type)
    pub fn isFlipH(r#type: i32) -> bool {
      // Java: return (type & 1) == 1;
      (r#type & 1) == 1
    }

    // Java: static boolean isFlipV(int type)
    pub fn isFlipV(r#type: i32) -> bool {
      // Java: return (type & 16) == 16;
      (r#type & 16) == 16
    }

}
