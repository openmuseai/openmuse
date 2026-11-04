// Java: public class LocalException extends RuntimeException
pub struct LocalException {
}

// Java: public class MyFunction implements UnivariateFunction
pub struct MyFunction {
}

// Java: public interface representing
// Java: public interface UnivariateFunction
pub trait UnivariateFunction {
    // Java: abstract double value(double x)
    fn value(&self, x: f64) -> f64;
}

// Java: public class LocalException extends RuntimeException
impl LocalException {
}

// Java: public class MyFunction implements UnivariateFunction
impl MyFunction {
}
