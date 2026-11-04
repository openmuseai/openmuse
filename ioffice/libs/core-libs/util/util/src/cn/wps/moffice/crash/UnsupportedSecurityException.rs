#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::fmt;

// Java: public class UnsupportedSecurityException extends RuntimeException
pub struct UnsupportedSecurityException {
    pub message: String,
}

// Java: public class UnsupportedSecurityException extends RuntimeException
impl UnsupportedSecurityException {
    // Java: UnsupportedSecurityException()
    pub fn new() -> Self {
        // Java: super();
        UnsupportedSecurityException { message: String::new() }
    }

    // Java: UnsupportedSecurityException(String detailMessage)
    pub fn with_message(detailMessage: String) -> Self {
        // Java: super(detailMessage);
        UnsupportedSecurityException { message: detailMessage }
    }

    // Java: UnsupportedSecurityException(String detailMessage, Throwable throwable)
    pub fn with_message_and_cause(detailMessage: String, _throwable: &str) -> Self {
        // Java: super(detailMessage, throwable);
        UnsupportedSecurityException { message: detailMessage }
    }

    // Java: UnsupportedSecurityException(Throwable throwable)
    pub fn with_cause(_throwable: &str) -> Self {
        // Java: super(throwable);
        UnsupportedSecurityException { message: _throwable.to_string() }
    }
}

impl fmt::Display for UnsupportedSecurityException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "UnsupportedSecurityException: {}", self.message)
    }
}
