use std::fmt;

// Java: public class OpenXML4JException extends Exception
#[derive(Debug)]
pub struct OpenXML4JException {
    pub(crate) message: String,
}

impl fmt::Display for OpenXML4JException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OpenXML4JException {}

// Java: public class OpenXML4JException extends Exception
impl OpenXML4JException {
    // Java: OpenXML4JException(String msg)
    pub fn new(msg: String) -> Self {
        // Java: super(msg);
        OpenXML4JException { message: msg }
    }
}
