// Java: public interface defines
pub trait defines {
}

// Java: public interface RealVectorPreservingVisitor
pub trait RealVectorPreservingVisitor {
    // Java: abstract void start(int dimension, int start, int end)
    fn start(&self);

    // Java: abstract void visit(int index, double value)
    fn visit(&self);

    // Java: abstract double end()
    fn end(&self) -> f64;

}
