// Java: import org.apache.commons.math3.analysis.differentiation.DerivativeStructure
use crate::org::apache::commons::math3::analysis::differentiation::DerivativeStructure;
// Java: import org.apache.commons.math3.analysis.differentiation.UnivariateDifferentiableFunction
use crate::org::apache::commons::math3::analysis::differentiation::UnivariateDifferentiableFunction;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;

// Java field: double DEFAULT_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_ABSOLUTE_ACCURACY: f64 = 0.0;

// Java: public class NewtonRaphsonSolver extends AbstractUnivariateDifferentiableSolver
pub struct NewtonRaphsonSolver {
}

// Java: public class NewtonRaphsonSolver extends AbstractUnivariateDifferentiableSolver
impl NewtonRaphsonSolver {
    // Java: NewtonRaphsonSolver()
    pub fn NewtonRaphsonSolver(&self) {
        // Java: this(DEFAULT_ABSOLUTE_ACCURACY);
    }

    // Java: NewtonRaphsonSolver(double absoluteAccuracy)
    pub fn NewtonRaphsonSolver_2(&self) {
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
