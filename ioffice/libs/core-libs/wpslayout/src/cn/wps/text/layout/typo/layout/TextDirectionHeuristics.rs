#![allow(non_snake_case)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

// Java: public class TextDirectionHeuristics
pub struct TextDirectionHeuristics {
    // Java field: int LTR
    // Java field: int RTL
    // Java field: int FIRSTSTRONG_LTR
    // Java field: int FIRSTSTRONG_RTL
    // Java field: int ANYRTL_LTR
}

pub const LTR: i32 = 0;
pub const RTL: i32 = 1;
pub const FIRSTSTRONG_LTR: i32 = 2;
pub const FIRSTSTRONG_RTL: i32 = 3;
pub const ANYRTL_LTR: i32 = 4;

// Java: public class TextDirectionHeuristics
impl TextDirectionHeuristics {
    // Java: TextDirectionHeuristics()
    pub fn new() -> Self {
      // Java: super();
      TextDirectionHeuristics {}
    }
}
