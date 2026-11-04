// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;

// Java: public interface defines
pub trait defines {
}

// Java: public interface FieldVectorPreservingVisitor
pub trait FieldVectorPreservingVisitor {
    // Java: abstract void start(int dimension, int start, int end)
    fn start(&self);

    // Java: abstract void visit(int index, T value)
    fn visit(&self);

    // Java: abstract T end()
    fn end(&self) -> Box<dyn std::any::Any>;

}
