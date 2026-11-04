// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;

// Java field: MathIllegalArgumentException, NullArgumentException
// Java: public interface UnivariateIntegrator
pub trait UnivariateIntegrator {
    // Java: abstract double getRelativeAccuracy()
    fn getRelativeAccuracy(&self) -> f64;

    // Java: abstract double getAbsoluteAccuracy()
    fn getAbsoluteAccuracy(&self) -> f64;

    // Java: abstract int getMinimalIterationCount()
    fn getMinimalIterationCount(&self) -> i32;

    // Java: abstract int getMaximalIterationCount()
    fn getMaximalIterationCount(&self) -> i32;

    // Java: abstract double integrate()
    fn integrate(&self) -> f64;

    // Java: abstract int getEvaluations()
    fn getEvaluations(&self) -> i32;

    // Java: abstract int getIterations()
    fn getIterations(&self) -> i32;

}
