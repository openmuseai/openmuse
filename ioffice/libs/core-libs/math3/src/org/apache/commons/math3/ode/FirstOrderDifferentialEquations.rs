// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public interface represents
pub trait represents {
}

// Java field: throws MaxCountExceededException, DimensionMismatchException
// Java: public interface FirstOrderDifferentialEquations
pub trait FirstOrderDifferentialEquations {
    // Java: abstract int getDimension()
    fn getDimension(&self) -> i32;

    // Java: abstract void computeDerivatives(double t, double[] y, double[] yDot)
    fn computeDerivatives(&self);

}
