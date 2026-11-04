use super::HPSFException::HPSFException;

// Java: public class NoPropertySetStreamException extends HPSFException
pub struct NoPropertySetStreamException {
    pub(crate) base: HPSFException,
}

// Java: public class NoPropertySetStreamException extends HPSFException
impl NoPropertySetStreamException {
    // Java: NoPropertySetStreamException()
    pub fn new() -> Self {
        // Java: super();
        NoPropertySetStreamException {
            base: HPSFException::new(),
        }
    }

    // Java: NoPropertySetStreamException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        NoPropertySetStreamException {
            base: HPSFException::new_with_msg(msg),
        }
    }

    // Java: NoPropertySetStreamException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(reason);
        NoPropertySetStreamException {
            base: HPSFException::new_with_reason(reason),
        }
    }

    // Java: NoPropertySetStreamException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg, reason);
        NoPropertySetStreamException {
            base: HPSFException::new_with_msg_and_reason(msg, reason),
        }
    }
}
