use std::fmt;
use std::error::Error;

// Java field: String ENOSPC
pub(crate) const ENOSPC: &str = "No space left";
// Java field: String ENOENT
pub(crate) const ENOENT: &str = "No such file or directory";

// Java: public class NoSpaceLeftException extends RuntimeException
#[derive(Debug)]
pub struct NoSpaceLeftException {
    message: Option<String>,
    source: Option<Box<dyn Error + Send + Sync>>,
}

impl fmt::Display for NoSpaceLeftException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "{}", msg),
            None => write!(f, "NoSpaceLeftException"),
        }
    }
}

impl Error for NoSpaceLeftException {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_ref().map(|e| e.as_ref() as &(dyn Error + 'static))
    }
}

// Java: public class NoSpaceLeftException extends RuntimeException
impl NoSpaceLeftException {
    // Java: NoSpaceLeftException()
    pub fn new() -> Self {
        // Java: super();
        NoSpaceLeftException { message: None, source: None }
    }

    // Java: NoSpaceLeftException(String detailMessage)
    pub fn new_with_message(detailMessage: String) -> Self {
        // Java: super(detailMessage);
        NoSpaceLeftException { message: Some(detailMessage), source: None }
    }

    // Java: NoSpaceLeftException(String detailMessage, Throwable throwable)
    pub fn new_with_message_and_cause(detailMessage: String, throwable: Box<dyn Error + Send + Sync>) -> Self {
        // Java: super(detailMessage, throwable);
        NoSpaceLeftException { message: Some(detailMessage), source: Some(throwable) }
    }

    // Java: NoSpaceLeftException(Throwable throwable)
    pub fn new_with_cause(throwable: Box<dyn Error + Send + Sync>) -> Self {
        // Java: super(throwable);
        NoSpaceLeftException { message: None, source: Some(throwable) }
    }

    // Java: static boolean isNoSpaceLeftException(Throwable throwable)
    pub fn isNoSpaceLeftException(throwable: Option<&dyn Error>) -> bool {
        // Java: if (null == throwable)
        if throwable.is_none() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: String msg = throwable.getMessage();
            let msg = throwable.unwrap().to_string();
            // Java: return msg != null && (msg.indexOf("No space left") != -1 || msg.indexOf("No such file or directory") != -1);
            msg.contains(ENOSPC) || msg.contains(ENOENT)
        }
    }

    // Java: static boolean isOnlyNoSpaceLeftException(Throwable throwable)
    pub fn isOnlyNoSpaceLeftException(throwable: Option<&dyn Error>) -> bool {
        // Java: if (null == throwable)
        if throwable.is_none() {
            // Java: return false;
            return false;
        // Java: else
        } else {
            // Java: String msg = throwable.getMessage();
            let msg = throwable.unwrap().to_string();
            // Java: return msg != null && msg.indexOf("No space left") != -1;
            msg.contains(ENOSPC)
        }
    }
}
