// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public interface allows
pub trait allows {
}

// Java field: throws MaxCountExceededException, DimensionMismatchException
// Java: public interface FieldSecondaryEquations extends RealFieldElement
pub trait FieldSecondaryEquations {
    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract void init(T t0, T[] primary0, T[] secondary0, T finalTime)
    fn init(&self);

    // Java: abstract T[] computeDerivatives(T t, T[] primary, T[] primaryDot, T[] secondary)
    fn computeDerivatives(&self) -> Box<dyn std::any::Any>;

}
