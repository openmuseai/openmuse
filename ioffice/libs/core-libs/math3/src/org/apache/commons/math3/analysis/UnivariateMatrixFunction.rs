// Java: public interface representing
// Java: public interface UnivariateMatrixFunction
pub trait UnivariateMatrixFunction {
    // Java: abstract double[][] value(double x)
    fn value(&self, x: f64) -> Vec<Vec<f64>>;
}
