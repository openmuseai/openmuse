// Java: import org.apache.commons.math3.exception.util.Localizable
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable;

// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const SERIAL_VERSION_UID: i64 = -7824848630829852237;

// Java: public class NotStrictlyPositiveException extends NumberIsTooSmallException
pub struct NotStrictlyPositiveException {
    value: f64,
}

// Java: public class NotStrictlyPositiveException extends NumberIsTooSmallException
#[allow(non_snake_case)]
impl NotStrictlyPositiveException {
    // Java: NotStrictlyPositiveException(Number value)
    pub fn new(value: f64) -> Self {
        // Java: super(value, INTEGER_ZERO, false);
        NotStrictlyPositiveException { value }
    }

    // Java: NotStrictlyPositiveException()
    pub fn new_with_specific(_specific: &str, value: f64) -> Self {
        NotStrictlyPositiveException { value }
    }

    pub fn getValue(&self) -> f64 { self.value }
}

impl std::fmt::Display for NotStrictlyPositiveException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotStrictlyPositiveException: {} is not strictly positive", self.value)
    }
}
impl std::fmt::Debug for NotStrictlyPositiveException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NotStrictlyPositiveException({})", self.value)
    }
}
impl std::error::Error for NotStrictlyPositiveException {}
