// Java: public interface is
pub trait is {
}

// Java: public interface BaseOptimizer
pub trait BaseOptimizer {
    // Java: abstract int getMaxEvaluations()
    fn getMaxEvaluations(&self) -> i32;

    // Java: abstract int getEvaluations()
    fn getEvaluations(&self) -> i32;

    // Java: abstract ConvergenceChecker<PAIR> getConvergenceChecker()
    fn getConvergenceChecker(&self) -> Box<dyn std::any::Any>;

}
