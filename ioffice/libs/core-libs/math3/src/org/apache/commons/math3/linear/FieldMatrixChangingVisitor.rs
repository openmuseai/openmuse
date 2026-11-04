// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;

// Java: public interface FieldMatrixChangingVisitor extends FieldElement
pub trait FieldMatrixChangingVisitor {
    // Java: abstract void start()
    fn start(&self);

    // Java: abstract T visit(int row, int column, T value)
    fn visit(&self) -> Box<dyn std::any::Any>;

    // Java: abstract T end()
    fn end(&self) -> Box<dyn std::any::Any>;

}
