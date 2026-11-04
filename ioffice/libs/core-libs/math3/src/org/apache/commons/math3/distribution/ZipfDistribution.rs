// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class ZipfDistribution extends AbstractIntegerDistribution
pub struct ZipfDistribution {
    // Java field: int numberOfElements
    pub(crate) numberOfElements: i32,
    // Java field: double exponent
    pub(crate) exponent: f64,
    // Java field: double numericalMean
    pub(crate) numericalMean: f64,
    // Java field: boolean numericalMeanIsCalculated
    numericalMeanIsCalculated: bool,
    // Java field: double numericalVariance
    pub(crate) numericalVariance: f64,
    // Java field: boolean numericalVarianceIsCalculated
    numericalVarianceIsCalculated: bool,
    // Java field: ZipfRejectionInversionSampler sampler
    pub(crate) sampler: Box<dyn std::any::Any>,
}

// Java: public class ZipfRejectionInversionSampler
pub struct ZipfRejectionInversionSampler {
    // Java field: double exponent
    pub(crate) exponent: f64,
    // Java field: int numberOfElements
    pub(crate) numberOfElements: i32,
    // Java field: double hIntegralX1
    pub(crate) hIntegralX1: f64,
    // Java field: double hIntegralNumberOfElements
    pub(crate) hIntegralNumberOfElements: f64,
    // Java field: double s
    pub(crate) s: f64,
}

// Java: public class ZipfDistribution extends AbstractIntegerDistribution
impl ZipfDistribution {
    // Java: ZipfDistribution(final int numberOfElements, final double exponent)
    pub fn ZipfDistribution(&self, numberOfElements: i32, exponent: f64) {
        // Java: this(new Well19937c(), numberOfElements, exponent);
    }

    // Java: ZipfDistribution()
    pub fn ZipfDistribution_1(&self) {
    }

    // Java: int getNumberOfElements()
    pub fn getNumberOfElements(&self) -> i32 {
        // Java: return numberOfElements;
        todo!();
    }

    // Java: double getExponent()
    pub fn getExponent(&self) -> f64 {
        // Java: return exponent;
        todo!();
    }

    // Java: double probability(final int x)
    pub fn probability(&self, x: i32) -> f64 {
        // Java: if (x <= 0 || x > numberOfElements)
        {
            // Java: return 0.0;
        }
        // Java: return (1.0 / FastMath.pow(x, exponent)) / generalizedHarmonic(numberOfElements, exponent);
        todo!();
    }

    // Java: double logProbability(int x)
    pub fn logProbability(&self, x: i32) -> f64 {
        // Java: if (x <= 0 || x > numberOfElements)
        {
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: return -FastMath.log(x) * exponent - FastMath.log(generalizedHarmonic(numberOfElements, exponent));
        todo!();
    }

    // Java: double cumulativeProbability(final int x)
    pub fn cumulativeProbability(&self, x: i32) -> f64 {
        // Java: if (x <= 0)
        {
            // Java: return 0.0;
        }
        // Java: else if (x >= numberOfElements)
        {
            // Java: return 1.0;
        }
        // Java: return generalizedHarmonic(x, exponent) / generalizedHarmonic(numberOfElements, exponent);
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: if (!numericalMeanIsCalculated)
        {
            // Java: numericalMean = calculateNumericalMean();
            // Java: numericalMeanIsCalculated = true;
        }
        // Java: return numericalMean;
        todo!();
    }

    // Java: double calculateNumericalMean()
    pub fn calculateNumericalMean(&self) -> f64 {
        // Java: final int N = getNumberOfElements();
        // Java: final double s = getExponent();
        // Java: final double Hs1 = generalizedHarmonic(N, s - 1);
        // Java: final double Hs = generalizedHarmonic(N, s);
        // Java: return Hs1 / Hs;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: if (!numericalVarianceIsCalculated)
        {
            // Java: numericalVariance = calculateNumericalVariance();
            // Java: numericalVarianceIsCalculated = true;
        }
        // Java: return numericalVariance;
        todo!();
    }

    // Java: double calculateNumericalVariance()
    pub fn calculateNumericalVariance(&self) -> f64 {
        // Java: final int N = getNumberOfElements();
        // Java: final double s = getExponent();
        // Java: final double Hs2 = generalizedHarmonic(N, s - 2);
        // Java: final double Hs1 = generalizedHarmonic(N, s - 1);
        // Java: final double Hs = generalizedHarmonic(N, s);
        // Java: return (Hs2 / Hs) - ((Hs1 * Hs1) / (Hs * Hs));
        todo!();
    }

    // Java: double generalizedHarmonic(final int n, final double m)
    pub fn generalizedHarmonic(&self, n: i32, m: f64) -> f64 {
        // Java: double value = 0;
        // Java: for (int k = n; k > 0; --k)
        {
            // Java: value += 1.0 / FastMath.pow(k, m);
        }
        // Java: return value;
        todo!();
    }

    // Java: int getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> i32 {
        // Java: return 1;
        todo!();
    }

    // Java: int getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> i32 {
        // Java: return getNumberOfElements();
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: int sample()
    pub fn sample(&self) -> i32 {
        // Java: if (sampler == null)
        {
            // Java: sampler = new ZipfRejectionInversionSampler(numberOfElements, exponent);
        }
        // Java: return sampler.sample(random);
        todo!();
    }

}

// Java: public class ZipfRejectionInversionSampler
impl ZipfRejectionInversionSampler {
    // Java: int sample(final RandomGenerator random)
    pub fn sample_1(&self, random: &dyn RandomGenerator) -> i32 {
            // Java: while(true)
            {
                // Java: final double u = hIntegralNumberOfElements + random.nextDouble() * (hIntegralX1 - hIntegralNumberOfElements);
                // Java: // u is uniformly distributed in (hIntegralX1, hIntegralNumberOfElements]
                // Java: double x = hIntegralInverse(u);
                // Java: int k = (int)(x + 0.5);
                // Java: // Limit k to the range [1, numberOfElements]
                // Java: // (k could be outside due to numerical inaccuracies)
                // Java: if (k < 1)
                {
                    // Java: k = 1;
                }
                // Java: else if (k > numberOfElements)
                {
                    // Java: k = numberOfElements;
                }
                // Java: // Here, the distribution of k is given by:
                // Java: //
                // Java: //   P(k = 1) = C * (hIntegral(1.5) - hIntegralX1) = C
                // Java: //   P(k = m) = C * (hIntegral(m + 1/2) - hIntegral(m - 1/2)) for m >= 2
                // Java: //
                // Java: //   where C := 1 / (hIntegralNumberOfElements - hIntegralX1)
                // Java: if (k - x <= s || u >= hIntegral(k + 0.5) - h(k))
                {
                    // Java: // Case k = 1:
                    // Java: //
                    // Java: //   The right inequality is always true, because replacing k by 1 gives
                    // Java: //   u >= hIntegral(1.5) - h(1) = hIntegralX1 and u is taken from
                    // Java: //   (hIntegralX1, hIntegralNumberOfElements].
                    // Java: //
                    // Java: //   Therefore, the acceptance rate for k = 1 is P(accepted | k = 1) = 1
                    // Java: //   and the probability that 1 is returned as random value is
                    // Java: //   P(k = 1 and accepted) = P(accepted | k = 1) * P(k = 1) = C = C / 1^exponent
                    // Java: //
                    // Java: // Case k >= 2:
                    // Java: //
                    // Java: //   The left inequality (k - x <= s) is just a short cut
                    // Java: //   to avoid the more expensive evaluation of the right inequality
                    // Java: //   (u >= hIntegral(k + 0.5) - h(k)) in many cases.
                    // Java: //
                    // Java: //   If the left inequality is true, the right inequality is also true:
                    // Java: //     Theorem 2 in the paper is valid for all positive exponents, because
                    // Java: //     the requirements h'(x) = -exponent/x^(exponent + 1) < 0 and
                    // Java: //     (-1/hInverse'(x))'' = (1+1/exponent) * x^(1/exponent-1) >= 0
                    // Java: //     are both fulfilled.
                    // Java: //     Therefore, f(x) := x - hIntegralInverse(hIntegral(x + 0.5) - h(x))
                    // Java: //     is a non-decreasing function. If k - x <= s holds,
                    // Java: //     k - x <= s + f(k) - f(2) is obviously also true which is equivalent to
                    // Java: //     -x <= -hIntegralInverse(hIntegral(k + 0.5) - h(k)),
                    // Java: //     -hIntegralInverse(u) <= -hIntegralInverse(hIntegral(k + 0.5) - h(k)),
                    // Java: //     and finally u >= hIntegral(k + 0.5) - h(k).
                    // Java: //
                    // Java: //   Hence, the right inequality determines the acceptance rate:
                    // Java: //   P(accepted | k = m) = h(m) / (hIntegrated(m+1/2) - hIntegrated(m-1/2))
                    // Java: //   The probability that m is returned is given by
                    // Java: //   P(k = m and accepted) = P(accepted | k = m) * P(k = m) = C * h(m) = C / m^exponent.
                    // Java: //
                    // Java: // In both cases the probabilities are proportional to the probability mass function
                    // Java: // of the Zipf distribution.
                    // Java: return k;
                }
            }
        todo!();
    }

    // Java: double hIntegral(final double x)
    pub fn hIntegral(&self, x: f64) -> f64 {
            // Java: final double logX = FastMath.log(x);
            // Java: return helper2((1d-exponent)*logX)*logX;
        todo!();
    }

    // Java: double h(final double x)
    pub fn h(&self, x: f64) -> f64 {
            // Java: return FastMath.exp(-exponent * FastMath.log(x));
        todo!();
    }

    // Java: double hIntegralInverse(final double x)
    pub fn hIntegralInverse(&self, x: f64) -> f64 {
            // Java: double t = x*(1d-exponent);
            // Java: if (t < -1d)
            {
                // Java: // Limit value to the range [-1, +inf).
                // Java: // t could be smaller than -1 in some rare cases due to numerical errors.
                // Java: t = -1;
            }
            // Java: return FastMath.exp(helper1(t)*x);
        todo!();
    }

    // Java: static double helper1(final double x)
    pub fn helper1(&self, x: f64) -> f64 {
            // Java: if (FastMath.abs(x)>1e-8)
            {
                // Java: return FastMath.log1p(x)/x;
            }
            // Java: else
            {
                // Java: return 1.-x*((1./2.)-x*((1./3.)-x*(1./4.)));
            }
        todo!();
    }

    // Java: static double helper2(final double x)
    pub fn helper2(&self, x: f64) -> f64 {
            // Java: if (FastMath.abs(x)>1e-8)
            {
                // Java: return FastMath.expm1(x)/x;
            }
            // Java: else
            {
                // Java: return 1.+x*(1./2.)*(1.+x*(1./3.)*(1.+x*(1./4.)));
            }
        todo!();
    }

}
