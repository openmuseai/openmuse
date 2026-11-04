use super::TextAutoNum::TextAutoNum;

// Java: public class PlainTextAutoNum extends TextAutoNum
pub struct PlainTextAutoNum {
}

// Java: public class PlainTextAutoNum extends TextAutoNum
impl PlainTextAutoNum {
    // Java: String convert(int value)
    pub fn convert(&self, value: i32) -> String {
      // Java: return intToStr(value);
      TextAutoNum::intToStr(value)
    }

}
