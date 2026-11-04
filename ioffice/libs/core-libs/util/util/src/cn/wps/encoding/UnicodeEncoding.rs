#![allow(dead_code, non_snake_case)]

// Java: public class UnicodeEncoding extends Encoding
pub struct UnicodeEncoding;

// Java: public class UnicodeEncoding extends Encoding
impl UnicodeEncoding {
    // Java: UnicodeEncoding()
    pub fn new() -> Self {
        // Java: super();
        UnicodeEncoding
    }

    // Java: String bodyName()
    pub fn bodyName(&self) -> &str {
        // Java: return "utf-16be";
        "utf-16be"
    }
}
