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

// Java: public class BicubicSplineInterpolatingFunction
pub struct BicubicSplineInterpolatingFunction {
}

// Java: public class BicubicSplineFunction implements BivariateFunction
pub struct BicubicSplineFunction {
    // Java field: double[][] a
    a: Vec<Vec<f64>>,
    // Java field: BivariateFunction partialDerivativeX
    pub(crate) partialDerivativeX: Box<dyn std::any::Any>,
    // Java field: BivariateFunction partialDerivativeY
    pub(crate) partialDerivativeY: Box<dyn std::any::Any>,
    // Java field: BivariateFunction partialDerivativeXX
    pub(crate) partialDerivativeXX: Box<dyn std::any::Any>,
    // Java field: BivariateFunction partialDerivativeYY
    pub(crate) partialDerivativeYY: Box<dyn std::any::Any>,
    // Java field: BivariateFunction partialDerivativeXY
    pub(crate) partialDerivativeXY: Box<dyn std::any::Any>,
}

// Java: public class BicubicSplineInterpolatingFunction
impl BicubicSplineInterpolatingFunction {
}

// Java: public class BicubicSplineFunction implements BivariateFunction
impl BicubicSplineFunction {
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
            // Java: for (int j = 0; j < N; j++)
            {
                // Java: result += coeff[i][j] * pX[i] * pY[j];
            }
        }
        // Java: return result;
        todo!()
    }

    // Java: BivariateFunction partialDerivativeX()
    pub fn partialDerivativeX(&self) -> Box<dyn std::any::Any> {
        // Java: return partialDerivativeX;
        todo!()
    }

    // Java: BivariateFunction partialDerivativeY()
    pub fn partialDerivativeY(&self) -> Box<dyn std::any::Any> {
        // Java: return partialDerivativeY;
        todo!()
    }

    // Java: BivariateFunction partialDerivativeXX()
    pub fn partialDerivativeXX(&self) -> Box<dyn std::any::Any> {
        // Java: return partialDerivativeXX;
        todo!()
    }

    // Java: BivariateFunction partialDerivativeYY()
    pub fn partialDerivativeYY(&self) -> Box<dyn std::any::Any> {
        // Java: return partialDerivativeYY;
        todo!()
    }

    // Java: BivariateFunction partialDerivativeXY()
    pub fn partialDerivativeXY(&self) -> Box<dyn std::any::Any> {
        // Java: return partialDerivativeXY;
        todo!()
    }

}

