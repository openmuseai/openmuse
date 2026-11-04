// Java: import org.apache.commons.math3.analysis.MultivariateFunction
use crate::org::apache::commons::math3::analysis::MultivariateFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java: public class MultivariateFunctionPenaltyAdapter implements MultivariateFunction
pub struct MultivariateFunctionPenaltyAdapter {
    // Java field: MultivariateFunction bounded
    pub(crate) bounded: Box<dyn std::any::Any>,
    // Java field: double[] lower
    lower: Vec<f64>,
    // Java field: double[] upper
    upper: Vec<f64>,
    // Java field: double offset
    pub(crate) offset: f64,
    // Java field: double[] scale
    scale: Vec<f64>,
}

// Java: public class MultivariateFunctionPenaltyAdapter implements MultivariateFunction
impl MultivariateFunctionPenaltyAdapter {
    // Java: MultivariateFunctionPenaltyAdapter()
    pub fn MultivariateFunctionPenaltyAdapter(&self) {
    }

    // Java: double value(double[] point)
    pub fn value(&self) -> f64 {
        // Java: for (int i = 0; i < scale.length; ++i)
        {
            // Java: if ((point[i] < lower[i]) || (point[i] > upper[i]))
            {
                // Java: // bound violation starting at this component
                // Java: double sum = 0;
                // Java: for (int j = i; j < scale.length; ++j)
                {
                    // Java: final double overshoot;
                    // Java: if (point[j] < lower[j])
                    {
                        // Java: overshoot = scale[j] * (lower[j] - point[j]);
                    }
                    // Java: else if (point[j] > upper[j])
                    {
                        // Java: overshoot = scale[j] * (point[j] - upper[j]);
                    }
                    // Java: else
                    {
                        // Java: overshoot = 0;
                    }
                    // Java: sum += FastMath.sqrt(overshoot);
                }
                // Java: return offset + sum;
            }
        }
        // Java: // all boundaries are fulfilled, we are in the expected
        // Java: // domain of the underlying function
        // Java: return bounded.value(point);
        todo!()
    }

}

