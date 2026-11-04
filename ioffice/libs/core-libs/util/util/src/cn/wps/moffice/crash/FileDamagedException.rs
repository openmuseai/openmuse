use std::fmt;
use std::error::Error;

// Java: public class FileDamagedException extends RuntimeException
#[derive(Debug)]
pub struct FileDamagedException {
    message: Option<String>,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for FileDamagedException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message { Some(msg) => write!(f, "{}", msg), None => write!(f, "FileDamagedException") }
    }
}
impl Error for FileDamagedException {}

// Java: public class FileDamagedException extends RuntimeException
impl FileDamagedException {
    // Java: FileDamagedException()
    pub fn empty() -> Self {
        // Java: super();
        FileDamagedException { message: None, source: None }
    }
    // Java: FileDamagedException(String message)
    pub fn new(message: String) -> Self {
        // Java: super();
        FileDamagedException { message: Some(message), source: None }
    }
    // Java: FileDamagedException(Throwable throwable)
    pub fn from_cause(throwable: Box<dyn Error + Send + Sync>) -> Self {
        // Java: super(throwable);
        FileDamagedException { message: None, source: Some(throwable) }
    }
    // Java: FileDamagedException(String detailMessage, Throwable throwable)
    pub fn new_with_cause(detailMessage: String, throwable: Box<dyn Error + Send + Sync>) -> Self {
        // Java: super(detailMessage, throwable);
        FileDamagedException { message: Some(detailMessage), source: Some(throwable) }
    }
}
