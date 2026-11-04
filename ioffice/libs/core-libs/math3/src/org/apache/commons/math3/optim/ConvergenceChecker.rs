// Java: public interface specifies
pub trait specifies {
}

// Java: public interface ConvergenceChecker
pub trait ConvergenceChecker {
    // Java: abstract boolean converged(int iteration, PAIR previous, PAIR current)
    fn converged(&self, iteration: i32, previous: &dyn std::any::Any, current: &dyn std::any::Any) -> bool;

}
