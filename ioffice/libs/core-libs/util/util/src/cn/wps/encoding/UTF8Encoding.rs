#![allow(dead_code, non_snake_case)]

// Java: public class UTF8Encoding extends Encoding
pub struct UTF8Encoding;

// Java: public class UTF8Encoding extends Encoding
impl UTF8Encoding {
    // Java: UTF8Encoding()
    pub fn new() -> Self {
        // Java: super();
        UTF8Encoding
    }

    // Java: String bodyName()
    pub fn bodyName(&self) -> &str {
        // Java: return "utf-8";
        "utf-8"
    }
}
