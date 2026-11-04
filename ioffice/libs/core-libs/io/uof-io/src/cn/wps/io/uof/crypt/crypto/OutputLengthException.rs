// Java: public class OutputLengthException extends DataLengthException
pub struct OutputLengthException {
    pub(crate) message: Option<String>,
}

// Java: public class OutputLengthException extends DataLengthException
impl OutputLengthException {
    // Java: OutputLengthException(String var1)
    pub fn new_with_message(var1: String) -> Self {
      // Java: super(var1);
      OutputLengthException { message: Some(var1) }
    }

}
