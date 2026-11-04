// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats;
use super::MathIllegalArgumentException::MathIllegalArgumentException;

// Java field: long serialVersionUID
#[allow(non_upper_case_globals, dead_code)]
pub(crate) const serialVersionUID: i64 = -6024911025449780478;

// Java: public class is
#[allow(non_camel_case_types)]
pub struct r#is {
}

// Java: public class NullArgumentException extends MathIllegalArgumentException
pub struct NullArgumentException {
    inner: MathIllegalArgumentException,
}

// Java: public class is
impl r#is {
}

// Java: public class NullArgumentException extends MathIllegalArgumentException
#[allow(non_snake_case)]
impl NullArgumentException {
    // Java: NullArgumentException()
    pub fn new() -> Self {
        // Java: this(LocalizedFormats.NULL_NOT_ALLOWED);
        NullArgumentException {
            inner: MathIllegalArgumentException::new(
                Box::new(LocalizedFormats::NULL_NOT_ALLOWED()),
                vec![],
            ),
        }
    }

    // Java: NullArgumentException()
    pub fn new_with_pattern(pattern: Box<dyn Localizable>, arguments: Vec<String>) -> Self {
        NullArgumentException {
            inner: MathIllegalArgumentException::new(pattern, arguments),
        }
    }

    pub fn getMessage(&self) -> String {
        self.inner.getMessage()
    }

    pub fn getLocalizedMessage(&self) -> String {
        self.inner.getLocalizedMessage()
    }
}

impl std::fmt::Display for NullArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.getMessage())
    }
}

impl std::fmt::Debug for NullArgumentException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NullArgumentException: {}", self.inner.getMessage())
    }
}

impl std::error::Error for NullArgumentException {}
