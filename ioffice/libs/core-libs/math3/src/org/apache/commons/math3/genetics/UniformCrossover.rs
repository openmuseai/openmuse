// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;

// Java: public class UniformCrossover implements CrossoverPolicy
pub struct UniformCrossover {
    // Java field: double ratio
    pub(crate) ratio: f64,
}

// Java: public class UniformCrossover implements CrossoverPolicy
impl UniformCrossover {
    // Java: UniformCrossover(final double ratio)
    pub fn new(ratio: f64) -> Self {
        // Java: if (ratio < 0.0d || ratio > 1.0d)
        if ratio < 0.0 || ratio > 1.0 {
            // Java: throw new OutOfRangeException(LocalizedFormats.CROSSOVER_RATE, ratio, 0.0d, 1.0d);
            panic!("Crossover ratio out of range: {}", ratio);
        }
        // Java: this.ratio = ratio;
        UniformCrossover { ratio }
    }

    // Java: double getRatio()
    pub fn getRatio(&self) -> f64 {
        // Java: return ratio;
        self.ratio
    }

    // Java: ChromosomePair crossover(final Chromosome first, final Chromosome second)
    pub fn crossover(&self, _first: &dyn std::any::Any, _second: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        todo!("UniformCrossover::crossover")
    }

    // Java: ChromosomePair mate()
    pub fn mate(&self) -> Box<dyn std::any::Any> {
        todo!("UniformCrossover::mate")
    }

}
