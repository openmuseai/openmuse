#![allow(unused_imports, dead_code, non_snake_case)]
// Java: import java.util.Arrays
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.NotFiniteNumberException
use crate::org::apache::commons::math3::exception::NotFiniteNumberException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.Localizable
use crate::org::apache::commons::math3::exception::util::Localizable;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: double TWO_PI
pub const TWO_PI: f64 = 2.0 * std::f64::consts::PI;
// Java field: double PI_SQUARED
pub const PI_SQUARED: f64 = std::f64::consts::PI * std::f64::consts::PI;

// Java: public class MathUtils
pub struct MathUtils;

// Java: public class MathUtils
impl MathUtils {
    // Java: MathUtils()
    pub fn new() -> Self { MathUtils }

    // Java: static int hash(double value)
    pub fn hash(value: f64) -> i32 {
        // Java: return new Double(value).hashCode();
        let bits = value.to_bits();
        (bits ^ (bits >> 32)) as i32
    }

    // Java: static boolean equals(double x, double y)
    pub fn equals(x: f64, y: f64) -> bool {
        // Java: return new Double(x).equals(new Double(y));
        x.to_bits() == y.to_bits()
    }

    // Java: static int hash(double[] value)
    pub fn hash_array(value: &[f64]) -> i32 {
        // Java: return Arrays.hashCode(value);
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for v in value {
            v.to_bits().hash(&mut hasher);
        }
        hasher.finish() as i32
    }

    // Java: static double normalizeAngle(double a, double center)
    pub fn normalizeAngle(a: f64, center: f64) -> f64 {
         // Java: return a - TWO_PI * FastMath.floor((a + FastMath.PI - center) / TWO_PI);
         a - TWO_PI * ((a + std::f64::consts::PI - center) / TWO_PI).floor()
    }

    // Java: static public static <T extends RealFieldElement<T>> T max(final T e1, final T e2)
    pub fn max_f64(e1: f64, e2: f64) -> f64 {
         // Java: return e1.subtract(e2).getReal() >= 0 ? e1 : e2;
         if e1 >= e2 { e1 } else { e2 }
    }

    // Java: static public static <T extends RealFieldElement<T>> T min(final T e1, final T e2)
    pub fn min_f64(e1: f64, e2: f64) -> f64 {
         // Java: return e1.subtract(e2).getReal() >= 0 ? e2 : e1;
         if e1 >= e2 { e2 } else { e1 }
    }

    // Java: static double reduce()
    pub fn reduce(a: f64, period: f64, offset: f64) -> f64 {
        let p = (a - offset).abs();
        p - period * (p / period).floor() + offset
    }

    // Java: static byte copySign(byte magnitude, byte sign)
    pub fn copySign_byte(magnitude: i8, sign: i8) -> i8 {
        if (magnitude >= 0 && sign >= 0) || (magnitude < 0 && sign < 0) { magnitude } else { -magnitude }
    }

    // Java: static short copySign(short magnitude, short sign)
    pub fn copySign_short(magnitude: i16, sign: i16) -> i16 {
        if (magnitude >= 0 && sign >= 0) || (magnitude < 0 && sign < 0) { magnitude } else { -magnitude }
    }

    // Java: static int copySign(int magnitude, int sign)
    pub fn copySign_int(magnitude: i32, sign: i32) -> i32 {
        if (magnitude >= 0 && sign >= 0) || (magnitude < 0 && sign < 0) { magnitude } else { -magnitude }
    }

    // Java: static long copySign(long magnitude, long sign)
    pub fn copySign_long(magnitude: i64, sign: i64) -> i64 {
        if (magnitude >= 0 && sign >= 0) || (magnitude < 0 && sign < 0) { magnitude } else { -magnitude }
    }

    // Java: static void checkFinite(final double x)
    pub fn checkFinite(x: f64) {
        if x.is_infinite() || x.is_nan() {
            panic!("Not a finite number: {}", x);
        }
    }

    // Java: static void checkFinite(final double[] val)
    pub fn checkFinite_array(val: &[f64]) {
        for &v in val {
            Self::checkFinite(v);
        }
    }

    // Java: static void checkNotNull()
    pub fn checkNotNull_default() {}

    // Java: static void checkNotNull(Object o)
    pub fn checkNotNull<T>(_o: &T) {}
}
