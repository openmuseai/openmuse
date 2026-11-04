// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;

// Java: public interface ParameterJacobianProvider extends Parameterizable
// Java field: throws DimensionMismatchException, MaxCountExceededException, UnknownParameterException
pub trait ParameterJacobianProvider {
    // Java: abstract void computeParameterJacobian()
    fn computeParameterJacobian(&self);

}
