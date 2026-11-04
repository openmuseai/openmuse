// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.special.Erf
use crate::org::apache::commons::math3::special::Erf;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;
// Java field: double SQRT2
pub(crate) const SQRT2: f64 = 0.0;

// Java: public class NormalDistribution extends AbstractRealDistribution
pub struct NormalDistribution {
    // Java field: double mean
    pub(crate) mean: f64,
    // Java field: double standardDeviation
    pub(crate) standardDeviation: f64,
    // Java field: double logStandardDeviationPlusHalfLog2Pi
    logStandardDeviationPlusHalfLog2Pi: f64,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
}

// Java: public class NormalDistribution extends AbstractRealDistribution
impl NormalDistribution {
    // Java: NormalDistribution()
    pub fn NormalDistribution(&self) {
        // Java: this(0, 1);
    }

    // Java: NormalDistribution(double mean, double sd)
    pub fn NormalDistribution_1(&self, mean: f64, sd: f64) {
    }

    // Java: NormalDistribution(double mean, double sd, double inverseCumAccuracy)
    pub fn NormalDistribution_2(&self, mean: f64, sd: f64, inverseCumAccuracy: f64) {
    }

    // Java: NormalDistribution(RandomGenerator rng, double mean, double sd)
    pub fn NormalDistribution_3(&self, rng: &dyn RandomGenerator, mean: f64, sd: f64) {
    }

    // Java: NormalDistribution()
    pub fn NormalDistribution_4(&self) {
    }

    // Java: double getMean()
    pub fn getMean(&self) -> f64 {
        // Java: return mean;
        todo!();
    }

    // Java: double getStandardDeviation()
    pub fn getStandardDeviation(&self) -> f64 {
        // Java: return standardDeviation;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: return FastMath.exp(logDensity(x));
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: final double x0 = x - mean;
        // Java: final double x1 = x0 / standardDeviation;
        // Java: return -0.5 * x1 * x1 - logStandardDeviationPlusHalfLog2Pi;
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: final double dev = x - mean;
        // Java: if (FastMath.abs(dev) > 40 * standardDeviation)
        {
            // Java: return dev < 0 ? 0.0d : 1.0d;
        }
        // Java: return 0.5 * Erf.erfc(-dev / (standardDeviation * SQRT2));
        todo!();
    }

    // Java: double inverseCumulativeProbability(final double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> f64 {
        // Java: if (p < 0.0 || p > 1.0)
        {
            // Java: throw new OutOfRangeException(p, 0, 1);
        }
        // Java: return mean + standardDeviation * SQRT2 * Erf.erfInv(2 * p - 1);
        todo!();
    }

    // Java: double cumulativeProbability(double x0, double x1)
    pub fn cumulativeProbability_1(&self, x0: f64, x1: f64) -> f64 {
        todo!();
    }

    // Java: double probability()
    pub fn probability(&self) -> f64 {
        todo!();
    }

    // Java: double getSolverAbsoluteAccuracy()
    pub fn getSolverAbsoluteAccuracy(&self) -> f64 {
        // Java: return solverAbsoluteAccuracy;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return getMean();
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: final double s = getStandardDeviation();
        // Java: return s * s;
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return Double.NEGATIVE_INFINITY;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return Double.POSITIVE_INFINITY;
        todo!();
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: return false;
        todo!();
    }

    // Java: boolean isSupportUpperBoundInclusive()
    pub fn isSupportUpperBoundInclusive(&self) -> bool {
        // Java: return false;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: double sample()
    pub fn sample(&self) -> f64 {
        // Java: return standardDeviation * random.nextGaussian() + mean;
        todo!();
    }

}
