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

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class UniformRealDistribution extends AbstractRealDistribution
pub struct UniformRealDistribution {
    // Java field: double lower
    pub(crate) lower: f64,
    // Java field: double upper
    pub(crate) upper: f64,
}

// Java: public class UniformRealDistribution extends AbstractRealDistribution
impl UniformRealDistribution {
    // Java: UniformRealDistribution()
    pub fn UniformRealDistribution(&self) {
        // Java: this(0, 1);
    }

    // Java: UniformRealDistribution(double lower, double upper)
    pub fn UniformRealDistribution_1(&self, lower: f64, upper: f64) {
    }

    // Java: UniformRealDistribution(double lower, double upper, double inverseCumAccuracy)
    pub fn UniformRealDistribution_2(&self, lower: f64, upper: f64, inverseCumAccuracy: f64) {
    }

    // Java: UniformRealDistribution()
    pub fn UniformRealDistribution_3(&self) {
    }

    // Java: UniformRealDistribution()
    pub fn UniformRealDistribution_4(&self) {
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: if (x < lower || x > upper)
        {
            // Java: return 0.0;
        }
        // Java: return 1 / (upper - lower);
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: if (x <= lower)
        {
            // Java: return 0;
        }
        // Java: if (x >= upper)
        {
            // Java: return 1;
        }
        // Java: return (x - lower) / (upper - lower);
        todo!();
    }

    // Java: double inverseCumulativeProbability(final double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> f64 {
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return 0.5 * (lower + upper);
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: double ul = upper - lower;
        // Java: return ul * ul / 12;
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return lower;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return upper;
        todo!();
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: boolean isSupportUpperBoundInclusive()
    pub fn isSupportUpperBoundInclusive(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: double sample()
    pub fn sample(&self) -> f64 {
        // Java: final double u = random.nextDouble();
        // Java: return u * upper + (1 - u) * lower;
        todo!();
    }

}
