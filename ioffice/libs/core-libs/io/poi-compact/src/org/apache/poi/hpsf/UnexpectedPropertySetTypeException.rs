use super::HPSFException::HPSFException;

// Java: public class UnexpectedPropertySetTypeException extends HPSFException
pub struct UnexpectedPropertySetTypeException {
    pub(crate) base: HPSFException,
}

// Java: public class UnexpectedPropertySetTypeException extends HPSFException
impl UnexpectedPropertySetTypeException {
    // Java: UnexpectedPropertySetTypeException()
    pub fn new() -> Self {
        // Java: super();
        UnexpectedPropertySetTypeException {
            base: HPSFException::new(),
        }
    }

    // Java: UnexpectedPropertySetTypeException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        UnexpectedPropertySetTypeException {
            base: HPSFException::new_with_msg(msg),
        }
    }

    // Java: UnexpectedPropertySetTypeException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(reason);
        UnexpectedPropertySetTypeException {
            base: HPSFException::new_with_reason(reason),
        }
    }

    // Java: UnexpectedPropertySetTypeException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg, reason);
        UnexpectedPropertySetTypeException {
            base: HPSFException::new_with_msg_and_reason(msg, reason),
        }
    }
}
