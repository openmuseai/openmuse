use std::fmt;

// Java: public class InvalidFormatException extends OpenXML4JException
#[derive(Debug)]
pub struct InvalidFormatException {
    pub(crate) message: String,
}

impl fmt::Display for InvalidFormatException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InvalidFormatException {}

// Java: public class InvalidFormatException extends OpenXML4JException
impl InvalidFormatException {
    // Java: InvalidFormatException(String message)
    pub fn new(message: String) -> Self {
        // Java: super(message);
        InvalidFormatException { message }
    }
}
