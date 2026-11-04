// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.util.ContinuedFraction
use crate::org::apache::commons::math3::util::ContinuedFraction;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_EPSILON
pub(crate) const DEFAULT_EPSILON: f64 = 1E-14;
// Java field: double HALF_LOG_TWO_PI
pub(crate) const HALF_LOG_TWO_PI: f64 = 0.9189385332046727;

// Java: public class that
pub struct that {
}

// Java: public class Beta
pub struct Beta {
}

// Java: public class that
impl that {
}

// Java: public class Beta
impl Beta {
    // Java: Beta()
    pub fn Beta(&self) {
    }

    // Java: static double regularizedBeta(double x, double a, double b)
    pub fn regularizedBeta(&self, x: f64, a: f64, b: f64) -> f64 {
        // Java: return regularizedBeta(x, a, b, DEFAULT_EPSILON, Integer.MAX_VALUE);
        todo!();
    }

    // Java: static double regularizedBeta()
    pub fn regularizedBeta_1(&self) -> f64 {
        todo!();
    }

    // Java: static double regularizedBeta()
    pub fn regularizedBeta_2(&self) -> f64 {
        todo!();
    }

    // Java: static double regularizedBeta()
    pub fn regularizedBeta_3(&self) -> f64 {
        todo!();
    }

    // Java: static double logBeta()
    pub fn logBeta(&self) -> f64 {
        todo!();
    }

    // Java: static double logGammaSum(final double a, final double b)
    pub fn logGammaSum(&self, a: f64, b: f64) -> f64 {
        todo!();
    }

    // Java: static double logGammaMinusLogGammaSum()
    pub fn logGammaMinusLogGammaSum(&self) -> f64 {
        todo!();
    }

    // Java: static double deltaMinusDeltaSum()
    pub fn deltaMinusDeltaSum(&self) -> f64 {
        todo!();
    }

    // Java: static double sumDeltaMinusDeltaSum()
    pub fn sumDeltaMinusDeltaSum(&self) -> f64 {
        todo!();
    }

    // Java: static double logBeta(final double p, final double q)
    pub fn logBeta_1(&self, p: f64, q: f64) -> f64 {
        // Java: if (Double.isNaN(p) || Double.isNaN(q) || (p <= 0.0) || (q <= 0.0))
        {
            // Java: return Double.NaN;
        }
        // Java: final double a = FastMath.min(p, q);
        // Java: final double b = FastMath.max(p, q);
        // Java: if (a >= 10.0)
        {
            // Java: final double w = sumDeltaMinusDeltaSum(a, b);
            // Java: final double h = a / b;
            // Java: final double c = h / (1.0 + h);
            // Java: final double u = -(a - 0.5) * FastMath.log(c);
            // Java: final double v = b * FastMath.log1p(h);
            // Java: if (u <= v)
            {
                // Java: return (((-0.5 * FastMath.log(b) + HALF_LOG_TWO_PI) + w) - u) - v;
            }
            // Java: else
            {
                // Java: return (((-0.5 * FastMath.log(b) + HALF_LOG_TWO_PI) + w) - v) - u;
            }
        }
        // Java: else if (a > 2.0)
        {
            // Java: if (b > 1000.0)
            {
                // Java: final int n = (int) FastMath.floor(a - 1.0);
                // Java: double prod = 1.0;
                // Java: double ared = a;
                // Java: for (int i = 0; i < n; i++)
                {
                    // Java: ared -= 1.0;
                    // Java: prod *= ared / (1.0 + ared / b);
                }
                // Java: return (FastMath.log(prod) - n * FastMath.log(b)) +
                        // Java: (Gamma.logGamma(ared) +
                         // Java: logGammaMinusLogGammaSum(ared, b));
            }
            // Java: else
            {
                // Java: double prod1 = 1.0;
                // Java: double ared = a;
                // Java: while (ared > 2.0)
                {
                    // Java: ared -= 1.0;
                    // Java: final double h = ared / b;
                    // Java: prod1 *= h / (1.0 + h);
                }
                // Java: if (b < 10.0)
                {
                    // Java: double prod2 = 1.0;
                    // Java: double bred = b;
                    // Java: while (bred > 2.0)
                    {
                        // Java: bred -= 1.0;
                        // Java: prod2 *= bred / (ared + bred);
                    }
                    // Java: return FastMath.log(prod1) +
                           // Java: FastMath.log(prod2) +
                           // Java: (Gamma.logGamma(ared) +
                           // Java: (Gamma.logGamma(bred) -
                            // Java: logGammaSum(ared, bred)));
                }
                // Java: else
                {
                    // Java: return FastMath.log(prod1) +
                           // Java: Gamma.logGamma(ared) +
                           // Java: logGammaMinusLogGammaSum(ared, b);
                }
            }
        }
        // Java: else if (a >= 1.0)
        {
            // Java: if (b > 2.0)
            {
                // Java: if (b < 10.0)
                {
                    // Java: double prod = 1.0;
                    // Java: double bred = b;
                    // Java: while (bred > 2.0)
                    {
                        // Java: bred -= 1.0;
                        // Java: prod *= bred / (a + bred);
                    }
                    // Java: return FastMath.log(prod) +
                           // Java: (Gamma.logGamma(a) +
                            // Java: (Gamma.logGamma(bred) -
                             // Java: logGammaSum(a, bred)));
                }
                // Java: else
                {
                    // Java: return Gamma.logGamma(a) +
                           // Java: logGammaMinusLogGammaSum(a, b);
                }
            }
            // Java: else
            {
                // Java: return Gamma.logGamma(a) +
                       // Java: Gamma.logGamma(b) -
                       // Java: logGammaSum(a, b);
            }
        }
        // Java: else
        {
            // Java: if (b >= 10.0)
            {
                // Java: return Gamma.logGamma(a) +
                       // Java: logGammaMinusLogGammaSum(a, b);
            }
            // Java: else
            {
                // Java: // The following command is the original NSWC implementation.
                // Java: // return Gamma.logGamma(a) +
                // Java: // (Gamma.logGamma(b) - Gamma.logGamma(a + b));
                // Java: // The following command turns out to be more accurate.
                // Java: return FastMath.log(Gamma.gamma(a) * Gamma.gamma(b) /
                                    // Java: Gamma.gamma(a + b));
            }
        }
        todo!();
    }

}
