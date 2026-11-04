use super::NumFmtException::NumFmtException;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 3365643528865347195;

// Java: public class WordFmtException extends NumFmtException
pub struct WordFmtException {
    pub(crate) base: NumFmtException,
}

// Java: public class WordFmtException extends NumFmtException
impl WordFmtException {
    // Java: WordFmtException()
    pub fn new() -> Self {
      // Java: super();
      WordFmtException {
          base: NumFmtException::new(),
      }
    }

    // Java: WordFmtException(String msg)
    pub fn new_with_message(msg: String) -> Self {
      // Java: super(msg);
      WordFmtException {
          base: NumFmtException::NumFmtException_with_message(msg),
      }
    }

}

impl std::fmt::Display for WordFmtException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl std::fmt::Debug for WordFmtException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WordFmtException: {}", self.base)
    }
}

impl std::error::Error for WordFmtException {}
