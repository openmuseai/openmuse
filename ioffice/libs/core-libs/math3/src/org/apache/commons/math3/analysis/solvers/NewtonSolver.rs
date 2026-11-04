// Java: import org.apache.commons.math3.analysis.DifferentiableUnivariateFunction
use crate::org::apache::commons::math3::analysis::DifferentiableUnivariateFunction;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;

// Java field: double DEFAULT_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_ABSOLUTE_ACCURACY: f64 = 0.0;

// Java: public class NewtonSolver extends AbstractDifferentiableUnivariateSolver
pub struct NewtonSolver {
}

// Java: public class NewtonSolver extends AbstractDifferentiableUnivariateSolver
impl NewtonSolver {
    // Java: NewtonSolver()
    pub fn NewtonSolver(&self) {
        // Java: this(DEFAULT_ABSOLUTE_ACCURACY);
    }

    // Java: NewtonSolver(double absoluteAccuracy)
    pub fn NewtonSolver_2(&self) {
        // Java: super(absoluteAccuracy);
    }

    // Java: double solve()
    pub fn solve(&self) -> f64 {
        todo!()
    }

    // Java: double doSolve()
    pub fn doSolve(&self) -> f64 {
        todo!()
    }

}
