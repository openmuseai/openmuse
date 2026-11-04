// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;

// Java: public class MaxEval implements OptimizationData
pub struct MaxEval {
    // Java field: int maxEval
    pub(crate) maxEval: i32,
}

// Java: public class MaxEval implements OptimizationData
impl MaxEval {
    // Java: MaxEval(int max)
    pub fn new(max: i32) -> Self {
        // Java: if (max <= 0)
        if max <= 0 {
            // Java: throw new NotStrictlyPositiveException(max);
            panic!("NotStrictlyPositiveException: {}", max);
        }
        // Java: maxEval = max;
        MaxEval { maxEval: max }
    }

    // Java: int getMaxEval()
    pub fn getMaxEval(&self) -> i32 {
        // Java: return maxEval;
        self.maxEval
    }

    // Java: static MaxEval unlimited()
    pub fn unlimited() -> MaxEval {
        // Java: return new MaxEval(Integer.MAX_VALUE);
        MaxEval::new(i32::MAX)
    }

}
