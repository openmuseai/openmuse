// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.special.Beta
use crate::org::apache::commons::math3::special::Beta;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class BinomialDistribution extends AbstractIntegerDistribution
pub struct BinomialDistribution {
    // Java field: int numberOfTrials
    pub(crate) numberOfTrials: i32,
    // Java field: double probabilityOfSuccess
    pub(crate) probabilityOfSuccess: f64,
}

// Java: public class BinomialDistribution extends AbstractIntegerDistribution
impl BinomialDistribution {
    // Java: BinomialDistribution(int trials, double p)
    pub fn BinomialDistribution(&self, trials: i32, p: f64) {
        // Java: this(new Well19937c(), trials, p);
    }

    // Java: BinomialDistribution()
    pub fn BinomialDistribution_1(&self) {
    }

    // Java: int getNumberOfTrials()
    pub fn getNumberOfTrials(&self) -> i32 {
        // Java: return numberOfTrials;
        todo!();
    }

    // Java: double getProbabilityOfSuccess()
    pub fn getProbabilityOfSuccess(&self) -> f64 {
        // Java: return probabilityOfSuccess;
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
        // Java: if (numberOfTrials == 0)
        {
            // Java: return (x == 0) ? 0. : Double.NEGATIVE_INFINITY;
        }
        // Java: double ret;
        // Java: if (x < 0 || x > numberOfTrials)
        {
            // Java: ret = Double.NEGATIVE_INFINITY;
        }
        // Java: else
        {
            // Java: ret = SaddlePointExpansion.logBinomialProbability(x,
                    // Java: numberOfTrials, probabilityOfSuccess,
                    // Java: 1.0 - probabilityOfSuccess);
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
        // Java: else if (x >= numberOfTrials)
        {
            // Java: ret = 1.0;
        }
        // Java: else
        {
            // Java: ret = 1.0 - Beta.regularizedBeta(probabilityOfSuccess,
                    // Java: x + 1.0, numberOfTrials - x);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return numberOfTrials * probabilityOfSuccess;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: final double p = probabilityOfSuccess;
        // Java: return numberOfTrials * p * (1 - p);
        todo!();
    }

    // Java: int getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> i32 {
        // Java: return probabilityOfSuccess < 1.0 ? 0 : numberOfTrials;
        todo!();
    }

    // Java: int getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> i32 {
        // Java: return probabilityOfSuccess > 0.0 ? numberOfTrials : 0;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

}
