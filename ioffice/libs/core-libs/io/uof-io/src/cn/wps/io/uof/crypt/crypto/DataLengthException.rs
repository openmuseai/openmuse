// Java: public class DataLengthException extends RuntimeCryptoException
pub struct DataLengthException {
    pub(crate) message: Option<String>,
}

// Java: public class DataLengthException extends RuntimeCryptoException
impl DataLengthException {
    // Java: DataLengthException()
    pub fn new() -> Self {
      // Java: super();
      DataLengthException { message: None }
    }

    // Java: DataLengthException(String var1)
    pub fn new_with_message(var1: String) -> Self {
      // Java: super(var1);
      DataLengthException { message: Some(var1) }
    }

}
