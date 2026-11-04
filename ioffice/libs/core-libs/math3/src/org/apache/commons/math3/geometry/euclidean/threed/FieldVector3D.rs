// Java: import java.io.Serializable
// Java: import java.text.NumberFormat
// Java: import org.apache.commons.math3.RealFieldElement
use crate::org::apache::commons::math3::RealFieldElement;
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays;

// Java field: long serialVersionUID
#[allow(non_upper_case_globals)]
pub(crate) const serialVersionUID: i64 = 20130224;

// Java: public class is
// Java: public class are
// Java: public class FieldVector3D
#[derive(Debug, Clone)]
pub struct FieldVector3D {
    // Java field: T x
    pub(crate) x: f64,
    // Java field: T y
    pub(crate) y: f64,
    // Java field: T z
    pub(crate) z: f64,
}

// Java: public class is
// Java: public class are
// Java: public class FieldVector3D
#[allow(non_snake_case)]
impl FieldVector3D {
    // Java: FieldVector3D(final T x, final T y, final T z)
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        // Java: this.x = x;
        // Java: this.y = y;
        // Java: this.z = z;
        FieldVector3D { x, y, z }
    }

    // Java: FieldVector3D(final T[] v)
    pub fn from_array(v: &[f64]) -> Self {
        // Java: if (v.length != 3)
        if v.len() != 3 {
            // Java: throw new DimensionMismatchException(v.length, 3);
            panic!("DimensionMismatchException");
        }
        // Java: this.x = v[0];
        // Java: this.y = v[1];
        // Java: this.z = v[2];
        FieldVector3D { x: v[0], y: v[1], z: v[2] }
    }

    // Java: FieldVector3D(final T alpha, final T delta)
    pub fn from_angles(alpha: f64, delta: f64) -> Self {
        // Java: T cosDelta = delta.cos();
        let cos_delta = delta.cos();
        // Java: this.x = alpha.cos().multiply(cosDelta);
        // Java: this.y = alpha.sin().multiply(cosDelta);
        // Java: this.z = delta.sin();
        FieldVector3D {
            x: alpha.cos() * cos_delta,
            y: alpha.sin() * cos_delta,
            z: delta.sin(),
        }
    }

    // Java: FieldVector3D(final T a, final FieldVector3D<T>u)
    pub fn scale(a: f64, u: &FieldVector3D) -> Self {
        // Java: this.x = a.multiply(u.x);
        // Java: this.y = a.multiply(u.y);
        // Java: this.z = a.multiply(u.z);
        FieldVector3D { x: a * u.x, y: a * u.y, z: a * u.z }
    }

    // Java: FieldVector3D(final T a, final Vector3D u)
    pub fn scale_v3d(a: f64, u: &FieldVector3D) -> Self {
        // Java: this.x = a.multiply(u.getX());
        // Java: this.y = a.multiply(u.getY());
        // Java: this.z = a.multiply(u.getZ());
        FieldVector3D { x: a * u.x, y: a * u.y, z: a * u.z }
    }

    // Java: FieldVector3D(final double a, final FieldVector3D<T> u)
    pub fn scale_double(a: f64, u: &FieldVector3D) -> Self {
        // Java: this.x = u.x.multiply(a);
        // Java: this.y = u.y.multiply(a);
        // Java: this.z = u.z.multiply(a);
        FieldVector3D { x: u.x * a, y: u.y * a, z: u.z * a }
    }

    // Java: FieldVector3D()
    // Java: FieldVector3D()
    // Java: FieldVector3D()
    // Java: FieldVector3D()
    // Java: FieldVector3D()
    // Java: FieldVector3D()
    // Java: FieldVector3D()
    // Java: FieldVector3D()
    // Java: FieldVector3D()
    pub fn linear_combination_2(a1: f64, u1: &FieldVector3D, a2: f64, u2: &FieldVector3D) -> Self {
        FieldVector3D {
            x: a1 * u1.x + a2 * u2.x,
            y: a1 * u1.y + a2 * u2.y,
            z: a1 * u1.z + a2 * u2.z,
        }
    }

    pub fn linear_combination_3(
        a1: f64, u1: &FieldVector3D,
        a2: f64, u2: &FieldVector3D,
        a3: f64, u3: &FieldVector3D,
    ) -> Self {
        FieldVector3D {
            x: a1 * u1.x + a2 * u2.x + a3 * u3.x,
            y: a1 * u1.y + a2 * u2.y + a3 * u3.y,
            z: a1 * u1.z + a2 * u2.z + a3 * u3.z,
        }
    }

    pub fn linear_combination_4(
        a1: f64, u1: &FieldVector3D,
        a2: f64, u2: &FieldVector3D,
        a3: f64, u3: &FieldVector3D,
        a4: f64, u4: &FieldVector3D,
    ) -> Self {
        FieldVector3D {
            x: a1 * u1.x + a2 * u2.x + a3 * u3.x + a4 * u4.x,
            y: a1 * u1.y + a2 * u2.y + a3 * u3.y + a4 * u4.y,
            z: a1 * u1.z + a2 * u2.z + a3 * u3.z + a4 * u4.z,
        }
    }

    // Java: T getX()
    pub fn getX(&self) -> f64 {
        // Java: return x;
        self.x
    }

    // Java: T getY()
    pub fn getY(&self) -> f64 {
        // Java: return y;
        self.y
    }

    // Java: T getZ()
    pub fn getZ(&self) -> f64 {
        // Java: return z;
        self.z
    }

    // Java: T[] toArray()
    pub fn toArray(&self) -> Vec<f64> {
        // Java: final T[] array = MathArrays.buildArray(x.getField(), 3);
        // Java: array[0] = x;
        // Java: array[1] = y;
        // Java: array[2] = z;
        // Java: return array;
        vec![self.x, self.y, self.z]
    }

    // Java: Vector3D toVector3D()
    pub fn toVector3D(&self) -> (f64, f64, f64) {
        // Java: return new Vector3D(x.getReal(), y.getReal(), z.getReal());
        (self.x, self.y, self.z)
    }

    // Java: T getNorm1()
    pub fn getNorm1(&self) -> f64 {
        // Java: return x.abs().add(y.abs()).add(z.abs());
        self.x.abs() + self.y.abs() + self.z.abs()
    }

    // Java: T getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: // there are no cancellation problems here, so we use the straightforward formula
        // Java: return x.multiply(x).add(y.multiply(y)).add(z.multiply(z)).sqrt();
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    // Java: T getNormSq()
    pub fn getNormSq(&self) -> f64 {
        // Java: // there are no cancellation problems here, so we use the straightforward formula
        // Java: return x.multiply(x).add(y.multiply(y)).add(z.multiply(z));
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Java: T getNormInf()
    pub fn getNormInf(&self) -> f64 {
        // Java: final T xAbs = x.abs();
        let x_abs = self.x.abs();
        // Java: final T yAbs = y.abs();
        let y_abs = self.y.abs();
        // Java: final T zAbs = z.abs();
        let z_abs = self.z.abs();
        // Java: if (xAbs.getReal() <= yAbs.getReal())
        if x_abs <= y_abs {
            // Java: if (yAbs.getReal() <= zAbs.getReal())
            if y_abs <= z_abs {
                // Java: return zAbs;
                z_abs
            // Java: else
            } else {
                // Java: return yAbs;
                y_abs
            }
        // Java: else
        } else {
            // Java: if (xAbs.getReal() <= zAbs.getReal())
            if x_abs <= z_abs {
                // Java: return zAbs;
                z_abs
            // Java: else
            } else {
                // Java: return xAbs;
                x_abs
            }
        }
    }

    // Java: T getAlpha()
    pub fn getAlpha(&self) -> f64 {
        // Java: return y.atan2(x);
        self.y.atan2(self.x)
    }

    // Java: T getDelta()
    pub fn getDelta(&self) -> f64 {
        // Java: return z.divide(getNorm()).asin();
        (self.z / self.getNorm()).asin()
    }

    // Java: FieldVector3D<T> add(final FieldVector3D<T> v)
    pub fn add(&self, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.add(v.x), y.add(v.y), z.add(v.z));
        FieldVector3D { x: self.x + v.x, y: self.y + v.y, z: self.z + v.z }
    }

    // Java: FieldVector3D<T> add(final Vector3D v)
    pub fn add_v3d(&self, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.add(v.getX()), y.add(v.getY()), z.add(v.getZ()));
        self.add(v)
    }

    // Java: FieldVector3D<T> add(final T factor, final FieldVector3D<T> v)
    pub fn add_scaled(&self, factor: f64, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.getField().getOne(), this, factor, v);
        FieldVector3D {
            x: self.x + factor * v.x,
            y: self.y + factor * v.y,
            z: self.z + factor * v.z,
        }
    }

    // Java: FieldVector3D<T> add(final T factor, final Vector3D v)
    pub fn add_scaled_factor_v3d(&self, factor: f64, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.add(factor.multiply(v.getX())),
        // Java: y.add(factor.multiply(v.getY())),
        // Java: z.add(factor.multiply(v.getZ())));
        self.add_scaled(factor, v)
    }

    // Java: FieldVector3D<T> add(final double factor, final FieldVector3D<T> v)
    pub fn add_scaled_double(&self, factor: f64, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(1.0, this, factor, v);
        self.add_scaled(factor, v)
    }

    // Java: FieldVector3D<T> add(final double factor, final Vector3D v)
    pub fn add_scaled_double_v3d(&self, factor: f64, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.add(factor * v.getX()),
        // Java: y.add(factor * v.getY()),
        // Java: z.add(factor * v.getZ()));
        self.add_scaled(factor, v)
    }

    // Java: FieldVector3D<T> subtract(final FieldVector3D<T> v)
    pub fn subtract(&self, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.subtract(v.x), y.subtract(v.y), z.subtract(v.z));
        FieldVector3D { x: self.x - v.x, y: self.y - v.y, z: self.z - v.z }
    }

    // Java: FieldVector3D<T> subtract(final Vector3D v)
    pub fn subtract_v3d(&self, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.subtract(v.getX()), y.subtract(v.getY()), z.subtract(v.getZ()));
        self.subtract(v)
    }

    // Java: FieldVector3D<T> subtract(final T factor, final FieldVector3D<T> v)
    pub fn subtract_scaled(&self, factor: f64, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.getField().getOne(), this, factor.negate(), v);
        FieldVector3D {
            x: self.x - factor * v.x,
            y: self.y - factor * v.y,
            z: self.z - factor * v.z,
        }
    }

    // Java: FieldVector3D<T> subtract(final T factor, final Vector3D v)
    pub fn subtract_scaled_factor_v3d(&self, factor: f64, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.subtract(factor.multiply(v.getX())),
        // Java: y.subtract(factor.multiply(v.getY())),
        // Java: z.subtract(factor.multiply(v.getZ())));
        self.subtract_scaled(factor, v)
    }

    // Java: FieldVector3D<T> subtract(final double factor, final FieldVector3D<T> v)
    pub fn subtract_scaled_double(&self, factor: f64, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(1.0, this, -factor, v);
        self.subtract_scaled(factor, v)
    }

    // Java: FieldVector3D<T> subtract(final double factor, final Vector3D v)
    pub fn subtract_scaled_double_v3d(&self, factor: f64, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.subtract(factor * v.getX()),
        // Java: y.subtract(factor * v.getY()),
        // Java: z.subtract(factor * v.getZ()));
        self.subtract_scaled(factor, v)
    }

    // Java: FieldVector3D<T> normalize()
    pub fn normalize(&self) -> FieldVector3D {
        // Java: final T s = getNorm();
        let s = self.getNorm();
        // Java: if (s.getReal() == 0)
        if s == 0.0 {
            // Java: throw new MathArithmeticException(LocalizedFormats.CANNOT_NORMALIZE_A_ZERO_NORM_VECTOR);
            panic!("MathArithmeticException: CANNOT_NORMALIZE_A_ZERO_NORM_VECTOR");
        }
        // Java: return scalarMultiply(s.reciprocal());
        self.scalarMultiply(1.0 / s)
    }

    // Java: FieldVector3D<T> orthogonal()
    pub fn orthogonal(&self) -> FieldVector3D {
        // Java: final double threshold = 0.6 * getNorm().getReal();
        let threshold = 0.6 * self.getNorm();
        // Java: if (threshold == 0)
        if threshold == 0.0 {
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM);
            panic!("MathArithmeticException: ZERO_NORM");
        }
        // Java: if (FastMath.abs(x.getReal()) <= threshold)
        if self.x.abs() <= threshold {
            // Java: final T inverse  = y.multiply(y).add(z.multiply(z)).sqrt().reciprocal();
            let inverse = 1.0 / (self.y * self.y + self.z * self.z).sqrt();
            // Java: return new FieldVector3D<T>(inverse.getField().getZero(), inverse.multiply(z), inverse.multiply(y).negate());
            FieldVector3D { x: 0.0, y: inverse * self.z, z: -inverse * self.y }
        // Java: else if (FastMath.abs(y.getReal()) <= threshold)
        } else if self.y.abs() <= threshold {
            // Java: final T inverse  = x.multiply(x).add(z.multiply(z)).sqrt().reciprocal();
            let inverse = 1.0 / (self.x * self.x + self.z * self.z).sqrt();
            // Java: return new FieldVector3D<T>(inverse.multiply(z).negate(), inverse.getField().getZero(), inverse.multiply(x));
            FieldVector3D { x: -inverse * self.z, y: 0.0, z: inverse * self.x }
        // Java: else
        } else {
            // Java: final T inverse  = x.multiply(x).add(y.multiply(y)).sqrt().reciprocal();
            let inverse = 1.0 / (self.x * self.x + self.y * self.y).sqrt();
            // Java: return new FieldVector3D<T>(inverse.multiply(y), inverse.multiply(x).negate(), inverse.getField().getZero());
            FieldVector3D { x: inverse * self.y, y: -inverse * self.x, z: 0.0 }
        }
    }

    // Java: public static <T extends RealFieldElement<T>> T angle(final FieldVector3D<T> v1, final FieldVector3D<T> v2)
    pub fn angle(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        // Java: final T normProduct = v1.getNorm().multiply(v2.getNorm());
        let norm_product = v1.getNorm() * v2.getNorm();
        // Java: if (normProduct.getReal() == 0)
        if norm_product == 0.0 {
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM);
            panic!("MathArithmeticException: ZERO_NORM");
        }
        // Java: final T dot = dotProduct(v1, v2);
        let dot = v1.dotProduct(v2);
        // Java: final double threshold = normProduct.getReal() * 0.9999;
        let threshold = norm_product * 0.9999;
        // Java: if ((dot.getReal() < -threshold) || (dot.getReal() > threshold))
        if dot < -threshold || dot > threshold {
            // Java: FieldVector3D<T> v3 = crossProduct(v1, v2);
            let v3 = v1.crossProduct(v2);
            // Java: if (dot.getReal() >= 0)
            if dot >= 0.0 {
                // Java: return v3.getNorm().divide(normProduct).asin();
                (v3.getNorm() / norm_product).asin()
            } else {
                // Java: return v3.getNorm().divide(normProduct).asin().subtract(FastMath.PI).negate();
                std::f64::consts::PI - (v3.getNorm() / norm_product).asin()
            }
        } else {
            // Java: return dot.divide(normProduct).acos();
            (dot / norm_product).acos()
        }
    }

    // Java: public static <T extends RealFieldElement<T>> T angle(final FieldVector3D<T> v1, final Vector3D v2)
    pub fn angle_fv_v3d(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        Self::angle(v1, v2)
    }

    // Java: public static <T extends RealFieldElement<T>> T angle(final Vector3D v1, final FieldVector3D<T> v2)
    pub fn angle_v3d_fv(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        // Java: return angle(v2, v1);
        Self::angle(v2, v1)
    }

    // Java: FieldVector3D<T> negate()
    pub fn negate(&self) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.negate(), y.negate(), z.negate());
        FieldVector3D { x: -self.x, y: -self.y, z: -self.z }
    }

    // Java: FieldVector3D<T> scalarMultiply(final T a)
    pub fn scalarMultiply(&self, a: f64) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.multiply(a), y.multiply(a), z.multiply(a));
        FieldVector3D { x: a * self.x, y: a * self.y, z: a * self.z }
    }

    // Java: FieldVector3D<T> scalarMultiply(final double a)
    pub fn scalarMultiply_double(&self, a: f64) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.multiply(a), y.multiply(a), z.multiply(a));
        self.scalarMultiply(a)
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: return Double.isNaN(x.getReal()) || Double.isNaN(y.getReal()) || Double.isNaN(z.getReal());
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    // Java: boolean isInfinite()
    pub fn isInfinite(&self) -> bool {
        // Java: return !isNaN() && (Double.isInfinite(x.getReal()) || Double.isInfinite(y.getReal()) || Double.isInfinite(z.getReal()));
        !self.isNaN() && (self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite())
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, other: &FieldVector3D) -> bool {
        // Java: if (this == other)
        // Java: return true;
        // Java: if (other instanceof FieldVector3D)
        // Java: @SuppressWarnings("unchecked")
        // Java: final FieldVector3D<T> rhs = (FieldVector3D<T>) other;
        // Java: if (rhs.isNaN())
        if other.isNaN() {
            // Java: return this.isNaN();
            return self.isNaN();
        }
        // Java: return x.equals(rhs.x) && y.equals(rhs.y) && z.equals(rhs.z);
        // Java: return false;
        self.x == other.x && self.y == other.y && self.z == other.z
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: if (isNaN())
        if self.isNaN() {
            // Java: return 409;
            return 409;
        }
        // Java: return 311 * (107 * x.hashCode() + 83 * y.hashCode() +  z.hashCode());
        let xh = self.x.to_bits() as i64;
        let yh = self.y.to_bits() as i64;
        let zh = self.z.to_bits() as i64;
        let xhi = (xh ^ (xh >> 32)) as i32;
        let yhi = (yh ^ (yh >> 32)) as i32;
        let zhi = (zh ^ (zh >> 32)) as i32;
        311_i32.wrapping_mul(
            107_i32.wrapping_mul(xhi)
                .wrapping_add(83_i32.wrapping_mul(yhi))
                .wrapping_add(zhi),
        )
    }

    // Java: T dotProduct(final FieldVector3D<T> v)
    pub fn dotProduct(&self, v: &FieldVector3D) -> f64 {
        // Java: return x.linearCombination(x, v.x, y, v.y, z, v.z);
        self.x * v.x + self.y * v.y + self.z * v.z
    }

    // Java: T dotProduct(final Vector3D v)
    pub fn dotProduct_v3d(&self, v: &FieldVector3D) -> f64 {
        // Java: return x.linearCombination(v.getX(), x, v.getY(), y, v.getZ(), z);
        self.dotProduct(v)
    }

    // Java: FieldVector3D<T> crossProduct(final FieldVector3D<T> v)
    pub fn crossProduct(&self, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.linearCombination(y, v.z, z.negate(), v.y),
        // Java: y.linearCombination(z, v.x, x.negate(), v.z),
        // Java: z.linearCombination(x, v.y, y.negate(), v.x));
        FieldVector3D {
            x: self.y * v.z - self.z * v.y,
            y: self.z * v.x - self.x * v.z,
            z: self.x * v.y - self.y * v.x,
        }
    }

    // Java: FieldVector3D<T> crossProduct(final Vector3D v)
    pub fn crossProduct_v3d(&self, v: &FieldVector3D) -> FieldVector3D {
        // Java: return new FieldVector3D<T>(x.linearCombination(v.getZ(), y, -v.getY(), z),
        // Java: y.linearCombination(v.getX(), z, -v.getZ(), x),
        // Java: z.linearCombination(v.getY(), x, -v.getX(), y));
        self.crossProduct(v)
    }

    // Java: T distance1(final FieldVector3D<T> v)
    pub fn distance1(&self, v: &FieldVector3D) -> f64 {
        // Java: final T dx = v.x.subtract(x).abs();
        // Java: final T dy = v.y.subtract(y).abs();
        // Java: final T dz = v.z.subtract(z).abs();
        // Java: return dx.add(dy).add(dz);
        (self.x - v.x).abs() + (self.y - v.y).abs() + (self.z - v.z).abs()
    }

    // Java: T distance1(final Vector3D v)
    pub fn distance1_v3d(&self, v: &FieldVector3D) -> f64 {
        // Java: final T dx = x.subtract(v.getX()).abs();
        // Java: final T dy = y.subtract(v.getY()).abs();
        // Java: final T dz = z.subtract(v.getZ()).abs();
        // Java: return dx.add(dy).add(dz);
        self.distance1(v)
    }

    // Java: T distance(final FieldVector3D<T> v)
    pub fn distance(&self, v: &FieldVector3D) -> f64 {
        // Java: final T dx = v.x.subtract(x);
        // Java: final T dy = v.y.subtract(y);
        // Java: final T dz = v.z.subtract(z);
        // Java: return dx.multiply(dx).add(dy.multiply(dy)).add(dz.multiply(dz)).sqrt();
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;
        (dx * dx + dy * dy + dz * dz).sqrt()
    }

    // Java: T distance(final Vector3D v)
    pub fn distance_v3d(&self, v: &FieldVector3D) -> f64 {
        // Java: final T dx = x.subtract(v.getX());
        // Java: final T dy = y.subtract(v.getY());
        // Java: final T dz = z.subtract(v.getZ());
        // Java: return dx.multiply(dx).add(dy.multiply(dy)).add(dz.multiply(dz)).sqrt();
        self.distance(v)
    }

    // Java: T distanceInf(final FieldVector3D<T> v)
    pub fn distanceInf(&self, v: &FieldVector3D) -> f64 {
        // Java: final T dx = v.x.subtract(x).abs();
        // Java: final T dy = v.y.subtract(y).abs();
        // Java: final T dz = v.z.subtract(z).abs();
        let dx = (self.x - v.x).abs();
        let dy = (self.y - v.y).abs();
        let dz = (self.z - v.z).abs();
        // Java: if (dx.getReal() <= dy.getReal())
        if dx <= dy {
            // Java: if (dy.getReal() <= dz.getReal())
            if dy <= dz {
                // Java: return dz;
                dz
            // Java: else
            } else {
                // Java: return dy;
                dy
            }
        // Java: else
        } else {
            // Java: if (dx.getReal() <= dz.getReal())
            if dx <= dz {
                // Java: return dz;
                dz
            // Java: else
            } else {
                // Java: return dx;
                dx
            }
        }
    }

    // Java: T distanceInf(final Vector3D v)
    pub fn distanceInf_v3d(&self, v: &FieldVector3D) -> f64 {
        // Java: final T dx = x.subtract(v.getX()).abs();
        // Java: final T dy = y.subtract(v.getY()).abs();
        // Java: final T dz = z.subtract(v.getZ()).abs();
        let dx = (self.x - v.x).abs();
        let dy = (self.y - v.y).abs();
        let dz = (self.z - v.z).abs();
        // Java: if (dx.getReal() <= dy.getReal())
        if dx <= dy {
            // Java: if (dy.getReal() <= dz.getReal())
            if dy <= dz {
                // Java: return dz;
                dz
            // Java: else
            } else {
                // Java: return dy;
                dy
            }
        // Java: else
        } else {
            // Java: if (dx.getReal() <= dz.getReal())
            if dx <= dz {
                // Java: return dz;
                dz
            // Java: else
            } else {
                // Java: return dx;
                dx
            }
        }
    }

    // Java: T distanceSq(final FieldVector3D<T> v)
    pub fn distanceSq(&self, v: &FieldVector3D) -> f64 {
        // Java: final T dx = v.x.subtract(x);
        // Java: final T dy = v.y.subtract(y);
        // Java: final T dz = v.z.subtract(z);
        // Java: return dx.multiply(dx).add(dy.multiply(dy)).add(dz.multiply(dz));
        let dx = self.x - v.x;
        let dy = self.y - v.y;
        let dz = self.z - v.z;
        dx * dx + dy * dy + dz * dz
    }

    // Java: T distanceSq(final Vector3D v)
    pub fn distanceSq_v3d(&self, v: &FieldVector3D) -> f64 {
        // Java: final T dx = x.subtract(v.getX());
        // Java: final T dy = y.subtract(v.getY());
        // Java: final T dz = z.subtract(v.getZ());
        // Java: return dx.multiply(dx).add(dy.multiply(dy)).add(dz.multiply(dz));
        self.distanceSq(v)
    }

    // Java: static public static <T extends RealFieldElement<T>> T dotProduct()
    pub fn dotProduct_static(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.dotProduct(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T dotProduct()
    pub fn dotProduct_static_fv_v3d(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.dotProduct(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T dotProduct()
    pub fn dotProduct_static_v3d_fv(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v2.dotProduct(v1)
    }

    // Java: static public static <T extends RealFieldElement<T>> FieldVector3D<T> crossProduct()
    pub fn crossProduct_static(v1: &FieldVector3D, v2: &FieldVector3D) -> FieldVector3D {
        v1.crossProduct(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> FieldVector3D<T> crossProduct()
    pub fn crossProduct_static_fv_v3d(v1: &FieldVector3D, v2: &FieldVector3D) -> FieldVector3D {
        v1.crossProduct(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> FieldVector3D<T> crossProduct()
    pub fn crossProduct_static_v3d_fv(v1: &FieldVector3D, v2: &FieldVector3D) -> FieldVector3D {
        v1.crossProduct(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distance1()
    pub fn distance1_static(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.distance1(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distance1()
    pub fn distance1_static_fv_v3d(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.distance1(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distance1()
    pub fn distance1_static_v3d_fv(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v2.distance1(v1)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distance()
    pub fn distance_static(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.distance(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distance()
    pub fn distance_static_fv_v3d(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.distance(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distance()
    pub fn distance_static_v3d_fv(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v2.distance(v1)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distanceInf()
    pub fn distanceInf_static(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.distanceInf(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distanceInf()
    pub fn distanceInf_static_fv_v3d(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.distanceInf(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distanceInf()
    pub fn distanceInf_static_v3d_fv(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v2.distanceInf(v1)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distanceSq()
    pub fn distanceSq_static(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.distanceSq(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distanceSq()
    pub fn distanceSq_static_fv_v3d(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v1.distanceSq(v2)
    }

    // Java: static public static <T extends RealFieldElement<T>> T distanceSq()
    pub fn distanceSq_static_v3d_fv(v1: &FieldVector3D, v2: &FieldVector3D) -> f64 {
        v2.distanceSq(v1)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return Vector3DFormat.getInstance().format(toVector3D());
        format!("{{{}, {}, {}}}", self.x, self.y, self.z)
    }

    // Java: String toString(final NumberFormat format)
    pub fn toString_format(&self) -> String {
        // Java: return new Vector3DFormat(format).format(toVector3D());
        format!("{{{}, {}, {}}}", self.x, self.y, self.z)
    }
}
