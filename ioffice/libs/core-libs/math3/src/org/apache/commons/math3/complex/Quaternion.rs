// Java: import java.io.Serializable
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.ZeroException
use crate::org::apache::commons::math3::exception::ZeroException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java field: Quaternion IDENTITY
static IDENTITY: std::sync::OnceLock<Quaternion> = std::sync::OnceLock::new();
// Java field: Quaternion ZERO
static ZERO: std::sync::OnceLock<Quaternion> = std::sync::OnceLock::new();
// Java field: Quaternion I
static I: std::sync::OnceLock<Quaternion> = std::sync::OnceLock::new();
// Java field: Quaternion J
static J: std::sync::OnceLock<Quaternion> = std::sync::OnceLock::new();
// Java field: Quaternion K
static K: std::sync::OnceLock<Quaternion> = std::sync::OnceLock::new();
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 20092012;

fn get_IDENTITY() -> &'static Quaternion { IDENTITY.get_or_init(|| Quaternion::new(1.0, 0.0, 0.0, 0.0)) }
fn get_ZERO() -> &'static Quaternion { ZERO.get_or_init(|| Quaternion::new(0.0, 0.0, 0.0, 0.0)) }
fn get_I() -> &'static Quaternion { I.get_or_init(|| Quaternion::new(0.0, 1.0, 0.0, 0.0)) }
fn get_J() -> &'static Quaternion { J.get_or_init(|| Quaternion::new(0.0, 0.0, 1.0, 0.0)) }
fn get_K() -> &'static Quaternion { K.get_or_init(|| Quaternion::new(0.0, 0.0, 0.0, 1.0)) }

// Java: public class implements
pub struct implements {
}

// Java: public class Quaternion
#[derive(Clone, Debug)]
pub struct Quaternion {
    // Java field: double q0
    pub(crate) q0: f64,
    // Java field: double q1
    pub(crate) q1: f64,
    // Java field: double q2
    pub(crate) q2: f64,
    // Java field: double q3
    pub(crate) q3: f64,
}

// Java: public class implements
impl implements {
}

// Java: public class Quaternion
impl Quaternion {
    // Java: Quaternion()
    pub fn new(a: f64, b: f64, c: f64, d: f64) -> Self {
        Quaternion { q0: a, q1: b, q2: c, q3: d }
    }

    // Java: Quaternion()
    pub fn new_scalar_vector(scalar: f64, v: &[f64]) -> Self {
        Quaternion { q0: scalar, q1: v[0], q2: v[1], q3: v[2] }
    }

    // Java: Quaternion(final double[] v)
    pub fn new_vector(v: &[f64]) -> Self {
        // Java: this(0, v);
        Self::new_scalar_vector(0.0, v)
    }

    // Java: Quaternion getConjugate()
    pub fn getConjugate(&self) -> Quaternion {
        // Java: return new Quaternion(q0, -q1, -q2, -q3);
        Quaternion::new(self.q0, -self.q1, -self.q2, -self.q3)
    }

    // Java: static Quaternion multiply(final Quaternion q1, final Quaternion q2)
    pub fn multiply_static(q1: &Quaternion, q2: &Quaternion) -> Quaternion {
        // Java: // Components of the first quaternion.
        // Java: final double q1a = q1.getQ0();
        let q1a = q1.getQ0();
        // Java: final double q1b = q1.getQ1();
        let q1b = q1.getQ1();
        // Java: final double q1c = q1.getQ2();
        let q1c = q1.getQ2();
        // Java: final double q1d = q1.getQ3();
        let q1d = q1.getQ3();
        // Java: // Components of the second quaternion.
        // Java: final double q2a = q2.getQ0();
        let q2a = q2.getQ0();
        // Java: final double q2b = q2.getQ1();
        let q2b = q2.getQ1();
        // Java: final double q2c = q2.getQ2();
        let q2c = q2.getQ2();
        // Java: final double q2d = q2.getQ3();
        let q2d = q2.getQ3();
        // Java: // Components of the product.
        // Java: final double w = q1a * q2a - q1b * q2b - q1c * q2c - q1d * q2d;
        let w = q1a * q2a - q1b * q2b - q1c * q2c - q1d * q2d;
        // Java: final double x = q1a * q2b + q1b * q2a + q1c * q2d - q1d * q2c;
        let x = q1a * q2b + q1b * q2a + q1c * q2d - q1d * q2c;
        // Java: final double y = q1a * q2c - q1b * q2d + q1c * q2a + q1d * q2b;
        let y = q1a * q2c - q1b * q2d + q1c * q2a + q1d * q2b;
        // Java: final double z = q1a * q2d + q1b * q2c - q1c * q2b + q1d * q2a;
        let z = q1a * q2d + q1b * q2c - q1c * q2b + q1d * q2a;
        // Java: return new Quaternion(w, x, y, z);
        Quaternion::new(w, x, y, z)
    }

    // Java: Quaternion multiply(final Quaternion q)
    pub fn multiply(&self, q: &Quaternion) -> Quaternion {
        // Java: return multiply(this, q);
        Self::multiply_static(self, q)
    }

    // Java: static Quaternion add()
    pub fn add_static(q1: &Quaternion, q2: &Quaternion) -> Quaternion {
        Quaternion::new(q1.q0 + q2.q0, q1.q1 + q2.q1, q1.q2 + q2.q2, q1.q3 + q2.q3)
    }

    // Java: Quaternion add(final Quaternion q)
    pub fn add(&self, q: &Quaternion) -> Quaternion {
        // Java: return add(this, q);
        Self::add_static(self, q)
    }

    // Java: static Quaternion subtract()
    pub fn subtract_static(q1: &Quaternion, q2: &Quaternion) -> Quaternion {
        Quaternion::new(q1.q0 - q2.q0, q1.q1 - q2.q1, q1.q2 - q2.q2, q1.q3 - q2.q3)
    }

    // Java: Quaternion subtract(final Quaternion q)
    pub fn subtract(&self, q: &Quaternion) -> Quaternion {
        // Java: return subtract(this, q);
        Self::subtract_static(self, q)
    }

    // Java: static double dotProduct()
    pub fn dotProduct_static(q1: &Quaternion, q2: &Quaternion) -> f64 {
        q1.q0 * q2.q0 + q1.q1 * q2.q1 + q1.q2 * q2.q2 + q1.q3 * q2.q3
    }

    // Java: double dotProduct(final Quaternion q)
    pub fn dotProduct(&self, q: &Quaternion) -> f64 {
        // Java: return dotProduct(this, q);
        Self::dotProduct_static(self, q)
    }

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: return FastMath.sqrt(q0 * q0 +
                             // Java: q1 * q1 +
                             // Java: q2 * q2 +
                             // Java: q3 * q3);
        (self.q0 * self.q0 + self.q1 * self.q1 + self.q2 * self.q2 + self.q3 * self.q3).sqrt()
    }

    // Java: Quaternion normalize()
    pub fn normalize(&self) -> Quaternion {
        // Java: final double norm = getNorm();
        let norm = self.getNorm();
        // Java: if (norm < Precision.SAFE_MIN)
        if norm < Precision::SAFE_MIN {
            // Java: throw new ZeroException(LocalizedFormats.NORM, norm);
            panic!("ZeroException: NORM: {}", norm);
        }
        // Java: return new Quaternion(q0 / norm,
                              // Java: q1 / norm,
                              // Java: q2 / norm,
                              // Java: q3 / norm);
        Quaternion::new(self.q0 / norm, self.q1 / norm, self.q2 / norm, self.q3 / norm)
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &Quaternion) -> bool {
        // Java: if (this == other)
        if std::ptr::eq(self, other) {
            // Java: return true;
            return true;
        }
        // Java: if (other instanceof Quaternion)
        {
            // Java: final Quaternion q = (Quaternion) other;
            let q = other;
            // Java: return q0 == q.getQ0() &&
                // Java: q1 == q.getQ1() &&
                // Java: q2 == q.getQ2() &&
                // Java: q3 == q.getQ3();
            return self.q0 == q.getQ0() &&
                self.q1 == q.getQ1() &&
                self.q2 == q.getQ2() &&
                self.q3 == q.getQ3();
        }
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: // "Effective Java" (second edition, p. 47).
        // Java: int result = 17;
        let mut result: i32 = 17;
        // Java: for (double comp : new double[] { q0, q1, q2, q3 })
        for comp in &[self.q0, self.q1, self.q2, self.q3] {
            // Java: final int c = MathUtils.hash(comp);
            let c = MathUtils::MathUtils::hash(*comp);
            // Java: result = 31 * result + c;
            result = 31i32.wrapping_mul(result).wrapping_add(c);
        }
        // Java: return result;
        result
    }

    // Java: boolean equals()
    pub fn equals_eps(q1: &Quaternion, q2: &Quaternion, eps: f64) -> bool {
        Precision::Precision::equals_f64_eps(q1.q0, q2.q0, eps) &&
            Precision::Precision::equals_f64_eps(q1.q1, q2.q1, eps) &&
            Precision::Precision::equals_f64_eps(q1.q2, q2.q2, eps) &&
            Precision::Precision::equals_f64_eps(q1.q3, q2.q3, eps)
    }

    // Java: boolean isUnitQuaternion(double eps)
    pub fn isUnitQuaternion(&self, eps: f64) -> bool {
        // Java: return Precision.equals(getNorm(), 1d, eps);
        Precision::Precision::equals_f64_eps(self.getNorm(), 1.0, eps)
    }

    // Java: boolean isPureQuaternion(double eps)
    pub fn isPureQuaternion(&self, eps: f64) -> bool {
        // Java: return FastMath.abs(getQ0()) <= eps;
        self.getQ0().abs() <= eps
    }

    // Java: Quaternion getPositivePolarForm()
    pub fn getPositivePolarForm(&self) -> Quaternion {
        // Java: if (getQ0() < 0)
        if self.getQ0() < 0.0 {
            // Java: final Quaternion unitQ = normalize();
            let unit_q = self.normalize();
            // Java: // The quaternion of rotation (normalized quaternion) q and -q
            // Java: // are equivalent (i.e. represent the same rotation).
            // Java: return new Quaternion(-unitQ.getQ0(),
                                  // Java: -unitQ.getQ1(),
                                  // Java: -unitQ.getQ2(),
                                  // Java: -unitQ.getQ3());
            Quaternion::new(-unit_q.getQ0(), -unit_q.getQ1(), -unit_q.getQ2(), -unit_q.getQ3())
        }
        // Java: else
        else {
            // Java: return this.normalize();
            self.normalize()
        }
    }

    // Java: Quaternion getInverse()
    pub fn getInverse(&self) -> Quaternion {
        // Java: final double squareNorm = q0 * q0 + q1 * q1 + q2 * q2 + q3 * q3;
        let square_norm = self.q0 * self.q0 + self.q1 * self.q1 + self.q2 * self.q2 + self.q3 * self.q3;
        // Java: if (squareNorm < Precision.SAFE_MIN)
        if square_norm < Precision::SAFE_MIN {
            // Java: throw new ZeroException(LocalizedFormats.NORM, squareNorm);
            panic!("ZeroException: NORM: {}", square_norm);
        }
        // Java: return new Quaternion(q0 / squareNorm,
                              // Java: -q1 / squareNorm,
                              // Java: -q2 / squareNorm,
                              // Java: -q3 / squareNorm);
        Quaternion::new(self.q0 / square_norm, -self.q1 / square_norm, -self.q2 / square_norm, -self.q3 / square_norm)
    }

    // Java: double getQ0()
    pub fn getQ0(&self) -> f64 {
        // Java: return q0;
        self.q0
    }

    // Java: double getQ1()
    pub fn getQ1(&self) -> f64 {
        // Java: return q1;
        self.q1
    }

    // Java: double getQ2()
    pub fn getQ2(&self) -> f64 {
        // Java: return q2;
        self.q2
    }

    // Java: double getQ3()
    pub fn getQ3(&self) -> f64 {
        // Java: return q3;
        self.q3
    }

    // Java: double getScalarPart()
    pub fn getScalarPart(&self) -> f64 {
        // Java: return getQ0();
        self.getQ0()
    }

    // Java: double[] getVectorPart()
    pub fn getVectorPart(&self) -> Vec<f64> {
        // Java: return new double[] { getQ1(), getQ2(), getQ3() };
        vec![self.getQ1(), self.getQ2(), self.getQ3()]
    }

    // Java: Quaternion multiply(final double alpha)
    pub fn multiply_f64(&self, alpha: f64) -> Quaternion {
        // Java: return new Quaternion(alpha * q0,
                              // Java: alpha * q1,
                              // Java: alpha * q2,
                              // Java: alpha * q3);
        Quaternion::new(alpha * self.q0, alpha * self.q1, alpha * self.q2, alpha * self.q3)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: final String sp = " ";
        // Java: final StringBuilder s = new StringBuilder();
        // Java: s.append("[")
            // Java: .append(q0).append(sp)
            // Java: .append(q1).append(sp)
            // Java: .append(q2).append(sp)
            // Java: .append(q3)
            // Java: .append("]");
        // Java: return s.toString();
        format!("[{} {} {} {}]", self.q0, self.q1, self.q2, self.q3)
    }
}

impl std::fmt::Display for Quaternion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {} {} {}]", self.q0, self.q1, self.q2, self.q3)
    }
}
