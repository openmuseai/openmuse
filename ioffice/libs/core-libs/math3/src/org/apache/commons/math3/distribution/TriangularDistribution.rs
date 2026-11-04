// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
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

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 0;

// Java: public class TriangularDistribution extends AbstractRealDistribution
pub struct TriangularDistribution {
    // Java field: double a
    pub(crate) a: f64,
    // Java field: double b
    pub(crate) b: f64,
    // Java field: double c
    pub(crate) c: f64,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
}

// Java: public class TriangularDistribution extends AbstractRealDistribution
impl TriangularDistribution {
    // Java: TriangularDistribution(double a, double c, double b)
    pub fn TriangularDistribution(&self, a: f64, c: f64, b: f64) {
    }

    // Java: TriangularDistribution()
    pub fn TriangularDistribution_1(&self) {
    }

    // Java: double getMode()
    pub fn getMode(&self) -> f64 {
        // Java: return c;
        todo!();
    }

    // Java: double getSolverAbsoluteAccuracy()
    pub fn getSolverAbsoluteAccuracy(&self) -> f64 {
        // Java: return solverAbsoluteAccuracy;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
        // Java: if (x < a)
        {
            // Java: return 0;
        }
        // Java: if (a <= x && x < c)
        {
            // Java: double divident = 2 * (x - a);
            // Java: double divisor = (b - a) * (c - a);
            // Java: return divident / divisor;
        }
        // Java: if (x == c)
        {
            // Java: return 2 / (b - a);
        }
        // Java: if (c < x && x <= b)
        {
            // Java: double divident = 2 * (b - x);
            // Java: double divisor = (b - a) * (b - c);
            // Java: return divident / divisor;
        }
        // Java: return 0;
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: if (x < a)
        {
            // Java: return 0;
        }
        // Java: if (a <= x && x < c)
        {
            // Java: double divident = (x - a) * (x - a);
            // Java: double divisor = (b - a) * (c - a);
            // Java: return divident / divisor;
        }
        // Java: if (x == c)
        {
            // Java: return (c - a) / (b - a);
        }
        // Java: if (c < x && x <= b)
        {
            // Java: double divident = (b - x) * (b - x);
            // Java: double divisor = (b - a) * (b - c);
            // Java: return 1 - (divident / divisor);
        }
        // Java: return 1;
        todo!();
    }

    // Java: double getNumericalMean()
    pub fn getNumericalMean(&self) -> f64 {
        // Java: return (a + b + c) / 3;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return (a * a + b * b + c * c - a * b - a * c - b * c) / 18;
        todo!();
    }

    // Java: double getSupportLowerBound()
    pub fn getSupportLowerBound(&self) -> f64 {
        // Java: return a;
        todo!();
    }

    // Java: double getSupportUpperBound()
    pub fn getSupportUpperBound(&self) -> f64 {
        // Java: return b;
        todo!();
    }

    // Java: boolean isSupportLowerBoundInclusive()
    pub fn isSupportLowerBoundInclusive(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: boolean isSupportUpperBoundInclusive()
    pub fn isSupportUpperBoundInclusive(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: boolean isSupportConnected()
    pub fn isSupportConnected(&self) -> bool {
        // Java: return true;
        todo!();
    }

    // Java: double inverseCumulativeProbability(double p)
    pub fn inverseCumulativeProbability(&self, p: f64) -> f64 {
        todo!();
    }

}
