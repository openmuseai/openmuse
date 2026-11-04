use std::fmt;
use super::OpenXML4JRuntimeException::OpenXML4JRuntimeException;

// Java: public class InvalidOperationException extends OpenXML4JRuntimeException
#[derive(Debug)]
pub struct InvalidOperationException {
    pub(crate) message: String,
}

impl fmt::Display for InvalidOperationException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InvalidOperationException {}

// Java: public class InvalidOperationException extends OpenXML4JRuntimeException
impl InvalidOperationException {
    // Java: InvalidOperationException(String message)
    pub fn new(message: String) -> Self {
        // Java: super(message);
        InvalidOperationException { message }
    }
}
