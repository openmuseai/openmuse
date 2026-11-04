// Java: import org.apache.commons.math3.exception.MaxCountExceededException
use crate::org::apache::commons::math3::exception::MaxCountExceededException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.util.ContinuedFraction
use crate::org::apache::commons::math3::util::ContinuedFraction;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double GAMMA
pub(crate) const GAMMA: f64 = 0.0;
// Java field: double LANCZOS_G
pub(crate) const LANCZOS_G: f64 = 0.0;
// Java field: double DEFAULT_EPSILON
pub(crate) const DEFAULT_EPSILON: f64 = 0.0;
// Java field: double HALF_LOG_2_PI
pub(crate) const HALF_LOG_2_PI: f64 = 0.0;
// Java field: double SQRT_TWO_PI
pub(crate) const SQRT_TWO_PI: f64 = 0.0;
// Java field: double C_LIMIT
pub(crate) const C_LIMIT: f64 = 0.0;
// Java field: double S_LIMIT
pub(crate) const S_LIMIT: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_A0
pub(crate) const INV_GAMMA1P_M1_A0: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_A1
pub(crate) const INV_GAMMA1P_M1_A1: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_B1
pub(crate) const INV_GAMMA1P_M1_B1: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_B2
pub(crate) const INV_GAMMA1P_M1_B2: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_B3
pub(crate) const INV_GAMMA1P_M1_B3: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_B4
pub(crate) const INV_GAMMA1P_M1_B4: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_B5
pub(crate) const INV_GAMMA1P_M1_B5: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_B6
pub(crate) const INV_GAMMA1P_M1_B6: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_B7
pub(crate) const INV_GAMMA1P_M1_B7: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_B8
pub(crate) const INV_GAMMA1P_M1_B8: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_P0
pub(crate) const INV_GAMMA1P_M1_P0: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_P1
pub(crate) const INV_GAMMA1P_M1_P1: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_P2
pub(crate) const INV_GAMMA1P_M1_P2: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_P3
pub(crate) const INV_GAMMA1P_M1_P3: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_P4
pub(crate) const INV_GAMMA1P_M1_P4: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_P5
pub(crate) const INV_GAMMA1P_M1_P5: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_P6
pub(crate) const INV_GAMMA1P_M1_P6: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_Q1
pub(crate) const INV_GAMMA1P_M1_Q1: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_Q2
pub(crate) const INV_GAMMA1P_M1_Q2: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_Q3
pub(crate) const INV_GAMMA1P_M1_Q3: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_Q4
pub(crate) const INV_GAMMA1P_M1_Q4: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C
pub(crate) const INV_GAMMA1P_M1_C: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C0
pub(crate) const INV_GAMMA1P_M1_C0: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C1
pub(crate) const INV_GAMMA1P_M1_C1: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C2
pub(crate) const INV_GAMMA1P_M1_C2: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C3
pub(crate) const INV_GAMMA1P_M1_C3: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C4
pub(crate) const INV_GAMMA1P_M1_C4: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C5
pub(crate) const INV_GAMMA1P_M1_C5: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C6
pub(crate) const INV_GAMMA1P_M1_C6: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C7
pub(crate) const INV_GAMMA1P_M1_C7: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C8
pub(crate) const INV_GAMMA1P_M1_C8: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C9
pub(crate) const INV_GAMMA1P_M1_C9: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C10
pub(crate) const INV_GAMMA1P_M1_C10: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C11
pub(crate) const INV_GAMMA1P_M1_C11: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C12
pub(crate) const INV_GAMMA1P_M1_C12: f64 = 0.0;
// Java field: double INV_GAMMA1P_M1_C13
pub(crate) const INV_GAMMA1P_M1_C13: f64 = 0.0;

// Java: public class that
pub struct that {
}

// Java: public class Gamma
pub struct Gamma {
}

// Java: public class that
impl that {
}

// Java: public class Gamma
impl Gamma {
    // Java: Gamma()
    pub fn Gamma(&self) {
    }

    // Java: static double logGamma(double x)
    pub fn logGamma(&self, x: f64) -> f64 {
        // Java: double ret;
        // Java: if (Double.isNaN(x) || (x <= 0.0))
        {
            // Java: ret = Double.NaN;
        }
        // Java: else if (x < 0.5)
        {
            // Java: return logGamma1p(x) - FastMath.log(x);
        }
        // Java: else if (x <= 2.5)
        {
            // Java: return logGamma1p((x - 0.5) - 0.5);
        }
        // Java: else if (x <= 8.0)
        {
            // Java: final int n = (int) FastMath.floor(x - 1.5);
            // Java: double prod = 1.0;
            // Java: for (int i = 1; i <= n; i++)
            {
                // Java: prod *= x - i;
            }
            // Java: return logGamma1p(x - (n + 1)) + FastMath.log(prod);
        }
        // Java: else
        {
            // Java: double sum = lanczos(x);
            // Java: double tmp = x + LANCZOS_G + .5;
            // Java: ret = ((x + .5) * FastMath.log(tmp)) - tmp +
                // Java: HALF_LOG_2_PI + FastMath.log(sum / x);
        }
        // Java: return ret;
        todo!();
    }

    // Java: static double regularizedGammaP(double a, double x)
    pub fn regularizedGammaP(&self, a: f64, x: f64) -> f64 {
        // Java: return regularizedGammaP(a, x, DEFAULT_EPSILON, Integer.MAX_VALUE);
        todo!();
    }

    // Java: static double regularizedGammaP()
    pub fn regularizedGammaP_1(&self) -> f64 {
        todo!();
    }

    // Java: static double regularizedGammaQ(double a, double x)
    pub fn regularizedGammaQ(&self, a: f64, x: f64) -> f64 {
        // Java: return regularizedGammaQ(a, x, DEFAULT_EPSILON, Integer.MAX_VALUE);
        todo!();
    }

    // Java: static double regularizedGammaQ()
    pub fn regularizedGammaQ_1(&self) -> f64 {
        todo!();
    }

    // Java: static double digamma(double x)
    pub fn digamma(&self, x: f64) -> f64 {
        // Java: if (Double.isNaN(x) || Double.isInfinite(x))
        {
            // Java: return x;
        }
        // Java: if (x > 0 && x <= S_LIMIT)
        {
            // Java: // use method 5 from Bernardo AS103
            // Java: // accurate to O(x)
            // Java: return -GAMMA - 1 / x;
        }
        // Java: if (x >= C_LIMIT)
        {
            // Java: // use method 4 (accurate to O(1/x^8)
            // Java: double inv = 1 / (x * x);
            // Java: //            1       1        1         1
            // Java: // log(x) -  --- - ------ + ------- - -------
            // Java: //           2 x   12 x^2   120 x^4   252 x^6
            // Java: return FastMath.log(x) - 0.5 / x - inv * ((1.0 / 12) + inv * (1.0 / 120 - inv / 252));
        }
        // Java: return digamma(x + 1) - 1 / x;
        todo!();
    }

    // Java: static double trigamma(double x)
    pub fn trigamma(&self, x: f64) -> f64 {
        // Java: if (Double.isNaN(x) || Double.isInfinite(x))
        {
            // Java: return x;
        }
        // Java: if (x > 0 && x <= S_LIMIT)
        {
            // Java: return 1 / (x * x);
        }
        // Java: if (x >= C_LIMIT)
        {
            // Java: double inv = 1 / (x * x);
            // Java: //  1    1      1       1       1
            // Java: //  - + ---- + ---- - ----- + -----
            // Java: //  x      2      3       5       7
            // Java: //      2 x    6 x    30 x    42 x
            // Java: return 1 / x + inv / 2 + inv / x * (1.0 / 6 - inv * (1.0 / 30 + inv / 42));
        }
        // Java: return trigamma(x + 1) + 1 / (x * x);
        todo!();
    }

    // Java: static double lanczos(final double x)
    pub fn lanczos(&self, x: f64) -> f64 {
        // Java: double sum = 0.0;
        // Java: for (int i = LANCZOS.length - 1; i > 0; --i)
        {
            // Java: sum += LANCZOS[i] / (x + i);
        }
        // Java: return sum + LANCZOS[0];
        todo!();
    }

    // Java: static double invGamma1pm1(final double x)
    pub fn invGamma1pm1(&self, x: f64) -> f64 {
        // Java: if (x < -0.5)
        {
            // Java: throw new NumberIsTooSmallException(x, -0.5, true);
        }
        // Java: if (x > 1.5)
        {
            // Java: throw new NumberIsTooLargeException(x, 1.5, true);
        }
        // Java: final double ret;
        // Java: final double t = x <= 0.5 ? x : (x - 0.5) - 0.5;
        // Java: if (t < 0.0)
        {
            // Java: final double a = INV_GAMMA1P_M1_A0 + t * INV_GAMMA1P_M1_A1;
            // Java: double b = INV_GAMMA1P_M1_B8;
            // Java: b = INV_GAMMA1P_M1_B7 + t * b;
            // Java: b = INV_GAMMA1P_M1_B6 + t * b;
            // Java: b = INV_GAMMA1P_M1_B5 + t * b;
            // Java: b = INV_GAMMA1P_M1_B4 + t * b;
            // Java: b = INV_GAMMA1P_M1_B3 + t * b;
            // Java: b = INV_GAMMA1P_M1_B2 + t * b;
            // Java: b = INV_GAMMA1P_M1_B1 + t * b;
            // Java: b = 1.0 + t * b;
            // Java: double c = INV_GAMMA1P_M1_C13 + t * (a / b);
            // Java: c = INV_GAMMA1P_M1_C12 + t * c;
            // Java: c = INV_GAMMA1P_M1_C11 + t * c;
            // Java: c = INV_GAMMA1P_M1_C10 + t * c;
            // Java: c = INV_GAMMA1P_M1_C9 + t * c;
            // Java: c = INV_GAMMA1P_M1_C8 + t * c;
            // Java: c = INV_GAMMA1P_M1_C7 + t * c;
            // Java: c = INV_GAMMA1P_M1_C6 + t * c;
            // Java: c = INV_GAMMA1P_M1_C5 + t * c;
            // Java: c = INV_GAMMA1P_M1_C4 + t * c;
            // Java: c = INV_GAMMA1P_M1_C3 + t * c;
            // Java: c = INV_GAMMA1P_M1_C2 + t * c;
            // Java: c = INV_GAMMA1P_M1_C1 + t * c;
            // Java: c = INV_GAMMA1P_M1_C + t * c;
            // Java: if (x > 0.5)
            {
                // Java: ret = t * c / x;
            }
            // Java: else
            {
                // Java: ret = x * ((c + 0.5) + 0.5);
            }
        }
        // Java: else
        {
            // Java: double p = INV_GAMMA1P_M1_P6;
            // Java: p = INV_GAMMA1P_M1_P5 + t * p;
            // Java: p = INV_GAMMA1P_M1_P4 + t * p;
            // Java: p = INV_GAMMA1P_M1_P3 + t * p;
            // Java: p = INV_GAMMA1P_M1_P2 + t * p;
            // Java: p = INV_GAMMA1P_M1_P1 + t * p;
            // Java: p = INV_GAMMA1P_M1_P0 + t * p;
            // Java: double q = INV_GAMMA1P_M1_Q4;
            // Java: q = INV_GAMMA1P_M1_Q3 + t * q;
            // Java: q = INV_GAMMA1P_M1_Q2 + t * q;
            // Java: q = INV_GAMMA1P_M1_Q1 + t * q;
            // Java: q = 1.0 + t * q;
            // Java: double c = INV_GAMMA1P_M1_C13 + (p / q) * t;
            // Java: c = INV_GAMMA1P_M1_C12 + t * c;
            // Java: c = INV_GAMMA1P_M1_C11 + t * c;
            // Java: c = INV_GAMMA1P_M1_C10 + t * c;
            // Java: c = INV_GAMMA1P_M1_C9 + t * c;
            // Java: c = INV_GAMMA1P_M1_C8 + t * c;
            // Java: c = INV_GAMMA1P_M1_C7 + t * c;
            // Java: c = INV_GAMMA1P_M1_C6 + t * c;
            // Java: c = INV_GAMMA1P_M1_C5 + t * c;
            // Java: c = INV_GAMMA1P_M1_C4 + t * c;
            // Java: c = INV_GAMMA1P_M1_C3 + t * c;
            // Java: c = INV_GAMMA1P_M1_C2 + t * c;
            // Java: c = INV_GAMMA1P_M1_C1 + t * c;
            // Java: c = INV_GAMMA1P_M1_C0 + t * c;
            // Java: if (x > 0.5)
            {
                // Java: ret = (t / x) * ((c - 0.5) - 0.5);
            }
            // Java: else
            {
                // Java: ret = x * c;
            }
        }
        // Java: return ret;
        todo!();
    }

    // Java: static double logGamma1p(final double x)
    pub fn logGamma1p(&self, x: f64) -> f64 {
        todo!();
    }

    // Java: static double gamma(final double x)
    pub fn gamma(&self, x: f64) -> f64 {
        // Java: if ((x == FastMath.rint(x)) && (x <= 0.0))
        {
            // Java: return Double.NaN;
        }
        // Java: final double ret;
        // Java: final double absX = FastMath.abs(x);
        // Java: if (absX <= 20.0)
        {
            // Java: if (x >= 1.0)
            {
                // Java: /*
                 // Java: * From the recurrence relation
                 // Java: * Gamma(x) = (x - 1) * ... * (x - n) * Gamma(x - n),
                 // Java: * then
                 // Java: * Gamma(t) = 1 / [1 + invGamma1pm1(t - 1)],
                 // Java: * where t = x - n. This means that t must satisfy
                 // Java: * -0.5 <= t - 1 <= 1.5.
                 // Java: */
                // Java: double prod = 1.0;
                // Java: double t = x;
                // Java: while (t > 2.5)
                {
                    // Java: t -= 1.0;
                    // Java: prod *= t;
                }
                // Java: ret = prod / (1.0 + invGamma1pm1(t - 1.0));
            }
            // Java: else
            {
                // Java: /*
                 // Java: * From the recurrence relation
                 // Java: * Gamma(x) = Gamma(x + n + 1) / [x * (x + 1) * ... * (x + n)]
                 // Java: * then
                 // Java: * Gamma(x + n + 1) = 1 / [1 + invGamma1pm1(x + n)],
                 // Java: * which requires -0.5 <= x + n <= 1.5.
                 // Java: */
                // Java: double prod = x;
                // Java: double t = x;
                // Java: while (t < -0.5)
                {
                    // Java: t += 1.0;
                    // Java: prod *= t;
                }
                // Java: ret = 1.0 / (prod * (1.0 + invGamma1pm1(t)));
            }
        }
        // Java: else
        {
            // Java: final double y = absX + LANCZOS_G + 0.5;
            // Java: final double gammaAbs = SQRT_TWO_PI / absX *
                                    // Java: FastMath.pow(y, absX + 0.5) *
                                    // Java: FastMath.exp(-y) * lanczos(absX);
            // Java: if (x > 0.0)
            {
                // Java: ret = gammaAbs;
            }
            // Java: else
            {
                // Java: /*
                 // Java: * From the reflection formula
                 // Java: * Gamma(x) * Gamma(1 - x) * sin(pi * x) = pi,
                 // Java: * and the recurrence relation
                 // Java: * Gamma(1 - x) = -x * Gamma(-x),
                 // Java: * it is found
                 // Java: * Gamma(x) = -pi / [x * sin(pi * x) * Gamma(-x)].
                 // Java: */
                // Java: ret = -FastMath.PI /
                      // Java: (x * FastMath.sin(FastMath.PI * x) * gammaAbs);
            }
        }
        // Java: return ret;
        todo!();
    }

}
