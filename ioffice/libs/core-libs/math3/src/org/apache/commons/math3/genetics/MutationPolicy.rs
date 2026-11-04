// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java: public interface MutationPolicy
pub trait MutationPolicy {
    // Java: abstract Chromosome mutate(Chromosome original)
    fn mutate(&self, original: &dyn std::any::Any) -> Box<dyn std::any::Any>;

}
