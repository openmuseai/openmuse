#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java field: int left_top
pub(crate) const left_top: i32 = 0;
// Java field: int top
pub(crate) const top: i32 = 1;
// Java field: int right_top
pub(crate) const right_top: i32 = 2;
// Java field: int left
pub(crate) const left: i32 = 3;
// Java field: int center
pub(crate) const center: i32 = 4;
// Java field: int right
pub(crate) const right: i32 = 5;
// Java field: int left_bottom
pub(crate) const left_bottom: i32 = 6;
// Java field: int bottom
pub(crate) const bottom: i32 = 7;
// Java field: int right_bottom
pub(crate) const right_bottom: i32 = 8;

// Java: public class FillAlignment
pub struct FillAlignment {
}

// Java: public class FillAlignment
impl FillAlignment {
    // Java: FillAlignment()
    pub fn new() -> Self {
      // Java: super();
      FillAlignment {}
    }

    // Java: static boolean alignRight(int type)
    pub fn alignRight(r#type: i32) -> bool {
      // Java: return type == 2 || type == 5 || type == 8;
      r#type == 2 || r#type == 5 || r#type == 8
    }

    // Java: static boolean alignBottom(int type)
    pub fn alignBottom(r#type: i32) -> bool {
      // Java: return type == 7 || type == 8 || type == 6;
      r#type == 7 || r#type == 8 || r#type == 6
    }

    // Java: static boolean alignLeft(int type)
    pub fn alignLeft(r#type: i32) -> bool {
      // Java: return type == 0 || type == 3 || type == 6;
      r#type == 0 || r#type == 3 || r#type == 6
    }

    // Java: static boolean alignTop(int type)
    pub fn alignTop(r#type: i32) -> bool {
      // Java: return type == 0 || type == 1 || type == 2;
      r#type == 0 || r#type == 1 || r#type == 2
    }

}
