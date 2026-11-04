// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java: public interface CrossoverPolicy
pub trait CrossoverPolicy {
    // Java: abstract ChromosomePair crossover(Chromosome first, Chromosome second)
    fn crossover(&self, first: &dyn std::any::Any, second: &dyn std::any::Any) -> Box<dyn std::any::Any>;

}
