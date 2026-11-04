// Java: import org.apache.commons.math3.exception.MultiDimensionMismatchException
use crate::org::apache::commons::math3::exception::MultiDimensionMismatchException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class MatrixDimensionMismatchException extends MultiDimensionMismatchException
pub struct MatrixDimensionMismatchException {
}

// Java: public class MatrixDimensionMismatchException extends MultiDimensionMismatchException
impl MatrixDimensionMismatchException {
    // Java: MatrixDimensionMismatchException()
    pub fn MatrixDimensionMismatchException(&self) {
    }

    // Java: int getWrongRowDimension()
    pub fn getWrongRowDimension(&self) -> i32 {
        // Java: return getWrongDimension(0);
        todo!();
    }

    // Java: int getExpectedRowDimension()
    pub fn getExpectedRowDimension(&self) -> i32 {
        // Java: return getExpectedDimension(0);
        todo!();
    }

    // Java: int getWrongColumnDimension()
    pub fn getWrongColumnDimension(&self) -> i32 {
        // Java: return getWrongDimension(1);
        todo!();
    }

    // Java: int getExpectedColumnDimension()
    pub fn getExpectedColumnDimension(&self) -> i32 {
        // Java: return getExpectedDimension(1);
        todo!();
    }

}
