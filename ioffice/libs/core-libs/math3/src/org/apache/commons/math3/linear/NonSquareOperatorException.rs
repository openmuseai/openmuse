// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID

// Java: public class NonSquareOperatorException extends DimensionMismatchException
pub struct NonSquareOperatorException {
}

// Java: public class NonSquareOperatorException extends DimensionMismatchException
impl NonSquareOperatorException {
    // Java: NonSquareOperatorException(int wrong, int expected)
    pub fn NonSquareOperatorException(&self) {
        // Java: super(LocalizedFormats.NON_SQUARE_OPERATOR, wrong, expected);
    }

}
