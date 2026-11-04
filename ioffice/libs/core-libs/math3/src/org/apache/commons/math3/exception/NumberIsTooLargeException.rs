#[allow(unused_imports)]
use super::MathIllegalNumberException::MathIllegalNumberException;
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats;

// Java field: long serialVersionUID
#[allow(non_upper_case_globals, dead_code)]
pub(crate) const serialVersionUID: i64 = 4330003017885151975;

// Java: public class NumberIsTooLargeException extends MathIllegalNumberException
pub struct NumberIsTooLargeException {
    // Java field: Number max
    max: f64,
    // Java field: boolean boundIsAllowed
    bound_is_allowed: bool,
    inner: MathIllegalNumberException,
}

#[allow(non_snake_case)]
impl NumberIsTooLargeException {
    // Java: NumberIsTooLargeException()
    pub fn new(wrong: f64, max: f64, bound_is_allowed: bool) -> Self {
        let pattern = if bound_is_allowed {
            Box::new(LocalizedFormats::NUMBER_TOO_LARGE()) as Box<dyn Localizable>
        } else {
            Box::new(LocalizedFormats::NUMBER_TOO_LARGE_BOUND_EXCLUDED()) as Box<dyn Localizable>
        };
        NumberIsTooLargeException {
            max, bound_is_allowed,
            inner: MathIllegalNumberException::new(pattern, wrong, vec![max.to_string()]),
        }
    }

    // Java: NumberIsTooLargeException()
    pub fn new_with_specific(specific: Box<dyn Localizable>, wrong: f64, max: f64, bound_is_allowed: bool) -> Self {
        NumberIsTooLargeException {
            max, bound_is_allowed,
            inner: MathIllegalNumberException::new(specific, wrong, vec![max.to_string()]),
        }
    }

    // Java: boolean getBoundIsAllowed()
    pub fn getBoundIsAllowed(&self) -> bool {
        // Java: return boundIsAllowed;
        self.bound_is_allowed
    }

    // Java: Number getMax()
    pub fn getMax(&self) -> f64 {
        // Java: return max;
        self.max
    }

    pub fn getMessage(&self) -> String { self.inner.getMessage() }
}

impl std::fmt::Display for NumberIsTooLargeException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.getMessage())
    }
}
impl std::fmt::Debug for NumberIsTooLargeException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NumberIsTooLargeException: {}", self.inner.getMessage())
    }
}
impl std::error::Error for NumberIsTooLargeException {}
