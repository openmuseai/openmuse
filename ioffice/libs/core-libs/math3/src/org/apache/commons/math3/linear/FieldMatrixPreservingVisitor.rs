// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;

// Java: public interface FieldMatrixPreservingVisitor extends FieldElement
pub trait FieldMatrixPreservingVisitor {
    // Java: abstract void start()
    fn start(&self);

    // Java: abstract void visit(int row, int column, T value)
    fn visit(&self);

    // Java: abstract T end()
    fn end(&self) -> Box<dyn std::any::Any>;

}
