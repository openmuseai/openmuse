// Java: import org.apache.commons.math3.exception.ConvergenceException
use crate::org::apache::commons::math3::exception::ConvergenceException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class FractionConversionException extends ConvergenceException
pub struct FractionConversionException {
}

// Java: public class FractionConversionException extends ConvergenceException
impl FractionConversionException {
    // Java: FractionConversionException(double value, int maxIterations)
    pub fn FractionConversionException(&self, value: f64, maxIterations: i32) {
        // Java: super(LocalizedFormats.FAILED_FRACTION_CONVERSION, value, maxIterations);
    }

    // Java: FractionConversionException(double value, long p, long q)
    pub fn FractionConversionException_1(&self, value: f64, p: i64, q: i64) {
        // Java: super(LocalizedFormats.FRACTION_CONVERSION_OVERFLOW, value, p, q);
    }

}
