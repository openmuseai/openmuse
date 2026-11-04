#![allow(non_snake_case, non_camel_case_types, unused_variables, unused_imports, dead_code, unreachable_code, unused_mut)]

// Java: public class PNumber
pub struct PNumber {
}

// Java: public class PInt
pub struct PInt {
    // Java field: int mInteger
    pub(crate) mInteger: i32,
}

// Java: public class PDouble
pub struct PDouble {
    // Java field: int mDouble
    pub(crate) mDouble: i32,
}

// Java: public class PNumber
impl PNumber {
    // Java: PNumber()
    pub fn new() -> Self {
        // Java: super();
        PNumber {}
    }
}

// Java: public class PInt
impl PInt {
    // Java: PInt()
    pub fn new() -> Self {
        // Java: super();
        PInt { mInteger: 0 }
    }
}

// Java: public class PDouble
impl PDouble {
    // Java: PDouble()
    pub fn new() -> Self {
        // Java: super();
        PDouble { mDouble: 0 }
    }
}
