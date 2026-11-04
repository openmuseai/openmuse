// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
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

// Java: public class implements implements the Nakagami distribution.
pub struct implements {
}

// Java: public class NakagamiDistribution
pub struct NakagamiDistribution {
    // Java field: double mu
    pub(crate) mu: f64,
    // Java field: double omega
    pub(crate) omega: f64,
    // Java field: double inverseAbsoluteAccuracy
    pub(crate) inverseAbsoluteAccuracy: f64,
}

// Java: public class implements implements the Nakagami distribution.
impl implements {
}

// Java: public class NakagamiDistribution
impl NakagamiDistribution {
    // Java: NakagamiDistribution(double mu, double omega)
    pub fn NakagamiDistribution(&self, mu: f64, omega: f64) {
        // Java: this(mu, omega, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: NakagamiDistribution(double mu, double omega, double inverseAbsoluteAccuracy)
    pub fn NakagamiDistribution_1(&self, mu: f64, omega: f64, inverseAbsoluteAccuracy: f64) {
        // Java: this(new Well19937c(), mu, omega, inverseAbsoluteAccuracy);
    }

    // Java: NakagamiDistribution(RandomGenerator rng, double mu, double omega, double inverseAbsoluteAccuracy)
    pub fn NakagamiDistribution_2(&self, rng: &dyn RandomGenerator, mu: f64, omega: f64, inverseAbsoluteAccuracy: f64) {
        // Java: super(rng);
        // Java: if (mu < 0.5)
        {
            // Java: throw new NumberIsTooSmallException(mu, 0.5, true);
        }
        // Java: if (omega <= 0)
        {
            // Java: throw new NotStrictlyPositiveException(LocalizedFormats.NOT_POSITIVE_SCALE, omega);
        }
        // Java: this.mu = mu;
        // Java: this.omega = omega;
        // Java: this.inverseAbsoluteAccuracy = inverseAbsoluteAccuracy;
    }

    // Java: double getShape()
    pub fn getShape(&self) -> f64 {
        // Java: return mu;
        todo!();
    }

    // Java: double getScale()
    pub fn getScale(&self) -> f64 {
        // Java: return omega;
        todo!();
    }

    // Java: double getSolverAbsoluteAccuracy()
    pub fn getSolverAbsoluteAccuracy(&self) -> f64 {
        // Java: return inverseAbsoluteAccuracy;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: if (x <= 0)
        {
            // Java: return 0.0;
        }
        // Java: return 2.0 * FastMath.pow(mu, mu) / (Gamma.gamma(mu) * FastMath.pow(omega, mu)) *
                     // Java: FastMath.pow(x, 2 * mu - 1) * FastMath.exp(-mu * x * x / omega);
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: return Gamma.regularizedGammaP(mu, mu * x * x / omega);
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return Gamma.gamma(mu + 0.5) / Gamma.gamma(mu) * FastMath.sqrt(omega / mu);
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: double v = Gamma.gamma(mu + 0.5) / Gamma.gamma(mu);
        // Java: return omega * (1 - 1 / mu * v * v);
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

impl implements {
}
