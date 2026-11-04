use super::HPSFRuntimeException::HPSFRuntimeException;

// Java: public class MissingSectionException extends HPSFRuntimeException
pub struct MissingSectionException {
    pub(crate) base: HPSFRuntimeException,
}

// Java: public class MissingSectionException extends HPSFRuntimeException
impl MissingSectionException {
    // Java: MissingSectionException()
    pub fn new() -> Self {
        // Java: super();
        MissingSectionException {
            base: HPSFRuntimeException::new(),
        }
    }

    // Java: MissingSectionException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        MissingSectionException {
            base: HPSFRuntimeException::new_with_msg(msg),
        }
    }

    // Java: MissingSectionException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(reason);
        MissingSectionException {
            base: HPSFRuntimeException::new_with_reason(reason),
        }
    }

    // Java: MissingSectionException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg, reason);
        MissingSectionException {
            base: HPSFRuntimeException::new_with_msg_and_reason(msg, reason),
        }
    }
}
