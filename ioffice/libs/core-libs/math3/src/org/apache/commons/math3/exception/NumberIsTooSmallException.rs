// Java: import org.apache.commons.math3.exception.util.Localizable
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats;
#[allow(unused_imports)]
use super::MathIllegalNumberException::MathIllegalNumberException;

// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const serialVersionUID: i64 = -6100997100383932834;

// Java: public class NumberIsTooSmallException extends MathIllegalNumberException
#[allow(non_snake_case)]
pub struct NumberIsTooSmallException {
    // Java field: Number min
    pub(crate) min: f64,
    // Java field: boolean boundIsAllowed
    pub(crate) boundIsAllowed: bool,
    inner: MathIllegalNumberException,
}

// Java: public class NumberIsTooSmallException extends MathIllegalNumberException
#[allow(non_snake_case)]
impl NumberIsTooSmallException {
    // Java: NumberIsTooSmallException()
    pub fn new(wrong: f64, min: f64, bound_is_allowed: bool) -> Self {
        let pattern = if bound_is_allowed {
            Box::new(LocalizedFormats::NUMBER_TOO_SMALL()) as Box<dyn Localizable>
        } else {
            Box::new(LocalizedFormats::NUMBER_TOO_SMALL_BOUND_EXCLUDED()) as Box<dyn Localizable>
        };
        NumberIsTooSmallException {
            min, boundIsAllowed: bound_is_allowed,
            inner: MathIllegalNumberException::new(pattern, wrong, vec![min.to_string()]),
        }
    }

    // Java: NumberIsTooSmallException()
    pub fn new_with_specific(specific: Box<dyn Localizable>, wrong: f64, min: f64, bound_is_allowed: bool) -> Self {
        NumberIsTooSmallException {
            min, boundIsAllowed: bound_is_allowed,
            inner: MathIllegalNumberException::new(specific, wrong, vec![min.to_string()]),
        }
    }

    // Java: boolean getBoundIsAllowed()
    pub fn getBoundIsAllowed(&self) -> bool {
        // Java: return boundIsAllowed;
        self.boundIsAllowed
    }

    // Java: Number getMin()
    pub fn getMin(&self) -> f64 {
        // Java: return min;
        self.min
    }

    pub fn getMessage(&self) -> String { self.inner.getMessage() }
}

impl std::fmt::Display for NumberIsTooSmallException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.getMessage())
    }
}
impl std::fmt::Debug for NumberIsTooSmallException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumberIsTooSmallException: {}", self.inner.getMessage())
    }
}
impl std::error::Error for NumberIsTooSmallException {}
