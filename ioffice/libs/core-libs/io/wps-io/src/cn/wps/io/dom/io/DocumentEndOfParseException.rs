use std::fmt;

// Java: public class DocumentEndOfParseException extends SAXException
#[derive(Debug)]
pub struct DocumentEndOfParseException {
    pub(crate) message: String,
    pub(crate) cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl fmt::Display for DocumentEndOfParseException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DocumentEndOfParseException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// Java: public class DocumentEndOfParseException extends SAXException
impl DocumentEndOfParseException {
    // Java: DocumentEndOfParseException()
    pub fn new() -> Self {
        // Java: super();
        DocumentEndOfParseException {
            message: String::new(),
            cause: None,
        }
    }

    // Java: DocumentEndOfParseException(String message)
    pub fn new_with_message(message: String) -> Self {
        // Java: super(message);
        DocumentEndOfParseException {
            message,
            cause: None,
        }
    }

    // Java: DocumentEndOfParseException(Exception e)
    pub fn new_with_cause(e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super();
        DocumentEndOfParseException {
            message: String::new(),
            cause: Some(e),
        }
    }

    // Java: DocumentEndOfParseException(String message, Exception e)
    pub fn new_with_message_cause(message: String, e: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(message, e);
        DocumentEndOfParseException {
            message,
            cause: Some(e),
        }
    }
}
