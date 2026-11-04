use super::UnivariateFunction::UnivariateFunction;

// Java: public interface DifferentiableUnivariateFunction
pub trait DifferentiableUnivariateFunction: UnivariateFunction {
    // Java: abstract UnivariateFunction derivative()
    fn derivative(&self) -> Box<dyn UnivariateFunction>;
}
