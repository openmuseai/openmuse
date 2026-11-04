use std::fmt;
use std::error::Error;

// Java: public class XPathException extends RuntimeException
#[derive(Debug)]
pub struct XPathException {
    // Java field: String xpath
    pub(crate) xpath: String,
    pub(crate) message: String,
}

// Java: public class XPathException extends RuntimeException
impl XPathException {
    // Java: XPathException(String xpath)
    pub fn new(xpath: &str) -> Self {
        // Java: super("Exception occurred evaluting XPath: " + xpath);
        let message = format!("Exception occurred evaluting XPath: {}", xpath);
        // Java: this.xpath = xpath;
        XPathException {
            xpath: xpath.to_string(),
            message,
        }
    }

    // Java: XPathException(String xpath, String reason)
    pub fn new_with_reason(xpath: &str, reason: &str) -> Self {
        // Java: super("Exception occurred evaluting XPath: " + xpath + " " + reason);
        let message = format!("Exception occurred evaluting XPath: {} {}", xpath, reason);
        // Java: this.xpath = xpath;
        XPathException {
            xpath: xpath.to_string(),
            message,
        }
    }

    // Java: XPathException(String xpath, Exception e)
    pub fn new_with_cause(xpath: &str, e: &dyn Error) -> Self {
        // Java: super("Exception occurred evaluting XPath: " + xpath + ". Exception: " + e.getMessage());
        let message = format!("Exception occurred evaluting XPath: {}. Exception: {}", xpath, e);
        // Java: this.xpath = xpath;
        XPathException {
            xpath: xpath.to_string(),
            message,
        }
    }

    // Java: String getXPath()
    pub fn getXPath(&self) -> &str {
        // Java: return this.xpath;
        &self.xpath
    }
}

impl fmt::Display for XPathException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Error for XPathException {}
