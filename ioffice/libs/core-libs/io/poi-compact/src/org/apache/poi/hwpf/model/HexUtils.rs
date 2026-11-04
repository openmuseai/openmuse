// Java: public class HexUtils
pub struct HexUtils;

// Java: public class HexUtils
impl HexUtils {
    pub fn new() -> Self { HexUtils }
    pub fn int2UpperChar(&self, i: i32) -> char {
        let c = (i & 0xF) as u8;
        if c < 10 { (b'0' + c) as char } else { (b'A' + c - 10) as char }
    }
    pub fn int2LowerChar(&self, i: i32) -> char {
        let c = (i & 0xF) as u8;
        if c < 10 { (b'0' + c) as char } else { (b'a' + c - 10) as char }
    }
    pub fn int2CharArray(&self, charArray: &mut [char], offset: i32, value: i32, lowercase: bool) -> i32 {
        let _ = (charArray, offset, value, lowercase);
        0
    }
}
