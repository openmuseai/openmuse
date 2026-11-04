// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class ConfidenceInterval
pub struct ConfidenceInterval {
    // Java field: double lowerBound
    pub(crate) lowerBound: f64,
    // Java field: double upperBound
    pub(crate) upperBound: f64,
    // Java field: double confidenceLevel
    pub(crate) confidenceLevel: f64,
}

// Java: public class ConfidenceInterval
impl ConfidenceInterval {
    // Java: ConfidenceInterval(double lowerBound, double upperBound, double confidenceLevel)
    pub fn ConfidenceInterval(&self) {
        // Java: checkParameters(lowerBound, upperBound, confidenceLevel);
        // Java: this.lowerBound = lowerBound;
        // Java: this.upperBound = upperBound;
        // Java: this.confidenceLevel = confidenceLevel;
    }

    // Java: double getLowerBound()
    pub fn getLowerBound(&self) -> f64 {
        // Java: return lowerBound;
        todo!()
    }

    // Java: double getUpperBound()
    pub fn getUpperBound(&self) -> f64 {
        // Java: return upperBound;
        todo!()
    }

    // Java: double getConfidenceLevel()
    pub fn getConfidenceLevel(&self) -> f64 {
        // Java: return confidenceLevel;
        todo!()
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "[" + lowerBound + ";" + upperBound + "] (confidence level:" + confidenceLevel + ")";
        todo!()
    }

    // Java: void checkParameters(double lower, double upper, double confidence)
    pub fn checkParameters(&self) {
        // Java: if (lower >= upper)
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.LOWER_BOUND_NOT_BELOW_UPPER_BOUND, lower, upper);
        }
        // Java: if (confidence <= 0 || confidence >= 1)
        {
            // Java: throw new MathIllegalArgumentException(LocalizedFormats.OUT_OF_BOUNDS_CONFIDENCE_LEVEL, confidence, 0, 1);
        }
    }

}
