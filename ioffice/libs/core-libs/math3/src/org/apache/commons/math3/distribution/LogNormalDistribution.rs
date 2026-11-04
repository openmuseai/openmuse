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
// Java: import org.apache.commons.math3.special.Erf
use crate::org::apache::commons::math3::special::Erf;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;
// Java field: double SQRT2PI
pub(crate) const SQRT2PI: f64 = 0.0;
// Java field: double SQRT2
pub(crate) const SQRT2: f64 = 0.0;

// Java: public class LogNormalDistribution extends AbstractRealDistribution
pub struct LogNormalDistribution {
    // Java field: double scale
    pub(crate) scale: f64,
    // Java field: double shape
    pub(crate) shape: f64,
    // Java field: double logShapePlusHalfLog2Pi
    logShapePlusHalfLog2Pi: f64,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
}

// Java: public class LogNormalDistribution extends AbstractRealDistribution
impl LogNormalDistribution {
    // Java: LogNormalDistribution()
    pub fn LogNormalDistribution(&self) {
        // Java: this(0, 1);
    }

    // Java: LogNormalDistribution(double scale, double shape)
    pub fn LogNormalDistribution_1(&self, scale: f64, shape: f64) {
    }

    // Java: LogNormalDistribution(double scale, double shape, double inverseCumAccuracy)
    pub fn LogNormalDistribution_2(&self, scale: f64, shape: f64, inverseCumAccuracy: f64) {
    }

    // Java: LogNormalDistribution(RandomGenerator rng, double scale, double shape)
    pub fn LogNormalDistribution_3(&self, rng: &dyn RandomGenerator, scale: f64, shape: f64) {
    }

    // Java: LogNormalDistribution()
    pub fn LogNormalDistribution_4(&self) {
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
        // Java: if (x <= 0)
        {
            // Java: return 0;
        }
        // Java: final double x0 = FastMath.log(x) - scale;
        // Java: final double x1 = x0 / shape;
        // Java: return FastMath.exp(-0.5 * x1 * x1) / (shape * SQRT2PI * x);
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: if (x <= 0)
        {
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: final double logX = FastMath.log(x);
        // Java: final double x0 = logX - scale;
        // Java: final double x1 = x0 / shape;
        // Java: return -0.5 * x1 * x1 - (logShapePlusHalfLog2Pi + logX);
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: if (x <= 0)
        {
            // Java: return 0;
        }
        // Java: final double dev = FastMath.log(x) - scale;
        // Java: if (FastMath.abs(dev) > 40 * shape)
        {
            // Java: return dev < 0 ? 0.0d : 1.0d;
        }
        // Java: return 0.5 + 0.5 * Erf.erf(dev / (shape * SQRT2));
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
        // Java: double s = shape;
        // Java: return FastMath.exp(scale + (s * s / 2));
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: final double s = shape;
        // Java: final double ss = s * s;
        // Java: return (FastMath.expm1(ss)) * FastMath.exp(2 * scale + ss);
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

    // Java: double sample()
    pub fn sample(&self) -> f64 {
        // Java: final double n = random.nextGaussian();
        // Java: return FastMath.exp(scale + shape * n);
        todo!();
    }

}
