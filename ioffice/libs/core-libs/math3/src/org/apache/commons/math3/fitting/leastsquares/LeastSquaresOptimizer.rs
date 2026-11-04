// Java: public interface LeastSquaresOptimizer
pub trait LeastSquaresOptimizer {
    // Java: abstract Optimum optimize(LeastSquaresProblem leastSquaresProblem)
    fn optimize(&self, least_squares_problem: &dyn std::any::Any) -> Box<dyn Optimum>;

}

// Java: public interface Optimum
// Java: extends LeastSquaresProblem.Evaluation
pub trait Optimum {
    // Java: abstract int getEvaluations()
    fn getEvaluations(&self) -> i32;

    // Java: abstract int getIterations()
    fn getIterations(&self) -> i32;

}
