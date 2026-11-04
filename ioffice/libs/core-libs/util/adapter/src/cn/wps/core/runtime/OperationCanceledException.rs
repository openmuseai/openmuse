// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 1;

// Java: public class OperationCanceledException extends RuntimeException
pub struct OperationCanceledException {
    pub(crate) message: Option<String>,
}

// Java: public class OperationCanceledException extends RuntimeException
impl OperationCanceledException {
    // Java: OperationCanceledException()
    pub fn new() -> Self {
      // Java: super();
      OperationCanceledException { message: None }
    }

    // Java: OperationCanceledException(String message)
    pub fn new_with_message(_message: &str) -> Self {
      // Java: super(message);
      OperationCanceledException { message: Some(_message.to_string()) }
    }

}
