// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 1;

// Java: public class NumFmtException extends RuntimeException
pub struct NumFmtException {
    message: String,
}

// Java: public class NumFmtException extends RuntimeException
impl NumFmtException {
    // Java: NumFmtException()
    pub fn new() -> Self {
      // Java: super();
      NumFmtException { message: String::new() }
    }

    // Java: NumFmtException(String detailMessage)
    pub fn NumFmtException_with_message(detail_message: String) -> Self {
      // Java: super(detailMessage);
      NumFmtException { message: detail_message }
    }

}

impl std::fmt::Display for NumFmtException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::fmt::Debug for NumFmtException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumFmtException: {}", self.message)
    }
}

impl std::error::Error for NumFmtException {}
