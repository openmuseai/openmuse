#![allow(non_snake_case, dead_code, unused_variables, unused_imports, unused_mut, unused_assignments, unused_parens)]

// Java field:    public static final int CHAR = 2;
pub const CHAR: i32 = 2;
// Java field:    public static final int DOUBLE = 8;
pub const DOUBLE: i32 = 8;
// Java field:    public static final int FLOAT = 4;
pub const FLOAT: i32 = 4;
// Java field:    public static final int INT = 4;
pub const INT: i32 = 4;
// Java field:    public static final int LONG = 8;
pub const LONG: i32 = 8;
// Java field:    public static final int SHORT = 2;
pub const SHORT: i32 = 2;

// Java: public class SizeOf 
#[derive(Default)]
pub struct SizeOf {}

// Java: public class SizeOf 
impl SizeOf {
    // Java: private SizeOf() 
    pub fn new() -> SizeOf {
        Self {}
    }
}
