use super::JaxenException::JaxenException;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 7908649612495640943;

// Java: public class FunctionCallException extends JaxenException
#[derive(Debug)]
pub struct FunctionCallException {
    pub(crate) message: String,
    pub(crate) cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

// Java: public class FunctionCallException extends JaxenException
impl FunctionCallException {
    // Java: FunctionCallException(String message)
    pub fn new(message: String) -> Self {
        // Java: super(message);
        FunctionCallException { message, cause: None }
    }

    // Java: FunctionCallException(Throwable nestedException)
    pub fn from_cause(nested_exception: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(nestedException);
        let message = nested_exception.to_string();
        FunctionCallException { message, cause: Some(nested_exception) }
    }

    // Java: FunctionCallException(String message, Exception nestedException)
    pub fn new_with_cause(message: String, nested_exception: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(message, nestedException);
        FunctionCallException { message, cause: Some(nested_exception) }
    }

    // Java: Throwable getNestedException()
    pub fn getNestedException(&self) -> Option<&(dyn std::error::Error + Send + Sync)> {
        // Java: return this.getCause();
        self.cause.as_deref()
    }
}

impl std::fmt::Display for FunctionCallException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for FunctionCallException {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        self.cause.as_ref().map(|e| e.as_ref() as &(dyn std::error::Error + 'static))
    }
}
