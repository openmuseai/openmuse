// Java: import org.apache.commons.math3.exception.util.Localizable
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const SERIAL_VERSION_UID: i64 = -6276776513966934846;
// Java field: String REPORT_URL
#[allow(dead_code)]
pub(crate) const REPORT_URL: &str = "https://issues.apache.org/jira/browse/MATH";

// Java: public class MathInternalError extends MathIllegalStateException
pub struct MathInternalError {
    message: String,
}

// Java: public class MathInternalError extends MathIllegalStateException
#[allow(non_snake_case)]
impl MathInternalError {
    // Java: MathInternalError()
    pub fn new() -> Self {
        // Java: getContext().addMessage(LocalizedFormats.INTERNAL_ERROR, REPORT_URL);
        MathInternalError {
            message: format!("internal error, please report to {}", REPORT_URL),
        }
    }

    // Java: MathInternalError(final Throwable cause)
    pub fn new_with_cause(cause: &str) -> Self {
        // Java: super(cause, LocalizedFormats.INTERNAL_ERROR, REPORT_URL);
        MathInternalError {
            message: format!("internal error ({}), please report to {}", cause, REPORT_URL),
        }
    }

    // Java: MathInternalError(Localizable pattern, Object ... args)
    pub fn new_with_pattern(_pattern: &str, _args: &[&str]) -> Self {
        // Java: super(pattern, args);
        MathInternalError {
            message: format!("internal error, please report to {}", REPORT_URL),
        }
    }

    pub fn getMessage(&self) -> &str { &self.message }
}

impl std::fmt::Display for MathInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}
impl std::fmt::Debug for MathInternalError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathInternalError: {}", self.message)
    }
}
impl std::error::Error for MathInternalError {}
