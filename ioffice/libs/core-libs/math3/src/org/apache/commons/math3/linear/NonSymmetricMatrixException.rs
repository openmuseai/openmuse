// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID

// Java: public class NonSymmetricMatrixException extends MathIllegalArgumentException
pub struct NonSymmetricMatrixException {
    // Java field: int row
    pub(crate) row: i32,
    // Java field: int column
    pub(crate) column: i32,
    // Java field: double threshold
    pub(crate) threshold: f64,
}

// Java: public class NonSymmetricMatrixException extends MathIllegalArgumentException
impl NonSymmetricMatrixException {
    // Java: NonSymmetricMatrixException()
    pub fn NonSymmetricMatrixException(&self) {
    }

    // Java: int getRow()
    pub fn getRow(&self) -> i32 {
        // Java: return row;
        todo!()
    }

    // Java: int getColumn()
    pub fn getColumn(&self) -> i32 {
        // Java: return column;
        todo!()
    }

    // Java: double getThreshold()
    pub fn getThreshold(&self) -> f64 {
        // Java: return threshold;
        todo!()
    }

}
