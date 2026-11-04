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
// Java: import org.apache.commons.math3.special.Beta
use crate::org::apache::commons::math3::special::Beta;
// Java: import org.apache.commons.math3.util.CombinatoricsUtils
use crate::org::apache::commons::math3::util::CombinatoricsUtils;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class in
pub struct r#in {
}

// Java: public class PascalDistribution
pub struct PascalDistribution {
    // Java field: int numberOfSuccesses
    pub(crate) numberOfSuccesses: i32,
    // Java field: double probabilityOfSuccess
    pub(crate) probabilityOfSuccess: f64,
    // Java field: double logProbabilityOfSuccess
    pub(crate) logProbabilityOfSuccess: f64,
    // Java field: double log1mProbabilityOfSuccess
    pub(crate) log1mProbabilityOfSuccess: f64,
}

// Java: public class in
impl r#in {
}

// Java: public class PascalDistribution
impl PascalDistribution {
    // Java: PascalDistribution(int r, double p)
    pub fn PascalDistribution(&self, r: i32, p: f64) {
    }

    // Java: PascalDistribution()
    pub fn PascalDistribution_1(&self) {
    }

    // Java: int getNumberOfSuccesses()
    pub fn getNumberOfSuccesses(&self) -> i32 {
        // Java: return numberOfSuccesses;
        todo!();
    }

    // Java: double getProbabilityOfSuccess()
    pub fn getProbabilityOfSuccess(&self) -> f64 {
        // Java: return probabilityOfSuccess;
        todo!();
    }

    // Java: double probability(int x)
    pub fn probability(&self, x: i32) -> f64 {
        // Java: double ret;
        // Java: if (x < 0)
        {
            // Java: ret = 0.0;
        }
        // Java: else
        {
            // Java: ret = CombinatoricsUtils.binomialCoefficientDouble(x +
                  // Java: numberOfSuccesses - 1, numberOfSuccesses - 1) *
                  // Java: FastMath.pow(probabilityOfSuccess, numberOfSuccesses) *
                  // Java: FastMath.pow(1.0 - probabilityOfSuccess, x);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double logProbability(int x)
    pub fn logProbability(&self, x: i32) -> f64 {
        // Java: double ret;
        // Java: if (x < 0)
        {
            // Java: ret = Double.NEGATIVE_INFINITY;
        }
        // Java: else
        {
            // Java: ret = CombinatoricsUtils.binomialCoefficientLog(x +
                  // Java: numberOfSuccesses - 1, numberOfSuccesses - 1) +
                  // Java: logProbabilityOfSuccess * numberOfSuccesses +
                  // Java: log1mProbabilityOfSuccess * x;
        }
        // Java: return ret;
        todo!();
    }

    // Java: double cumulativeProbability(int x)
    pub fn cumulativeProbability(&self, x: i32) -> f64 {
        // Java: double ret;
        // Java: if (x < 0)
        {
            // Java: ret = 0.0;
        }
        // Java: else
        {
            // Java: ret = Beta.regularizedBeta(probabilityOfSuccess,
                    // Java: numberOfSuccesses, x + 1.0);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: final double p = getProbabilityOfSuccess();
        // Java: final double r = getNumberOfSuccesses();
        // Java: return (r * (1 - p)) / p;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: final double p = getProbabilityOfSuccess();
        // Java: final double r = getNumberOfSuccesses();
        // Java: return r * (1 - p) / (p * p);
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

}
