// Java: public interface representing
// Java: public interface MultivariateMatrixFunction
pub trait MultivariateMatrixFunction {
    // Java field: throws IllegalArgumentException
    // Java: abstract double[][] value(double[] point)
    fn value(&self, point: &[f64]) -> Vec<Vec<f64>>;
}
