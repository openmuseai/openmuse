use std::fmt;

// Java: public class CryptoException extends Exception
pub struct CryptoException {
    message: Option<String>,
    // Java field: Throwable mCause
    pub(crate) mCause: Option<Box<dyn std::error::Error + Send + Sync>>,
}

// Java: public class CryptoException extends Exception
impl CryptoException {
    // Java: CryptoException()
    pub fn CryptoException() -> Self {
      // Java: super();
      CryptoException {
          message: None,
          mCause: None,
      }
    }

    // Java: CryptoException(String message)
    pub fn CryptoException_msg(message: String) -> Self {
      // Java: super(message);
      CryptoException {
          message: Some(message),
          mCause: None,
      }
    }

    // Java: CryptoException(String message, Throwable cause)
    pub fn CryptoException_cause(message: String, cause: Box<dyn std::error::Error + Send + Sync>) -> Self {
      // Java: super(message);
      let mut e = CryptoException {
          message: Some(message),
          mCause: None,
      };
      // Java: this.mCause = cause;
      e.mCause = Some(cause);
      e
    }

    // Java: Throwable getCause()
    pub fn getCause(&self) -> Option<&(dyn std::error::Error + Send + Sync)> {
      // Java: return this.mCause;
      self.mCause.as_deref()
    }

}

impl fmt::Display for CryptoException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.message {
            Some(msg) => write!(f, "CryptoException: {}", msg),
            None => write!(f, "CryptoException"),
        }
    }
}

impl fmt::Debug for CryptoException {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl std::error::Error for CryptoException {}
