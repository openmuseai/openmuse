use super::MultivariateFunction::MultivariateFunction;

// Java: public interface DifferentiableMultivariateFunction extends MultivariateFunction
pub trait DifferentiableMultivariateFunction: MultivariateFunction {
    // Java: abstract MultivariateFunction partialDerivative(int k)
    fn partialDerivative(&self, k: i32) -> Box<dyn MultivariateFunction>;

    // Java: abstract MultivariateVectorFunction gradient()
    fn gradient(&self) -> Box<dyn MultivariateVectorFunction>;
}

pub trait MultivariateVectorFunction {
    fn value(&self, point: &[f64]) -> Vec<f64>;
}
