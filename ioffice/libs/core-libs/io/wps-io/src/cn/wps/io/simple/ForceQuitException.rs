// Java: import org.xml.sax.SAXException
use std::fmt;

// Java: public class ForceQuitException extends SAXException
#[derive(Debug)]
pub struct ForceQuitException {
    // Java field: int errorCode
    pub(crate) errorCode: i32,
}

impl fmt::Display for ForceQuitException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ForceQuitException(errorCode={})", self.errorCode)
    }
}

impl std::error::Error for ForceQuitException {}

// Java: public class ForceQuitException extends SAXException
impl ForceQuitException {
    // Java: ForceQuitException()
    pub fn new() -> Self {
      // Java: this(-1);
      ForceQuitException { errorCode: -1 }
    }

    // Java: ForceQuitException(int errorCode)
    pub fn new_with_code(error_code: i32) -> Self {
      // Java: super();
      // Java: this.errorCode = errorCode;
      ForceQuitException { errorCode: error_code }
    }

    // Java: int getErrorCode()
    pub fn getErrorCode(&self) -> i32 {
      // Java: return this.errorCode;
      self.errorCode
    }

}
