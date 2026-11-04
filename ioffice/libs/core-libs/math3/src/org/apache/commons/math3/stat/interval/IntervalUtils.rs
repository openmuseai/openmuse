// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: BinomialConfidenceInterval AGRESTI_COULL
static AGRESTI_COULL: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: BinomialConfidenceInterval CLOPPER_PEARSON
static CLOPPER_PEARSON: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: BinomialConfidenceInterval NORMAL_APPROXIMATION
static NORMAL_APPROXIMATION: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: BinomialConfidenceInterval WILSON_SCORE
static WILSON_SCORE: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();

// Java: public class IntervalUtils
pub struct IntervalUtils {
}

// Java: public class IntervalUtils
impl IntervalUtils {
    // Java: IntervalUtils()
    pub fn IntervalUtils(&self) {
    }

    // Java: static ConfidenceInterval getAgrestiCoullInterval()
    pub fn getAgrestiCoullInterval(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static ConfidenceInterval getClopperPearsonInterval()
    pub fn getClopperPearsonInterval(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static ConfidenceInterval getNormalApproximationInterval()
    pub fn getNormalApproximationInterval(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static ConfidenceInterval getWilsonScoreInterval()
    pub fn getWilsonScoreInterval(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static void checkParameters(int numberOfTrials, int numberOfSuccesses, double confidenceLevel)
    pub fn checkParameters(&self) {
        // Java: if (numberOfTrials <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.NUMBER_OF_TRIALS, numberOfTrials);
        }
        // Java: if (numberOfSuccesses < 0)
        {
            // Java: throw new NotPositiveException(LocalizedFormats.NEGATIVE_NUMBER_OF_SUCCESSES, numberOfSuccesses);
        }
        // Java: if (numberOfSuccesses > numberOfTrials)
        {
            // Java: throw new NumberIsTooLargeException(LocalizedFormats.NUMBER_OF_SUCCESS_LARGER_THAN_POPULATION_SIZE,
                                                // Java: numberOfSuccesses, numberOfTrials, true);
        }
        // Java: if (confidenceLevel <= 0 || confidenceLevel >= 1)
        {
            // Java: throw new OutOfRangeException(LocalizedFormats.OUT_OF_BOUNDS_CONFIDENCE_LEVEL,
                                          // Java: confidenceLevel, 0, 1);
        }
    }

}
