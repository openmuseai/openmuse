// Java: import java.util.List

// Java: public class RichTextRange
pub struct RichTextRange {
    // Java field: int startRange
    pub(crate) startRange: i32,
    // Java field: int endRange
    pub(crate) endRange: i32,
    // Java field: String element
    pub(crate) element: String,
    // Java field: List<String> proList
    pub(crate) proList: Vec<String>,
    // Java field: List<String> styleList
    pub(crate) styleList: Vec<String>,
}

// Java: public class RichTextRange
impl RichTextRange {
    // Java: RichTextRange()
    pub fn new() -> Self {
      // Java: super();
      RichTextRange {
          startRange: 0,
          endRange: 0,
          element: String::new(),
          proList: Vec::new(),
          styleList: Vec::new(),
      }
    }

}
