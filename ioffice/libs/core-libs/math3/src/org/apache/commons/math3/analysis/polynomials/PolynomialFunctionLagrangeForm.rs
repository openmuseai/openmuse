// Java: import org.apache.commons.math3.analysis.UnivariateFunction
use crate::org::apache::commons::math3::analysis::UnivariateFunction;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.NonMonotonicSequenceException
use crate::org::apache::commons::math3::exception::NonMonotonicSequenceException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class PolynomialFunctionLagrangeForm implements UnivariateFunction
pub struct PolynomialFunctionLagrangeForm {
    // Java field: boolean coefficientsComputed
    coefficientsComputed: bool,
}

// Java: public class PolynomialFunctionLagrangeForm implements UnivariateFunction
impl PolynomialFunctionLagrangeForm {
    // Java: PolynomialFunctionLagrangeForm(double x[], double y[])
    pub fn PolynomialFunctionLagrangeForm(&self) {
    }

    // Java: double value(double z)
    pub fn value(&self) -> f64 {
        // Java: return evaluateInternal(x, y, z);
        todo!()
    }

    // Java: int degree()
    pub fn degree(&self) -> i32 {
        // Java: return x.length - 1;
        todo!()
    }

    // Java: double[] getInterpolatingPoints()
    pub fn getInterpolatingPoints(&self) -> Vec<f64> {
        // Java: double[] out = new double[x.length];
        // Java: System.arraycopy(x, 0, out, 0, x.length);
        // Java: return out;
        todo!()
    }

    // Java: double[] getInterpolatingValues()
    pub fn getInterpolatingValues(&self) -> Vec<f64> {
        // Java: double[] out = new double[y.length];
        // Java: System.arraycopy(y, 0, out, 0, y.length);
        // Java: return out;
        todo!()
    }

    // Java: double[] getCoefficients()
    pub fn getCoefficients(&self) -> Vec<f64> {
        // Java: if (!coefficientsComputed)
        {
            // Java: computeCoefficients();
        }
        // Java: double[] out = new double[coefficients.length];
        // Java: System.arraycopy(coefficients, 0, out, 0, coefficients.length);
        // Java: return out;
        todo!()
    }

    // Java: static double evaluate(double x[], double y[], double z)
    pub fn evaluate(&self) -> f64 {
        todo!()
    }

    // Java: static double evaluateInternal(double x[], double y[], double z)
    pub fn evaluateInternal(&self) -> f64 {
        // Java: int nearest = 0;
        // Java: final int n = x.length;
        // Java: final double[] c = new double[n];
        // Java: final double[] d = new double[n];
        // Java: double min_dist = Double.POSITIVE_INFINITY;
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: // initialize the difference arrays
            // Java: c[i] = y[i];
            // Java: d[i] = y[i];
            // Java: // find out the abscissa closest to z
            // Java: final double dist = FastMath.abs(z - x[i]);
            // Java: if (dist < min_dist)
            {
                // Java: nearest = i;
                // Java: min_dist = dist;
            }
        }
        // Java: // initial approximation to the function value at z
        // Java: double value = y[nearest];
        // Java: for (int i = 1; i < n; i++)
        {
            // Java: for (int j = 0; j < n-i; j++)
            {
                // Java: final double tc = x[j] - z;
                // Java: final double td = x[i+j] - z;
                // Java: final double divider = x[j] - x[i+j];
                // Java: // update the difference arrays
                // Java: final double w = (c[j+1] - d[j]) / divider;
                // Java: c[j] = tc * w;
                // Java: d[j] = td * w;
            }
            // Java: // sum up the difference terms to get the final value
            // Java: if (nearest < 0.5*(n-i+1))
            {
                // Java: value += c[nearest];    // fork down
            }
            // Java: else
            {
                // Java: nearest--;
                // Java: value += d[nearest];    // fork up
            }
        }
        // Java: return value;
        todo!()
    }

    // Java: void computeCoefficients()
    pub fn computeCoefficients(&self) {
        // Java: final int n = degree() + 1;
        // Java: coefficients = new double[n];
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: coefficients[i] = 0.0;
        }
        // Java: // c[] are the coefficients of P(x) = (x-x[0])(x-x[1])...(x-x[n-1])
        // Java: final double[] c = new double[n+1];
        // Java: c[0] = 1.0;
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: for (int j = i; j > 0; j--)
            {
                // Java: c[j] = c[j-1] - c[j] * x[i];
            }
            // Java: c[0] *= -x[i];
            // Java: c[i+1] = 1;
        }
        // Java: final double[] tc = new double[n];
        // Java: for (int i = 0; i < n; i++)
        {
            // Java: // d = (x[i]-x[0])...(x[i]-x[i-1])(x[i]-x[i+1])...(x[i]-x[n-1])
            // Java: double d = 1;
            // Java: for (int j = 0; j < n; j++)
            {
                // Java: if (i != j)
                {
                    // Java: d *= x[i] - x[j];
                }
            }
            // Java: final double t = y[i] / d;
            // Java: // Lagrange polynomial is the sum of n terms, each of which is a
            // Java: // polynomial of degree n-1. tc[] are the coefficients of the i-th
            // Java: // numerator Pi(x) = (x-x[0])...(x-x[i-1])(x-x[i+1])...(x-x[n-1]).
            // Java: tc[n-1] = c[n];     // actually c[n] = 1
            // Java: coefficients[n-1] += t * tc[n-1];
            // Java: for (int j = n-2; j >= 0; j--)
            {
                // Java: tc[j] = c[j+1] + tc[j+1] * x[i];
                // Java: coefficients[j] += t * tc[j];
            }
        }
        // Java: coefficientsComputed = true;
    }

    // Java: static boolean verifyInterpolationArray(double x[], double y[], boolean abort)
    pub fn verifyInterpolationArray(&self) -> bool {
        todo!()
    }

}

