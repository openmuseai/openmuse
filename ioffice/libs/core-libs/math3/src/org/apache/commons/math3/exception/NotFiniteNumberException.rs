// Java: import org.apache.commons.math3.exception.util.Localizable
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const serialVersionUID: i64 = -6100997100383932834;

// Java: public class NotFiniteNumberException extends MathIllegalNumberException
#[allow(non_snake_case)]
pub struct NotFiniteNumberException {
}

// Java: public class NotFiniteNumberException extends MathIllegalNumberException
#[allow(non_snake_case)]
impl NotFiniteNumberException {
    // Java: NotFiniteNumberException()
    pub fn new(_wrong: f64) -> Self {
        NotFiniteNumberException {}
    }

    // Java: NotFiniteNumberException()
    pub fn new_with_specific(_specific: &str, _wrong: f64) -> Self {
        NotFiniteNumberException {}
    }

}
