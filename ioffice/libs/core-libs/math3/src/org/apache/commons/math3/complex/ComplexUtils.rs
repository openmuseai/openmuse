// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

use super::Complex::Complex;

// Java: public class ComplexUtils
pub struct ComplexUtils {
}

// Java: public class ComplexUtils
impl ComplexUtils {
    // Java: ComplexUtils()
    pub fn ComplexUtils() -> Self {
        ComplexUtils {}
    }

    // Java: static Complex polar2Complex(double r, double theta)
    pub fn polar2Complex(r: f64, theta: f64) -> Complex {
        // Java: if (r < 0)
        if r < 0.0 {
            // Java: throw new MathIllegalArgumentException(
                  // Java: LocalizedFormats.NEGATIVE_COMPLEX_MODULE, r);
            panic!("MathIllegalArgumentException: NEGATIVE_COMPLEX_MODULE: {}", r);
        }
        // Java: return new Complex(r * FastMath.cos(theta), r * FastMath.sin(theta));
        Complex::new(r * theta.cos(), r * theta.sin())
    }

    // Java: static Complex[] convertToComplex(double[] real)
    pub fn convertToComplex(real: &[f64]) -> Vec<Complex> {
        // Java: final Complex c[] = new Complex[real.length];
        let mut c = Vec::with_capacity(real.len());
        // Java: for (int i = 0; i < real.length; i++)
        for i in 0..real.len() {
            // Java: c[i] = new Complex(real[i], 0);
            c.push(Complex::new(real[i], 0.0));
        }
        // Java: return c;
        c
    }
}
