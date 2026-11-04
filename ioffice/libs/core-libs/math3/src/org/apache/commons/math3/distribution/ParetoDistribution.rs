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

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class ParetoDistribution extends AbstractRealDistribution
pub struct ParetoDistribution {
    // Java field: double scale
    pub(crate) scale: f64,
    // Java field: double shape
    pub(crate) shape: f64,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
}

// Java: public class ParetoDistribution extends AbstractRealDistribution
impl ParetoDistribution {
    // Java: ParetoDistribution()
    pub fn ParetoDistribution(&self) {
        // Java: this(1, 1);
    }

    // Java: ParetoDistribution(double scale, double shape)
    pub fn ParetoDistribution_1(&self, scale: f64, shape: f64) {
    }

    // Java: ParetoDistribution(double scale, double shape, double inverseCumAccuracy)
    pub fn ParetoDistribution_2(&self, scale: f64, shape: f64, inverseCumAccuracy: f64) {
    }

    // Java: ParetoDistribution(RandomGenerator rng, double scale, double shape)
    pub fn ParetoDistribution_3(&self, rng: &dyn RandomGenerator, scale: f64, shape: f64) {
    }

    // Java: ParetoDistribution()
    pub fn ParetoDistribution_4(&self) {
    }

    // Java: double getScale()
    pub fn getScale(&self) -> f64 {
        // Java: return scale;
        todo!();
    }

    // Java: double getShape()
    pub fn getShape(&self) -> f64 {
        // Java: return shape;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: if (x < scale)
        {
            // Java: return 0;
        }
        // Java: return FastMath.pow(scale, shape) / FastMath.pow(x, shape + 1) * shape;
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: if (x < scale)
        {
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: return FastMath.log(scale) * shape - FastMath.log(x) * (shape + 1) + FastMath.log(shape);
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: if (x <= scale)
        {
            // Java: return 0;
        }
        // Java: return 1 - FastMath.pow(scale / x, shape);
        todo!();
    }

    // Java: double cumulativeProbability(double x0, double x1)
    pub fn cumulativeProbability_1(&self, x0: f64, x1: f64) -> f64 {
        todo!();
    }

    // Java: double getSolverAbsoluteAccuracy()
    pub fn getSolverAbsoluteAccuracy(&self) -> f64 {
        // Java: return solverAbsoluteAccuracy;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: if (shape <= 1)
        {
            // Java: return Double.POSITIVE_INFINITY;
        }
        // Java: return shape * scale / (shape - 1);
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: if (shape <= 2)
        {
            // Java: return Double.POSITIVE_INFINITY;
        }
        // Java: double s = shape - 1;
        // Java: return scale * scale * shape / (s * s) / (shape - 2);
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return scale;
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

    // Java: double sample()
    pub fn sample(&self) -> f64 {
        // Java: final double n = random.nextDouble();
        // Java: return scale / FastMath.pow(n, 1 / shape);
        todo!();
    }

}
