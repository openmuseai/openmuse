// Java: import org.apache.commons.math3.fitting.leastsquares.LeastSquaresProblem.Evaluation
use crate::org::apache::commons::math3::fitting::leastsquares::LeastSquaresProblem::Evaluation;
// Java: import org.apache.commons.math3.optim.ConvergenceChecker
use crate::org::apache::commons::math3::optim::ConvergenceChecker;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java: public class EvaluationRmsChecker implements ConvergenceChecker
pub struct EvaluationRmsChecker {
    // Java field: double relTol
    relTol: f64,
    // Java field: double absTol
    absTol: f64,
}

// Java: public class EvaluationRmsChecker implements ConvergenceChecker
impl EvaluationRmsChecker {
    // Java: EvaluationRmsChecker(final double tol)
    pub fn EvaluationRmsChecker(&self) {
        // Java: this(tol, tol);
    }

    // Java: EvaluationRmsChecker(final double relTol, final double absTol)
    pub fn EvaluationRmsChecker_2(&self) {
        // Java: this.relTol = relTol;
        // Java: this.absTol = absTol;
    }

    // Java: boolean converged()
    pub fn converged(&self) -> bool {
        todo!()
    }

}

