use std::fmt;

// Java: public class PropertyException extends RuntimeException
#[derive(Debug)]
pub struct PropertyException {
    // Java field: long serialVersionUID
    pub message: String,
}

// Java: public class PropertyException extends RuntimeException
impl PropertyException {
    // Java: PropertyException(String reason)
    pub fn new(reason: &str) -> Self {
        // Java: super(reason);
        Self { message: reason.to_string() }
    }
}

impl fmt::Display for PropertyException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PropertyException: {}", self.message)
    }
}

impl std::error::Error for PropertyException {}
