// Java: public interface defines
pub trait defines {
}

// Java: public interface RealVectorChangingVisitor
pub trait RealVectorChangingVisitor {
    // Java: abstract void start(int dimension, int start, int end)
    fn start(&self);

    // Java: abstract double visit(int index, double value)
    fn visit(&self) -> f64;

    // Java: abstract double end()
    fn end(&self) -> f64;

}
