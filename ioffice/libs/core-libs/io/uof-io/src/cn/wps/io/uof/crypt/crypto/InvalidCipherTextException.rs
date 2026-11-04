use std::fmt;

// Java: public class InvalidCipherTextException extends CryptoException
pub struct InvalidCipherTextException {
    message: Option<String>,
    cause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

// Java: public class InvalidCipherTextException extends CryptoException
impl InvalidCipherTextException {
    // Java: InvalidCipherTextException()
    pub fn InvalidCipherTextException() -> Self {
      // Java: super();
      InvalidCipherTextException {
          message: None,
          cause: None,
      }
    }

    // Java: InvalidCipherTextException(String message)
    pub fn InvalidCipherTextException_msg(message: String) -> Self {
      // Java: super(message);
      InvalidCipherTextException {
          message: Some(message),
          cause: None,
      }
    }

    // Java: InvalidCipherTextException(String message, Throwable cause)
    pub fn InvalidCipherTextException_cause(message: String, cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
      // Java: super(message, cause);
      InvalidCipherTextException {
          message: Some(message),
          cause: Some(cause),
      }
    }

}

impl fmt::Display for InvalidCipherTextException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "InvalidCipherTextException: {}", msg),
            None => write!(f, "InvalidCipherTextException"),
        }
    }
}

impl fmt::Debug for InvalidCipherTextException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for InvalidCipherTextException {}
