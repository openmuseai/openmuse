use super::HPSFRuntimeException::HPSFRuntimeException;

// Java: public class NoFormatIDException extends HPSFRuntimeException
pub struct NoFormatIDException {
    pub(crate) base: HPSFRuntimeException,
}

// Java: public class NoFormatIDException extends HPSFRuntimeException
impl NoFormatIDException {
    // Java: NoFormatIDException()
    pub fn new() -> Self {
        // Java: super();
        NoFormatIDException {
            base: HPSFRuntimeException::new(),
        }
    }

    // Java: NoFormatIDException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        NoFormatIDException {
            base: HPSFRuntimeException::new_with_msg(msg),
        }
    }

    // Java: NoFormatIDException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(reason);
        NoFormatIDException {
            base: HPSFRuntimeException::new_with_reason(reason),
        }
    }

    // Java: NoFormatIDException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg, reason);
        NoFormatIDException {
            base: HPSFRuntimeException::new_with_msg_and_reason(msg, reason),
        }
    }
}
