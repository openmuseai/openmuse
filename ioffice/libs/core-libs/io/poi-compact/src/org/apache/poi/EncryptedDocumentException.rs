use std::fmt;

// Java: public class EncryptedDocumentException extends IllegalStateException
pub struct EncryptedDocumentException {
    message: String,
}

// Java: public class EncryptedDocumentException extends IllegalStateException
impl EncryptedDocumentException {
    // Java: EncryptedDocumentException(String s)
    pub fn new(s: &str) -> Self {
        // Java: super(s);
        EncryptedDocumentException {
            message: s.to_string(),
        }
    }
}

impl fmt::Display for EncryptedDocumentException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl fmt::Debug for EncryptedDocumentException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "EncryptedDocumentException: {}", self.message)
    }
}

impl std::error::Error for EncryptedDocumentException {}
