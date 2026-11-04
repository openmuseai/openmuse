// Java: public interface representing
#[allow(non_camel_case_types)]
pub trait representing {
}

// Java: public interface MultivariateFunction
pub trait MultivariateFunction {
    // Java: abstract double value(double[] point)
    fn value(&self, point: &[f64]) -> f64;
}
