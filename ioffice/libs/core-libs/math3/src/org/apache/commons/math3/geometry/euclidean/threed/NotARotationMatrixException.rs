#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable;

// Java: public class represents
pub struct represents {
}

// Java: public class NotARotationMatrixException
pub struct NotARotationMatrixException {
    pub(crate) message: String,
}

// Java: public class represents
impl represents {
}

// Java: public class NotARotationMatrixException
impl NotARotationMatrixException {
    // Java: NotARotationMatrixException(Localizable specifier, Object... args)
    pub fn new(message: String) -> Self {
        NotARotationMatrixException { message }
    }
}
