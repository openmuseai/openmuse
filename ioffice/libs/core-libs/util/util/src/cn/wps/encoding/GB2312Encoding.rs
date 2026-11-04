#![allow(dead_code, non_snake_case)]

// Java: public class GB2312Encoding extends Encoding
pub struct GB2312Encoding;

// Java: public class GB2312Encoding extends Encoding
impl GB2312Encoding {
    // Java: GB2312Encoding()
    pub fn new() -> Self {
        // Java: super();
        GB2312Encoding
    }

    // Java: String bodyName()
    pub fn bodyName(&self) -> &str {
        // Java: return "gb2312";
        "gb2312"
    }
}
