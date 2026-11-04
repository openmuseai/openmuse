#![allow(non_snake_case, non_upper_case_globals, dead_code)]

// Java field: int None
pub const NONE: i32 = -1;
// Java field: int Bottom
pub const Bottom: i32 = 0;
// Java field: int BottomLeft
pub const BottomLeft: i32 = 1;
// Java field: int BottomRight
pub const BottomRight: i32 = 2;
// Java field: int Center
pub const Center: i32 = 3;
// Java field: int Left
pub const Left: i32 = 4;
// Java field: int Right
pub const Right: i32 = 5;
// Java field: int Top
pub const Top: i32 = 6;
// Java field: int TopLeft
pub const TopLeft: i32 = 7;
// Java field: int TopRight
pub const TopRight: i32 = 8;

// Java: public class RectAlignmentType
pub struct RectAlignmentType {
    // Java field: int None
    // Java field: int Bottom
    // Java field: int BottomLeft
    // Java field: int BottomRight
    // Java field: int Center
    // Java field: int Left
    // Java field: int Right
    // Java field: int Top
    // Java field: int TopLeft
    // Java field: int TopRight
}

// Java: public class RectAlignmentType
impl RectAlignmentType {
    // Java: RectAlignmentType()
    pub fn new() -> Self {
      // Java: super();
      RectAlignmentType {}
    }

    // Java: static boolean alignRight(int type)
    pub fn alignRight(r#type: i32) -> bool {
      // Java: return type == 5 || type == 8 || type == 2;
      r#type == 5 || r#type == 8 || r#type == 2
    }

    // Java: static boolean alignBottom(int type)
    pub fn alignBottom(r#type: i32) -> bool {
      // Java: return type == 0 || type == 1 || type == 2;
      r#type == 0 || r#type == 1 || r#type == 2
    }

    // Java: static boolean alignLeft(int type)
    pub fn alignLeft(r#type: i32) -> bool {
      // Java: return type == 4 || type == 7 || type == 2;
      r#type == 4 || r#type == 7 || r#type == 2
    }

    // Java: static boolean alignTop(int type)
    pub fn alignTop(r#type: i32) -> bool {
      // Java: return type == 6 || type == 7 || type == 8;
      r#type == 6 || r#type == 7 || r#type == 8
    }

}
