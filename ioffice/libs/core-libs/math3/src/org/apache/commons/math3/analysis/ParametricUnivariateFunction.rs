// Java: public interface representing
// Java: public interface ParametricUnivariateFunction
pub trait ParametricUnivariateFunction {
    // Java: abstract double value(double x, double ... parameters)
    fn value(&self, x: f64, parameters: &[f64]) -> f64;

    // Java: abstract double[] gradient(double x, double ... parameters)
    fn gradient(&self, x: f64, parameters: &[f64]) -> Vec<f64>;
}
