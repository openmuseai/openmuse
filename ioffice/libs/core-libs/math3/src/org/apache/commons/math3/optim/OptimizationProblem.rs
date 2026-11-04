// Java: import org.apache.commons.math3.util.Incrementor
use crate::org::apache::commons::math3::util::Incrementor;

// Java: public interface OptimizationProblem
pub trait OptimizationProblem {
    // Java: abstract Incrementor getEvaluationCounter()
    fn getEvaluationCounter(&self) -> Incrementor::Incrementor;

    // Java: abstract Incrementor getIterationCounter()
    fn getIterationCounter(&self) -> Incrementor::Incrementor;

    // Java: abstract ConvergenceChecker<PAIR> getConvergenceChecker()
    fn getConvergenceChecker(&self) -> Box<dyn std::any::Any>;

}
