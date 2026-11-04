// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.exception.util.ExceptionContext
use crate::org::apache::commons::math3::exception::util::ExceptionContext;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class NonPositiveDefiniteMatrixException extends NumberIsTooSmallException
pub struct NonPositiveDefiniteMatrixException {
    // Java field: int index
    pub(crate) index: i32,
    // Java field: double threshold
    pub(crate) threshold: f64,
}

// Java: public class NonPositiveDefiniteMatrixException extends NumberIsTooSmallException
impl NonPositiveDefiniteMatrixException {
    // Java: NonPositiveDefiniteMatrixException()
    pub fn NonPositiveDefiniteMatrixException(&self) {
    }

    // Java: int getRow()
    pub fn getRow(&self) -> i32 {
        // Java: return index;
        todo!();
    }

    // Java: int getColumn()
    pub fn getColumn(&self) -> i32 {
        // Java: return index;
        todo!();
    }

    // Java: double getThreshold()
    pub fn getThreshold(&self) -> f64 {
        // Java: return threshold;
        todo!();
    }

}
