// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;

// Java: public class MaxIter implements OptimizationData
pub struct MaxIter {
    // Java field: int maxIter
    pub(crate) maxIter: i32,
}

// Java: public class MaxIter implements OptimizationData
impl MaxIter {
    // Java: MaxIter(int max)
    pub fn MaxIter(&self) {
        // Java: if (max <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(max);
        }
        // Java: maxIter = max;
    }

    // Java: int getMaxIter()
    pub fn getMaxIter(&self) -> i32 {
        // Java: return maxIter;
        todo!()
    }

    // Java: static MaxIter unlimited()
    pub fn unlimited(&self) -> MaxIter {
        // Java: return new MaxIter(Integer.MAX_VALUE);
        todo!()
    }

}

