// Java: import org.apache.commons.math3.analysis.BivariateFunction
use crate::org::apache::commons::math3::analysis::BivariateFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NonMonotonicSequenceException
use crate::org::apache::commons::math3::exception::NonMonotonicSequenceException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;

// Java field: NonMonotonicSequenceException, NumberIsTooSmallException
// Java: public interface BivariateGridInterpolator
pub trait BivariateGridInterpolator {
    // Java: abstract BivariateFunction interpolate()
    fn interpolate(&self) -> Box<dyn std::any::Any>;

}
