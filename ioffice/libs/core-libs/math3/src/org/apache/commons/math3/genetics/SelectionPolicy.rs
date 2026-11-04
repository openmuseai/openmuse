// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java: public interface SelectionPolicy
pub trait SelectionPolicy {
    // Java: abstract ChromosomePair select(Population population)
    fn select(&self, population: &dyn std::any::Any) -> Box<dyn std::any::Any>;

}
