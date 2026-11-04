// Java: public interface DecompositionSolver
pub trait DecompositionSolver {
    // Java: abstract RealVector solve(final RealVector b)
    fn solve(&self, b: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract RealMatrix solve(final RealMatrix b)
    fn solve_matrix(&self, b: &dyn std::any::Any) -> Box<dyn std::any::Any>;

    // Java: abstract boolean isNonSingular()
    fn isNonSingular(&self) -> bool;

    // Java: abstract RealMatrix getInverse()
    fn getInverse(&self) -> Box<dyn std::any::Any>;

}
