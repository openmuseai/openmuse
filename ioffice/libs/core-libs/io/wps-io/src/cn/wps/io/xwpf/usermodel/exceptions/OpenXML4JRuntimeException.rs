use std::fmt;

// Java: public class OpenXML4JRuntimeException extends RuntimeException
#[derive(Debug)]
pub struct OpenXML4JRuntimeException {
    pub(crate) message: String,
}

impl fmt::Display for OpenXML4JRuntimeException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for OpenXML4JRuntimeException {}

// Java: public class OpenXML4JRuntimeException extends RuntimeException
impl OpenXML4JRuntimeException {
    // Java: OpenXML4JRuntimeException(String msg)
    pub fn new(msg: String) -> Self {
        // Java: super(msg);
        OpenXML4JRuntimeException { message: msg }
    }
}
