use std::fmt;

// Java: public class InvalidXPathException extends IllegalArgumentException
#[derive(Debug)]
pub struct InvalidXPathException {
    pub(crate) message: String,
}

// Java: public class InvalidXPathException extends IllegalArgumentException
impl InvalidXPathException {
    // Java: InvalidXPathException(String xpath)
    pub fn new(xpath: &str) -> Self {
        // Java: super("Invalid XPath expression: " + xpath);
        InvalidXPathException {
            message: format!("Invalid XPath expression: {}", xpath),
        }
    }

    // Java: InvalidXPathException(String xpath, String reason)
    pub fn new_with_reason(xpath: &str, reason: &str) -> Self {
        // Java: super("Invalid XPath expression: " + xpath + " " + reason);
        InvalidXPathException {
            message: format!("Invalid XPath expression: {} {}", xpath, reason),
        }
    }
}

impl fmt::Display for InvalidXPathException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for InvalidXPathException {}
