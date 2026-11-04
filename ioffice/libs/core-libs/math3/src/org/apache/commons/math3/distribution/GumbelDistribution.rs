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
// Java field: double EULER
pub(crate) const EULER: f64 = 0.0;

// Java: public class implements implements the Gumbel distribution.
pub struct implements {
}

// Java: public class GumbelDistribution
pub struct GumbelDistribution {
    // Java field: double mu
    pub(crate) mu: f64,
    // Java field: double beta
    pub(crate) beta: f64,
}

// Java: public class implements implements the Gumbel distribution.
impl implements {
}

// Java: public class GumbelDistribution
impl GumbelDistribution {
    // Java: GumbelDistribution(double mu, double beta)
    pub fn GumbelDistribution(&self, mu: f64, beta: f64) {
        // Java: this(new Well19937c(), mu, beta);
    }

    // Java: GumbelDistribution(RandomGenerator rng, double mu, double beta)
    pub fn GumbelDistribution_1(&self, rng: &dyn RandomGenerator, mu: f64, beta: f64) {
        // Java: super(rng);
        // Java: if (beta <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.SCALE, beta);
        }
        // Java: this.beta = beta;
        // Java: this.mu = mu;
    }

    // Java: double getLocation()
    pub fn getLocation(&self) -> f64 {
        // Java: return mu;
        todo!();
    }

    // Java: double getScale()
    pub fn getScale(&self) -> f64 {
        // Java: return beta;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: final double z = (x - mu) / beta;
        // Java: final double t = FastMath.exp(-z);
        // Java: return FastMath.exp(-z - t) / beta;
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: final double z = (x - mu) / beta;
        // Java: return FastMath.exp(-FastMath.exp(-z));
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
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: else if (p == 1)
        {
            // Java: return Double.POSITIVE_INFINITY;
        }
        // Java: return mu - FastMath.log(-FastMath.log(p)) * beta;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return mu + EULER * beta;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return (MathUtils.PI_SQUARED) / 6.0 * (beta * beta);
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
