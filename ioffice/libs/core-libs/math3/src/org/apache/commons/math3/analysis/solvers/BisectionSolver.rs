// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.exception.TooManyEvaluationsException
use crate::org::apache::commons::math3::exception::TooManyEvaluationsException;

// Java field: double DEFAULT_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_ABSOLUTE_ACCURACY: f64 = 0.0;

// Java: public class BisectionSolver extends AbstractUnivariateSolver
pub struct BisectionSolver {
}

// Java: public class BisectionSolver extends AbstractUnivariateSolver
impl BisectionSolver {
    // Java: BisectionSolver()
    pub fn BisectionSolver(&self) {
        // Java: this(DEFAULT_ABSOLUTE_ACCURACY);
    }

    // Java: BisectionSolver(double absoluteAccuracy)
    pub fn BisectionSolver_2(&self) {
        // Java: super(absoluteAccuracy);
    }

    // Java: BisectionSolver()
    pub fn BisectionSolver_3(&self) {
    }

    // Java: double doSolve()
    pub fn doSolve(&self) -> f64 {
        todo!()
    }

}
