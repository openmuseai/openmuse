#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::fmt;

// Java: public class SuffixErrorException extends RuntimeException
pub struct SuffixErrorException {
    pub message: String,
}

// Java: public class SuffixErrorException extends RuntimeException
impl SuffixErrorException {
    // Java: SuffixErrorException()
    pub fn new() -> Self {
        // Java: super();
        SuffixErrorException { message: String::new() }
    }

    // Java: SuffixErrorException(String detailMessage)
    pub fn with_message(detailMessage: String) -> Self {
        // Java: super(detailMessage);
        SuffixErrorException { message: detailMessage }
    }

    // Java: SuffixErrorException(String detailMessage, Throwable throwable)
    pub fn with_message_and_cause(detailMessage: String, _throwable: &str) -> Self {
        // Java: super(detailMessage, throwable);
        SuffixErrorException { message: detailMessage }
    }

    // Java: SuffixErrorException(Throwable throwable)
    pub fn with_cause(_throwable: &str) -> Self {
        // Java: super(throwable);
        SuffixErrorException { message: _throwable.to_string() }
    }
}

impl fmt::Display for SuffixErrorException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SuffixErrorException: {}", self.message)
    }
}
