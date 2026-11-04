// Java: public interface specifies
pub trait specifies {
}

// Java: public interface ConvergenceChecker
pub trait ConvergenceChecker {
    // Java: abstract boolean converged(int iteration, PAIR previous, PAIR current)
    fn converged(&self) -> bool;

}
