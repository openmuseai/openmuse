use super::DifferentiableMultivariateFunction::MultivariateVectorFunction;

// Java: public interface DifferentiableMultivariateVectorFunction
pub trait DifferentiableMultivariateVectorFunction: MultivariateVectorFunction {
    // Java: abstract MultivariateMatrixFunction jacobian()
    fn jacobian(&self) -> Box<dyn MultivariateMatrixFunction>;
}

pub trait MultivariateMatrixFunction {
    fn value(&self, point: &[f64]) -> Vec<Vec<f64>>;
}
