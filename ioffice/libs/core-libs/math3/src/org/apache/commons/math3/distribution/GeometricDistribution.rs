// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
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

// Java: public class GeometricDistribution extends AbstractIntegerDistribution
pub struct GeometricDistribution {
    // Java field: double probabilityOfSuccess
    pub(crate) probabilityOfSuccess: f64,
    // Java field: double logProbabilityOfSuccess
    logProbabilityOfSuccess: f64,
    // Java field: double log1mProbabilityOfSuccess
    log1mProbabilityOfSuccess: f64,
}

// Java: public class GeometricDistribution extends AbstractIntegerDistribution
impl GeometricDistribution {
    // Java: GeometricDistribution(double p)
    pub fn GeometricDistribution(&self, p: f64) {
        // Java: this(new Well19937c(), p);
    }

    // Java: GeometricDistribution(RandomGenerator rng, double p)
    pub fn GeometricDistribution_1(&self, rng: &dyn RandomGenerator, p: f64) {
        // Java: super(rng);
        // Java: if (p <= 0 || p > 1)
        {
            // Java: throw new OutOfRangeException(LocalizedFormats.OUT_OF_RANGE_LEFT, p, 0, 1);
        }
        // Java: probabilityOfSuccess = p;
        // Java: logProbabilityOfSuccess = FastMath.log(p);
        // Java: log1mProbabilityOfSuccess = FastMath.log1p(-p);
    }

    // Java: double getProbabilityOfSuccess()
    pub fn getProbabilityOfSuccess(&self) -> f64 {
        // Java: return probabilityOfSuccess;
        todo!();
    }

    // Java: double probability(int x)
    pub fn probability(&self, x: i32) -> f64 {
        // Java: if (x < 0)
        {
            // Java: return 0.0;
        }
        // Java: else
        {
            // Java: return FastMath.exp(log1mProbabilityOfSuccess * x) * probabilityOfSuccess;
        }
        todo!();
    }

    // Java: double logProbability(int x)
    pub fn logProbability(&self, x: i32) -> f64 {
        // Java: if (x < 0)
        {
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: else
        {
            // Java: return x * log1mProbabilityOfSuccess + logProbabilityOfSuccess;
        }
        todo!();
    }

    // Java: double cumulativeProbability(int x)
    pub fn cumulativeProbability(&self, x: i32) -> f64 {
        // Java: if (x < 0)
        {
            // Java: return 0.0;
        }
        // Java: else
        {
            // Java: return -FastMath.expm1(log1mProbabilityOfSuccess * (x + 1));
        }
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return (1 - probabilityOfSuccess) / probabilityOfSuccess;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return (1 - probabilityOfSuccess) / (probabilityOfSuccess * probabilityOfSuccess);
        todo!();
    }

    // Java: int getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> i32 {
        // Java: return 0;
        todo!();
    }

    // Java: int getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> i32 {
        // Java: return Integer.MAX_VALUE;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: int inverseCumulativeProbability(double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> i32 {
        // Java: if (p < 0 || p > 1)
        {
            // Java: throw new OutOfRangeException(p, 0, 1);
        }
        // Java: if (p == 1)
        {
            // Java: return Integer.MAX_VALUE;
        }
        // Java: if (p == 0)
        {
            // Java: return 0;
        }
        // Java: return Math.max(0, (int) Math.ceil(FastMath.log1p(-p)/log1mProbabilityOfSuccess-1));
        todo!();
    }

}
