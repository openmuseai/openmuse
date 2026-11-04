// Java: import org.apache.commons.math3.exception.util.Localizable
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable;

// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const SERIAL_VERSION_UID: i64 = -2250556892093726375;

// Java: public class NotPositiveException extends NumberIsTooSmallException
pub struct NotPositiveException {
    value: f64,
}

// Java: public class NotPositiveException extends NumberIsTooSmallException
#[allow(non_snake_case)]
impl NotPositiveException {
    // Java: NotPositiveException(Number value)
    pub fn new(value: f64) -> Self {
        // Java: super(value, INTEGER_ZERO, true);
        NotPositiveException { value }
    }

    // Java: NotPositiveException()
    pub fn new_with_specific(_specific: &str, value: f64) -> Self {
        NotPositiveException { value }
    }

    pub fn getValue(&self) -> f64 { self.value }
}

impl std::fmt::Display for NotPositiveException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotPositiveException: {} is not positive", self.value)
    }
}
impl std::fmt::Debug for NotPositiveException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotPositiveException({})", self.value)
    }
}
impl std::error::Error for NotPositiveException {}
