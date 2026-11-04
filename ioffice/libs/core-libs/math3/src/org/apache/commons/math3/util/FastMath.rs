#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.io.PrintStream
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: double PI
pub const PI: f64 = std::f64::consts::PI;
// Java field: double E
pub const E: f64 = std::f64::consts::E;
// Java field: int EXP_INT_TABLE_MAX_INDEX
pub(crate) const EXP_INT_TABLE_MAX_INDEX: i32 = 750;
// Java field: int EXP_INT_TABLE_LEN
pub(crate) const EXP_INT_TABLE_LEN: i32 = 1500;
// Java field: int LN_MANT_LEN
pub(crate) const LN_MANT_LEN: i32 = 1024;
// Java field: int EXP_FRAC_TABLE_LEN
pub(crate) const EXP_FRAC_TABLE_LEN: i32 = 1025;
// Java field: double LOG_MAX_VALUE
pub(crate) const LOG_MAX_VALUE: f64 = 709.782712893384;
// Java field: boolean RECOMPUTE_TABLES_AT_RUNTIME
pub(crate) const RECOMPUTE_TABLES_AT_RUNTIME: bool = false;
// Java field: double LN_2_A
pub(crate) const LN_2_A: f64 = 0.6931471805599453;
// Java field: double LN_2_B
pub(crate) const LN_2_B: f64 = 1.4901120858556236e-17;
// Java field: int SINE_TABLE_LEN
pub(crate) const SINE_TABLE_LEN: i32 = 14;
// Java field: long HEX_40000000
pub(crate) const HEX_40000000: i64 = 0x40000000;
// Java field: long MASK_30BITS
pub(crate) const MASK_30BITS: i64 = -1073741824;
// Java field: int MASK_NON_SIGN_INT
pub(crate) const MASK_NON_SIGN_INT: i32 = 0x7fffffff;
// Java field: long MASK_NON_SIGN_LONG
pub(crate) const MASK_NON_SIGN_LONG: i64 = 0x7fffffffffffffff;
// Java field: long MASK_DOUBLE_EXPONENT
pub(crate) const MASK_DOUBLE_EXPONENT: i64 = 0x7ff0000000000000;
// Java field: long MASK_DOUBLE_MANTISSA
pub(crate) const MASK_DOUBLE_MANTISSA: i64 = 0x000fffffffffffff;
// Java field: long IMPLICIT_HIGH_BIT
pub(crate) const IMPLICIT_HIGH_BIT: i64 = 0x0010000000000000;
// Java field: double TWO_POWER_52
pub(crate) const TWO_POWER_52: f64 = 4503599627370496.0;
// Java field: double F_1_3
pub(crate) const F_1_3: f64 = 1.0 / 3.0;
// Java field: double F_1_5
pub(crate) const F_1_5: f64 = 1.0 / 5.0;
// Java field: double F_1_7
pub(crate) const F_1_7: f64 = 1.0 / 7.0;
// Java field: double F_1_9
pub(crate) const F_1_9: f64 = 1.0 / 9.0;
// Java field: double F_1_11
pub(crate) const F_1_11: f64 = 1.0 / 11.0;
// Java field: double F_1_13
pub(crate) const F_1_13: f64 = 1.0 / 13.0;
// Java field: double F_1_15
pub(crate) const F_1_15: f64 = 1.0 / 15.0;
// Java field: double F_1_17
pub(crate) const F_1_17: f64 = 1.0 / 17.0;
// Java field: double F_3_4
pub(crate) const F_3_4: f64 = 3.0 / 4.0;
// Java field: double F_15_16
pub(crate) const F_15_16: f64 = 15.0 / 16.0;
// Java field: double F_13_14
pub(crate) const F_13_14: f64 = 13.0 / 14.0;
// Java field: double F_11_12
pub(crate) const F_11_12: f64 = 11.0 / 12.0;
// Java field: double F_9_10
pub(crate) const F_9_10: f64 = 9.0 / 10.0;
// Java field: double F_7_8
pub(crate) const F_7_8: f64 = 7.0 / 8.0;
// Java field: double F_5_6
pub(crate) const F_5_6: f64 = 5.0 / 6.0;
// Java field: double F_1_2
pub(crate) const F_1_2: f64 = 1.0 / 2.0;
// Java field: double F_1_4
pub(crate) const F_1_4: f64 = 1.0 / 4.0;

// Java field: Split NAN
// Java field: Split POSITIVE_INFINITY
// Java field: Split NEGATIVE_INFINITY
// Java field: double[] EXP_INT_TABLE_A
// Java field: double[] EXP_INT_TABLE_B
// Java field: double[] EXP_FRAC_TABLE_A
// Java field: double[] EXP_FRAC_TABLE_B
// Java field: double[][] LN_MANT

// Java: public class FastMath
pub struct FastMath;

// Java: public class Split
pub struct Split {
    // Java field: double full
    pub(crate) full: f64,
    // Java field: double high
    pub(crate) high: f64,
    // Java field: double low
    pub(crate) low: f64,
}

// Java: public class ExpIntTable
pub struct ExpIntTable;

// Java: public class ExpFracTable
pub struct ExpFracTable;

// Java: public class lnMant
pub struct lnMant;

// Java: public class CodyWaite
pub struct CodyWaite {
    // Java field: int finalK
    pub(crate) finalK: i32,
    // Java field: double finalRemA
    pub(crate) finalRemA: f64,
    // Java field: double finalRemB
    pub(crate) finalRemB: f64,
}

// Java: public class FastMath
impl FastMath {
    // Java: FastMath()
    pub fn new() -> Self { FastMath }

    // Java: static double doubleHighPart(double d)
    pub fn doubleHighPart(d: f64) -> f64 {
        let bits = d.to_bits();
        let cleared = bits & 0xFFFFFFFFC0000000u64;
        f64::from_bits(cleared)
    }

    // Java: static double sqrt(final double a)
    pub fn sqrt(a: f64) -> f64 { a.sqrt() }

    // Java: static double cosh(double x)
    pub fn cosh(x: f64) -> f64 { x.cosh() }

    // Java: static double sinh(double x)
    pub fn sinh(x: f64) -> f64 { x.sinh() }

    // Java: static double tanh(double x)
    pub fn tanh(x: f64) -> f64 { x.tanh() }

    // Java: static double acosh(final double a)
    pub fn acosh(a: f64) -> f64 { a.acosh() }

    // Java: static double asinh(double a)
    pub fn asinh(a: f64) -> f64 { a.asinh() }

    // Java: static double atanh(double a)
    pub fn atanh(a: f64) -> f64 { a.atanh() }

    // Java: static double signum(final double a)
    pub fn signum(a: f64) -> f64 { a.signum() }

    // Java: static float signum(final float a)
    pub fn signum_float(a: f32) -> f32 { a.signum() }

    // Java: static double nextUp(final double a)
    pub fn nextUp(a: f64) -> f64 {
        if a.is_nan() || a == f64::INFINITY { return a; }
        if a == 0.0 { return f64::MIN_POSITIVE; }
        if a > 0.0 { f64::from_bits(a.to_bits() + 1) } else { f64::from_bits(a.to_bits() - 1) }
    }

    // Java: static float nextUp(final float a)
    pub fn nextUp_float(a: f32) -> f32 {
        if a.is_nan() || a == f32::INFINITY { return a; }
        if a == 0.0 { return f32::MIN_POSITIVE; }
        if a > 0.0 { f32::from_bits(a.to_bits() + 1) } else { f32::from_bits(a.to_bits() - 1) }
    }

    // Java: static double nextDown(final double a)
    pub fn nextDown(a: f64) -> f64 { -Self::nextUp(-a) }

    // Java: static float nextDown(final float a)
    pub fn nextDown_float(a: f32) -> f32 { -Self::nextUp_float(-a) }

    // Java: static double random()
    pub fn random() -> f64 {
        // Simplified: use a basic approach
        use std::time::SystemTime;
        let t = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap_or_default().subsec_nanos();
        (t as f64) / 4_294_967_296.0
    }

    // Java: static double exp(double x)
    pub fn exp(x: f64) -> f64 { x.exp() }

    // Java: static double exp(double x, double extra, double[] hiPrec)
    pub fn exp_hiprec(x: f64, extra: f64, hi_prec: &mut [f64]) -> f64 {
        let result = (x + extra).exp();
        if hi_prec.len() >= 2 { hi_prec[0] = result; hi_prec[1] = 0.0; }
        result
    }

    // Java: static double expm1(double x)
    pub fn expm1(x: f64) -> f64 { x.exp_m1() }

    // Java: static double expm1(double x, double[] hiPrecOut)
    pub fn expm1_hiprec(x: f64, hi_prec_out: &mut [f64]) -> f64 {
        let result = x.exp_m1();
        if hi_prec_out.len() >= 2 { hi_prec_out[0] = result; hi_prec_out[1] = 0.0; }
        result
    }

    // Java: static double log(final double x)
    pub fn log(x: f64) -> f64 { x.ln() }

    // Java: static double log(final double x, final double[] hiPrec)
    pub fn log_hiprec(x: f64, hi_prec: &mut [f64]) -> f64 {
        let result = x.ln();
        if hi_prec.len() >= 2 { hi_prec[0] = result; hi_prec[1] = 0.0; }
        result
    }

    // Java: static double log1p(final double x)
    pub fn log1p(x: f64) -> f64 { x.ln_1p() }

    // Java: static double log10(final double x)
    pub fn log10(x: f64) -> f64 { x.log10() }

    // Java: static double log(double base, double x)
    pub fn log_base(base: f64, x: f64) -> f64 { x.ln() / base.ln() }

    // Java: static double pow(final double x, final double y)
    pub fn pow(x: f64, y: f64) -> f64 { x.powf(y) }

    // Java: static double pow(double d, int e)
    pub fn pow_int(d: f64, e: i32) -> f64 { d.powi(e) }

    // Java: static double pow(double d, long e)
    pub fn pow_long(d: f64, e: i64) -> f64 { d.powf(e as f64) }

    // Java: static double polySine(final double x)
    fn polySine(x: f64) -> f64 {
        let x2 = x * x;
        let mut p = 2.7553817452272217E-6;
        p = p * x2 + (-1.9841269659586505E-4);
        p = p * x2 + 0.008333333333329196;
        p = p * x2 + (-0.16666666666666666);
        p * x2 * x
    }

    // Java: static double polyCosine(double x)
    fn polyCosine(x: f64) -> f64 {
        let x2 = x * x;
        let mut p = 2.479773539153719E-5;
        p = p * x2 + (-0.0013888888889764455);
        p = p * x2 + 0.04166666666666666;
        p = p * x2 + (-0.5);
        p * x2
    }

    // Java: static double sinQ(double xa, double xb)
    fn sinQ(xa: f64, xb: f64) -> f64 { (xa + xb).sin() }

    // Java: static double cosQ(double xa, double xb)
    fn cosQ(xa: f64, xb: f64) -> f64 { (xa + xb).cos() }

    // Java: static double tanQ(double xa, double xb, boolean cotanFlag)
    fn tanQ(xa: f64, xb: f64, cotan_flag: bool) -> f64 {
        if cotan_flag { 1.0 / (xa + xb).tan() } else { (xa + xb).tan() }
    }

    // Java: static void reducePayneHanek(double x, double[] result)
    fn reducePayneHanek(_x: f64, _result: &mut [f64]) {}

    // Java: static double sin(double x)
    pub fn sin(x: f64) -> f64 { x.sin() }

    // Java: static double cos(double x)
    pub fn cos(x: f64) -> f64 { x.cos() }

    // Java: static double tan(double x)
    pub fn tan(x: f64) -> f64 { x.tan() }

    // Java: static double atan(double x)
    pub fn atan(x: f64) -> f64 { x.atan() }

    // Java: static double atan(double xa, double xb, boolean leftPlane)
    fn atan_hiprec(xa: f64, xb: f64, _left_plane: bool) -> f64 { (xa + xb).atan() }

    // Java: static double atan2(double y, double x)
    pub fn atan2(y: f64, x: f64) -> f64 { y.atan2(x) }

    // Java: static double asin(double x)
    pub fn asin(x: f64) -> f64 { x.asin() }

    // Java: static double acos(double x)
    pub fn acos(x: f64) -> f64 { x.acos() }

    // Java: static double cbrt(double x)
    pub fn cbrt(x: f64) -> f64 { x.cbrt() }

    // Java: static double toRadians(double x)
    pub fn toRadians(x: f64) -> f64 { x.to_radians() }

    // Java: static double toDegrees(double x)
    pub fn toDegrees(x: f64) -> f64 { x.to_degrees() }

    // Java: static int abs(final int x)
    pub fn abs_int(x: i32) -> i32 { x.abs() }

    // Java: static long abs(final long x)
    pub fn abs_long(x: i64) -> i64 { x.abs() }

    // Java: static float abs(final float x)
    pub fn abs_float(x: f32) -> f32 { x.abs() }

    // Java: static double abs(double x)
    pub fn abs(x: f64) -> f64 { x.abs() }

    // Java: static double ulp(double x)
    pub fn ulp(x: f64) -> f64 {
        if x.is_infinite() { return f64::INFINITY; }
        if x.is_nan() { return f64::NAN; }
        let ax = x.abs();
        let next = f64::from_bits(ax.to_bits() + 1);
        next - ax
    }

    // Java: static float ulp(final float x)
    pub fn ulp_float(x: f32) -> f32 {
        if x.is_infinite() { return f32::INFINITY; }
        if x.is_nan() { return f32::NAN; }
        let ax = x.abs();
        let next = f32::from_bits(ax.to_bits() + 1);
        next - ax
    }

    // Java: static double scalb(final double d, final int n)
    pub fn scalb(d: f64, n: i32) -> f64 { d * (2.0f64).powi(n) }

    // Java: static float scalb(final float f, final int n)
    pub fn scalb_float(f: f32, n: i32) -> f32 { f * (2.0f32).powi(n) }

    // Java: static double nextAfter(double d, double direction)
    pub fn nextAfter(d: f64, direction: f64) -> f64 {
        if d.is_nan() || direction.is_nan() { return f64::NAN; }
        if d == direction { return direction; }
        if direction > d { Self::nextUp(d) } else { -Self::nextUp(-d) }
    }

    // Java: static float nextAfter(final float f, final double direction)
    pub fn nextAfter_float(f: f32, direction: f64) -> f32 {
        if f.is_nan() || direction.is_nan() { return f32::NAN; }
        if (f as f64) == direction { return f; }
        if direction > f as f64 { Self::nextUp_float(f) } else { -Self::nextUp_float(-f) }
    }

    // Java: static double floor(double x)
    pub fn floor(x: f64) -> f64 { x.floor() }

    // Java: static double ceil(double x)
    pub fn ceil(x: f64) -> f64 { x.ceil() }

    // Java: static double rint(double x)
    pub fn rint(x: f64) -> f64 { x.round() }

    // Java: static long round(double x)
    pub fn round(x: f64) -> i64 { x.round() as i64 }

    // Java: static int round(final float x)
    pub fn round_float(x: f32) -> i32 { x.round() as i32 }

    // Java: static int min(final int a, final int b)
    pub fn min_int(a: i32, b: i32) -> i32 { a.min(b) }

    // Java: static long min(final long a, final long b)
    pub fn min_long(a: i64, b: i64) -> i64 { a.min(b) }

    // Java: static float min(final float a, final float b)
    pub fn min_float(a: f32, b: f32) -> f32 { a.min(b) }

    // Java: static double min(final double a, final double b)
    pub fn min(a: f64, b: f64) -> f64 { a.min(b) }

    // Java: static int max(final int a, final int b)
    pub fn max_int(a: i32, b: i32) -> i32 { a.max(b) }

    // Java: static long max(final long a, final long b)
    pub fn max_long(a: i64, b: i64) -> i64 { a.max(b) }

    // Java: static float max(final float a, final float b)
    pub fn max_float(a: f32, b: f32) -> f32 { a.max(b) }

    // Java: static double max(final double a, final double b)
    pub fn max(a: f64, b: f64) -> f64 { a.max(b) }

    // Java: static double hypot(final double x, final double y)
    pub fn hypot(x: f64, y: f64) -> f64 { x.hypot(y) }

    // Java: static double IEEEremainder(double dividend, double divisor)
    pub fn IEEEremainder(dividend: f64, divisor: f64) -> f64 {
        dividend - (dividend / divisor).round() * divisor
    }

    // Java: static long toIntExact(final int n)
    pub fn toIntExact(n: i64) -> i32 { n as i32 }

    // Java: static int incrementExact(final int n)
    pub fn incrementExact_int(n: i32) -> i32 { n.checked_add(1).unwrap() }

    // Java: static long incrementExact(final long n)
    pub fn incrementExact_long(n: i64) -> i64 { n.checked_add(1).unwrap() }

    // Java: static int decrementExact(final int n)
    pub fn decrementExact_int(n: i32) -> i32 { n.checked_sub(1).unwrap() }

    // Java: static long decrementExact(final long n)
    pub fn decrementExact_long(n: i64) -> i64 { n.checked_sub(1).unwrap() }

    // Java: static int addExact(final int a, final int b)
    pub fn addExact_int(a: i32, b: i32) -> i32 { a.checked_add(b).unwrap() }

    // Java: static long addExact(final long a, final long b)
    pub fn addExact_long(a: i64, b: i64) -> i64 { a.checked_add(b).unwrap() }

    // Java: static int subtractExact(final int a, final int b)
    pub fn subtractExact_int(a: i32, b: i32) -> i32 { a.checked_sub(b).unwrap() }

    // Java: static long subtractExact(final long a, final long b)
    pub fn subtractExact_long(a: i64, b: i64) -> i64 { a.checked_sub(b).unwrap() }

    // Java: static int multiplyExact(final int a, final int b)
    pub fn multiplyExact_int(a: i32, b: i32) -> i32 { a.checked_mul(b).unwrap() }

    // Java: static long multiplyExact(final long a, final long b)
    pub fn multiplyExact_long(a: i64, b: i64) -> i64 { a.checked_mul(b).unwrap() }

    // Java: static int floorDiv(final int a, final int b)
    pub fn floorDiv_int(a: i32, b: i32) -> i32 { a.div_euclid(b) }

    // Java: static long floorDiv(final long a, final long b)
    pub fn floorDiv_long(a: i64, b: i64) -> i64 { a.div_euclid(b) }

    // Java: static int floorMod(final int a, final int b)
    pub fn floorMod_int(a: i32, b: i32) -> i32 { a.rem_euclid(b) }

    // Java: static long floorMod(final long a, final long b)
    pub fn floorMod_long(a: i64, b: i64) -> i64 { a.rem_euclid(b) }

    // Java: static double copySign(double magnitude, double sign)
    pub fn copySign(magnitude: f64, sign: f64) -> f64 { magnitude.copysign(sign) }

    // Java: static float copySign(float magnitude, float sign)
    pub fn copySign_float(magnitude: f32, sign: f32) -> f32 { magnitude.copysign(sign) }

    // Java: static int getExponent(final double d)
    pub fn getExponent(d: f64) -> i32 {
        ((d.to_bits() >> 52) & 0x7ff) as i32 - 1023
    }

    // Java: static int getExponent(final float f)
    pub fn getExponent_float(f: f32) -> i32 {
        ((f.to_bits() >> 23) & 0xff) as i32 - 127
    }

    // Java: static void main(String[] a)
    pub fn main(_a: &[String]) {}
}

// Java: public class Split
impl Split {
    // Java: Split(final double x)
    pub fn new(x: f64) -> Self {
        let high = Self::high_part(x);
        Split { full: x, high, low: x - high }
    }

    // Java: Split(final double high, final double low)
    pub fn new_parts(high: f64, low: f64) -> Self {
        Split { full: high + low, high, low }
    }

    fn high_part(x: f64) -> f64 {
        let bits = x.to_bits();
        let cleared = bits & 0xFFFFFFFFC0000000u64;
        f64::from_bits(cleared)
    }

    // Java: Split multiply(final Split b)
    pub fn multiply(&self, b: &Split) -> Split {
        Split::new(self.full * b.full)
    }

    // Java: Split reciprocal()
    pub fn reciprocal(&self) -> Split {
        Split::new(1.0 / self.full)
    }

    // Java: double toDouble()
    pub fn toDouble(&self) -> f64 { self.full }

    // Java: double[] toArray()
    pub fn toArray(&self) -> [f64; 2] { [self.high, self.low] }
}

// Java: public class ExpIntTable
impl ExpIntTable {
    // Java: static double[] EXP_INT_TABLE_A
    // Java: static double[] EXP_INT_TABLE_B
}

// Java: public class ExpFracTable
impl ExpFracTable {
    // Java: static double[] EXP_FRAC_TABLE_A
    // Java: static double[] EXP_FRAC_TABLE_B
}

// Java: public class lnMant
impl lnMant {
    // Java: static double[][] LN_MANT
}

// Java: public class CodyWaite
impl CodyWaite {
    // Java: CodyWaite(double xa)
    pub fn new(xa: f64) -> Self {
        let k = (xa * 0.6366197723675814).round() as i32;
        let rem_a = xa - k as f64 * 1.570796251296997;
        let rem_b = -(rem_a - xa + k as f64 * 1.570796251296997);
        CodyWaite { finalK: k, finalRemA: rem_a, finalRemB: rem_b }
    }

    // Java: int getK()
    pub fn getK(&self) -> i32 { self.finalK }

    // Java: double getRemA()
    pub fn getRemA(&self) -> f64 { self.finalRemA }

    // Java: double getRemB()
    pub fn getRemB(&self) -> f64 { self.finalRemB }
}
