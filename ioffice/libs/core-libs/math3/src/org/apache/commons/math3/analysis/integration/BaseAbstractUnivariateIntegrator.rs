// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.analysis.solvers.UnivariateSolverUtils
use crate::org::apache::commons::math3::analysis::solvers::UnivariateSolverUtils;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;
// Java: import org.apache.commons.math3.util.IntegerSequence
use crate::org::apache::commons::math3::util::IntegerSequence;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: double DEFAULT_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: double DEFAULT_RELATIVE_ACCURACY
pub(crate) const DEFAULT_RELATIVE_ACCURACY: f64 = 0.0;
// Java field: int DEFAULT_MIN_ITERATIONS_COUNT
pub(crate) const DEFAULT_MIN_ITERATIONS_COUNT: i32 = 0;
// Java field: int DEFAULT_MAX_ITERATIONS_COUNT
pub(crate) const DEFAULT_MAX_ITERATIONS_COUNT: i32 = 0;

// Java: public class BaseAbstractUnivariateIntegrator implements UnivariateIntegrator
pub struct BaseAbstractUnivariateIntegrator {
    // Java field: org.apache.commons.math3.util.Incrementor iterations
    pub(crate) iterations: Box<dyn std::any::Any>,
    // Java field: IntegerSequence.Incrementor count
    pub(crate) count: Box<dyn std::any::Any>,
    // Java field: double absoluteAccuracy
    pub(crate) absoluteAccuracy: f64,
    // Java field: double relativeAccuracy
    pub(crate) relativeAccuracy: f64,
    // Java field: int minimalIterationCount
    pub(crate) minimalIterationCount: i32,
    // Java field: IntegerSequence.Incrementor evaluations
    pub(crate) evaluations: Box<dyn std::any::Any>,
    // Java field: UnivariateFunction function
    function: Option<Box<dyn std::any::Any>>,
    // Java field: double min
    min: f64,
    // Java field: double max
    max: f64,
    // Java field: throws TooManyEvaluationsException, MaxCountExceededException
}

// Java: public class BaseAbstractUnivariateIntegrator implements UnivariateIntegrator
impl BaseAbstractUnivariateIntegrator {
    // Java: BaseAbstractUnivariateIntegrator()
    pub fn BaseAbstractUnivariateIntegrator(&self) {
    }

    // Java: BaseAbstractUnivariateIntegrator()
    pub fn BaseAbstractUnivariateIntegrator_2(&self) {
    }

    // Java: BaseAbstractUnivariateIntegrator()
    pub fn BaseAbstractUnivariateIntegrator_3(&self) {
    }

    // Java: double getRelativeAccuracy()
    pub fn getRelativeAccuracy(&self) -> f64 {
        // Java: return relativeAccuracy;
        todo!()
    }

    // Java: double getAbsoluteAccuracy()
    pub fn getAbsoluteAccuracy(&self) -> f64 {
        // Java: return absoluteAccuracy;
        todo!()
    }

    // Java: int getMinimalIterationCount()
    pub fn getMinimalIterationCount(&self) -> i32 {
        // Java: return minimalIterationCount;
        todo!()
    }

    // Java: int getMaximalIterationCount()
    pub fn getMaximalIterationCount(&self) -> i32 {
        // Java: return count.getMaximalCount();
        todo!()
    }

    // Java: int getEvaluations()
    pub fn getEvaluations(&self) -> i32 {
        // Java: return evaluations.getCount();
        todo!()
    }

    // Java: int getIterations()
    pub fn getIterations(&self) -> i32 {
        // Java: return count.getCount();
        todo!()
    }

    // Java: void incrementCount()
    pub fn incrementCount(&self) {
        // Java: count.increment();
    }

    // Java: double getMin()
    pub fn getMin(&self) -> f64 {
        // Java: return min;
        todo!()
    }

    // Java: double getMax()
    pub fn getMax(&self) -> f64 {
        // Java: return max;
        todo!()
    }

    // Java: double computeObjectiveValue(final double point)
    pub fn computeObjectiveValue(&self) -> f64 {
        todo!()
    }

    // Java: void setup()
    pub fn setup(&self) {
    }

    // Java: double integrate()
    pub fn integrate(&self) -> f64 {
        todo!()
    }

    // Java: abstract double doIntegrate()
    pub fn doIntegrate(&self) -> f64 {
        todo!()
    }

}

