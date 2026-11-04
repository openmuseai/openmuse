// Java: public interface RealMatrixPreservingVisitor
pub trait RealMatrixPreservingVisitor {
    // Java: abstract void start()
    fn start(&self);

    // Java: abstract void visit(int row, int column, double value)
    fn visit(&self);

    // Java: abstract double end()
    fn end(&self) -> f64;

}
