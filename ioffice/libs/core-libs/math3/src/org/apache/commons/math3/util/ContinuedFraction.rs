// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: double DEFAULT_EPSILON
pub(crate) const DEFAULT_EPSILON: f64 = 10e-9;

// Java: public class ContinuedFraction
pub struct ContinuedFraction {
}

// Java: public class ContinuedFraction
impl ContinuedFraction {
    // Java: ContinuedFraction()
    pub fn ContinuedFraction(&self) {
        // Java: super();
    }

    // Java: abstract double getA(int n, double x)
    pub fn getA(&self, n: i32, x: f64) -> f64 {
        todo!();
    }

    // Java: abstract double getB(int n, double x)
    pub fn getB(&self, n: i32, x: f64) -> f64 {
        todo!();
    }

    // Java: double evaluate(double x)
    pub fn evaluate(&self, x: f64) -> f64 {
        // Java: return evaluate(x, DEFAULT_EPSILON, Integer.MAX_VALUE);
        todo!();
    }

    // Java: double evaluate(double x, double epsilon)
    pub fn evaluate_1(&self, x: f64, epsilon: f64) -> f64 {
        // Java: return evaluate(x, epsilon, Integer.MAX_VALUE);
        todo!();
    }

    // Java: double evaluate(double x, int maxIterations)
    pub fn evaluate_2(&self, x: f64, maxIterations: i32) -> f64 {
        todo!();
    }

    // Java: double evaluate(double x, double epsilon, int maxIterations)
    pub fn evaluate_3(&self, x: f64, epsilon: f64, maxIterations: i32) -> f64 {
        todo!();
    }

}
