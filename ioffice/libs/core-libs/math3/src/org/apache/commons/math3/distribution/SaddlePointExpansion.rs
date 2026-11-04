// Java: import org.apache.commons.math3.special.Gamma
use crate::org::apache::commons::math3::special::Gamma;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: double HALF_LOG_2_PI
pub(crate) const HALF_LOG_2_PI: f64 = 0.0;

// Java: public class used
pub struct used {
}

// Java: public class SaddlePointExpansion
pub struct SaddlePointExpansion {
}

// Java: public class used
impl used {
}

// Java: public class SaddlePointExpansion
impl SaddlePointExpansion {
    // Java: SaddlePointExpansion()
    pub fn SaddlePointExpansion(&self) {
        // Java: super();
    }

    // Java: static double getStirlingError(double z)
    pub fn getStirlingError(&self, z: f64) -> f64 {
        // Java: double ret;
        // Java: if (z < 15.0)
        {
            // Java: double z2 = 2.0 * z;
            // Java: if (FastMath.floor(z2) == z2)
            {
                // Java: ret = EXACT_STIRLING_ERRORS[(int) z2];
            }
            // Java: else
            {
                // Java: ret = Gamma.logGamma(z + 1.0) - (z + 0.5) * FastMath.log(z) +
                      // Java: z - HALF_LOG_2_PI;
            }
        }
        // Java: else
        {
            // Java: double z2 = z * z;
            // Java: ret = (0.083333333333333333333 -
                    // Java: (0.00277777777777777777778 -
                            // Java: (0.00079365079365079365079365 -
                                    // Java: (0.000595238095238095238095238 -
                                            // Java: 0.0008417508417508417508417508 /
                                            // Java: z2) / z2) / z2) / z2) / z;
        }
        // Java: return ret;
        todo!();
    }

    // Java: static double getDeviancePart(double x, double mu)
    pub fn getDeviancePart(&self, x: f64, mu: f64) -> f64 {
        // Java: double ret;
        // Java: if (FastMath.abs(x - mu) < 0.1 * (x + mu))
        {
            // Java: double d = x - mu;
            // Java: double v = d / (x + mu);
            // Java: double s1 = v * d;
            // Java: double s = Double.NaN;
            // Java: double ej = 2.0 * x * v;
            // Java: v *= v;
            // Java: int j = 1;
            // Java: while (s1 != s)
            {
                // Java: s = s1;
                // Java: ej *= v;
                // Java: s1 = s + ej / ((j * 2) + 1);
                // Java: ++j;
            }
            // Java: ret = s1;
        }
        // Java: else
        {
            // Java: ret = x * FastMath.log(x / mu) + mu - x;
        }
        // Java: return ret;
        todo!();
    }

    // Java: static double logBinomialProbability(int x, int n, double p, double q)
    pub fn logBinomialProbability(&self, x: i32, n: i32, p: f64, q: f64) -> f64 {
        // Java: double ret;
        // Java: if (x == 0)
        {
            // Java: if (p < 0.1)
            {
                // Java: ret = -getDeviancePart(n, n * q) - n * p;
            }
            // Java: else
            {
                // Java: ret = n * FastMath.log(q);
            }
        }
        // Java: else if (x == n)
        {
            // Java: if (q < 0.1)
            {
                // Java: ret = -getDeviancePart(n, n * p) - n * q;
            }
            // Java: else
            {
                // Java: ret = n * FastMath.log(p);
            }
        }
        // Java: else
        {
            // Java: ret = getStirlingError(n) - getStirlingError(x) -
                  // Java: getStirlingError(n - x) - getDeviancePart(x, n * p) -
                  // Java: getDeviancePart(n - x, n * q);
            // Java: double f = (MathUtils.TWO_PI * x * (n - x)) / n;
            // Java: ret = -0.5 * FastMath.log(f) + ret;
        }
        // Java: return ret;
        todo!();
    }

}
