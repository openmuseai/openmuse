// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.special.Erf
use crate::org::apache::commons::math3::special::Erf;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class implements implements the
pub struct implements {
}

// Java: public class LevyDistribution
pub struct LevyDistribution {
    // Java field: double mu
    pub(crate) mu: f64,
    // Java field: double c
    pub(crate) c: f64,
    // Java field: double halfC
    pub(crate) halfC: f64,
}

// Java: public class implements implements the
impl implements {
}

// Java: public class LevyDistribution
impl LevyDistribution {
    // Java: LevyDistribution(final double mu, final double c)
    pub fn LevyDistribution(&self, mu: f64, c: f64) {
        // Java: this(new Well19937c(), mu, c);
    }

    // Java: LevyDistribution(final RandomGenerator rng, final double mu, final double c)
    pub fn LevyDistribution_1(&self, rng: &dyn RandomGenerator, mu: f64, c: f64) {
        // Java: super(rng);
        // Java: this.mu    = mu;
        // Java: this.c     = c;
        // Java: this.halfC = 0.5 * c;
    }

    // Java: double density(final double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: if (x < mu)
        {
            // Java: return Double.NaN;
        }
        // Java: final double delta = x - mu;
        // Java: final double f     = halfC / delta;
        // Java: return FastMath.sqrt(f / FastMath.PI) * FastMath.exp(-f) /delta;
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: if (x < mu)
        {
            // Java: return Double.NaN;
        }
        // Java: final double delta = x - mu;
        // Java: final double f     = halfC / delta;
        // Java: return 0.5 * FastMath.log(f / FastMath.PI) - f - FastMath.log(delta);
        todo!();
    }

    // Java: double cumulativeProbability(final double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: if (x < mu)
        {
            // Java: return Double.NaN;
        }
        // Java: return Erf.erfc(FastMath.sqrt(halfC / (x - mu)));
        todo!();
    }

    // Java: double inverseCumulativeProbability(final double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> f64 {
        // Java: if (p < 0.0 || p > 1.0)
        {
            // Java: throw new OutOfRangeException(p, 0, 1);
        }
        // Java: final double t = Erf.erfcInv(p);
        // Java: return mu + halfC / (t * t);
        todo!();
    }

    // Java: double getScale()
    pub fn getScale(&self) -> f64 {
        // Java: return c;
        todo!();
    }

    // Java: double getLocation()
    pub fn getLocation(&self) -> f64 {
        // Java: return mu;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return Double.POSITIVE_INFINITY;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return Double.POSITIVE_INFINITY;
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return mu;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return Double.POSITIVE_INFINITY;
        todo!();
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: // there is a division by x-mu in the computation, so density
        // Java: // is not finite at lower bound, bound must be excluded
        // Java: return false;
        todo!();
    }

    // Java: boolean isSupportUpperBoundInclusive()
    pub fn isSupportUpperBoundInclusive(&self) -> bool {
        // Java: // upper bound is infinite, so it must be excluded
        // Java: return false;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

}

// impl the for implements {
// }
