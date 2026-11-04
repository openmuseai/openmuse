// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.random.RandomGenerator
use crate::org::apache::commons::math3::random::RandomGenerator::RandomGenerator;
// Java: import org.apache.commons.math3.random.Well19937c
use crate::org::apache::commons::math3::random::Well19937c;
// Java: import org.apache.commons.math3.special.Beta
use crate::org::apache::commons::math3::special::Beta;
// Java: import org.apache.commons.math3.special.Gamma
use crate::org::apache::commons::math3::special::Gamma;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class in
pub struct r#in {
}

// Java: public class BetaDistribution
pub struct BetaDistribution {
    // Java field: double alpha
    pub(crate) alpha: f64,
    // Java field: double beta
    pub(crate) beta: f64,
    // Java field: double z
    pub(crate) z: f64,
    // Java field: double solverAbsoluteAccuracy
    pub(crate) solverAbsoluteAccuracy: f64,
}

// Java: public class ChengBetaSampler
pub struct ChengBetaSampler {
}

// Java: public class in
impl r#in {
}

// Java: public class BetaDistribution
impl BetaDistribution {
    // Java: BetaDistribution(double alpha, double beta)
    pub fn BetaDistribution(&self, alpha: f64, beta: f64) {
        // Java: this(alpha, beta, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: BetaDistribution(double alpha, double beta, double inverseCumAccuracy)
    pub fn BetaDistribution_1(&self, alpha: f64, beta: f64, inverseCumAccuracy: f64) {
        // Java: this(new Well19937c(), alpha, beta, inverseCumAccuracy);
    }

    // Java: BetaDistribution(RandomGenerator rng, double alpha, double beta)
    pub fn BetaDistribution_2(&self, rng: &dyn RandomGenerator, alpha: f64, beta: f64) {
        // Java: this(rng, alpha, beta, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: BetaDistribution()
    pub fn BetaDistribution_3(&self) {
    }

    // Java: double getAlpha()
    pub fn getAlpha(&self) -> f64 {
        // Java: return alpha;
        todo!();
    }

    // Java: double getBeta()
    pub fn getBeta(&self) -> f64 {
        // Java: return beta;
        todo!();
    }

    // Java: void recomputeZ()
    pub fn recomputeZ(&self) {
        // Java: if (Double.isNaN(z))
        {
            // Java: z = Gamma.logGamma(alpha) + Gamma.logGamma(beta) - Gamma.logGamma(alpha + beta);
        }
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: final double logDensity = logDensity(x);
        // Java: return logDensity == Double.NEGATIVE_INFINITY ? 0 : FastMath.exp(logDensity);
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: recomputeZ();
        // Java: if (x < 0 || x > 1)
        {
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: else if (x == 0)
        {
            // Java: if (alpha < 1)
            {
                // Java: throw new NumberIsTooSmallException(LocalizedFormats.CANNOT_COMPUTE_BETA_DENSITY_AT_0_FOR_SOME_ALPHA, alpha, 1, false);
            }
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: else if (x == 1)
        {
            // Java: if (beta < 1)
            {
                // Java: throw new NumberIsTooSmallException(LocalizedFormats.CANNOT_COMPUTE_BETA_DENSITY_AT_1_FOR_SOME_BETA, beta, 1, false);
            }
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: else
        {
            // Java: double logX = FastMath.log(x);
            // Java: double log1mX = FastMath.log1p(-x);
            // Java: return (alpha - 1) * logX + (beta - 1) * log1mX - z;
        }
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: if (x <= 0)
        {
            // Java: return 0;
        }
        // Java: else if (x >= 1)
        {
            // Java: return 1;
        }
        // Java: else
        {
            // Java: return Beta.regularizedBeta(x, alpha, beta);
        }
        todo!();
    }

    // Java: double getSolverAbsoluteAccuracy()
    pub fn getSolverAbsoluteAccuracy(&self) -> f64 {
        // Java: return solverAbsoluteAccuracy;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: final double a = getAlpha();
        // Java: return a / (a + getBeta());
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: final double a = getAlpha();
        // Java: final double b = getBeta();
        // Java: final double alphabetasum = a + b;
        // Java: return (a * b) / ((alphabetasum * alphabetasum) * (alphabetasum + 1));
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return 0;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return 1;
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

    // Java: double sample()
    pub fn sample(&self) -> f64 {
        // Java: return ChengBetaSampler.sample(random, alpha, beta);
        todo!();
    }

}

// Java: public class ChengBetaSampler
impl ChengBetaSampler {
    // Java: static double sample(RandomGenerator random, final double alpha, final double beta)
    pub fn sample_1(&self, random: &dyn RandomGenerator, alpha: f64, beta: f64) -> f64 {
            // Java: final double a = FastMath.min(alpha, beta);
            // Java: final double b = FastMath.max(alpha, beta);
            // Java: if (a > 1)
            {
                // Java: return algorithmBB(random, alpha, a, b);
            }
            // Java: else
            {
                // Java: return algorithmBC(random, alpha, b, a);
            }
        todo!();
    }

    // Java: static double algorithmBB()
    pub fn algorithmBB(&self) -> f64 {
        todo!();
    }

    // Java: static double algorithmBC()
    pub fn algorithmBC(&self) -> f64 {
        todo!();
    }

}
