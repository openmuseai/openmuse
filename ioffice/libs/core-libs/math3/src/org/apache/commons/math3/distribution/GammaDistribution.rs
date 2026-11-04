// Java: import org.apache.commons.math3.exception.NotStrictlyPositiveException
use crate::org::apache::commons::math3::exception::NotStrictlyPositiveException;
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

// Java: public class GammaDistribution extends AbstractRealDistribution
pub struct GammaDistribution {
    // Java field: double shape
    pub(crate) shape: f64,
    // Java field: double scale
    pub(crate) scale: f64,
    // Java field: double shiftedShape
    shiftedShape: f64,
    // Java field: double densityPrefactor1
    pub(crate) densityPrefactor1: f64,
    // Java field: double logDensityPrefactor1
    pub(crate) logDensityPrefactor1: f64,
    // Java field: double densityPrefactor2
    pub(crate) densityPrefactor2: f64,
    // Java field: double logDensityPrefactor2
    pub(crate) logDensityPrefactor2: f64,
    // Java field: double minY
    minY: f64,
    // Java field: double maxLogY
    maxLogY: f64,
    // Java field: double solverAbsoluteAccuracy
    solverAbsoluteAccuracy: f64,
}

// Java: public class GammaDistribution extends AbstractRealDistribution
impl GammaDistribution {
    // Java: GammaDistribution(double shape, double scale)
    pub fn GammaDistribution(&self, shape: f64, scale: f64) {
        // Java: this(shape, scale, DEFAULT_INVERSE_ABSOLUTE_ACCURACY);
    }

    // Java: GammaDistribution(double shape, double scale, double inverseCumAccuracy)
    pub fn GammaDistribution_1(&self, shape: f64, scale: f64, inverseCumAccuracy: f64) {
    }

    // Java: GammaDistribution(RandomGenerator rng, double shape, double scale)
    pub fn GammaDistribution_2(&self, rng: &dyn RandomGenerator, shape: f64, scale: f64) {
    }

    // Java: GammaDistribution()
    pub fn GammaDistribution_3(&self) {
    }

    // Java: double getAlpha()
    pub fn getAlpha(&self) -> f64 {
        // Java: return shape;
        todo!();
    }

    // Java: double getShape()
    pub fn getShape(&self) -> f64 {
        // Java: return shape;
        todo!();
    }

    // Java: double getBeta()
    pub fn getBeta(&self) -> f64 {
        // Java: return scale;
        todo!();
    }

    // Java: double getScale()
    pub fn getScale(&self) -> f64 {
        // Java: return scale;
        todo!();
    }

    // Java: double density(double x)
    pub fn density(&self, x: f64) -> f64 {
       // Java: /* The present method must return the value of
        // Java: *
        // Java: *     1       x a     - x
        // Java: * ---------- (-)  exp(---)
        // Java: * x Gamma(a)  b        b
        // Java: *
        // Java: * where a is the shape parameter, and b the scale parameter.
        // Java: * Substituting the Lanczos approximation of Gamma(a) leads to the
        // Java: * following expression of the density
        // Java: *
        // Java: * a              e            1         y      a
        // Java: * - sqrt(------------------) ---- (-----------)  exp(a - y + g),
        // Java: * x      2 pi (a + g + 0.5)  L(a)  a + g + 0.5
        // Java: *
        // Java: * where y = x / b. The above formula is the "natural" computation, which
        // Java: * is implemented when no overflow is likely to occur. If overflow occurs
        // Java: * with the natural computation, the following identity is used. It is
        // Java: * based on the BOOST library
        // Java: * http://www.boost.org/doc/libs/1_35_0/libs/math/doc/sf_and_dist/html/math_toolkit/special/sf_gamma/igamma.html
        // Java: * Formula (15) needs adaptations, which are detailed below.
        // Java: *
        // Java: *       y      a
        // Java: * (-----------)  exp(a - y + g)
        // Java: *  a + g + 0.5
        // Java: *                              y - a - g - 0.5    y (g + 0.5)
        // Java: *               = exp(a log1pm(---------------) - ----------- + g),
        // Java: *                                a + g + 0.5      a + g + 0.5
        // Java: *
        // Java: *  where log1pm(z) = log(1 + z) - z. Therefore, the value to be
        // Java: *  returned is
        // Java: *
        // Java: * a              e            1
        // Java: * - sqrt(------------------) ----
        // Java: * x      2 pi (a + g + 0.5)  L(a)
        // Java: *                              y - a - g - 0.5    y (g + 0.5)
        // Java: *               * exp(a log1pm(---------------) - ----------- + g).
        // Java: *                                a + g + 0.5      a + g + 0.5
        // Java: */
        // Java: if (x < 0)
        {
            // Java: return 0;
        }
        // Java: final double y = x / scale;
        // Java: if ((y <= minY) || (FastMath.log(y) >= maxLogY))
        {
            // Java: /*
             // Java: * Overflow.
             // Java: */
            // Java: final double aux1 = (y - shiftedShape) / shiftedShape;
            // Java: final double aux2 = shape * (FastMath.log1p(aux1) - aux1);
            // Java: final double aux3 = -y * (Gamma.LANCZOS_G + 0.5) / shiftedShape +
                    // Java: Gamma.LANCZOS_G + aux2;
            // Java: return densityPrefactor2 / x * FastMath.exp(aux3);
        }
        // Java: /*
         // Java: * Natural calculation.
         // Java: */
        // Java: return densityPrefactor1 * FastMath.exp(-y) * FastMath.pow(y, shape - 1);
        todo!();
    }

    // Java: double logDensity(double x)
    pub fn logDensity(&self, x: f64) -> f64 {
        // Java: /*
         // Java: * see the comment in {@link #density(double)} for computation details
         // Java: */
        // Java: if (x < 0)
        {
            // Java: return Double.NEGATIVE_INFINITY;
        }
        // Java: final double y = x / scale;
        // Java: if ((y <= minY) || (FastMath.log(y) >= maxLogY))
        {
            // Java: /*
             // Java: * Overflow.
             // Java: */
            // Java: final double aux1 = (y - shiftedShape) / shiftedShape;
            // Java: final double aux2 = shape * (FastMath.log1p(aux1) - aux1);
            // Java: final double aux3 = -y * (Gamma.LANCZOS_G + 0.5) / shiftedShape +
                    // Java: Gamma.LANCZOS_G + aux2;
            // Java: return logDensityPrefactor2 - FastMath.log(x) + aux3;
        }
        // Java: /*
         // Java: * Natural calculation.
         // Java: */
        // Java: return logDensityPrefactor1 - y + FastMath.log(y) * (shape - 1);
        todo!();
    }

    // Java: double cumulativeProbability(double x)
    pub fn cumulativeProbability(&self, x: f64) -> f64 {
        // Java: double ret;
        // Java: if (x <= 0)
        {
            // Java: ret = 0;
        }
        // Java: else
        {
            // Java: ret = Gamma.regularizedGammaP(shape, x / scale);
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
        // Java: return shape * scale;
        todo!();
    }

    // Java: double getNumericalVariance()
    pub fn getNumericalVariance(&self) -> f64 {
        // Java: return shape * scale * scale;
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
        // Java: if (shape < 1)
        {
            // Java: // [1]: p. 228, Algorithm GS
            // Java: while (true)
            {
                // Java: // Step 1:
                // Java: final double u = random.nextDouble();
                // Java: final double bGS = 1 + shape / FastMath.E;
                // Java: final double p = bGS * u;
                // Java: if (p <= 1)
                {
                    // Java: // Step 2:
                    // Java: final double x = FastMath.pow(p, 1 / shape);
                    // Java: final double u2 = random.nextDouble();
                    // Java: if (u2 > FastMath.exp(-x))
                    {
                        // Java: // Reject
                        // Java: continue;
                    }
                    // Java: else
                    {
                        // Java: return scale * x;
                    }
                }
                // Java: else
                {
                    // Java: // Step 3:
                    // Java: final double x = -1 * FastMath.log((bGS - p) / shape);
                    // Java: final double u2 = random.nextDouble();
                    // Java: if (u2 > FastMath.pow(x, shape - 1))
                    {
                        // Java: // Reject
                        // Java: continue;
                    }
                    // Java: else
                    {
                        // Java: return scale * x;
                    }
                }
            }
        }
        // Java: // Now shape >= 1
        // Java: final double d = shape - 0.333333333333333333;
        // Java: final double c = 1 / (3 * FastMath.sqrt(d));
        // Java: while (true)
        {
            // Java: final double x = random.nextGaussian();
            // Java: final double v = (1 + c * x) * (1 + c * x) * (1 + c * x);
            // Java: if (v <= 0)
            {
                // Java: continue;
            }
            // Java: final double x2 = x * x;
            // Java: final double u = random.nextDouble();
            // Java: // Squeeze
            // Java: if (u < 1 - 0.0331 * x2 * x2)
            {
                // Java: return scale * d * v;
            }
            // Java: if (FastMath.log(u) < 0.5 * x2 + d * (1 - v + FastMath.log(v)))
            {
                // Java: return scale * d * v;
            }
        }
        todo!();
    }

}
