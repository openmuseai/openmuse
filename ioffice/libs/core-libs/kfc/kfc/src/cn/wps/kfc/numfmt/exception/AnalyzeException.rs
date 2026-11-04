use super::NumFmtException::NumFmtException;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 3621125743119596162;

// Java: public class AnalyzeException extends NumFmtException
pub struct AnalyzeException {
    pub(crate) base: NumFmtException,
}

// Java: public class AnalyzeException extends NumFmtException
impl AnalyzeException {
    // Java: AnalyzeException()
    pub fn new() -> Self {
      // Java: this("Analyze sourceCode failed");
      Self::new_with_message("Analyze sourceCode failed".to_string())
    }

    // Java: AnalyzeException(String detailMessage)
    pub fn new_with_message(detail_message: String) -> Self {
      // Java: super(detailMessage);
      AnalyzeException {
          base: NumFmtException::NumFmtException_with_message(detail_message),
      }
    }

}

impl std::fmt::Display for AnalyzeException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl std::fmt::Debug for AnalyzeException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "AnalyzeException: {}", self.base)
    }
}

impl std::error::Error for AnalyzeException {}
