use std::fmt;
use std::error::Error;

// Java: public class RecordFormatException extends RuntimeException
#[derive(Debug)]
pub struct RecordFormatException {
    message: Option<String>,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for RecordFormatException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message { Some(msg) => write!(f, "{}", msg), None => write!(f, "RecordFormatException") }
    }
}
impl Error for RecordFormatException {}

// Java: public class RecordFormatException extends RuntimeException
impl RecordFormatException {
    // Java: RecordFormatException(String exception)
    pub fn new(exception: String) -> Self {
        // Java: super(exception);
        RecordFormatException { message: Some(exception), source: None }
    }
    // Java: RecordFormatException(String exception, Throwable thr)
    pub fn new_with_cause(exception: String, thr: Box<dyn Error + Send + Sync>) -> Self {
        // Java: super(exception, thr);
        RecordFormatException { message: Some(exception), source: Some(thr) }
    }
    // Java: RecordFormatException(Throwable thr)
    pub fn from_cause(thr: Box<dyn Error + Send + Sync>) -> Self {
        // Java: super(thr);
        RecordFormatException { message: None, source: Some(thr) }
    }
}
