// Java: public interface RealMatrixChangingVisitor
pub trait RealMatrixChangingVisitor {
    // Java: abstract void start()
    fn start(&self);

    // Java: abstract double visit(int row, int column, double value)
    fn visit(&self) -> f64;

    // Java: abstract double end()
    fn end(&self) -> f64;

}
