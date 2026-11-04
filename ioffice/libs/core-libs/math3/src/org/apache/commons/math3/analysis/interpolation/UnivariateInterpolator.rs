// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;

// Java field: throws MathIllegalArgumentException, DimensionMismatchException
// Java: public interface UnivariateInterpolator
pub trait UnivariateInterpolator {
    // Java: abstract UnivariateFunction interpolate(double xval[], double yval[])
    fn interpolate(&self) -> Box<dyn std::any::Any>;

}
