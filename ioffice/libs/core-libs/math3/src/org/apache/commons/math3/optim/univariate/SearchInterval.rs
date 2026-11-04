// Java: import org.apache.commons.math3.optim.OptimizationData
use crate::org::apache::commons::math3::optim::OptimizationData;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;

// Java: public class SearchInterval implements OptimizationData
pub struct SearchInterval {
    // Java field: double lower
    pub(crate) lower: f64,
    // Java field: double upper
    pub(crate) upper: f64,
    // Java field: double start
    pub(crate) start: f64,
}

// Java: public class SearchInterval implements OptimizationData
impl SearchInterval {
    // Java: SearchInterval()
    pub fn SearchInterval(&self) {
    }

    // Java: SearchInterval()
    pub fn SearchInterval_2(&self) {
    }

    // Java: double getMin()
    pub fn getMin(&self) -> f64 {
        // Java: return lower;
        todo!()
    }

    // Java: double getMax()
    pub fn getMax(&self) -> f64 {
        // Java: return upper;
        todo!()
    }

    // Java: double getStartValue()
    pub fn getStartValue(&self) -> f64 {
        // Java: return start;
        todo!()
    }

}

