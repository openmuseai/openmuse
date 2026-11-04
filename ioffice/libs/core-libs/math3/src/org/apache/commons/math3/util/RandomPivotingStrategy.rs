// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;

// Java field: long serialVersionUID

// Java: public class RandomPivotingStrategy implements PivotingStrategyInterface, Serializable
pub struct RandomPivotingStrategy {
    // Java field: RandomGenerator random
    random: Box<dyn std::any::Any>,
}

// Java: public class RandomPivotingStrategy implements PivotingStrategyInterface, Serializable
impl RandomPivotingStrategy {
    // Java: RandomPivotingStrategy(final RandomGenerator random)
    pub fn RandomPivotingStrategy(&self) {
        // Java: this.random = random;
    }

    // Java: int pivotIndex(final double[] work, final int begin, final int end)
    pub fn pivotIndex(&self) -> i32 {
        todo!()
    }

}


