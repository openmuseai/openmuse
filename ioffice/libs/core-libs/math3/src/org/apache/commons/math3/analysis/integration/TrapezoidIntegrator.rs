// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
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

// Java field: int TRAPEZOID_MAX_ITERATIONS_COUNT
pub(crate) const TRAPEZOID_MAX_ITERATIONS_COUNT: i32 = 0;

// Java: public class TrapezoidIntegrator extends BaseAbstractUnivariateIntegrator
pub struct TrapezoidIntegrator {
    // Java field: double s
    s: f64,
}

// Java: public class TrapezoidIntegrator extends BaseAbstractUnivariateIntegrator
impl TrapezoidIntegrator {
    // Java: TrapezoidIntegrator()
    pub fn TrapezoidIntegrator(&self) {
    }

    // Java: TrapezoidIntegrator()
    pub fn TrapezoidIntegrator_2(&self) {
    }

    // Java: TrapezoidIntegrator()
    pub fn TrapezoidIntegrator_3(&self) {
        // Java: super(DEFAULT_MIN_ITERATIONS_COUNT, TRAPEZOID_MAX_ITERATIONS_COUNT);
    }

    // Java: double stage(final BaseAbstractUnivariateIntegrator baseIntegrator, final int n)
    pub fn stage(&self) -> f64 {
        todo!()
    }

    // Java: double doIntegrate()
    pub fn doIntegrate(&self) -> f64 {
        todo!()
    }

}
