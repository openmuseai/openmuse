// Java: public interface representing
#[allow(non_camel_case_types)]
pub trait representing {
}

// Java: public interface BivariateFunction
pub trait BivariateFunction {
    // Java: abstract double value(double x, double y)
    fn value(&self, x: f64, y: f64) -> f64;
}
