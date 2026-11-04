use super::UofHandlerBase::UofHandlerBase;

// Java: public class UosHandlerBase extends UofHandlerBase
pub struct UosHandlerBase {
    pub(crate) base: UofHandlerBase,
}

// Java: public class UosHandlerBase extends UofHandlerBase
impl UosHandlerBase {
    // Java: UosHandlerBase()
    pub fn UosHandlerBase() -> Self {
      // Java: super();
      UosHandlerBase {
          base: UofHandlerBase::UofHandlerBase(),
      }
    }

    // Java: String parseString(char[] buffer, int start, int end)
    pub fn parseString(&self, buffer: &[char], start: i32, end: i32) -> String {
      // Java: return new String(buffer, 0, end - start);
      buffer[0..(end - start) as usize].iter().collect()
    }

}
