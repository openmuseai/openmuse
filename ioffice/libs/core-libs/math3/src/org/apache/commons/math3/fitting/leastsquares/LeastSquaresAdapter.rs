// Java: import org.apache.commons.math3.linear.RealVector
use crate::org::apache::commons::math3::linear::RealVector;
// Java: import org.apache.commons.math3.optim.ConvergenceChecker
use crate::org::apache::commons::math3::optim::ConvergenceChecker;
// Java: import org.apache.commons.math3.util.Incrementor
use crate::org::apache::commons::math3::util::Incrementor;

// Java: public class LeastSquaresAdapter implements LeastSquaresProblem
pub struct LeastSquaresAdapter {
    // Java field: LeastSquaresProblem problem
    pub(crate) problem: Box<dyn std::any::Any>,
}

// Java: public class LeastSquaresAdapter implements LeastSquaresProblem
impl LeastSquaresAdapter {
    // Java: LeastSquaresAdapter(final LeastSquaresProblem problem)
    pub fn LeastSquaresAdapter(&self) {
        // Java: this.problem = problem;
    }

    // Java: RealVector getStart()
    pub fn getStart(&self) -> Box<dyn std::any::Any> {
        // Java: return problem.getStart();
        todo!()
    }

    // Java: int getObservationSize()
    pub fn getObservationSize(&self) -> i32 {
        // Java: return problem.getObservationSize();
        todo!()
    }

    // Java: int getParameterSize()
    pub fn getParameterSize(&self) -> i32 {
        // Java: return problem.getParameterSize();
        todo!()
    }

    // Java: Evaluation evaluate(final RealVector point)
    pub fn evaluate(&self) -> Box<dyn std::any::Any> {
        // Java: return problem.evaluate(point);
        todo!()
    }

    // Java: Incrementor getEvaluationCounter()
    pub fn getEvaluationCounter(&self) -> Box<dyn std::any::Any> {
        // Java: return problem.getEvaluationCounter();
        todo!()
    }

    // Java: Incrementor getIterationCounter()
    pub fn getIterationCounter(&self) -> Box<dyn std::any::Any> {
        // Java: return problem.getIterationCounter();
        todo!()
    }

    // Java: ConvergenceChecker<Evaluation> getConvergenceChecker()
    pub fn getConvergenceChecker(&self) -> Box<dyn std::any::Any> {
        // Java: return problem.getConvergenceChecker();
        todo!()
    }

}

