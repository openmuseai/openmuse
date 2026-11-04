// Java: import java.io.Serializable
// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.FieldElement
use crate::org::apache::commons::math3::FieldElement;
// Java: import org.apache.commons.math3.exception.NotPositiveException
use crate::org::apache::commons::math3::exception::NotPositiveException;
// Java: import org.apache.commons.math3.exception.NullArgumentException
use crate::org::apache::commons::math3::exception::NullArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: Complex I
static I: std::sync::OnceLock<Complex> = std::sync::OnceLock::new();
// Java field: Complex NaN
static NAN_COMPLEX: std::sync::OnceLock<Complex> = std::sync::OnceLock::new();
// Java field: Complex INF
static INF: std::sync::OnceLock<Complex> = std::sync::OnceLock::new();
// Java field: Complex ONE
static ONE: std::sync::OnceLock<Complex> = std::sync::OnceLock::new();
// Java field: Complex ZERO
static ZERO: std::sync::OnceLock<Complex> = std::sync::OnceLock::new();
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = -6195664516687396620;

fn get_I() -> &'static Complex {
    I.get_or_init(|| Complex::new(0.0, 1.0))
}
fn get_NaN() -> &'static Complex {
    NAN_COMPLEX.get_or_init(|| Complex::new(f64::NAN, f64::NAN))
}
fn get_INF() -> &'static Complex {
    INF.get_or_init(|| Complex::new(f64::INFINITY, f64::INFINITY))
}
fn get_ONE() -> &'static Complex {
    ONE.get_or_init(|| Complex::new(1.0, 0.0))
}
fn get_ZERO() -> &'static Complex {
    ZERO.get_or_init(|| Complex::new(0.0, 0.0))
}

// Java: public class conforms
pub struct conforms {
}

// Java: public class Complex
#[derive(Clone, Debug)]
pub struct Complex {
    // Java field: double imaginary
    pub(crate) imaginary: f64,
    // Java field: double real
    pub(crate) real: f64,
    // Java field: boolean isNaN
    pub(crate) isNaN: bool,
    // Java field: boolean isInfinite
    pub(crate) isInfinite: bool,
}

// Java: public class conforms
impl conforms {
}

// Java: public class Complex
impl Complex {
    // Java: Complex(double real)
    pub fn new_real(real: f64) -> Self {
        // Java: this(real, 0.0);
        Self::new(real, 0.0)
    }

    // Java: Complex(double real, double imaginary)
    pub fn new(real: f64, imaginary: f64) -> Self {
        // Java: this.real = real;
        // Java: this.imaginary = imaginary;
        // Java: isNaN = Double.isNaN(real) || Double.isNaN(imaginary);
        let is_nan = real.is_nan() || imaginary.is_nan();
        // Java: isInfinite = !isNaN &&
            // Java: (Double.isInfinite(real) || Double.isInfinite(imaginary));
        let is_infinite = !is_nan && (real.is_infinite() || imaginary.is_infinite());
        Complex { real, imaginary, isNaN: is_nan, isInfinite: is_infinite }
    }

    // Java: double abs()
    pub fn abs(&self) -> f64 {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return Double.NaN;
            return f64::NAN;
        }
        // Java: if (isInfinite())
        if self.isInfinite() {
            // Java: return Double.POSITIVE_INFINITY;
            return f64::INFINITY;
        }
        // Java: if (FastMath.abs(real) < FastMath.abs(imaginary))
        if self.real.abs() < self.imaginary.abs() {
            // Java: if (imaginary == 0.0)
            if self.imaginary == 0.0 {
                // Java: return FastMath.abs(real);
                return self.real.abs();
            }
            // Java: double q = real / imaginary;
            let q = self.real / self.imaginary;
            // Java: return FastMath.abs(imaginary) * FastMath.sqrt(1 + q * q);
            self.imaginary.abs() * (1.0 + q * q).sqrt()
        }
        // Java: else
        else {
            // Java: if (real == 0.0)
            if self.real == 0.0 {
                // Java: return FastMath.abs(imaginary);
                return self.imaginary.abs();
            }
            // Java: double q = imaginary / real;
            let q = self.imaginary / self.real;
            // Java: return FastMath.abs(real) * FastMath.sqrt(1 + q * q);
            self.real.abs() * (1.0 + q * q).sqrt()
        }
    }

    // Java: Complex add(Complex addend)
    pub fn add(&self, addend: &Complex) -> Complex {
        // Java: MathUtils.checkNotNull(addend);
        // Java: if (isNaN || addend.isNaN)
        if self.isNaN || addend.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(real + addend.getReal(),
                             // Java: imaginary + addend.getImaginary());
        Self::createComplex(self.real + addend.getReal(), self.imaginary + addend.getImaginary())
    }

    // Java: Complex add(double addend)
    pub fn add_f64(&self, addend: f64) -> Complex {
        // Java: if (isNaN || Double.isNaN(addend))
        if self.isNaN || addend.is_nan() {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(real + addend, imaginary);
        Self::createComplex(self.real + addend, self.imaginary)
    }

    // Java: Complex conjugate()
    pub fn conjugate(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(real, -imaginary);
        Self::createComplex(self.real, -self.imaginary)
    }

    // Java: Complex divide(Complex divisor)
    pub fn divide(&self, divisor: &Complex) -> Complex {
        if self.isNaN || divisor.isNaN {
            return get_NaN().clone();
        }
        let c = divisor.getReal();
        let d = divisor.getImaginary();
        if c == 0.0 && d == 0.0 {
            return get_NaN().clone();
        }
        if divisor.isInfinite && !self.isInfinite {
            return get_ZERO().clone();
        }
        if self.real.abs() < self.imaginary.abs() {
            let q = c / d;
            let denom = c * q + d;
            Self::createComplex((self.real * q + self.imaginary) / denom, (self.imaginary * q - self.real) / denom)
        } else {
            let q = d / c;
            let denom = d * q + c;
            Self::createComplex((self.imaginary * q + self.real) / denom, (self.imaginary - self.real * q) / denom)
        }
    }

    // Java: Complex divide(double divisor)
    pub fn divide_f64(&self, divisor: f64) -> Complex {
        // Java: if (isNaN || Double.isNaN(divisor))
        if self.isNaN || divisor.is_nan() {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: if (divisor == 0d)
        if divisor == 0.0 {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: if (Double.isInfinite(divisor))
        if divisor.is_infinite() {
            // Java: return !isInfinite() ? ZERO : NaN;
            return if !self.isInfinite() { get_ZERO().clone() } else { get_NaN().clone() };
        }
        // Java: return createComplex(real / divisor,
                             // Java: imaginary  / divisor);
        Self::createComplex(self.real / divisor, self.imaginary / divisor)
    }

    // Java: Complex reciprocal()
    pub fn reciprocal(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: if (real == 0.0 && imaginary == 0.0)
        if self.real == 0.0 && self.imaginary == 0.0 {
            // Java: return INF;
            return get_INF().clone();
        }
        // Java: if (isInfinite)
        if self.isInfinite {
            // Java: return ZERO;
            return get_ZERO().clone();
        }
        // Java: if (FastMath.abs(real) < FastMath.abs(imaginary))
        if self.real.abs() < self.imaginary.abs() {
            // Java: double q = real / imaginary;
            let q = self.real / self.imaginary;
            // Java: double scale = 1. / (real * q + imaginary);
            let scale = 1.0 / (self.real * q + self.imaginary);
            // Java: return createComplex(scale * q, -scale);
            Self::createComplex(scale * q, -scale)
        }
        // Java: else
        else {
            // Java: double q = imaginary / real;
            let q = self.imaginary / self.real;
            // Java: double scale = 1. / (imaginary * q + real);
            let scale = 1.0 / (self.imaginary * q + self.real);
            // Java: return createComplex(scale, -scale * q);
            Self::createComplex(scale, -scale * q)
        }
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &Complex) -> bool {
        // Java: if (this == other)
        if std::ptr::eq(self, other) {
            // Java: return true;
            return true;
        }
        // Java: if (other instanceof Complex)
        {
            // Java: Complex c = (Complex) other;
            let c = other;
            // Java: if (c.isNaN)
            if c.isNaN {
                // Java: return isNaN;
                return self.isNaN;
            }
            // Java: else
            else {
                // Java: return MathUtils.equals(real, c.real) &&
                    // Java: MathUtils.equals(imaginary, c.imaginary);
                return MathUtils::MathUtils::equals(self.real, c.real) &&
                    MathUtils::MathUtils::equals(self.imaginary, c.imaginary);
            }
        }
        // Java: return false;
        false
    }

    // Java: static boolean equals(Complex x, Complex y, int maxUlps)
    pub fn equals_ulps(x: &Complex, y: &Complex, max_ulps: i32) -> bool {
        // Java: return Precision.equals(x.real, y.real, maxUlps) &&
            // Java: Precision.equals(x.imaginary, y.imaginary, maxUlps);
        Precision::Precision::equals_f64_ulps(x.real, y.real, max_ulps) &&
            Precision::Precision::equals_f64_ulps(x.imaginary, y.imaginary, max_ulps)
    }

    // Java: static boolean equals(Complex x, Complex y)
    pub fn equals_static(x: &Complex, y: &Complex) -> bool {
        // Java: return equals(x, y, 1);
        Self::equals_ulps(x, y, 1)
    }

    // Java: static boolean equals(Complex x, Complex y, double eps)
    pub fn equals_eps(x: &Complex, y: &Complex, eps: f64) -> bool {
        // Java: return Precision.equals(x.real, y.real, eps) &&
            // Java: Precision.equals(x.imaginary, y.imaginary, eps);
        Precision::Precision::equals_f64_eps(x.real, y.real, eps) &&
            Precision::Precision::equals_f64_eps(x.imaginary, y.imaginary, eps)
    }

    // Java: static boolean equalsWithRelativeTolerance()
    pub fn equalsWithRelativeTolerance(x: &Complex, y: &Complex, eps: f64) -> bool {
        Precision::Precision::equalsWithRelativeTolerance(x.real, y.real, eps) &&
            Precision::Precision::equalsWithRelativeTolerance(x.imaginary, y.imaginary, eps)
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return 7;
            return 7;
        }
        // Java: return 37 * (17 * MathUtils.hash(imaginary) +
            // Java: MathUtils.hash(real));
        37 * (17 * MathUtils::MathUtils::hash(self.imaginary) +
            MathUtils::MathUtils::hash(self.real))
    }

    // Java: double getImaginary()
    pub fn getImaginary(&self) -> f64 {
        // Java: return imaginary;
        self.imaginary
    }

    // Java: double getReal()
    pub fn getReal(&self) -> f64 {
        // Java: return real;
        self.real
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: return isNaN;
        self.isNaN
    }

    // Java: boolean isInfinite()
    pub fn isInfinite(&self) -> bool {
        // Java: return isInfinite;
        self.isInfinite
    }

    // Java: Complex multiply(Complex factor)
    pub fn multiply(&self, factor: &Complex) -> Complex {
        if self.isNaN || factor.isNaN {
            return get_NaN().clone();
        }
        if self.real.is_infinite() || self.imaginary.is_infinite() ||
           factor.real.is_infinite() || factor.imaginary.is_infinite() {
            return get_INF().clone();
        }
        Self::createComplex(
            self.real * factor.real - self.imaginary * factor.imaginary,
            self.real * factor.imaginary + self.imaginary * factor.real,
        )
    }

    // Java: Complex multiply(final int factor)
    pub fn multiply_i32(&self, factor: i32) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: if (Double.isInfinite(real) ||
            // Java: Double.isInfinite(imaginary))
        if self.real.is_infinite() || self.imaginary.is_infinite() {
            // Java: return INF;
            return get_INF().clone();
        }
        // Java: return createComplex(real * factor, imaginary * factor);
        Self::createComplex(self.real * factor as f64, self.imaginary * factor as f64)
    }

    // Java: Complex multiply(double factor)
    pub fn multiply_f64(&self, factor: f64) -> Complex {
        // Java: if (isNaN || Double.isNaN(factor))
        if self.isNaN || factor.is_nan() {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: if (Double.isInfinite(real) ||
            // Java: Double.isInfinite(imaginary) ||
            // Java: Double.isInfinite(factor))
        if self.real.is_infinite() || self.imaginary.is_infinite() || factor.is_infinite() {
            // Java: // we don't use isInfinite() to avoid testing for NaN again
            // Java: return INF;
            return get_INF().clone();
        }
        // Java: return createComplex(real * factor, imaginary * factor);
        Self::createComplex(self.real * factor, self.imaginary * factor)
    }

    // Java: Complex negate()
    pub fn negate(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(-real, -imaginary);
        Self::createComplex(-self.real, -self.imaginary)
    }

    // Java: Complex subtract(Complex subtrahend)
    pub fn subtract(&self, subtrahend: &Complex) -> Complex {
        if self.isNaN || subtrahend.isNaN {
            return get_NaN().clone();
        }
        Self::createComplex(self.real - subtrahend.real, self.imaginary - subtrahend.imaginary)
    }

    // Java: Complex subtract(double subtrahend)
    pub fn subtract_f64(&self, subtrahend: f64) -> Complex {
        // Java: if (isNaN || Double.isNaN(subtrahend))
        if self.isNaN || subtrahend.is_nan() {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(real - subtrahend, imaginary);
        Self::createComplex(self.real - subtrahend, self.imaginary)
    }

    // Java: Complex acos()
    pub fn acos(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return this.add(this.sqrt1z().multiply(I)).log().multiply(I.negate());
        self.add(&self.sqrt1z().multiply(get_I())).log().multiply(&get_I().negate())
    }

    // Java: Complex asin()
    pub fn asin(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return sqrt1z().add(this.multiply(I)).log().multiply(I.negate());
        self.sqrt1z().add(&self.multiply(get_I())).log().multiply(&get_I().negate())
    }

    // Java: Complex atan()
    pub fn atan(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return this.add(I).divide(I.subtract(this)).log()
                // Java: .multiply(I.divide(createComplex(2.0, 0.0)));
        self.add(get_I()).divide(&get_I().subtract(self)).log()
                .multiply(&get_I().divide(&Self::createComplex(2.0, 0.0)))
    }

    // Java: Complex cos()
    pub fn cos(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(FastMath.cos(real) * FastMath.cosh(imaginary),
                             // Java: -FastMath.sin(real) * FastMath.sinh(imaginary));
        Self::createComplex(self.real.cos() * self.imaginary.cosh(),
                             -self.real.sin() * self.imaginary.sinh())
    }

    // Java: Complex cosh()
    pub fn cosh(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(FastMath.cosh(real) * FastMath.cos(imaginary),
                             // Java: FastMath.sinh(real) * FastMath.sin(imaginary));
        Self::createComplex(self.real.cosh() * self.imaginary.cos(),
                             self.real.sinh() * self.imaginary.sin())
    }

    // Java: Complex exp()
    pub fn exp(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: double expReal = FastMath.exp(real);
        let exp_real = self.real.exp();
        // Java: return createComplex(expReal *  FastMath.cos(imaginary),
                             // Java: expReal * FastMath.sin(imaginary));
        Self::createComplex(exp_real * self.imaginary.cos(),
                             exp_real * self.imaginary.sin())
    }

    // Java: Complex log()
    pub fn log(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(FastMath.log(abs()),
                             // Java: FastMath.atan2(imaginary, real));
        Self::createComplex(self.abs().ln(),
                             self.imaginary.atan2(self.real))
    }

    // Java: Complex pow(Complex x)
    pub fn pow(&self, x: &Complex) -> Complex {
        self.log().multiply(x).exp()
    }

    // Java: Complex pow(double x)
    pub fn pow_f64(&self, x: f64) -> Complex {
        // Java: return this.log().multiply(x).exp();
        self.log().multiply_f64(x).exp()
    }

    // Java: Complex sin()
    pub fn sin(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(FastMath.sin(real) * FastMath.cosh(imaginary),
                             // Java: FastMath.cos(real) * FastMath.sinh(imaginary));
        Self::createComplex(self.real.sin() * self.imaginary.cosh(),
                             self.real.cos() * self.imaginary.sinh())
    }

    // Java: Complex sinh()
    pub fn sinh(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return createComplex(FastMath.sinh(real) * FastMath.cos(imaginary),
            // Java: FastMath.cosh(real) * FastMath.sin(imaginary));
        Self::createComplex(self.real.sinh() * self.imaginary.cos(),
            self.real.cosh() * self.imaginary.sin())
    }

    // Java: Complex sqrt()
    pub fn sqrt(&self) -> Complex {
        // Java: if (isNaN)
        if self.isNaN {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: if (real == 0.0 && imaginary == 0.0)
        if self.real == 0.0 && self.imaginary == 0.0 {
            // Java: return createComplex(0.0, 0.0);
            return Self::createComplex(0.0, 0.0);
        }
        // Java: double t = FastMath.sqrt((FastMath.abs(real) + abs()) / 2.0);
        let t = ((self.real.abs() + self.abs()) / 2.0).sqrt();
        // Java: if (real >= 0.0)
        if self.real >= 0.0 {
            // Java: return createComplex(t, imaginary / (2.0 * t));
            Self::createComplex(t, self.imaginary / (2.0 * t))
        }
        // Java: else
        else {
            // Java: return createComplex(FastMath.abs(imaginary) / (2.0 * t),
                                 // Java: FastMath.copySign(1d, imaginary) * t);
            Self::createComplex(self.imaginary.abs() / (2.0 * t),
                                 self.imaginary.signum() * t)
        }
    }

    // Java: Complex sqrt1z()
    pub fn sqrt1z(&self) -> Complex {
        // Java: return createComplex(1.0, 0.0).subtract(this.multiply(this)).sqrt();
        Self::createComplex(1.0, 0.0).subtract(&self.multiply(self)).sqrt()
    }

    // Java: Complex tan()
    pub fn tan(&self) -> Complex {
        // Java: if (isNaN || Double.isInfinite(real))
        if self.isNaN || self.real.is_infinite() {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: if (imaginary > 20.0)
        if self.imaginary > 20.0 {
            // Java: return createComplex(0.0, 1.0);
            return Self::createComplex(0.0, 1.0);
        }
        // Java: if (imaginary < -20.0)
        if self.imaginary < -20.0 {
            // Java: return createComplex(0.0, -1.0);
            return Self::createComplex(0.0, -1.0);
        }
        // Java: double real2 = 2.0 * real;
        let real2 = 2.0 * self.real;
        // Java: double imaginary2 = 2.0 * imaginary;
        let imaginary2 = 2.0 * self.imaginary;
        // Java: double d = FastMath.cos(real2) + FastMath.cosh(imaginary2);
        let d = real2.cos() + imaginary2.cosh();
        // Java: return createComplex(FastMath.sin(real2) / d,
                             // Java: FastMath.sinh(imaginary2) / d);
        Self::createComplex(real2.sin() / d,
                             imaginary2.sinh() / d)
    }

    // Java: Complex tanh()
    pub fn tanh(&self) -> Complex {
        // Java: if (isNaN || Double.isInfinite(imaginary))
        if self.isNaN || self.imaginary.is_infinite() {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: if (real > 20.0)
        if self.real > 20.0 {
            // Java: return createComplex(1.0, 0.0);
            return Self::createComplex(1.0, 0.0);
        }
        // Java: if (real < -20.0)
        if self.real < -20.0 {
            // Java: return createComplex(-1.0, 0.0);
            return Self::createComplex(-1.0, 0.0);
        }
        // Java: double real2 = 2.0 * real;
        let real2 = 2.0 * self.real;
        // Java: double imaginary2 = 2.0 * imaginary;
        let imaginary2 = 2.0 * self.imaginary;
        // Java: double d = FastMath.cosh(real2) + FastMath.cos(imaginary2);
        let d = real2.cosh() + imaginary2.cos();
        // Java: return createComplex(FastMath.sinh(real2) / d,
                             // Java: FastMath.sin(imaginary2) / d);
        Self::createComplex(real2.sinh() / d,
                             imaginary2.sin() / d)
    }

    // Java: double getArgument()
    pub fn getArgument(&self) -> f64 {
        // Java: return FastMath.atan2(getImaginary(), getReal());
        self.getImaginary().atan2(self.getReal())
    }

    // Java: List<Complex> nthRoot(int n)
    pub fn nthRoot(&self, n: i32) -> Vec<Complex> {
        // Java: if (n <= 0)
        if n <= 0 {
            // Java: throw new NotPositiveException(LocalizedFormats.CANNOT_COMPUTE_NTH_ROOT_FOR_NEGATIVE_N,
                                           // Java: n);
            panic!("NotPositiveException: CANNOT_COMPUTE_NTH_ROOT_FOR_NEGATIVE_N: {}", n);
        }
        // Java: final List<Complex> result = new ArrayList<Complex>();
        let mut result = Vec::new();
        // Java: if (isNaN)
        if self.isNaN {
            // Java: result.add(NaN);
            result.push(get_NaN().clone());
            // Java: return result;
            return result;
        }
        // Java: if (isInfinite())
        if self.isInfinite() {
            // Java: result.add(INF);
            result.push(get_INF().clone());
            // Java: return result;
            return result;
        }
        // Java: // nth root of abs -- faster / more accurate to use a solver here?
        // Java: final double nthRootOfAbs = FastMath.pow(abs(), 1.0 / n);
        let nth_root_of_abs = self.abs().powf(1.0 / n as f64);
        // Java: // Compute nth roots of complex number with k = 0, 1, ... n-1
        // Java: final double nthPhi = getArgument() / n;
        let nth_phi = self.getArgument() / n as f64;
        // Java: final double slice = 2 * FastMath.PI / n;
        let slice = 2.0 * std::f64::consts::PI / n as f64;
        // Java: double innerPart = nthPhi;
        let mut inner_part = nth_phi;
        // Java: for (int k = 0; k < n ; k++)
        for _k in 0..n {
            // Java: // inner part
            // Java: final double realPart = nthRootOfAbs *  FastMath.cos(innerPart);
            let real_part = nth_root_of_abs * inner_part.cos();
            // Java: final double imaginaryPart = nthRootOfAbs *  FastMath.sin(innerPart);
            let imaginary_part = nth_root_of_abs * inner_part.sin();
            // Java: result.add(createComplex(realPart, imaginaryPart));
            result.push(Self::createComplex(real_part, imaginary_part));
            // Java: innerPart += slice;
            inner_part += slice;
        }
        // Java: return result;
        result
    }

    // Java: Complex createComplex()
    pub fn createComplex(real_part: f64, imaginary_part: f64) -> Complex {
        Complex::new(real_part, imaginary_part)
    }

    // Java: static Complex valueOf()
    pub fn valueOf(real_part: f64, imaginary_part: f64) -> Complex {
        if real_part.is_nan() || imaginary_part.is_nan() {
            return get_NaN().clone();
        }
        Complex::new(real_part, imaginary_part)
    }

    // Java: static Complex valueOf(double realPart)
    pub fn valueOf_real(real_part: f64) -> Complex {
        // Java: if (Double.isNaN(realPart))
        if real_part.is_nan() {
            // Java: return NaN;
            return get_NaN().clone();
        }
        // Java: return new Complex(realPart);
        Complex::new_real(real_part)
    }

    // Java: Object readResolve()
    pub fn readResolve(&self) -> Complex {
        // Java: return createComplex(real, imaginary);
        Self::createComplex(self.real, self.imaginary)
    }

    // Java: ComplexField getField()
    pub fn getField(&self) -> ! {
        // Java: return ComplexField.getInstance();
        todo!("ComplexField not yet implemented")
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return "(" + real + ", " + imaginary + ")";
        format!("({}, {})", self.real, self.imaginary)
    }
}

impl std::fmt::Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.real, self.imaginary)
    }
}
