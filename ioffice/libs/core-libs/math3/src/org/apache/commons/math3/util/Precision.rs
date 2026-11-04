// Java: import java.math.BigDecimal
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: double EPSILON
pub(crate) const EPSILON: f64 = f64::EPSILON;
// Java field: double SAFE_MIN
pub(crate) const SAFE_MIN: f64 = f64::MIN_POSITIVE;
// Java field: long EXPONENT_OFFSET
pub(crate) const EXPONENT_OFFSET: i64 = 1023;
// Java field: long SGN_MASK
pub(crate) const SGN_MASK: i64 = -0x8000000000000000i64;
// Java field: int SGN_MASK_FLOAT
pub(crate) const SGN_MASK_FLOAT: i32 = -0x80000000i32;
// Java field: double POSITIVE_ZERO
pub(crate) const POSITIVE_ZERO: f64 = 0.0f64;
// Java field: long POSITIVE_ZERO_DOUBLE_BITS
pub(crate) const POSITIVE_ZERO_DOUBLE_BITS: i64 = 0i64;
// Java field: long NEGATIVE_ZERO_DOUBLE_BITS
pub(crate) const NEGATIVE_ZERO_DOUBLE_BITS: i64 = -0x8000000000000000i64;
// Java field: int POSITIVE_ZERO_FLOAT_BITS
pub(crate) const POSITIVE_ZERO_FLOAT_BITS: i32 = 0i32;
// Java field: int NEGATIVE_ZERO_FLOAT_BITS
pub(crate) const NEGATIVE_ZERO_FLOAT_BITS: i32 = -0x80000000i32;

const ROUND_CEILING: i32 = 2;
const ROUND_DOWN: i32 = 1;
const ROUND_FLOOR: i32 = 3;
const ROUND_HALF_DOWN: i32 = 5;
const ROUND_HALF_EVEN: i32 = 6;
const ROUND_HALF_UP: i32 = 4;
const ROUND_UNNECESSARY: i32 = 7;
const ROUND_UP: i32 = 0;

// Java: public class Precision
pub struct Precision {
}

// Java: public class Precision
impl Precision {
    // Java: Precision()
    pub fn Precision() -> Self {
        Precision {}
    }

    // Java: static int compareTo(double x, double y, double eps)
    pub fn compareTo(x: f64, y: f64, eps: f64) -> i32 {
        // Java: if (equals(x, y, eps))
        if Self::equals_f64_eps(x, y, eps) {
            // Java: return 0;
            return 0;
        }
        // Java: else if (x < y)
        else if x < y {
            // Java: return -1;
            return -1;
        }
        // Java: return 1;
        1
    }

    // Java: static int compareTo(final double x, final double y, final int maxUlps)
    pub fn compareTo_ulps(x: f64, y: f64, max_ulps: i32) -> i32 {
        // Java: if (equals(x, y, maxUlps))
        if Self::equals_f64_ulps(x, y, max_ulps) {
            // Java: return 0;
            return 0;
        }
        // Java: else if (x < y)
        else if x < y {
            // Java: return -1;
            return -1;
        }
        // Java: return 1;
        1
    }

    // Java: static boolean equals(float x, float y)
    pub fn equals_f32(x: f32, y: f32) -> bool {
        // Java: return equals(x, y, 1);
        Self::equals_f32_ulps(x, y, 1)
    }

    // Java: static boolean equalsIncludingNaN(float x, float y)
    pub fn equalsIncludingNaN_f32(x: f32, y: f32) -> bool {
        // Java: return (x != x || y != y) ? !(x != x ^ y != y) : equals(x, y, 1);
        if x.is_nan() || y.is_nan() { !(x.is_nan() ^ y.is_nan()) } else { Self::equals_f32_ulps(x, y, 1) }
    }

    // Java: static boolean equals(float x, float y, float eps)
    pub fn equals_f32_eps(x: f32, y: f32, eps: f32) -> bool {
        // Java: return equals(x, y, 1) || FastMath.abs(y - x) <= eps;
        Self::equals_f32_ulps(x, y, 1) || (y - x).abs() <= eps
    }

    // Java: static boolean equalsIncludingNaN(float x, float y, float eps)
    pub fn equalsIncludingNaN_f32_eps(x: f32, y: f32, eps: f32) -> bool {
        // Java: return equalsIncludingNaN(x, y) || (FastMath.abs(y - x) <= eps);
        Self::equalsIncludingNaN_f32(x, y) || (y - x).abs() <= eps
    }

    // Java: static boolean equals(final float x, final float y, final int maxUlps)
    pub fn equals_f32_ulps(x: f32, y: f32, max_ulps: i32) -> bool {
        // Java: final int xInt = Float.floatToRawIntBits(x);
        let x_int = x.to_bits() as i32;
        // Java: final int yInt = Float.floatToRawIntBits(y);
        let y_int = y.to_bits() as i32;
        // Java: final boolean isEqual;
        let is_equal: bool;
        // Java: if (((xInt ^ yInt) & SGN_MASK_FLOAT) == 0)
        if ((x_int ^ y_int) & SGN_MASK_FLOAT) == 0 {
            // Java: // number have same sign, there is no risk of overflow
            // Java: isEqual = FastMath.abs(xInt - yInt) <= maxUlps;
            is_equal = (x_int - y_int).abs() <= max_ulps;
        }
        // Java: else
        else {
            // Java: // number have opposite signs, take care of overflow
            // Java: final int deltaPlus;
            let delta_plus: i32;
            // Java: final int deltaMinus;
            let delta_minus: i32;
            // Java: if (xInt < yInt)
            if x_int < y_int {
                // Java: deltaPlus  = yInt - POSITIVE_ZERO_FLOAT_BITS;
                delta_plus = y_int - POSITIVE_ZERO_FLOAT_BITS;
                // Java: deltaMinus = xInt - NEGATIVE_ZERO_FLOAT_BITS;
                delta_minus = x_int - NEGATIVE_ZERO_FLOAT_BITS;
            }
            // Java: else
            else {
                // Java: deltaPlus  = xInt - POSITIVE_ZERO_FLOAT_BITS;
                delta_plus = x_int - POSITIVE_ZERO_FLOAT_BITS;
                // Java: deltaMinus = yInt - NEGATIVE_ZERO_FLOAT_BITS;
                delta_minus = y_int - NEGATIVE_ZERO_FLOAT_BITS;
            }
            // Java: if (deltaPlus > maxUlps)
            if delta_plus > max_ulps {
                // Java: isEqual = false;
                is_equal = false;
            }
            // Java: else
            else {
                // Java: isEqual = deltaMinus <= (maxUlps - deltaPlus);
                is_equal = delta_minus <= (max_ulps - delta_plus);
            }
        }
        // Java: return isEqual && !Float.isNaN(x) && !Float.isNaN(y);
        is_equal && !x.is_nan() && !y.is_nan()
    }

    // Java: static boolean equalsIncludingNaN(float x, float y, int maxUlps)
    pub fn equalsIncludingNaN_f32_ulps(x: f32, y: f32, max_ulps: i32) -> bool {
        // Java: return (x != x || y != y) ? !(x != x ^ y != y) : equals(x, y, maxUlps);
        if x.is_nan() || y.is_nan() { !(x.is_nan() ^ y.is_nan()) } else { Self::equals_f32_ulps(x, y, max_ulps) }
    }

    // Java: static boolean equals(double x, double y)
    pub fn equals_f64(x: f64, y: f64) -> bool {
        // Java: return equals(x, y, 1);
        Self::equals_f64_ulps(x, y, 1)
    }

    // Java: static boolean equalsIncludingNaN(double x, double y)
    pub fn equalsIncludingNaN_f64(x: f64, y: f64) -> bool {
        // Java: return (x != x || y != y) ? !(x != x ^ y != y) : equals(x, y, 1);
        if x.is_nan() || y.is_nan() { !(x.is_nan() ^ y.is_nan()) } else { Self::equals_f64_ulps(x, y, 1) }
    }

    // Java: static boolean equals(double x, double y, double eps)
    pub fn equals_f64_eps(x: f64, y: f64, eps: f64) -> bool {
        // Java: return equals(x, y, 1) || FastMath.abs(y - x) <= eps;
        Self::equals_f64_ulps(x, y, 1) || (y - x).abs() <= eps
    }

    // Java: static boolean equalsWithRelativeTolerance(double x, double y, double eps)
    pub fn equalsWithRelativeTolerance(x: f64, y: f64, eps: f64) -> bool {
        // Java: if (equals(x, y, 1))
        if Self::equals_f64_ulps(x, y, 1) {
            // Java: return true;
            return true;
        }
        // Java: final double absoluteMax = FastMath.max(FastMath.abs(x), FastMath.abs(y));
        let absolute_max = x.abs().max(y.abs());
        // Java: final double relativeDifference = FastMath.abs((x - y) / absoluteMax);
        let relative_difference = ((x - y) / absolute_max).abs();
        // Java: return relativeDifference <= eps;
        relative_difference <= eps
    }

    // Java: static boolean equalsIncludingNaN(double x, double y, double eps)
    pub fn equalsIncludingNaN_f64_eps(x: f64, y: f64, eps: f64) -> bool {
        // Java: return equalsIncludingNaN(x, y) || (FastMath.abs(y - x) <= eps);
        Self::equalsIncludingNaN_f64(x, y) || (y - x).abs() <= eps
    }

    // Java: static boolean equals(final double x, final double y, final int maxUlps)
    pub fn equals_f64_ulps(x: f64, y: f64, max_ulps: i32) -> bool {
        // Java: final long xInt = Double.doubleToRawLongBits(x);
        let x_int = x.to_bits() as i64;
        // Java: final long yInt = Double.doubleToRawLongBits(y);
        let y_int = y.to_bits() as i64;
        // Java: final boolean isEqual;
        let is_equal: bool;
        // Java: if (((xInt ^ yInt) & SGN_MASK) == 0l)
        if ((x_int ^ y_int) & SGN_MASK) == 0i64 {
            // Java: // number have same sign, there is no risk of overflow
            // Java: isEqual = FastMath.abs(xInt - yInt) <= maxUlps;
            is_equal = (x_int - y_int).abs() <= max_ulps as i64;
        }
        // Java: else
        else {
            // Java: // number have opposite signs, take care of overflow
            // Java: final long deltaPlus;
            let delta_plus: i64;
            // Java: final long deltaMinus;
            let delta_minus: i64;
            // Java: if (xInt < yInt)
            if x_int < y_int {
                // Java: deltaPlus  = yInt - POSITIVE_ZERO_DOUBLE_BITS;
                delta_plus = y_int - POSITIVE_ZERO_DOUBLE_BITS;
                // Java: deltaMinus = xInt - NEGATIVE_ZERO_DOUBLE_BITS;
                delta_minus = x_int - NEGATIVE_ZERO_DOUBLE_BITS;
            }
            // Java: else
            else {
                // Java: deltaPlus  = xInt - POSITIVE_ZERO_DOUBLE_BITS;
                delta_plus = x_int - POSITIVE_ZERO_DOUBLE_BITS;
                // Java: deltaMinus = yInt - NEGATIVE_ZERO_DOUBLE_BITS;
                delta_minus = y_int - NEGATIVE_ZERO_DOUBLE_BITS;
            }
            // Java: if (deltaPlus > maxUlps)
            if delta_plus > max_ulps as i64 {
                // Java: isEqual = false;
                is_equal = false;
            }
            // Java: else
            else {
                // Java: isEqual = deltaMinus <= (maxUlps - deltaPlus);
                is_equal = delta_minus <= (max_ulps as i64 - delta_plus);
            }
        }
        // Java: return isEqual && !Double.isNaN(x) && !Double.isNaN(y);
        is_equal && !x.is_nan() && !y.is_nan()
    }

    // Java: static boolean equalsIncludingNaN(double x, double y, int maxUlps)
    pub fn equalsIncludingNaN_f64_ulps(x: f64, y: f64, max_ulps: i32) -> bool {
        // Java: return (x != x || y != y) ? !(x != x ^ y != y) : equals(x, y, maxUlps);
        if x.is_nan() || y.is_nan() { !(x.is_nan() ^ y.is_nan()) } else { Self::equals_f64_ulps(x, y, max_ulps) }
    }

    // Java: static double round(double x, int scale)
    pub fn round_f64(x: f64, scale: i32) -> f64 {
        // Java: return round(x, scale, BigDecimal.ROUND_HALF_UP);
        Self::round_f64_method(x, scale, ROUND_HALF_UP)
    }

    // Java: static double round(double x, int scale, int roundingMethod)
    pub fn round_f64_method(x: f64, scale: i32, rounding_method: i32) -> f64 {
        // Java: try
        if !x.is_infinite() && !x.is_nan() {
            // Java: final double rounded = (new BigDecimal(Double.toString(x))
                   // Java: .setScale(scale, roundingMethod))
                   // Java: .doubleValue();
            let factor = 10f64.powi(scale);
            let shifted = x * factor;
            let rounded_val = Self::roundUnscaled(shifted.abs(), shifted.signum(), rounding_method);
            let rounded = rounded_val * shifted.signum() / factor;
            // Java: // MATH-1089: negative values rounded to zero should result in negative zero
            // Java: return rounded == POSITIVE_ZERO ? POSITIVE_ZERO * x : rounded;
            if rounded == POSITIVE_ZERO { POSITIVE_ZERO * x } else { rounded }
        }
        // Java: catch (NumberFormatException ex)
        else {
            // Java: if (Double.isInfinite(x))
            if x.is_infinite() {
                // Java: return x;
                x
            }
            // Java: else
            else {
                // Java: return Double.NaN;
                f64::NAN
            }
        }
    }

    // Java: static float round(float x, int scale)
    pub fn round_f32(x: f32, scale: i32) -> f32 {
        // Java: return round(x, scale, BigDecimal.ROUND_HALF_UP);
        Self::round_f32_method(x, scale, ROUND_HALF_UP)
    }

    // Java: static float round(float x, int scale, int roundingMethod)
    pub fn round_f32_method(x: f32, scale: i32, rounding_method: i32) -> f32 {
        let sign = x.signum();
        let factor = (10.0f32).powi(scale) * sign;
        (Self::roundUnscaled((x * factor) as f64, sign as f64, rounding_method) as f32) / factor
    }

    // Java: static double roundUnscaled()
    pub fn roundUnscaled(mut unscaled: f64, sign: f64, rounding_method: i32) -> f64 {
        match rounding_method {
            ROUND_CEILING => {
                if sign == -1.0 {
                    unscaled = (unscaled - f64::EPSILON).floor();
                } else {
                    unscaled = (unscaled + f64::EPSILON).ceil();
                }
            }
            ROUND_DOWN => {
                unscaled = (unscaled - f64::EPSILON).floor();
            }
            ROUND_FLOOR => {
                if sign == -1.0 {
                    unscaled = (unscaled + f64::EPSILON).ceil();
                } else {
                    unscaled = (unscaled - f64::EPSILON).floor();
                }
            }
            ROUND_HALF_DOWN => {
                unscaled = unscaled - f64::EPSILON;
                let fraction = unscaled - unscaled.floor();
                if fraction > 0.5 {
                    unscaled = unscaled.ceil();
                } else {
                    unscaled = unscaled.floor();
                }
            }
            ROUND_HALF_EVEN => {
                let fraction = unscaled - unscaled.floor();
                if fraction > 0.5 {
                    unscaled = unscaled.ceil();
                } else if fraction < 0.5 {
                    unscaled = unscaled.floor();
                } else {
                    if unscaled.floor() / 2.0 == (unscaled.floor() / 2.0).floor() {
                        unscaled = unscaled.floor();
                    } else {
                        unscaled = unscaled.ceil();
                    }
                }
            }
            ROUND_HALF_UP => {
                unscaled = unscaled + f64::EPSILON;
                let fraction = unscaled - unscaled.floor();
                if fraction >= 0.5 {
                    unscaled = unscaled.ceil();
                } else {
                    unscaled = unscaled.floor();
                }
            }
            ROUND_UNNECESSARY => {
                if unscaled != unscaled.floor() {
                    todo!("MathArithmeticException");
                }
            }
            ROUND_UP => {
                if unscaled != unscaled.floor() {
                    unscaled = (unscaled + f64::EPSILON).ceil();
                }
            }
            _ => {
                todo!("MathIllegalArgumentException");
            }
        }
        unscaled
    }

    // Java: static double representableDelta()
    pub fn representableDelta(x: f64, original_delta: f64) -> f64 {
        x + original_delta - x
    }
}
