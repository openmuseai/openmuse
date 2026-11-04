#![allow(non_snake_case, dead_code, unused_variables, unused_imports)]

use std::fmt;

// Java: public class FileOpenException extends RuntimeException
pub struct FileOpenException {
    pub message: String,
}

// Java: public class FileOpenException extends RuntimeException
impl FileOpenException {
    // Java: FileOpenException()
    pub fn new() -> Self {
        // Java: super();
        FileOpenException { message: String::new() }
    }

    // Java: FileOpenException(String message)
    pub fn with_message(message: String) -> Self {
        // Java: super(message);
        FileOpenException { message }
    }

    // Java: FileOpenException(Throwable throwable)
    pub fn with_cause(_throwable: &str) -> Self {
        // Java: super(throwable);
        FileOpenException { message: _throwable.to_string() }
    }

    // Java: FileOpenException(String detailMessage, Throwable throwable)
    pub fn with_message_and_cause(detailMessage: String, _throwable: &str) -> Self {
        // Java: super(detailMessage, throwable);
        FileOpenException { message: detailMessage }
    }
}

impl fmt::Display for FileOpenException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FileOpenException: {}", self.message)
    }
}
