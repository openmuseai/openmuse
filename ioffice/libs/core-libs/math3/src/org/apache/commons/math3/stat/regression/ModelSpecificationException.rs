// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;

// Java field: long serialVersionUID
#[allow(non_upper_case_globals)]
pub(crate) const serialVersionUID: i64 = 4206514456095401070;

// Java: public class ModelSpecificationException extends MathIllegalArgumentException
pub struct ModelSpecificationException {
    inner: MathIllegalArgumentException::MathIllegalArgumentException,
}

// Java: public class ModelSpecificationException extends MathIllegalArgumentException
#[allow(non_snake_case)]
impl ModelSpecificationException {
    // Java: ModelSpecificationException()
    // Java: ModelSpecificationException(Localizable pattern, Object ... args)
    pub fn new(pattern: Box<dyn Localizable>, args: Vec<String>) -> Self {
        ModelSpecificationException {
            inner: MathIllegalArgumentException::MathIllegalArgumentException::new(pattern, args),
        }
    }

    pub fn getMessage(&self) -> String {
        self.inner.getMessage()
    }
}

impl std::fmt::Display for ModelSpecificationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner)
    }
}

impl std::fmt::Debug for ModelSpecificationException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ModelSpecificationException: {}", self.inner)
    }
}

impl std::error::Error for ModelSpecificationException {}
