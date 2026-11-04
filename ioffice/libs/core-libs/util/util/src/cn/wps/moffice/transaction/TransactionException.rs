use std::fmt;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = -646346488458400147;

// Java: public class TransactionException extends RuntimeException
pub struct TransactionException {
    pub message: String,
}

// Java: public class TransactionException extends RuntimeException
impl TransactionException {
    // Java: TransactionException(String message)
    pub fn new(message: &str) -> Self {
        // Java: super(message);
        TransactionException {
            message: message.to_string(),
        }
    }
}

impl fmt::Display for TransactionException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TransactionException: {}", self.message)
    }
}

impl fmt::Debug for TransactionException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "TransactionException {{ message: {:?} }}", self.message)
    }
}

impl std::error::Error for TransactionException {}
