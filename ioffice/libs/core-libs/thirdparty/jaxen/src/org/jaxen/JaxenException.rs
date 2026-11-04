use super::saxpath::SAXPathException::SAXPathException;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -5386352985042209375;
// Java field: double javaVersion
pub(crate) const JAVA_VERSION: f64 = 1.4;

// Java: public class JaxenException extends SAXPathException
#[derive(Debug)]
pub struct JaxenException {
    // Java field: String message (from SAXPathException/Exception)
    pub(crate) message: String,
    // Java field: Throwable cause
    pub(crate) cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

// Java: public class JaxenException extends SAXPathException
impl JaxenException {
    // Java: JaxenException(String message)
    pub fn new(message: String) -> Self {
        // Java: super(message);
        JaxenException { message, cause: None }
    }

    // Java: JaxenException(Throwable rootCause)
    pub fn from_cause(root_cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(rootCause);
        let message = root_cause.to_string();
        JaxenException { message, cause: Some(root_cause) }
    }

    // Java: JaxenException(String message, Throwable nestedException)
    pub fn new_with_cause(message: String, nested_exception: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(message, nestedException);
        JaxenException { message, cause: Some(nested_exception) }
    }
}

impl std::fmt::Display for JaxenException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for JaxenException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}
