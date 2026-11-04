// Java: import org.apache.commons.math3.analysis.TrivariateFunction
use crate::org::apache::commons::math3::analysis::TrivariateFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NonMonotonicSequenceException
use crate::org::apache::commons::math3::exception::NonMonotonicSequenceException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;

// Java field: DimensionMismatchException, NonMonotonicSequenceException
// Java: public interface TrivariateGridInterpolator
pub trait TrivariateGridInterpolator {
    // Java: abstract TrivariateFunction interpolate()
    fn interpolate(&self) -> Box<dyn std::any::Any>;

}
