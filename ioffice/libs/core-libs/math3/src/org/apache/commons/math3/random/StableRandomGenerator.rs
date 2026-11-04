// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java: public class provides
pub struct provides {
}

// Java: public class StableRandomGenerator
pub struct StableRandomGenerator {
    // Java field: RandomGenerator generator
    pub(crate) generator: Box<dyn std::any::Any>,
    // Java field: double alpha
    pub(crate) alpha: f64,
    // Java field: double beta
    pub(crate) beta: f64,
    // Java field: double zeta
    pub(crate) zeta: f64,
}

// Java: public class provides
impl provides {
}

// Java: public class StableRandomGenerator
impl StableRandomGenerator {
    // Java: StableRandomGenerator()
    pub fn StableRandomGenerator(&self) {
    }

    // Java: double nextNormalizedDouble()
    pub fn nextNormalizedDouble(&self) -> f64 {
        // Java: // we need 2 uniform random numbers to calculate omega and phi
        // Java: double omega = -FastMath.log(generator.nextDouble());
        // Java: double phi = FastMath.PI * (generator.nextDouble() - 0.5);
        // Java: // Normal distribution case (Box-Muller algorithm)
        // Java: if (alpha == 2d)
        {
            // Java: return FastMath.sqrt(2d * omega) * FastMath.sin(phi);
        }
        // Java: double x;
        // Java: // when beta = 0, zeta is zero as well
        // Java: // Thus we can exclude it from the formula
        // Java: if (beta == 0d)
        {
            // Java: // Cauchy distribution case
            // Java: if (alpha == 1d)
            {
                // Java: x = FastMath.tan(phi);
            }
            // Java: else
            {
                // Java: x = FastMath.pow(omega * FastMath.cos((1 - alpha) * phi),
                    // Java: 1d / alpha - 1d) *
                    // Java: FastMath.sin(alpha * phi) /
                    // Java: FastMath.pow(FastMath.cos(phi), 1d / alpha);
            }
        }
        // Java: else
        {
            // Java: // Generic stable distribution
            // Java: double cosPhi = FastMath.cos(phi);
            // Java: // to avoid rounding errors around alpha = 1
            // Java: if (FastMath.abs(alpha - 1d) > 1e-8)
            {
                // Java: double alphaPhi = alpha * phi;
                // Java: double invAlphaPhi = phi - alphaPhi;
                // Java: x = (FastMath.sin(alphaPhi) + zeta * FastMath.cos(alphaPhi)) / cosPhi *
                    // Java: (FastMath.cos(invAlphaPhi) + zeta * FastMath.sin(invAlphaPhi)) /
                     // Java: FastMath.pow(omega * cosPhi, (1 - alpha) / alpha);
            }
            // Java: else
            {
                // Java: double betaPhi = FastMath.PI / 2 + beta * phi;
                // Java: x = 2d / FastMath.PI * (betaPhi * FastMath.tan(phi) - beta *
                    // Java: FastMath.log(FastMath.PI / 2d * omega * cosPhi / betaPhi));
                // Java: if (alpha != 1d)
                {
                    // Java: x += beta * FastMath.tan(FastMath.PI * alpha / 2);
                }
            }
        }
        // Java: return x;
        todo!()
    }

}
