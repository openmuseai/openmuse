// Java: import org.apache.commons.math3.exception.util.Localizable
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const serialVersionUID: i64 = -3629324471511904459;

// Java: public class NoDataException extends MathIllegalArgumentException
#[allow(non_snake_case)]
pub struct NoDataException {
}

// Java: public class NoDataException extends MathIllegalArgumentException
#[allow(non_snake_case)]
impl NoDataException {
    // Java: NoDataException()
    pub fn new() -> Self {
        // Java: this(LocalizedFormats.NO_DATA);
        // TODO_RUST_CODE
        NoDataException {}
    }

    // Java: NoDataException(Localizable specific)
    pub fn new_with_message(_specific: &str) -> Self {
        // Java: super(specific);
        // TODO_RUST_CODE
        NoDataException {}
    }

}
