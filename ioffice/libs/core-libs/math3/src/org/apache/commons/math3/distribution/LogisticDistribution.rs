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
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class implements implements the Logistic distribution.
pub struct implements {
}

// Java: public class LogisticDistribution
pub struct LogisticDistribution {
    // Java field: double mu
    pub(crate) mu: f64,
    // Java field: double s
    pub(crate) s: f64,
}

// Java: public class implements implements the Logistic distribution.
impl implements {
}

// Java: public class LogisticDistribution
impl LogisticDistribution {
    // Java: LogisticDistribution(double mu, double s)
    pub fn LogisticDistribution(&self, mu: f64, s: f64) {
        // Java: this(new Well19937c(), mu, s);
    }

    // Java: LogisticDistribution(RandomGenerator rng, double mu, double s)
    pub fn LogisticDistribution_1(&self, rng: &dyn RandomGenerator, mu: f64, s: f64) {
        // Java: super(rng);
        // Java: if (s <= 0.0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.NOT_POSITIVE_SCALE, s);
        }
        // Java: this.mu = mu;
        // Java: this.s = s;
    }

    // Java: double getLocation()
    pub fn getLocation(&self) -> f64 {
        // Java: return mu;
        todo!();
    }

    // Java: double getScale()
    pub fn getScale(&self) -> f64 {
        // Java: return s;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: double z = (x - mu) / s;
        // Java: double v = FastMath.exp(-z);
        // Java: return 1 / s * v / ((1.0 + v) * (1.0 + v));
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: double z = 1 / s * (x - mu);
        // Java: return 1.0 / (1.0 + FastMath.exp(-z));
        todo!();
    }

    // Java: double inverseCumulativeProbability(double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> f64 {
        // Java: if (p < 0.0 || p > 1.0)
        {
            // Java: throw new OutOfRangeException(p, 0.0, 1.0);
        }
        // Java: else if (p == 0)
        {
            // Java: return 0.0;
        }
        // Java: else if (p == 1)
        {
            // Java: return Double.POSITIVE_INFINITY;
        }
        // Java: return s * Math.log(p / (1.0 - p)) + mu;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return mu;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return (MathUtils.PI_SQUARED / 3.0) * (1.0 / (s * s));
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

impl implements {
}
