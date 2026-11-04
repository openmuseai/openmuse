use super::HPSFRuntimeException::HPSFRuntimeException;
use std::ops::Deref;

// Java: public class IllegalPropertySetDataException extends HPSFRuntimeException
pub struct IllegalPropertySetDataException {
    pub(crate) base: HPSFRuntimeException,
}

impl Deref for IllegalPropertySetDataException {
    type Target = HPSFRuntimeException;
    fn deref(&self) -> &Self::Target {
        &self.base
    }
}

// Java: public class IllegalPropertySetDataException extends HPSFRuntimeException
impl IllegalPropertySetDataException {
    // Java: IllegalPropertySetDataException()
    pub fn new() -> Self {
        // Java: super();
        IllegalPropertySetDataException {
            base: HPSFRuntimeException::new(),
        }
    }

    // Java: IllegalPropertySetDataException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        IllegalPropertySetDataException {
            base: HPSFRuntimeException::new_with_msg(msg),
        }
    }

    // Java: IllegalPropertySetDataException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(reason);
        IllegalPropertySetDataException {
            base: HPSFRuntimeException::new_with_reason(reason),
        }
    }

    // Java: IllegalPropertySetDataException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg, reason);
        IllegalPropertySetDataException {
            base: HPSFRuntimeException::new_with_msg_and_reason(msg, reason),
        }
    }
}
