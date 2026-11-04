// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;

// Java: public interface FieldDecompositionSolver extends FieldElement
pub trait FieldDecompositionSolver {
    // Java: abstract FieldVector<T> solve(final FieldVector<T> b)
    fn solve(&self) -> Box<dyn std::any::Any>;

    // Java: abstract FieldMatrix<T> solve(final FieldMatrix<T> b)
    fn solve_matrix(&self) -> Box<dyn std::any::Any>;

    // Java: abstract boolean isNonSingular()
    fn isNonSingular(&self) -> bool;

    // Java: abstract FieldMatrix<T> getInverse()
    fn getInverse(&self) -> Box<dyn std::any::Any>;

}
