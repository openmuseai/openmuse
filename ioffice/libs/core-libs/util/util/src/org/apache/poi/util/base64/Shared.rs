#![allow(non_snake_case, dead_code, unused_variables, non_upper_case_globals)]

// Java field: String chars
pub const CHARS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
// Java field: char pad
pub const PAD: char = '=';

// Java: public class Shared
pub struct Shared {
}

// Java: public class Shared
impl Shared {
    // Java: Shared()
    pub fn new() -> Self {
        // Java: super();
        Self {}
    }
}
