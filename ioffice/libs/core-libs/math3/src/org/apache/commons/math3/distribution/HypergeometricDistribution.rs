// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
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

// Java: public class HypergeometricDistribution extends AbstractIntegerDistribution
pub struct HypergeometricDistribution {
    // Java field: int numberOfSuccesses
    pub(crate) numberOfSuccesses: i32,
    // Java field: int populationSize
    pub(crate) populationSize: i32,
    // Java field: int sampleSize
    pub(crate) sampleSize: i32,
    // Java field: double numericalVariance
    pub(crate) numericalVariance: f64,
    // Java field: boolean numericalVarianceIsCalculated
    numericalVarianceIsCalculated: bool,
}

// Java: public class HypergeometricDistribution extends AbstractIntegerDistribution
impl HypergeometricDistribution {
    // Java: HypergeometricDistribution(int populationSize, int numberOfSuccesses, int sampleSize)
    pub fn HypergeometricDistribution(&self, populationSize: i32, numberOfSuccesses: i32, sampleSize: i32) {
    }

    // Java: HypergeometricDistribution()
    pub fn HypergeometricDistribution_1(&self) {
    }

    // Java: double cumulativeProbability(int x)
    pub fn cumulativeProbability(&self, x: i32) -> f64 {
        // Java: double ret;
        // Java: int[] domain = getDomain(populationSize, numberOfSuccesses, sampleSize);
        // Java: if (x < domain[0])
        {
            // Java: ret = 0.0;
        }
        // Java: else if (x >= domain[1])
        {
            // Java: ret = 1.0;
        }
        // Java: else
        {
            // Java: ret = innerCumulativeProbability(domain[0], x, 1);
        }
        // Java: return ret;
        todo!();
    }

    // Java: int[] getDomain(int n, int m, int k)
    pub fn getDomain(&self, n: i32, m: i32, k: i32) -> Vec<i32> {
        // Java: return new int[] { getLowerDomain(n, m, k), getUpperDomain(m, k) };
        todo!();
    }

    // Java: int getLowerDomain(int n, int m, int k)
    pub fn getLowerDomain(&self, n: i32, m: i32, k: i32) -> i32 {
        // Java: return FastMath.max(0, m - (n - k));
        todo!();
    }

    // Java: int getNumberOfSuccesses()
    pub fn getNumberOfSuccesses(&self) -> i32 {
        // Java: return numberOfSuccesses;
        todo!();
    }

    // Java: int getPopulationSize()
    pub fn getPopulationSize(&self) -> i32 {
        // Java: return populationSize;
        todo!();
    }

    // Java: int getSampleSize()
    pub fn getSampleSize(&self) -> i32 {
        // Java: return sampleSize;
        todo!();
    }

    // Java: int getUpperDomain(int m, int k)
    pub fn getUpperDomain(&self, m: i32, k: i32) -> i32 {
        // Java: return FastMath.min(k, m);
        todo!();
    }

    // Java: double probability(int x)
    pub fn probability(&self, x: i32) -> f64 {
        // Java: final double logProbability = logProbability(x);
        // Java: return logProbability == Double.NEGATIVE_INFINITY ? 0 : FastMath.exp(logProbability);
        todo!();
    }

    // Java: double logProbability(int x)
    pub fn logProbability(&self, x: i32) -> f64 {
        // Java: double ret;
        // Java: int[] domain = getDomain(populationSize, numberOfSuccesses, sampleSize);
        // Java: if (x < domain[0] || x > domain[1])
        {
            // Java: ret = Double.NEGATIVE_INFINITY;
        }
        // Java: else
        {
            // Java: double p = (double) sampleSize / (double) populationSize;
            // Java: double q = (double) (populationSize - sampleSize) / (double) populationSize;
            // Java: double p1 = SaddlePointExpansion.logBinomialProbability(x,
                    // Java: numberOfSuccesses, p, q);
            // Java: double p2 =
                    // Java: SaddlePointExpansion.logBinomialProbability(sampleSize - x,
                            // Java: populationSize - numberOfSuccesses, p, q);
            // Java: double p3 =
                    // Java: SaddlePointExpansion.logBinomialProbability(sampleSize, populationSize, p, q);
            // Java: ret = p1 + p2 - p3;
        }
        // Java: return ret;
        todo!();
    }

    // Java: double upperCumulativeProbability(int x)
    pub fn upperCumulativeProbability(&self, x: i32) -> f64 {
        // Java: double ret;
        // Java: final int[] domain = getDomain(populationSize, numberOfSuccesses, sampleSize);
        // Java: if (x <= domain[0])
        {
            // Java: ret = 1.0;
        }
        // Java: else if (x > domain[1])
        {
            // Java: ret = 0.0;
        }
        // Java: else
        {
            // Java: ret = innerCumulativeProbability(domain[1], x, -1);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double innerCumulativeProbability(int x0, int x1, int dx)
    pub fn innerCumulativeProbability(&self, x0: i32, x1: i32, dx: i32) -> f64 {
        // Java: double ret = probability(x0);
        // Java: while (x0 != x1)
        {
            // Java: x0 += dx;
            // Java: ret += probability(x0);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return getSampleSize() * (getNumberOfSuccesses() / (double) getPopulationSize());
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
        // Java: final double N = getPopulationSize();
        // Java: final double m = getNumberOfSuccesses();
        // Java: final double n = getSampleSize();
        // Java: return (n * m * (N - n) * (N - m)) / (N * N * (N - 1));
        todo!();
    }

    // Java: int getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> i32 {
        // Java: return FastMath.max(0,
                            // Java: getSampleSize() + getNumberOfSuccesses() - getPopulationSize());
        todo!();
    }

    // Java: int getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> i32 {
        // Java: return FastMath.min(getNumberOfSuccesses(), getSampleSize());
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

}
