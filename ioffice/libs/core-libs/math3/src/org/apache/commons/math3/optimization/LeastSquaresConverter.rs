// Java: import org.apache.commons.math3.analysis.MultivariateFunction
use crate::org::apache::commons::math3::analysis::MultivariateFunction;
// Java: import org.apache.commons.math3.analysis.MultivariateVectorFunction
use crate::org::apache::commons::math3::analysis::MultivariateVectorFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix;

// Java: public class converts
pub struct converts {
}

// Java: public class is
pub struct is {
}

// Java: public class support
pub struct support {
}

// Java: public class LeastSquaresConverter
pub struct LeastSquaresConverter {
    // Java field: MultivariateVectorFunction function
    pub(crate) function: Box<dyn std::any::Any>,
    // Java field: double[] observations
    pub(crate) observations: Box<dyn std::any::Any>,
    // Java field: double[] weights
    pub(crate) weights: Box<dyn std::any::Any>,
    // Java field: RealMatrix scale
    pub(crate) scale: Box<dyn std::any::Any>,
}

// Java: public class converts
impl converts {
}

// Java: public class is
impl is {
}

// Java: public class support
impl support {
}

// Java: public class LeastSquaresConverter
impl LeastSquaresConverter {
    // Java: LeastSquaresConverter()
    pub fn LeastSquaresConverter(&self) {
    }

    // Java: LeastSquaresConverter()
    pub fn LeastSquaresConverter_2(&self) {
    }

    // Java: LeastSquaresConverter()
    pub fn LeastSquaresConverter_3(&self) {
    }

    // Java: double value(final double[] point)
    pub fn value(&self) -> f64 {
        // Java: // compute residuals
        // Java: final double[] residuals = function.value(point);
        // Java: if (residuals.length != observations.length)
        {
            // Java: throw new DimensionMismatchException(residuals.length, observations.length);
        }
        // Java: for (int i = 0; i < residuals.length; ++i)
        {
            // Java: residuals[i] -= observations[i];
        }
        // Java: // compute sum of squares
        // Java: double sumSquares = 0;
        // Java: if (weights != null)
        {
            // Java: for (int i = 0; i < residuals.length; ++i)
            {
                // Java: final double ri = residuals[i];
                // Java: sumSquares +=  weights[i] * ri * ri;
            }
        }
        // Java: else if (scale != null)
        {
            // Java: for (final double yi : scale.operate(residuals))
            {
                // Java: sumSquares += yi * yi;
            }
        }
        // Java: else
        {
            // Java: for (final double ri : residuals)
            {
                // Java: sumSquares += ri * ri;
            }
        }
        // Java: return sumSquares;
        todo!()
    }

}
