// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;
use super::MathIllegalArgumentException::MathIllegalArgumentException;

// Java field: Integer INTEGER_ZERO
#[allow(non_upper_case_globals, dead_code)]
pub(crate) static INTEGER_ZERO: i32 = 0;
// Java field: long serialVersionUID
#[allow(non_upper_case_globals, dead_code)]
pub(crate) const serialVersionUID: i64 = -7447085893598031110;

// Java: public class for
#[allow(non_camel_case_types)]
pub struct r#for {
}

// Java: public class MathIllegalNumberException
pub struct MathIllegalNumberException {
    // Java field: Number argument
    pub(crate) argument: f64,
    inner: MathIllegalArgumentException,
}

// Java: public class for
impl r#for {
}

// Java: public class MathIllegalNumberException
#[allow(non_snake_case)]
impl MathIllegalNumberException {
    // Java: MathIllegalNumberException()
    pub fn new(pattern: Box<dyn Localizable>, wrong: f64, arguments: Vec<String>) -> Self {
        let mut args = vec![wrong.to_string()];
        args.extend(arguments);
        MathIllegalNumberException {
            argument: wrong,
            inner: MathIllegalArgumentException::new(pattern, args),
        }
    }

    // Java: Number getArgument()
    pub fn getArgument(&self) -> f64 {
        // Java: return argument;
        self.argument
    }

    pub fn getMessage(&self) -> String {
        self.inner.getMessage()
    }
}

impl std::fmt::Display for MathIllegalNumberException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.getMessage())
    }
}

impl std::fmt::Debug for MathIllegalNumberException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MathIllegalNumberException: {}", self.inner.getMessage())
    }
}

impl std::error::Error for MathIllegalNumberException {}
