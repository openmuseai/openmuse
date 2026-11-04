// Java: public interface DifferentiableUnivariateVectorFunction
pub trait DifferentiableUnivariateVectorFunction: UnivariateVectorFunction {
    // Java: abstract UnivariateVectorFunction derivative()
    fn derivative(&self) -> Box<dyn UnivariateVectorFunction>;
}

pub trait UnivariateVectorFunction {
    fn value(&self, x: f64) -> Vec<f64>;
}
