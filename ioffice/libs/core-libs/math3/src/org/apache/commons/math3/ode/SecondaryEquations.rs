// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public interface allows
pub trait allows {
}

// Java: public interface SecondaryEquations
// Java field: throws MaxCountExceededException, DimensionMismatchException
pub trait SecondaryEquations {
    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract void computeDerivatives()
    fn computeDerivatives(&self);

}
