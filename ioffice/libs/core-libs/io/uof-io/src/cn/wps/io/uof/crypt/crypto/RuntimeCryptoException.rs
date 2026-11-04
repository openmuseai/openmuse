// Java: public class RuntimeCryptoException extends RuntimeException
pub struct RuntimeCryptoException {
    pub(crate) message: Option<String>,
}

// Java: public class RuntimeCryptoException extends RuntimeException
impl RuntimeCryptoException {
    // Java: RuntimeCryptoException()
    pub fn new() -> Self {
      // Java: super();
      RuntimeCryptoException { message: None }
    }

    // Java: RuntimeCryptoException(String var1)
    pub fn new_with_message(var1: String) -> Self {
      // Java: super(var1);
      RuntimeCryptoException { message: Some(var1) }
    }

}
