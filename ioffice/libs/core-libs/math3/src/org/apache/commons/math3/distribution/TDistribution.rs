// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
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

// Java field: double DEFAULT_INVERSE_ABSOLUTE_ACCURACY
pub(crate) const DEFAULT_INVERSE_ABSOLUTE_ACCURACY: f64 = 0.0;
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class TDistribution extends AbstractRealDistribution
pub struct TDistribution {
    // Java field: double degreesOfFreedom
    pub(crate) degreesOfFreedom: f64,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
    // Java field: double factor
    pub(crate) factor: f64,
}

// Java: public class TDistribution extends AbstractRealDistribution
impl TDistribution {
    // Java: TDistribution(double degreesOfFreedom)
    pub fn TDistribution(&self, degreesOfFreedom: f64) {
    }

    // Java: TDistribution(double degreesOfFreedom, double inverseCumAccuracy)
    pub fn TDistribution_1(&self, degreesOfFreedom: f64, inverseCumAccuracy: f64) {
    }

    // Java: TDistribution(RandomGenerator rng, double degreesOfFreedom)
    pub fn TDistribution_2(&self, rng: &dyn RandomGenerator, degreesOfFreedom: f64) {
    }

    // Java: TDistribution()
    pub fn TDistribution_3(&self) {
    }

    // Java: double getDegreesOfFreedom()
    pub fn getDegreesOfFreedom(&self) -> f64 {
        // Java: return degreesOfFreedom;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: return FastMath.exp(logDensity(x));
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: final double n = degreesOfFreedom;
        // Java: final double nPlus1Over2 = (n + 1) / 2;
        // Java: return factor - nPlus1Over2 * FastMath.log(1 + x * x / n);
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: double ret;
        // Java: if (x == 0)
        {
            // Java: ret = 0.5;
        }
        // Java: else
        {
            // Java: double t =
                // Java: Beta.regularizedBeta(
                    // Java: degreesOfFreedom / (degreesOfFreedom + (x * x)),
                    // Java: 0.5 * degreesOfFreedom,
                    // Java: 0.5);
            // Java: if (x < 0.0)
            {
                // Java: ret = 0.5 * t;
            }
            // Java: else
            {
                // Java: ret = 1.0 - 0.5 * t;
            }
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
        // Java: final double df = getDegreesOfFreedom();
        // Java: if (df > 1)
        {
            // Java: return 0;
        }
        // Java: return Double.NaN;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: final double df = getDegreesOfFreedom();
        // Java: if (df > 2)
        {
            // Java: return df / (df - 2);
        }
        // Java: if (df > 1 && df <= 2)
        {
            // Java: return Double.POSITIVE_INFINITY;
        }
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
