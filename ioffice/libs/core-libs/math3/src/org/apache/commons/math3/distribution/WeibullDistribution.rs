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
// Java: import org.apache.commons.math3.special.Gamma
use crate::org::apache::commons::math3::special::Gamma;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class in
pub struct r#in {
}

// Java: public class WeibullDistribution
pub struct WeibullDistribution {
    // Java field: double shape
    pub(crate) shape: f64,
    // Java field: double scale
    pub(crate) scale: f64,
    // Java field: double solverAbsoluteAccuracy
    pub(crate) solverAbsoluteAccuracy: f64,
    // Java field: double numericalMean
    pub(crate) numericalMean: f64,
    // Java field: boolean numericalMeanIsCalculated
    pub(crate) numericalMeanIsCalculated: bool,
    // Java field: double numericalVariance
    pub(crate) numericalVariance: f64,
    // Java field: boolean numericalVarianceIsCalculated
    pub(crate) numericalVarianceIsCalculated: bool,
}

// Java: public class in
impl r#in {
}

// Java: public class WeibullDistribution
impl WeibullDistribution {
    // Java: WeibullDistribution(double alpha, double beta)
    pub fn WeibullDistribution(&self, alpha: f64, beta: f64) {
    }

    // Java: WeibullDistribution()
    pub fn WeibullDistribution_1(&self) {
    }

    // Java: WeibullDistribution(RandomGenerator rng, double alpha, double beta)
    pub fn WeibullDistribution_2(&self, rng: &dyn RandomGenerator, alpha: f64, beta: f64) {
    }

    // Java: WeibullDistribution()
    pub fn WeibullDistribution_3(&self) {
    }

    // Java: double getShape()
    pub fn getShape(&self) -> f64 {
        // Java: return shape;
        todo!();
    }

    // Java: double getScale()
    pub fn getScale(&self) -> f64 {
        // Java: return scale;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: if (x < 0)
        {
            // Java: return 0;
        }
        // Java: final double xscale = x / scale;
        // Java: final double xscalepow = FastMath.pow(xscale, shape - 1);
        // Java: /*
         // Java: * FastMath.pow(x / scale, shape) =
         // Java: * FastMath.pow(xscale, shape) =
         // Java: * FastMath.pow(xscale, shape - 1) * xscale
         // Java: */
        // Java: final double xscalepowshape = xscalepow * xscale;
        // Java: return (shape / scale) * xscalepow * FastMath.exp(-xscalepowshape);
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: if (x < 0)
        {
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: final double xscale = x / scale;
        // Java: final double logxscalepow = FastMath.log(xscale) * (shape - 1);
        // Java: /*
         // Java: * FastMath.pow(x / scale, shape) =
         // Java: * FastMath.pow(xscale, shape) =
         // Java: * FastMath.pow(xscale, shape - 1) * xscale
         // Java: */
        // Java: final double xscalepowshape = FastMath.exp(logxscalepow) * xscale;
        // Java: return FastMath.log(shape / scale) + logxscalepow - xscalepowshape;
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
            // Java: ret = 1.0 - FastMath.exp(-FastMath.pow(x / scale, shape));
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
        // Java: else if (p == 0)
        {
            // Java: ret = 0.0;
        }
        // Java: else  if (p == 1)
        {
            // Java: ret = Double.POSITIVE_INFINITY;
        }
        // Java: else
        {
            // Java: ret = scale * FastMath.pow(-FastMath.log1p(-p), 1.0 / shape);
        }
        // Java: return ret;
        todo!();
    }

    // Java: double getSolverAbsoluteAccuracy()
    pub fn getSolverAbsoluteAccuracy(&self) -> f64 {
        // Java: return solverAbsoluteAccuracy;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: if (!numericalMeanIsCalculated)
        {
            // Java: numericalMean = calculateNumericalMean();
            // Java: numericalMeanIsCalculated = true;
        }
        // Java: return numericalMean;
        todo!();
    }

    // Java: double calculateNumericalMean()
    pub fn calculateNumericalMean(&self) -> f64 {
        // Java: final double sh = getShape();
        // Java: final double sc = getScale();
        // Java: return sc * FastMath.exp(Gamma.logGamma(1 + (1 / sh)));
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: if (!numericalVarianceIsCalculated)
        {
            // Java: numericalVariance = calculateNumericalVariance();
            // Java: numericalVarianceIsCalculated = true;
        }
        // Java: return numericalVariance;
        todo!();
    }

    // Java: double calculateNumericalVariance()
    pub fn calculateNumericalVariance(&self) -> f64 {
        // Java: final double sh = getShape();
        // Java: final double sc = getScale();
        // Java: final double mn = getNumericalMean();
        // Java: return (sc * sc) * FastMath.exp(Gamma.logGamma(1 + (2 / sh))) -
               // Java: (mn * mn);
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
