// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 4330003017885151975;

// Java: public class MaxCountExceededException extends MathIllegalStateException
#[derive(Debug, Clone)]
pub struct MaxCountExceededException {
    // Java field: Number max
    pub(crate) max: i64,
}

// Java: public class MaxCountExceededException extends MathIllegalStateException
impl MaxCountExceededException {
    // Java: MaxCountExceededException(Number max)
    pub fn MaxCountExceededException(max: i64) -> Self {
        // Java: this(LocalizedFormats.MAX_COUNT_EXCEEDED, max);
        MaxCountExceededException { max }
    }

    // Java: MaxCountExceededException()
    pub fn MaxCountExceededException_specific(_specific: &str, max: i64) -> Self {
        MaxCountExceededException { max }
    }

    // Java: Number getMax()
    pub fn getMax(&self) -> i64 {
        // Java: return max;
        self.max
    }
}

impl std::fmt::Display for MaxCountExceededException {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "MaxCountExceededException: max={}", self.max)
    }
}

impl std::error::Error for MaxCountExceededException {}
