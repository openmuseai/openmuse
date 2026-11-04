// Java: import java.io.Serializable
// Java: import java.math.BigDecimal
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.fraction.BigFraction
use crate::org::apache::commons::math3::fraction::BigFraction::BigFraction;
// Java: import org.apache.commons.math3.fraction.BigFractionField
use crate::org::apache::commons::math3::fraction::BigFractionField;
// Java: import org.apache.commons.math3.fraction.FractionConversionException
use crate::org::apache::commons::math3::fraction::FractionConversionException;
// Java: import org.apache.commons.math3.linear.Array2DRowFieldMatrix
use crate::org::apache::commons::math3::linear::Array2DRowFieldMatrix;
// Java: import org.apache.commons.math3.linear.Array2DRowRealMatrix
use crate::org::apache::commons::math3::linear::Array2DRowRealMatrix;
// Java: import org.apache.commons.math3.linear.FieldMatrix
use crate::org::apache::commons::math3::linear::FieldMatrix::FieldMatrix;
// Java: import org.apache.commons.math3.linear.RealMatrix
use crate::org::apache::commons::math3::linear::RealMatrix::RealMatrix;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class KolmogorovSmirnovDistribution implements Serializable
pub struct KolmogorovSmirnovDistribution {
    // Java field: int n
    n: i32,
}

// Java: public class KolmogorovSmirnovDistribution implements Serializable
impl KolmogorovSmirnovDistribution {
    // Java: KolmogorovSmirnovDistribution(int n)
    pub fn KolmogorovSmirnovDistribution(&self, n: i32) {
    }

    // Java: double cdf(double d)
    pub fn cdf(&self, d: f64) -> f64 {
        // Java: return this.cdf(d, false);
        todo!();
    }

    // Java: double cdfExact(double d)
    pub fn cdfExact(&self, d: f64) -> f64 {
        // Java: return this.cdf(d, true);
        todo!();
    }

    // Java: double cdf(double d, boolean exact)
    pub fn cdf_1(&self, d: f64, exact: bool) -> f64 {
        // Java: final double ninv = 1 / ((double) n);
        // Java: final double ninvhalf = 0.5 * ninv;
        // Java: if (d <= ninvhalf)
        {
            // Java: return 0;
        }
        // Java: else if (ninvhalf < d && d <= ninv)
        {
            // Java: double res = 1;
            // Java: double f = 2 * d - ninv;
            // Java: // n! f^n = n*f * (n-1)*f * ... * 1*x
            // Java: for (int i = 1; i <= n; ++i)
            {
                // Java: res *= i * f;
            }
            // Java: return res;
        }
        // Java: else if (1 - ninv <= d && d < 1)
        {
            // Java: return 1 - 2 * FastMath.pow(1 - d, n);
        }
        // Java: else if (1 <= d)
        {
            // Java: return 1;
        }
        // Java: return exact ? exactK(d) : roundedK(d);
        todo!();
    }

    // Java: double exactK(double d)
    pub fn exactK(&self, d: f64) -> f64 {
        // Java: final int k = (int) FastMath.ceil(n * d);
        // Java: final FieldMatrix<BigFraction> H = this.createH(d);
        // Java: final FieldMatrix<BigFraction> Hpower = H.power(n);
        // Java: BigFraction pFrac = Hpower.getEntry(k - 1, k - 1);
        // Java: for (int i = 1; i <= n; ++i)
        {
            // Java: pFrac = pFrac.multiply(i).divide(n);
        }
        // Java: /*
         // Java: * BigFraction.doubleValue converts numerator to double and the
         // Java: * denominator to double and divides afterwards. That gives NaN quite
         // Java: * easy. This does not (scale is the number of digits):
         // Java: */
        // Java: return pFrac.bigDecimalValue(20, BigDecimal.ROUND_HALF_UP).doubleValue();
        todo!();
    }

    // Java: double roundedK(double d)
    pub fn roundedK(&self, d: f64) -> f64 {
        // Java: final int k = (int) FastMath.ceil(n * d);
        // Java: final FieldMatrix<BigFraction> HBigFraction = this.createH(d);
        // Java: final int m = HBigFraction.getRowDimension();
        // Java: /*
         // Java: * Here the rounding part comes into play: use
         // Java: * RealMatrix instead of FieldMatrix<BigFraction>
         // Java: */
        // Java: final RealMatrix H = new Array2DRowRealMatrix(m, m);
        // Java: for (int i = 0; i < m; ++i)
        {
            // Java: for (int j = 0; j < m; ++j)
            {
                // Java: H.setEntry(i, j, HBigFraction.getEntry(i, j).doubleValue());
            }
        }
        // Java: final RealMatrix Hpower = H.power(n);
        // Java: double pFrac = Hpower.getEntry(k - 1, k - 1);
        // Java: for (int i = 1; i <= n; ++i)
        {
            // Java: pFrac *= (double) i / (double) n;
        }
        // Java: return pFrac;
        todo!();
    }

    // Java: FieldMatrix<BigFraction> createH(double d)
    pub fn createH(&self, d: f64) -> Box<dyn std::any::Any> {
        todo!();
    }

}

// impl Serializable for KolmogorovSmirnovDistribution {
// }
