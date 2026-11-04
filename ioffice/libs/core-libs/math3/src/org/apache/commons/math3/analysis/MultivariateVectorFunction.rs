// Java: public interface representing
#[allow(non_camel_case_types)]
pub trait representing {
}

// Java field: throws IllegalArgumentException

// Java: public interface MultivariateVectorFunction
pub trait MultivariateVectorFunction {
    // Java: abstract double[] value(double[] point)
    fn value(&self, point: &[f64]) -> Vec<f64>;

}
