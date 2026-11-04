use super::HPSFException::HPSFException;

// Java: public class MarkUnsupportedException extends HPSFException
pub struct MarkUnsupportedException {
    pub(crate) base: HPSFException,
}

// Java: public class MarkUnsupportedException extends HPSFException
impl MarkUnsupportedException {
    // Java: MarkUnsupportedException()
    pub fn new() -> Self {
        // Java: super();
        MarkUnsupportedException {
            base: HPSFException::new(),
        }
    }

    // Java: MarkUnsupportedException(String msg)
    pub fn new_with_msg(msg: &str) -> Self {
        // Java: super(msg);
        MarkUnsupportedException {
            base: HPSFException::new_with_msg(msg),
        }
    }

    // Java: MarkUnsupportedException(Throwable reason)
    pub fn new_with_reason(reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(reason);
        MarkUnsupportedException {
            base: HPSFException::new_with_reason(reason),
        }
    }

    // Java: MarkUnsupportedException(String msg, Throwable reason)
    pub fn new_with_msg_and_reason(msg: &str, reason: Box<dyn std::error::Error + Send + Sync>) -> Self {
        // Java: super(msg, reason);
        MarkUnsupportedException {
            base: HPSFException::new_with_msg_and_reason(msg, reason),
        }
    }
}
