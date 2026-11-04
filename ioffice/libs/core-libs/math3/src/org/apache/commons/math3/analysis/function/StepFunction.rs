// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.NonMonotonicSequenceException
use crate::org::apache::commons::math3::exception::NonMonotonicSequenceException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java: public class StepFunction implements UnivariateFunction
pub struct StepFunction {
    // Java field: double[] abscissa
    abscissa: Vec<f64>,
    // Java field: double[] ordinate
    ordinate: Vec<f64>,
}

// Java: public class StepFunction implements UnivariateFunction
impl StepFunction {
    // Java: StepFunction()
    pub fn StepFunction(&self) {
    }

    // Java: double value(double x)
    pub fn value(&self) -> f64 {
        // Java: int index = Arrays.binarySearch(abscissa, x);
        // Java: double fx = 0;
        // Java: if (index < -1)
        {
            // Java: // "x" is between "abscissa[-index-2]" and "abscissa[-index-1]".
            // Java: fx = ordinate[-index-2];
        }
        // Java: else if (index >= 0)
        {
            // Java: // "x" is exactly "abscissa[index]".
            // Java: fx = ordinate[index];
        }
        // Java: else
        {
            // Java: // Otherwise, "x" is smaller than the first value in "abscissa"
            // Java: // (hence the returned value should be "ordinate[0]").
            // Java: fx = ordinate[0];
        }
        // Java: return fx;
        todo!()
    }

}

