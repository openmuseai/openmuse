// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;

// Java: public class FixedGenerationCount implements StoppingCondition
pub struct FixedGenerationCount {
    // Java field: int numGenerations
    pub(crate) numGenerations: i32,
    // Java field: int maxGenerations
    maxGenerations: i32,
}

// Java: public class FixedGenerationCount implements StoppingCondition
impl FixedGenerationCount {
    // Java: FixedGenerationCount(final int maxGenerations)
    pub fn new(max_generations: i32) -> Self {
        // Java: if (maxGenerations <= 0)
        if max_generations <= 0 {
            // Java: throw new NumberIsTooSmallException(maxGenerations, 1, true);
            panic!("Max generations must be positive: {}", max_generations);
        }
        // Java: this.maxGenerations = maxGenerations;
        FixedGenerationCount { numGenerations: 0, maxGenerations: max_generations }
    }

    // Java: boolean isSatisfied(final Population population)
    pub fn isSatisfied(&mut self, _population: &dyn std::any::Any) -> bool {
        // Java: if (this.numGenerations < this.maxGenerations)
        if self.numGenerations < self.maxGenerations {
            // Java: numGenerations++;
            self.numGenerations += 1;
            // Java: return false;
            return false;
        }
        // Java: return true;
        true
    }

    // Java: int getNumGenerations()
    pub fn getNumGenerations(&self) -> i32 {
        // Java: return numGenerations;
        self.numGenerations
    }

}
