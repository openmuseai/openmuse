// Java: import java.util.ArrayList
// Java: import java.util.HashMap
// Java: import java.util.List
// Java: import java.util.Map
// Java: import org.apache.commons.math3.fraction.BigFraction
use crate::org::apache::commons::math3::fraction::BigFraction;
// Java: import org.apache.commons.math3.util.CombinatoricsUtils
use crate::org::apache::commons::math3::util::CombinatoricsUtils;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: List<BigFraction> CHEBYSHEV_COEFFICIENTS
static CHEBYSHEV_COEFFICIENTS: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: List<BigFraction> HERMITE_COEFFICIENTS
static HERMITE_COEFFICIENTS: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: List<BigFraction> LAGUERRE_COEFFICIENTS
static LAGUERRE_COEFFICIENTS: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: List<BigFraction> LEGENDRE_COEFFICIENTS
static LEGENDRE_COEFFICIENTS: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();
// Java field: Map<JacobiKey, List<BigFraction>> JACOBI_COEFFICIENTS
static JACOBI_COEFFICIENTS: std::sync::OnceLock<Box<dyn std::any::Any + Send + Sync>> = std::sync::OnceLock::new();

// Java: public interface RecurrenceCoefficientsGenerator
pub trait RecurrenceCoefficientsGenerator {
    // Java: BigFraction[] generate(int k)
    fn generate(&self, k: i32) -> Vec<Box<dyn std::any::Any>>;
}

// Java: public class PolynomialsUtils
pub struct PolynomialsUtils {
}

// Java: public class JacobiKey
pub struct JacobiKey {
    // Java field: int v
    pub(crate) v: i32,
    // Java field: int w
    pub(crate) w: i32,
}

// Java: public class PolynomialsUtils
impl PolynomialsUtils {
    // Java: PolynomialsUtils()
    pub fn PolynomialsUtils(&self) {
    }

    // Java: static PolynomialFunction createChebyshevPolynomial(final int degree)
    pub fn createChebyshevPolynomial(&self) -> Box<dyn std::any::Any> {
        // Java: return buildPolynomial(degree, CHEBYSHEV_COEFFICIENTS,
                // Java: new RecurrenceCoefficientsGenerator()
                {
            // Java: /** Fixed recurrence coefficients. */
            // Java: private final BigFraction[] coeffs = { BigFraction.ZERO, BigFraction.TWO, BigFraction.ONE };
            // Java: /** {@inheritDoc} */
            // Java: public BigFraction[] generate(int k)
            {
                // Java: return coeffs;
            }
                }
        todo!()
    }

    // Java: static PolynomialFunction createHermitePolynomial(final int degree)
    pub fn createHermitePolynomial(&self) -> Box<dyn std::any::Any> {
        // Java: return buildPolynomial(degree, HERMITE_COEFFICIENTS,
                // Java: new RecurrenceCoefficientsGenerator()
                {
            // Java: /** {@inheritDoc} */
            // Java: public BigFraction[] generate(int k)
            {
                // Java: return new BigFraction[]
                {
                        // Java: BigFraction.ZERO,
                        // Java: BigFraction.TWO,
                }
            }
                }
        todo!()
    }

    // Java: static PolynomialFunction createLaguerrePolynomial(final int degree)
    pub fn createLaguerrePolynomial(&self) -> Box<dyn std::any::Any> {
        // Java: return buildPolynomial(degree, LAGUERRE_COEFFICIENTS,
                // Java: new RecurrenceCoefficientsGenerator()
                {
            // Java: /** {@inheritDoc} */
            // Java: public BigFraction[] generate(int k)
            {
                // Java: final int kP1 = k + 1;
                // Java: return new BigFraction[]
                {
                        // Java: new BigFraction(2 * k + 1, kP1),
                        // Java: new BigFraction(-1, kP1),
                }
            }
                }
        todo!()
    }

    // Java: static PolynomialFunction createLegendrePolynomial(final int degree)
    pub fn createLegendrePolynomial(&self) -> Box<dyn std::any::Any> {
        // Java: return buildPolynomial(degree, LEGENDRE_COEFFICIENTS,
                               // Java: new RecurrenceCoefficientsGenerator()
                               {
            // Java: /** {@inheritDoc} */
            // Java: public BigFraction[] generate(int k)
            {
                // Java: final int kP1 = k + 1;
                // Java: return new BigFraction[]
                {
                        // Java: BigFraction.ZERO,
                        // Java: new BigFraction(k + kP1, kP1),
                }
            }
                               }
        todo!()
    }

    // Java: static PolynomialFunction createJacobiPolynomial(final int degree, final int v, final int w)
    pub fn createJacobiPolynomial(&self) -> Box<dyn std::any::Any> {
        // Java: // select the appropriate list
        // Java: final JacobiKey key = new JacobiKey(v, w);
        // Java: if (!JACOBI_COEFFICIENTS.containsKey(key))
        {
            // Java: // allocate a new list for v, w
            // Java: final List<BigFraction> list = new ArrayList<BigFraction>();
            // Java: JACOBI_COEFFICIENTS.put(key, list);
            // Java: // Pv,w,0(x) = 1;
            // Java: list.add(BigFraction.ONE);
            // Java: // P1(x) = (v - w) / 2 + (2 + v + w) * X / 2
            // Java: list.add(new BigFraction(v - w, 2));
            // Java: list.add(new BigFraction(2 + v + w, 2));
        }
        // Java: return buildPolynomial(degree, JACOBI_COEFFICIENTS.get(key),
                               // Java: new RecurrenceCoefficientsGenerator()
                               {
            // Java: /** {@inheritDoc} */
            // Java: public BigFraction[] generate(int k)
            {
                // Java: k++;
                // Java: final int kvw      = k + v + w;
                // Java: final int twoKvw   = kvw + k;
                // Java: final int twoKvwM1 = twoKvw - 1;
                // Java: final int twoKvwM2 = twoKvw - 2;
                // Java: final int den      = 2 * k *  kvw * twoKvwM2;
                // Java: return new BigFraction[]
                {
                    // Java: new BigFraction(twoKvwM1 * (v * v - w * w), den),
                    // Java: new BigFraction(twoKvwM1 * twoKvw * twoKvwM2, den),
                    // Java: new BigFraction(2 * (k + v - 1) * (k + w - 1) * twoKvw, den)
                }
            }
                               }
        todo!()
    }

    // Java: static double[] shift()
    pub fn shift(&self) -> Vec<f64> {
        todo!()
    }

    // Java: static PolynomialFunction buildPolynomial()
    pub fn buildPolynomial(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: static void computeUpToDegree()
    pub fn computeUpToDegree(&self) {
    }

}

// Java: public class JacobiKey
impl JacobiKey {
    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
            // Java: return (v << 16) ^ w;
        todo!()
    }

    // Java: boolean equals(final Object key)
    pub fn equals(&self) -> bool {
            // Java: if ((key == null) || !(key instanceof JacobiKey))
            {
                // Java: return false;
            }
            // Java: final JacobiKey otherK = (JacobiKey) key;
            // Java: return (v == otherK.v) && (w == otherK.w);
        todo!()
    }

}
