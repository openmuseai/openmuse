// Java: public interface DifferentiableUnivariateMatrixFunction
pub trait DifferentiableUnivariateMatrixFunction: UnivariateMatrixFunction {
    // Java: abstract UnivariateMatrixFunction derivative()
    fn derivative(&self) -> Box<dyn UnivariateMatrixFunction>;
}

pub trait UnivariateMatrixFunction {
    fn value(&self, x: f64) -> Vec<Vec<f64>>;
}
