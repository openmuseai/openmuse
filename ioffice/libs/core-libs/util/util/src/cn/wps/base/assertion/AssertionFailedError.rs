#![allow(non_snake_case, dead_code)]

use std::fmt;

// Java: public class AssertionFailedError extends Error
pub struct AssertionFailedError {
    message: String,
}

// Java: public class AssertionFailedError extends Error
impl AssertionFailedError {
    // Java: AssertionFailedError()
    pub fn new() -> Self {
        // Java: super();
        Self {
            message: String::new(),
        }
    }

    // Java: AssertionFailedError(String message)
    pub fn with_message(message: &str) -> Self {
        // Java: super(message);
        Self {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for AssertionFailedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for AssertionFailedError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AssertionFailedError: {}", self.message)
    }
}

impl std::error::Error for AssertionFailedError {}
