// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.util.ExceptionContext
use crate::org::apache::commons::math3::exception::util::ExceptionContext;
// Java: import org.apache.commons.math3.exception.util.ExceptionContextProvider
use crate::org::apache::commons::math3::exception::util::ExceptionContextProvider;

// Java: public class for
pub struct r#for {
}

// Java: public class MathUnsupportedOperationException extends UnsupportedOperationException
#[derive(Debug, Clone)]
pub struct MathUnsupportedOperationException {
    pub(crate) message: String,
}

// Java: public class for
impl r#for {
}

// Java: public class MathUnsupportedOperationException extends UnsupportedOperationException
impl MathUnsupportedOperationException {
    pub fn new() -> Self {
        MathUnsupportedOperationException { message: "UnsupportedOperation".to_string() }
    }

    pub fn new_with_message(msg: &str) -> Self {
        MathUnsupportedOperationException { message: msg.to_string() }
    }
}

impl std::fmt::Display for MathUnsupportedOperationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathUnsupportedOperationException: {}", self.message)
    }
}

impl std::error::Error for MathUnsupportedOperationException {}
