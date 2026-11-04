// Java: import org.apache.commons.math3.distribution.NormalDistribution
use crate::org::apache::commons::math3::distribution::NormalDistribution;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class AgrestiCoullInterval implements BinomialConfidenceInterval
pub struct AgrestiCoullInterval {
}

// Java: public class AgrestiCoullInterval implements BinomialConfidenceInterval
impl AgrestiCoullInterval {
    // Java: ConfidenceInterval createInterval(int numberOfTrials, int numberOfSuccesses, double confidenceLevel)
    pub fn createInterval(&self) -> Box<dyn std::any::Any> {
        // Java: IntervalUtils.checkParameters(numberOfTrials, numberOfSuccesses, confidenceLevel);
        // Java: final double alpha = (1.0 - confidenceLevel) / 2;
        // Java: final NormalDistribution normalDistribution = new NormalDistribution();
        // Java: final double z = normalDistribution.inverseCumulativeProbability(1 - alpha);
        // Java: final double zSquared = FastMath.pow(z, 2);
        // Java: final double modifiedNumberOfTrials = numberOfTrials + zSquared;
        // Java: final double modifiedSuccessesRatio = (1.0 / modifiedNumberOfTrials) * (numberOfSuccesses + 0.5 * zSquared);
        // Java: final double difference = z *
                                  // Java: FastMath.sqrt(1.0 / modifiedNumberOfTrials * modifiedSuccessesRatio *
                                                // Java: (1 - modifiedSuccessesRatio));
        // Java: return new ConfidenceInterval(modifiedSuccessesRatio - difference, modifiedSuccessesRatio + difference,
                                      // Java: confidenceLevel);
        todo!()
    }

}

