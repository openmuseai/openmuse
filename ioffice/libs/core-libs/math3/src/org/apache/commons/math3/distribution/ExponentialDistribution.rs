// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.util.CombinatoricsUtils
use crate::org::apache::commons::math3::util::CombinatoricsUtils;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.ResizableDoubleArray
use crate::org::apache::commons::math3::util::ResizableDoubleArray;

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;
// Java field: double[] EXPONENTIAL_SA_QI
static EXPONENTIAL_SA_QI: std::sync::OnceLock<Vec<f64>> = std::sync::OnceLock::new();

// Java: public class ExponentialDistribution extends AbstractRealDistribution
pub struct ExponentialDistribution {
    // Java field: double mean
    pub(crate) mean: f64,
    // Java field: double logMean
    logMean: f64,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
}

// Java: public class ExponentialDistribution extends AbstractRealDistribution
impl ExponentialDistribution {
    // Java: ExponentialDistribution(double mean)
    pub fn ExponentialDistribution(&self, mean: f64) {
        // Java: this(mean, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: ExponentialDistribution(double mean, double inverseCumAccuracy)
    pub fn ExponentialDistribution_1(&self, mean: f64, inverseCumAccuracy: f64) {
        // Java: this(new Well19937c(), mean, inverseCumAccuracy);
    }

    // Java: ExponentialDistribution(RandomGenerator rng, double mean)
    pub fn ExponentialDistribution_2(&self, rng: &dyn RandomGenerator, mean: f64) {
    }

    // Java: ExponentialDistribution()
    pub fn ExponentialDistribution_3(&self) {
    }

    // Java: double getMean()
    pub fn getMean(&self) -> f64 {
        // Java: return mean;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: final double logDensity = logDensity(x);
        // Java: return logDensity == Double.NEGATIVE_INFINITY ? 0 : FastMath.exp(logDensity);
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: if (x < 0)
        {
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: return -x / mean - logMean;
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: double ret;
        // Java: if (x <= 0.0)
        {
            // Java: ret = 0.0;
        }
        // Java: else
        {
            // Java: ret = 1.0 - FastMath.exp(-x / mean);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double inverseCumulativeProbability(double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> f64 {
        // Java: double ret;
        // Java: if (p < 0.0 || p > 1.0)
        {
            // Java: throw new OutOfRangeException(p, 0.0, 1.0);
        }
        // Java: else if (p == 1.0)
        {
            // Java: ret = Double.POSITIVE_INFINITY;
        }
        // Java: else
        {
            // Java: ret = -mean * FastMath.log(1.0 - p);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double sample()
    pub fn sample(&self) -> f64 {
        // Java: // Step 1:
        // Java: double a = 0;
        // Java: double u = random.nextDouble();
        // Java: // Step 2 and 3:
        // Java: while (u < 0.5)
        {
            // Java: a += EXPONENTIAL_SA_QI[0];
            // Java: u *= 2;
        }
        // Java: // Step 4 (now u >= 0.5):
        // Java: u += u - 1;
        // Java: // Step 5:
        // Java: if (u <= EXPONENTIAL_SA_QI[0])
        {
            // Java: return mean * (a + u);
        }
        // Java: // Step 6:
        // Java: int i = 0; // Should be 1, be we iterate before it in while using 0
        // Java: double u2 = random.nextDouble();
        // Java: double umin = u2;
        // Java: // Step 7 and 8:
        // Java: do
        {
            // Java: ++i;
            // Java: u2 = random.nextDouble();
            // Java: if (u2 < umin)
            {
                // Java: umin = u2;
            }
            // Java: // Step 8:
        }
        // Java: return mean * (a + umin * EXPONENTIAL_SA_QI[0]);
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
        // Java: final double m = getMean();
        // Java: return m * m;
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return 0;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return Double.POSITIVE_INFINITY;
        todo!();
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: return true;
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

}
