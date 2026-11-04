use std::fmt;
use std::ops::Deref;

pub struct Exception {
    pub(crate) message: String,
}

impl Exception {
    pub fn new() -> Self {
        Exception { message: String::new() }
    }
    pub fn new_with_msg(msg: &str) -> Self {
        Exception { message: msg.to_string() }
    }
    pub fn getMessage(&self) -> &str {
        &self.message
    }
}

// Java: public class HPSFException extends Exception
pub struct HPSFException {
    // Java field: Throwable reason
    pub(crate) reason: Option<Box<dyn std::error::Error + Send + Sync>>,
    parent: Exception,
}

impl Deref for HPSFException {
    type Target = Exception;
    fn deref(&self) -> &Self::Target {
        &self.parent
    }
}

// Java: public class HPSFException extends Exception
impl HPSFException {
    // Java: HPSFException()
    pub fn new() -> Self {
        // Java: super();
        HPSFException {
            reason: None,
            parent: Exception::new(),
        }
    }

    // Java: HPSFException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        HPSFException {
            reason: None,
            parent: Exception::new_with_msg(msg),
        }
    }

    // Java: HPSFException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super();
        // Java: this.reason = reason;
        HPSFException {
            reason: Some(reason),
            parent: Exception::new(),
        }
    }

    // Java: HPSFException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg);
        // Java: this.reason = reason;
        HPSFException {
            reason: Some(reason),
            parent: Exception::new_with_msg(msg),
        }
    }

    // Java: Throwable getReason()
    pub fn getReason(&self) -> &Option<Box<dyn std::error::Error + Send + Sync>> {
        // Java: return this.reason;
        &self.reason
    }
}

impl fmt::Display for HPSFException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.parent.message)
    }
}

impl fmt::Debug for HPSFException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "HPSFException: {}", self.parent.message)
    }
}

impl std::error::Error for HPSFException {}
