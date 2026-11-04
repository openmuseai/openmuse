// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java field: throws MaxCountExceededException, DimensionMismatchException
// Java: public interface MainStateJacobianProvider extends FirstOrderDifferentialEquations
pub trait MainStateJacobianProvider {
    // Java: abstract void computeMainStateJacobian(double t, double[] y, double[] yDot, double[][] dFdY)
    fn computeMainStateJacobian(&self);

}
