use super::HPSFRuntimeException::HPSFRuntimeException;

// Java: public class NoSingleSectionException extends HPSFRuntimeException
pub struct NoSingleSectionException {
    pub(crate) base: HPSFRuntimeException,
}

// Java: public class NoSingleSectionException extends HPSFRuntimeException
impl NoSingleSectionException {
    // Java: NoSingleSectionException()
    pub fn new() -> Self {
        // Java: super();
        NoSingleSectionException {
            base: HPSFRuntimeException::new(),
        }
    }

    // Java: NoSingleSectionException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        NoSingleSectionException {
            base: HPSFRuntimeException::new_with_msg(msg),
        }
    }

    // Java: NoSingleSectionException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(reason);
        NoSingleSectionException {
            base: HPSFRuntimeException::new_with_reason(reason),
        }
    }

    // Java: NoSingleSectionException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg, reason);
        NoSingleSectionException {
            base: HPSFRuntimeException::new_with_msg_and_reason(msg, reason),
        }
    }
}
