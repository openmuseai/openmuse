// Java: import org.xml.sax.SAXException
use std::fmt;

// Java: public class ReadingLimitException extends SAXException
#[derive(Debug)]
pub struct ReadingLimitException {
}

impl fmt::Display for ReadingLimitException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "ReadingLimitException")
    }
}

impl std::error::Error for ReadingLimitException {}

// Java: public class ReadingLimitException extends SAXException
impl ReadingLimitException {
    // Java: ReadingLimitException()
    pub fn new() -> Self {
      // Java: super();
      ReadingLimitException {}
    }

}
