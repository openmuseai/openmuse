use super::JaxenException::JaxenException;

// Java field: long serialVersionUID
pub(crate) const SERIAL_VERSION_UID: i64 = 953578478331961473;

// Java: public class UnresolvableException extends JaxenException
pub struct UnresolvableException {
    pub(crate) base: JaxenException,
}

// Java: public class UnresolvableException extends JaxenException
impl UnresolvableException {
    // Java: UnresolvableException(String message)
    pub fn new(message: String) -> Self {
      // Java: super(message);
      UnresolvableException { base: JaxenException::new(message) }
    }
}

impl std::fmt::Display for UnresolvableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.base)
    }
}

impl std::fmt::Debug for UnresolvableException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "UnresolvableException({:?})", self.base)
    }
}

impl std::error::Error for UnresolvableException {}
