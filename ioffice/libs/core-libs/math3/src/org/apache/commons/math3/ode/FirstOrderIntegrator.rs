// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NoBracketingException
use crate::org::apache::commons::math3::exception::NoBracketingException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;

// Java: public interface represents
pub trait represents {
}

// Java field: MaxCountExceededException, NoBracketingException
// Java: public interface FirstOrderIntegrator extends ODEIntegrator
pub trait FirstOrderIntegrator {
    // Java: abstract double integrate()
    fn integrate(&self) -> f64;

}
