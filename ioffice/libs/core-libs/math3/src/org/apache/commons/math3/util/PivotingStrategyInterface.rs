// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java field: throws MathIllegalArgumentException
// Java: public interface PivotingStrategyInterface
pub trait PivotingStrategyInterface {
    // Java: abstract int pivotIndex(double[] work, int begin, int end)
    fn pivotIndex(&self) -> i32;

}
