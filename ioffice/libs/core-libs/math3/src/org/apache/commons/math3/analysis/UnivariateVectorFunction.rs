// Java: public interface representing
// Java: public interface UnivariateVectorFunction
pub trait UnivariateVectorFunction {
    // Java: abstract double[] value(double x)
    fn value(&self, x: f64) -> Vec<f64>;
}
