// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: int SIMPSON_MAX_ITERATIONS_COUNT
pub(crate) const SIMPSON_MAX_ITERATIONS_COUNT: i32 = 0;

// Java: public class SimpsonIntegrator extends BaseAbstractUnivariateIntegrator
pub struct SimpsonIntegrator {
}

// Java: public class SimpsonIntegrator extends BaseAbstractUnivariateIntegrator
impl SimpsonIntegrator {
    // Java: SimpsonIntegrator()
    pub fn SimpsonIntegrator(&self) {
    }

    // Java: SimpsonIntegrator()
    pub fn SimpsonIntegrator_2(&self) {
    }

    // Java: SimpsonIntegrator()
    pub fn SimpsonIntegrator_3(&self) {
        // Java: super(DEFAULT_MIN_ITERATIONS_COUNT, SIMPSON_MAX_ITERATIONS_COUNT);
    }

    // Java: double doIntegrate()
    pub fn doIntegrate(&self) -> f64 {
        todo!()
    }

}
