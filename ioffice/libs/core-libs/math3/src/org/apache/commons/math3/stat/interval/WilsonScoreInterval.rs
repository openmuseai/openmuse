// Java: import org.apache.commons.math3.distribution.NormalDistribution
use crate::org::apache::commons::math3::distribution::NormalDistribution;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class WilsonScoreInterval implements BinomialConfidenceInterval
pub struct WilsonScoreInterval {
}

// Java: public class WilsonScoreInterval implements BinomialConfidenceInterval
impl WilsonScoreInterval {
    // Java: ConfidenceInterval createInterval(int numberOfTrials, int numberOfSuccesses, double confidenceLevel)
    pub fn createInterval(&self) -> Box<dyn std::any::Any> {
        // Java: IntervalUtils.checkParameters(numberOfTrials, numberOfSuccesses, confidenceLevel);
        // Java: final double alpha = (1.0 - confidenceLevel) / 2;
        // Java: final NormalDistribution normalDistribution = new NormalDistribution();
        // Java: final double z = normalDistribution.inverseCumulativeProbability(1 - alpha);
        // Java: final double zSquared = FastMath.pow(z, 2);
        // Java: final double mean = (double) numberOfSuccesses / (double) numberOfTrials;
        // Java: final double factor = 1.0 / (1 + (1.0 / numberOfTrials) * zSquared);
        // Java: final double modifiedSuccessRatio = mean + (1.0 / (2 * numberOfTrials)) * zSquared;
        // Java: final double difference = z *
                                  // Java: FastMath.sqrt(1.0 / numberOfTrials * mean * (1 - mean) +
                                                // Java: (1.0 / (4 * FastMath.pow(numberOfTrials, 2)) * zSquared));
        // Java: final double lowerBound = factor * (modifiedSuccessRatio - difference);
        // Java: final double upperBound = factor * (modifiedSuccessRatio + difference);
        // Java: return new ConfidenceInterval(lowerBound, upperBound, confidenceLevel);
        todo!()
    }

}

