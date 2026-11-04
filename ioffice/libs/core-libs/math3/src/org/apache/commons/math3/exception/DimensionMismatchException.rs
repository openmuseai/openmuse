// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable::Localizable;
use super::MathIllegalNumberException::MathIllegalNumberException;

// Java field: long serialVersionUID
#[allow(non_upper_case_globals, dead_code)]
pub(crate) const serialVersionUID: i64 = -8415396756375798143;

// Java: public class DimensionMismatchException extends MathIllegalNumberException
pub struct DimensionMismatchException {
    // Java field: int dimension
    pub(crate) dimension: i32,
    inner: MathIllegalNumberException,
}

// Java: public class DimensionMismatchException extends MathIllegalNumberException
#[allow(non_snake_case)]
impl DimensionMismatchException {
    // Java: DimensionMismatchException()
    pub fn new_with_specific(specific: Box<dyn Localizable>, wrong: i32, expected: i32) -> Self {
        DimensionMismatchException {
            dimension: expected,
            inner: MathIllegalNumberException::new(
                specific,
                wrong as f64,
                vec![expected.to_string()],
            ),
        }
    }

    // Java: DimensionMismatchException()
    pub fn new(wrong: i32, expected: i32) -> Self {
        Self::new_with_specific(
            Box::new(LocalizedFormats::DIMENSIONS_MISMATCH_SIMPLE()),
            wrong,
            expected,
        )
    }

    // Java: int getDimension()
    pub fn getDimension(&self) -> i32 {
        // Java: return dimension;
        self.dimension
    }

    pub fn getMessage(&self) -> String {
        self.inner.getMessage()
    }
}

impl std::fmt::Display for DimensionMismatchException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.inner.getMessage())
    }
}

impl std::fmt::Debug for DimensionMismatchException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "DimensionMismatchException: {}", self.inner.getMessage())
    }
}

impl std::error::Error for DimensionMismatchException {}
