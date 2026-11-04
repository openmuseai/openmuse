// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java: public interface can
pub trait can {
}

// Java: public interface NumberTransformer
pub trait NumberTransformer {
    // Java: abstract double transform(Object o)
    fn transform(&self) -> f64;

}
