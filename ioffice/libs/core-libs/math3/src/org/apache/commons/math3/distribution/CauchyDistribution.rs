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
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class in
pub struct r#in {
}

// Java: public class CauchyDistribution
pub struct CauchyDistribution {
    // Java field: double median
    pub(crate) median: f64,
    // Java field: double scale
    pub(crate) scale: f64,
    // Java field: double solverAbsoluteAccuracy
    pub(crate) solverAbsoluteAccuracy: f64,
}

// Java: public class in
impl r#in {
}

// Java: public class CauchyDistribution
impl CauchyDistribution {
    // Java: CauchyDistribution()
    pub fn CauchyDistribution(&self) {
        // Java: this(0, 1);
    }

    // Java: CauchyDistribution(double median, double scale)
    pub fn CauchyDistribution_1(&self, median: f64, scale: f64) {
        // Java: this(median, scale, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: CauchyDistribution()
    pub fn CauchyDistribution_2(&self) {
    }

    // Java: CauchyDistribution(RandomGenerator rng, double median, double scale)
    pub fn CauchyDistribution_3(&self, rng: &dyn RandomGenerator, median: f64, scale: f64) {
        // Java: this(rng, median, scale, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: CauchyDistribution()
    pub fn CauchyDistribution_4(&self) {
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: return 0.5 + (FastMath.atan((x - median) / scale) / FastMath.PI);
        todo!();
    }

    // Java: double getMedian()
    pub fn getMedian(&self) -> f64 {
        // Java: return median;
        todo!();
    }

    // Java: double getScale()
    pub fn getScale(&self) -> f64 {
        // Java: return scale;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: final double dev = x - median;
        // Java: return (1 / FastMath.PI) * (scale / (dev * dev + scale * scale));
        todo!();
    }

    // Java: double inverseCumulativeProbability(double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> f64 {
        // Java: double ret;
        // Java: if (p < 0 || p > 1)
        {
            // Java: throw new OutOfRangeException(p, 0, 1);
        }
        // Java: else if (p == 0)
        {
            // Java: ret = Double.NEGATIVE_INFINITY;
        }
        // Java: else  if (p == 1)
        {
            // Java: ret = Double.POSITIVE_INFINITY;
        }
        // Java: else
        {
            // Java: ret = median + scale * FastMath.tan(FastMath.PI * (p - .5));
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
        // Java: return Double.NaN;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return Double.NaN;
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return Double.NEGATIVE_INFINITY;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return Double.POSITIVE_INFINITY;
        todo!();
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: return false;
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
