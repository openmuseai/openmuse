// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -1960874856936000015;

// Java: public class ZeroException extends MathIllegalNumberException
#[derive(Debug, Clone)]
pub struct ZeroException {
    pub(crate) message: String,
}

// Java: public class ZeroException extends MathIllegalNumberException
impl ZeroException {
    // Java: ZeroException()
    pub fn ZeroException() -> Self {
        // Java: this(LocalizedFormats.ZERO_NOT_ALLOWED);
        ZeroException { message: "ZERO_NOT_ALLOWED".to_string() }
    }

    // Java: ZeroException(Localizable specific, Object ... arguments)
    pub fn ZeroException_specific(specific: &str) -> Self {
        // Java: super(specific, INTEGER_ZERO, arguments);
        ZeroException { message: specific.to_string() }
    }
}

impl std::fmt::Display for ZeroException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ZeroException: {}", self.message)
    }
}

impl std::error::Error for ZeroException {}
