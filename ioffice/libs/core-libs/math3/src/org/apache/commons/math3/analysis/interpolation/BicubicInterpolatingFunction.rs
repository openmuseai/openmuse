// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.analysis.BivariateFunction
use crate::org::apache::commons::math3::analysis::BivariateFunction;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NoDataException
use crate::org::apache::commons::math3::exception::NoDataException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.NonMonotonicSequenceException
use crate::org::apache::commons::math3::exception::NonMonotonicSequenceException;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java field: short N
pub(crate) const N: i16 = 0;

// Java: public class BicubicInterpolatingFunction
pub struct BicubicInterpolatingFunction {
}

// Java: public class BicubicFunction implements BivariateFunction
pub struct BicubicFunction {
    // Java field: double[][] a
    a: Vec<Vec<f64>>,
}

// Java: public class BicubicInterpolatingFunction
impl BicubicInterpolatingFunction {
}

// Java: public class BicubicFunction implements BivariateFunction
impl BicubicFunction {
    // Java: double value(double x, double y)
    pub fn value(&self) -> f64 {
        // Java: if (x < 0 || x > 1)
        {
            // Java: throw new OutOfRangeException(x, 0, 1);
        }
        // Java: if (y < 0 || y > 1)
        {
            // Java: throw new OutOfRangeException(y, 0, 1);
        }
        // Java: final double x2 = x * x;
        // Java: final double x3 = x2 * x;
        // Java: final double[] pX = {1, x, x2, x3};
        // Java: final double y2 = y * y;
        // Java: final double y3 = y2 * y;
        // Java: final double[] pY = {1, y, y2, y3};
        // Java: return apply(pX, pY, a);
        todo!()
    }

    // Java: double apply(double[] pX, double[] pY, double[][] coeff)
    pub fn apply(&self) -> f64 {
        // Java: double result = 0;
        // Java: for (int i = 0; i < N; i++)
        {
            // Java: final double r = MathArrays.linearCombination(coeff[i], pY);
            // Java: result += r * pX[i];
        }
        // Java: return result;
        todo!()
    }

}

