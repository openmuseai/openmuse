// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement::RealFieldElement;

// Java: public class LocalException extends RuntimeException
pub struct LocalException {
}

// Java: public class MyFunction implements FieldUnivariateFunction
pub struct MyFunction {
}

// Java: public interface representing
// Java: public interface RealFieldUnivariateFunction extends RealFieldElement
pub trait RealFieldUnivariateFunction<T: RealFieldElement> {
    // Java: abstract T value(T x)
    fn value(&self, x: T) -> T;
}

// Java: public class LocalException extends RuntimeException
impl LocalException {
}

// Java: public class MyFunction implements FieldUnivariateFunction
impl MyFunction {
}
