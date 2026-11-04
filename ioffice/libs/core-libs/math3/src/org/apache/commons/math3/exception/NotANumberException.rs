// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
#[allow(unused_imports)]
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
#[allow(dead_code)]
pub(crate) const serialVersionUID: i64 = 20120906;

// Java: public class NotANumberException extends MathIllegalNumberException
#[allow(non_snake_case)]
pub struct NotANumberException {
}

// Java: public class NotANumberException extends MathIllegalNumberException
#[allow(non_snake_case)]
impl NotANumberException {
    // Java: NotANumberException()
    pub fn new() -> Self {
        // Java: super(LocalizedFormats.NAN_NOT_ALLOWED, Double.valueOf(Double.NaN));
        // TODO_RUST_CODE
        NotANumberException {}
    }

}
