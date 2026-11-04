// Java: public class SAXModifyException extends RuntimeException
pub struct SAXModifyException {
    pub(crate) message: String,
}

// Java: public class SAXModifyException extends RuntimeException
impl SAXModifyException {
    // Java: SAXModifyException(Throwable cause)
    pub fn new(message: String) -> Self {
        // Java: super(cause);
        SAXModifyException { message }
    }
}

impl std::fmt::Display for SAXModifyException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SAXModifyException: {}", self.message)
    }
}
