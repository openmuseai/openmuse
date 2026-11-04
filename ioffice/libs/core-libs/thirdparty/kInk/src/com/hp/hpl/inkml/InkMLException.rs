// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -3284213657128760183;

// Java: public class InkMLException extends Exception
pub struct InkMLException {
    pub message: String,
}

// Java: public class InkMLException extends Exception
impl InkMLException {
    // Java: InkMLException(String message)
    pub fn InkMLException(message: &str) -> Self {
      // Java: super(message);
      Self { message: message.to_string() }
    }

    // Java: InkMLException(String message, Exception exception)
    pub fn InkMLException_with_cause(message: &str, exception: &str) -> Self {
      // Java: super(message + "\n" + exception.getMessage());
      Self { message: format!("{}\n{}", message, exception) }
    }

    // Java: InkMLException(Exception exception)
    pub fn InkMLException_from(exception: &str) -> Self {
      // Java: super(exception.getMessage());
      Self { message: exception.to_string() }
    }

}
