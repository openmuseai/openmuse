use std::fmt;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 8054940496400696935;

// Java: public class AtomException extends Exception
pub struct AtomException {
    pub message: String,
}

// Java: public class AtomException extends Exception
impl AtomException {
    // Java: AtomException()
    pub fn new() -> Self {
        // Java: super();
        AtomException {
            message: String::new(),
        }
    }
}

impl fmt::Display for AtomException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AtomException: {}", self.message)
    }
}

impl fmt::Debug for AtomException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "AtomException {{ message: {:?} }}", self.message)
    }
}

impl std::error::Error for AtomException {}
