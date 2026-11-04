// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;

// Java field: NumberIsTooLargeException, OutOfRangeException

// Java: public interface BinomialConfidenceInterval
pub trait BinomialConfidenceInterval {
    // Java: abstract ConfidenceInterval createInterval(int numberOfTrials, int numberOfSuccesses, double confidenceLevel)
    fn createInterval(&self) -> Box<dyn std::any::Any>;

}
