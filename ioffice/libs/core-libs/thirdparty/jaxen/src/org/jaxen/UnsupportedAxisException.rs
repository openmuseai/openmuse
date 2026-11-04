use super::JaxenException::JaxenException;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -9042756278183969899;

// Java: public class UnsupportedAxisException extends JaxenException
#[derive(Debug)]
pub struct UnsupportedAxisException {
    pub(crate) message: String,
}

// Java: public class UnsupportedAxisException extends JaxenException
impl UnsupportedAxisException {
    // Java: UnsupportedAxisException(String message)
    pub fn new(message: String) -> Self {
        // Java: super(message);
        UnsupportedAxisException { message }
    }
}

impl std::fmt::Display for UnsupportedAxisException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for UnsupportedAxisException {}
