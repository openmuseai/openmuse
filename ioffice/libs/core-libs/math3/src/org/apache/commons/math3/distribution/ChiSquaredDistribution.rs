// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
use super::GammaDistribution::GammaDistribution;
use std::sync::Arc;

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class ChiSquaredDistribution extends AbstractRealDistribution
pub struct ChiSquaredDistribution {
    // Java field: GammaDistribution gamma
    pub(crate) gamma: Arc<GammaDistribution>,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
}

// Java: public class ChiSquaredDistribution extends AbstractRealDistribution
impl ChiSquaredDistribution {
    // Java: ChiSquaredDistribution(double degreesOfFreedom)
    pub fn ChiSquaredDistribution(&self, degreesOfFreedom: f64) {
        // Java: this(degreesOfFreedom, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: ChiSquaredDistribution()
    pub fn ChiSquaredDistribution_1(&self) {
    }

    // Java: ChiSquaredDistribution(RandomGenerator rng, double degreesOfFreedom)
    pub fn ChiSquaredDistribution_2(&self, rng: &dyn RandomGenerator, degreesOfFreedom: f64) {
        // Java: this(rng, degreesOfFreedom, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: ChiSquaredDistribution()
    pub fn ChiSquaredDistribution_3(&self) {
    }

    // Java: double getDegreesOfFreedom()
    pub fn getDegreesOfFreedom(&self) -> f64 {
        // Java: return gamma.getShape() * 2.0;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: return gamma.density(x);
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: return gamma.logDensity(x);
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: return gamma.cumulativeProbability(x);
        todo!();
    }

    // Java: double getSolverAbsoluteAccuracy()
    pub fn getSolverAbsoluteAccuracy(&self) -> f64 {
        // Java: return solverAbsoluteAccuracy;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return getDegreesOfFreedom();
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return 2 * getDegreesOfFreedom();
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
