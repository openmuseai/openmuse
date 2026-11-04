use std::fmt;

// Java: public class POIXMLException extends RuntimeException
#[derive(Debug)]
pub struct POIXMLException {
    // Java field: long serialVersionUID
    pub(crate) message: String,
    pub(crate) cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

impl fmt::Display for POIXMLException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for POIXMLException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}

// Java: public class POIXMLException extends RuntimeException
impl POIXMLException {
    // Java: POIXMLException()
    pub fn new() -> Self {
        // Java: super();
        POIXMLException {
            message: String::new(),
            cause: None,
        }
    }

    // Java: POIXMLException(String msg)
    pub fn new_with_message(msg: String) -> Self {
        // Java: super(msg);
        POIXMLException {
            message: msg,
            cause: None,
        }
    }

    // Java: POIXMLException(String msg, Throwable cause)
    pub fn new_with_message_cause(msg: String, cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg, cause);
        POIXMLException {
            message: msg,
            cause: Some(cause),
        }
    }

    // Java: POIXMLException(Throwable cause)
    pub fn new_with_cause(cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(cause);
        let message = cause.to_string();
        POIXMLException {
            message,
            cause: Some(cause),
        }
    }
}
