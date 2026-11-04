#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import java.io.Serializable
// (no direct Rust equivalent)
// Java: import java.text.NumberFormat
// (no direct Rust equivalent)
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.Vector
use crate::org::apache::commons::math3::geometry::Vector;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats as LocalizedFormatsEnum;

use std::sync::OnceLock;

// Java field: Vector3D ZERO
static ZERO: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D PLUS_I
static PLUS_I: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D MINUS_I
static MINUS_I: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D PLUS_J
static PLUS_J: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D MINUS_J
static MINUS_J: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D PLUS_K
static PLUS_K: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D MINUS_K
static MINUS_K: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D NaN
static NaN_VEC: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D POSITIVE_INFINITY
static POSITIVE_INFINITY: OnceLock<Vector3D> = OnceLock::new();
// Java field: Vector3D NEGATIVE_INFINITY
static NEGATIVE_INFINITY: OnceLock<Vector3D> = OnceLock::new();
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 1313493323784566947;

// Java: public class implements implements vectors in a three
pub struct implements {
}

// Java: public class Vector3D
pub struct Vector3D {
    // Java field: double x
    pub(crate) x: f64,
    // Java field: double y
    pub(crate) y: f64,
    // Java field: double z
    pub(crate) z: f64,
}

// Java: public class implements implements vectors in a three
impl implements {
}

fn get_ZERO() -> &'static Vector3D {
    ZERO.get_or_init(|| Vector3D { x: 0.0, y: 0.0, z: 0.0 })
}
fn get_PLUS_I() -> &'static Vector3D {
    PLUS_I.get_or_init(|| Vector3D { x: 1.0, y: 0.0, z: 0.0 })
}
fn get_MINUS_I() -> &'static Vector3D {
    MINUS_I.get_or_init(|| Vector3D { x: -1.0, y: 0.0, z: 0.0 })
}
fn get_PLUS_J() -> &'static Vector3D {
    PLUS_J.get_or_init(|| Vector3D { x: 0.0, y: 1.0, z: 0.0 })
}
fn get_MINUS_J() -> &'static Vector3D {
    MINUS_J.get_or_init(|| Vector3D { x: 0.0, y: -1.0, z: 0.0 })
}
fn get_PLUS_K() -> &'static Vector3D {
    PLUS_K.get_or_init(|| Vector3D { x: 0.0, y: 0.0, z: 1.0 })
}
fn get_MINUS_K() -> &'static Vector3D {
    MINUS_K.get_or_init(|| Vector3D { x: 0.0, y: 0.0, z: -1.0 })
}
fn get_NaN() -> &'static Vector3D {
    NaN_VEC.get_or_init(|| Vector3D { x: f64::NAN, y: f64::NAN, z: f64::NAN })
}
fn get_POSITIVE_INFINITY() -> &'static Vector3D {
    // Java: new Vector3D(Double.POSITIVE_INFINITY, Double.POSITIVE_INFINITY, Double.POSITIVE_INFINITY)
    POSITIVE_INFINITY.get_or_init(|| Vector3D { x: f64::INFINITY, y: f64::INFINITY, z: f64::INFINITY })
}
fn get_NEGATIVE_INFINITY() -> &'static Vector3D {
    // Java: new Vector3D(Double.NEGATIVE_INFINITY, Double.NEGATIVE_INFINITY, Double.NEGATIVE_INFINITY)
    NEGATIVE_INFINITY.get_or_init(|| Vector3D { x: f64::NEG_INFINITY, y: f64::NEG_INFINITY, z: f64::NEG_INFINITY })
}

// Java: public class Vector3D
impl Vector3D {
    // Java: Vector3D(double x, double y, double z)
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        // Java: this.x = x;
        // Java: this.y = y;
        // Java: this.z = z;
        Vector3D { x, y, z }
    }

    // Java: Vector3D(double[] v)
    pub fn new_from_array(v: &[f64]) -> Self {
        // Java: if (v.length != 3)
        if v.len() != 3 {
            // Java: throw new DimensionMismatchException(v.length, 3);
            panic!("DimensionMismatchException: {} != 3", v.len());
        }
        // Java: this.x = v[0];
        // Java: this.y = v[1];
        // Java: this.z = v[2];
        Vector3D { x: v[0], y: v[1], z: v[2] }
    }

    // Java: Vector3D(double alpha, double delta)
    pub fn new_from_angles(alpha: f64, delta: f64) -> Self {
        // Java: double cosDelta = FastMath.cos(delta);
        let cos_delta = FastMath::cos(delta);
        // Java: this.x = FastMath.cos(alpha) * cosDelta;
        // Java: this.y = FastMath.sin(alpha) * cosDelta;
        // Java: this.z = FastMath.sin(delta);
        Vector3D {
            x: FastMath::cos(alpha) * cos_delta,
            y: FastMath::sin(alpha) * cos_delta,
            z: FastMath::sin(delta),
        }
    }

    // Java: Vector3D(double a, Vector3D u)
    pub fn new_scaled(a: f64, u: &Vector3D) -> Self {
        // Java: this.x = a * u.x;
        // Java: this.y = a * u.y;
        // Java: this.z = a * u.z;
        Vector3D { x: a * u.x, y: a * u.y, z: a * u.z }
    }

    // Java: Vector3D(double a1, Vector3D u1, double a2, Vector3D u2)
    pub fn new_linear_combination_2(a1: f64, u1: &Vector3D, a2: f64, u2: &Vector3D) -> Self {
        // Java: this.x = MathArrays.linearCombination(a1, u1.x, a2, u2.x);
        // Java: this.y = MathArrays.linearCombination(a1, u1.y, a2, u2.y);
        // Java: this.z = MathArrays.linearCombination(a1, u1.z, a2, u2.z);
        Vector3D {
            x: MathArrays::linearCombination_2(a1, u1.x, a2, u2.x),
            y: MathArrays::linearCombination_2(a1, u1.y, a2, u2.y),
            z: MathArrays::linearCombination_2(a1, u1.z, a2, u2.z),
        }
    }

    // Java: Vector3D()
    pub fn new_default_1() -> Self {
        todo!()
    }

    // Java: Vector3D()
    pub fn new_default_2() -> Self {
        todo!()
    }

    // Java: double getX()
    pub fn getX(&self) -> f64 {
        // Java: return x;
        self.x
    }

    // Java: double getY()
    pub fn getY(&self) -> f64 {
        // Java: return y;
        self.y
    }

    // Java: double getZ()
    pub fn getZ(&self) -> f64 {
        // Java: return z;
        self.z
    }

    // Java: double[] toArray()
    pub fn toArray(&self) -> Vec<f64> {
        // Java: return new double[] { x, y, z };
        vec![self.x, self.y, self.z]
    }

    // Java: Space getSpace()
    pub fn getSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return Euclidean3D.getInstance();
        todo!()
    }

    // Java: Vector3D getZero()
    pub fn getZero() -> &'static Vector3D {
        // Java: return ZERO;
        get_ZERO()
    }

    // Java: double getNorm1()
    pub fn getNorm1(&self) -> f64 {
        // Java: return FastMath.abs(x) + FastMath.abs(y) + FastMath.abs(z);
        FastMath::abs(self.x) + FastMath::abs(self.y) + FastMath::abs(self.z)
    }

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: // there are no cancellation problems here, so we use the straightforward formula
        // Java: return FastMath.sqrt (x * x + y * y + z * z);
        FastMath::sqrt(self.x * self.x + self.y * self.y + self.z * self.z)
    }

    // Java: double getNormSq()
    pub fn getNormSq(&self) -> f64 {
        // Java: // there are no cancellation problems here, so we use the straightforward formula
        // Java: return x * x + y * y + z * z;
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    // Java: double getNormInf()
    pub fn getNormInf(&self) -> f64 {
        // Java: return FastMath.max(FastMath.max(FastMath.abs(x), FastMath.abs(y)), FastMath.abs(z));
        FastMath::max(FastMath::max(FastMath::abs(self.x), FastMath::abs(self.y)), FastMath::abs(self.z))
    }

    // Java: double getAlpha()
    pub fn getAlpha(&self) -> f64 {
        // Java: return FastMath.atan2(y, x);
        FastMath::atan2(self.y, self.x)
    }

    // Java: double getDelta()
    pub fn getDelta(&self) -> f64 {
        // Java: return FastMath.asin(z / getNorm());
        FastMath::asin(self.z / self.getNorm())
    }

    // Java: Vector3D add(final Vector<Euclidean3D> v)
    pub fn add(&self, v: &Vector3D) -> Vector3D {
        // Java: final Vector3D v3 = (Vector3D) v;
        // Java: return new Vector3D(x + v3.x, y + v3.y, z + v3.z);
        Vector3D::new(self.x + v.x, self.y + v.y, self.z + v.z)
    }

    // Java: Vector3D add(double factor, final Vector<Euclidean3D> v)
    pub fn add_scaled(&self, factor: f64, v: &Vector3D) -> Vector3D {
        // Java: return new Vector3D(1, this, factor, (Vector3D) v);
        Vector3D::new_linear_combination_2(1.0, self, factor, v)
    }

    // Java: Vector3D subtract(final Vector<Euclidean3D> v)
    pub fn subtract(&self, v: &Vector3D) -> Vector3D {
        // Java: final Vector3D v3 = (Vector3D) v;
        // Java: return new Vector3D(x - v3.x, y - v3.y, z - v3.z);
        Vector3D::new(self.x - v.x, self.y - v.y, self.z - v.z)
    }

    // Java: Vector3D subtract(final double factor, final Vector<Euclidean3D> v)
    pub fn subtract_scaled(&self, factor: f64, v: &Vector3D) -> Vector3D {
        // Java: return new Vector3D(1, this, -factor, (Vector3D) v);
        Vector3D::new_linear_combination_2(1.0, self, -factor, v)
    }

    // Java: Vector3D normalize()
    pub fn normalize(&self) -> Vector3D {
        // Java: double s = getNorm();
        let s = self.getNorm();
        // Java: if (s == 0)
        if s == 0.0 {
            // Java: throw new MathArithmeticException(LocalizedFormats.CANNOT_NORMALIZE_A_ZERO_NORM_VECTOR);
            panic!("MathArithmeticException: CANNOT_NORMALIZE_A_ZERO_NORM_VECTOR");
        }
        // Java: return scalarMultiply(1 / s);
        self.scalarMultiply(1.0 / s)
    }

    // Java: Vector3D orthogonal()
    pub fn orthogonal(&self) -> Vector3D {
        // Java: double threshold = 0.6 * getNorm();
        let threshold = 0.6 * self.getNorm();
        // Java: if (threshold == 0)
        if threshold == 0.0 {
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM);
            panic!("MathArithmeticException: {:?}", LocalizedFormatsEnum::ZERO_NORM());
        }
        // Java: if (FastMath.abs(x) <= threshold)
        if FastMath::abs(self.x) <= threshold {
            // Java: double inverse  = 1 / FastMath.sqrt(y * y + z * z);
            let inverse = 1.0 / FastMath::sqrt(self.y * self.y + self.z * self.z);
            // Java: return new Vector3D(0, inverse * z, -inverse * y);
            return Vector3D::new(0.0, inverse * self.z, -inverse * self.y);
        }
        // Java: else if (FastMath.abs(y) <= threshold)
        else if FastMath::abs(self.y) <= threshold {
            // Java: double inverse  = 1 / FastMath.sqrt(x * x + z * z);
            let inverse = 1.0 / FastMath::sqrt(self.x * self.x + self.z * self.z);
            // Java: return new Vector3D(-inverse * z, 0, inverse * x);
            return Vector3D::new(-inverse * self.z, 0.0, inverse * self.x);
        }
        // Java: double inverse  = 1 / FastMath.sqrt(x * x + y * y);
        let inverse = 1.0 / FastMath::sqrt(self.x * self.x + self.y * self.y);
        // Java: return new Vector3D(inverse * y, -inverse * x, 0);
        Vector3D::new(inverse * self.y, -inverse * self.x, 0.0)
    }

    // Java: static double angle(Vector3D v1, Vector3D v2)
    pub fn angle(v1: &Vector3D, v2: &Vector3D) -> f64 {
        // Java: double normProduct = v1.getNorm() * v2.getNorm();
        let norm_product = v1.getNorm() * v2.getNorm();
        // Java: if (normProduct == 0)
        if norm_product == 0.0 {
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM);
            panic!("MathArithmeticException: {:?}", LocalizedFormatsEnum::ZERO_NORM());
        }
        // Java: double dot = v1.dotProduct(v2);
        let dot = v1.dotProduct(v2);
        // Java: double threshold = normProduct * 0.9999;
        let threshold = norm_product * 0.9999;
        // Java: if ((dot < -threshold) || (dot > threshold))
        if (dot < -threshold) || (dot > threshold) {
            // Java: // the vectors are almost aligned, compute using the sine
            // Java: Vector3D v3 = crossProduct(v1, v2);
            let v3 = Vector3D::crossProduct_static(v1, v2);
            // Java: if (dot >= 0)
            if dot >= 0.0 {
                // Java: return FastMath.asin(v3.getNorm() / normProduct);
                return FastMath::asin(v3.getNorm() / norm_product);
            }
            // Java: return FastMath.PI - FastMath.asin(v3.getNorm() / normProduct);
            return std::f64::consts::PI - FastMath::asin(v3.getNorm() / norm_product);
        }
        // Java: // the vectors are sufficiently separated to use the cosine
        // Java: return FastMath.acos(dot / normProduct);
        FastMath::acos(dot / norm_product)
    }

    // Java: Vector3D negate()
    pub fn negate(&self) -> Vector3D {
        // Java: return new Vector3D(-x, -y, -z);
        Vector3D::new(-self.x, -self.y, -self.z)
    }

    // Java: Vector3D scalarMultiply(double a)
    pub fn scalarMultiply(&self, a: f64) -> Vector3D {
        // Java: return new Vector3D(a * x, a * y, a * z);
        Vector3D::new(a * self.x, a * self.y, a * self.z)
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: return Double.isNaN(x) || Double.isNaN(y) || Double.isNaN(z);
        self.x.is_nan() || self.y.is_nan() || self.z.is_nan()
    }

    // Java: boolean isInfinite()
    pub fn isInfinite(&self) -> bool {
        // Java: return !isNaN() && (Double.isInfinite(x) || Double.isInfinite(y) || Double.isInfinite(z));
        !self.isNaN() && (self.x.is_infinite() || self.y.is_infinite() || self.z.is_infinite())
    }

    // Java: boolean equals(Object other)
    pub fn equals_obj(&self, other: &Vector3D) -> bool {
        // Java: if (this == other)
        // Java: return true;
        // (skip identity check in Rust)
        // Java: if (other instanceof Vector3D)
        // Java: final Vector3D rhs = (Vector3D)other;
        // Java: if (rhs.isNaN())
        if other.isNaN() {
            // Java: return this.isNaN();
            return self.isNaN();
        }
        // Java: return (x == rhs.x) && (y == rhs.y) && (z == rhs.z);
        if (self.x == other.x) && (self.y == other.y) && (self.z == other.z) {
            return true;
        }
        // Java: return false;
        false
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: if (isNaN())
        if self.isNaN() {
            // Java: return 642;
            return 642;
        }
        // Java: return 643 * (164 * MathUtils.hash(x) +  3 * MathUtils.hash(y) +  MathUtils.hash(z));
        let hx = (self.x.to_bits() as i64) as i32;
        let hy = (self.y.to_bits() as i64) as i32;
        let hz = (self.z.to_bits() as i64) as i32;
        643_i32.wrapping_mul(164_i32.wrapping_mul(hx).wrapping_add(3_i32.wrapping_mul(hy)).wrapping_add(hz))
    }

    // Java: double dotProduct(final Vector<Euclidean3D> v)
    pub fn dotProduct(&self, v: &Vector3D) -> f64 {
        // Java: final Vector3D v3 = (Vector3D) v;
        // Java: return MathArrays.linearCombination(x, v3.x, y, v3.y, z, v3.z);
        MathArrays::linearCombination_3(self.x, v.x, self.y, v.y, self.z, v.z)
    }

    // Java: Vector3D crossProduct(final Vector<Euclidean3D> v)
    pub fn crossProduct(&self, v: &Vector3D) -> Vector3D {
        // Java: final Vector3D v3 = (Vector3D) v;
        // Java: return new Vector3D(MathArrays.linearCombination(y, v3.z, -z, v3.y),
        // Java: MathArrays.linearCombination(z, v3.x, -x, v3.z),
        // Java: MathArrays.linearCombination(x, v3.y, -y, v3.x));
        Vector3D::new(
            MathArrays::linearCombination_2(self.y, v.z, -self.z, v.y),
            MathArrays::linearCombination_2(self.z, v.x, -self.x, v.z),
            MathArrays::linearCombination_2(self.x, v.y, -self.y, v.x),
        )
    }

    // Java: double distance1(Vector<Euclidean3D> v)
    pub fn distance1(&self, v: &Vector3D) -> f64 {
        // Java: final Vector3D v3 = (Vector3D) v;
        // Java: final double dx = FastMath.abs(v3.x - x);
        let dx = FastMath::abs(v.x - self.x);
        // Java: final double dy = FastMath.abs(v3.y - y);
        let dy = FastMath::abs(v.y - self.y);
        // Java: final double dz = FastMath.abs(v3.z - z);
        let dz = FastMath::abs(v.z - self.z);
        // Java: return dx + dy + dz;
        dx + dy + dz
    }

    // Java: double distance(Vector<Euclidean3D> v)
    pub fn distance_vec(&self, v: &Vector3D) -> f64 {
        // Java: return distance((Point<Euclidean3D>) v);
        self.distance_point(v)
    }

    // Java: double distance(Point<Euclidean3D> v)
    pub fn distance_point(&self, v: &Vector3D) -> f64 {
        // Java: final Vector3D v3 = (Vector3D) v;
        // Java: final double dx = v3.x - x;
        let dx = v.x - self.x;
        // Java: final double dy = v3.y - y;
        let dy = v.y - self.y;
        // Java: final double dz = v3.z - z;
        let dz = v.z - self.z;
        // Java: return FastMath.sqrt(dx * dx + dy * dy + dz * dz);
        FastMath::sqrt(dx * dx + dy * dy + dz * dz)
    }

    // Java: double distanceInf(Vector<Euclidean3D> v)
    pub fn distanceInf(&self, v: &Vector3D) -> f64 {
        // Java: final Vector3D v3 = (Vector3D) v;
        // Java: final double dx = FastMath.abs(v3.x - x);
        let dx = FastMath::abs(v.x - self.x);
        // Java: final double dy = FastMath.abs(v3.y - y);
        let dy = FastMath::abs(v.y - self.y);
        // Java: final double dz = FastMath.abs(v3.z - z);
        let dz = FastMath::abs(v.z - self.z);
        // Java: return FastMath.max(FastMath.max(dx, dy), dz);
        FastMath::max(FastMath::max(dx, dy), dz)
    }

    // Java: double distanceSq(Vector<Euclidean3D> v)
    pub fn distanceSq(&self, v: &Vector3D) -> f64 {
        // Java: final Vector3D v3 = (Vector3D) v;
        // Java: final double dx = v3.x - x;
        let dx = v.x - self.x;
        // Java: final double dy = v3.y - y;
        let dy = v.y - self.y;
        // Java: final double dz = v3.z - z;
        let dz = v.z - self.z;
        // Java: return dx * dx + dy * dy + dz * dz;
        dx * dx + dy * dy + dz * dz
    }

    // Java: static double dotProduct(Vector3D v1, Vector3D v2)
    pub fn dotProduct_static(v1: &Vector3D, v2: &Vector3D) -> f64 {
        // Java: return v1.dotProduct(v2);
        v1.dotProduct(v2)
    }

    // Java: static Vector3D crossProduct(final Vector3D v1, final Vector3D v2)
    pub fn crossProduct_static(v1: &Vector3D, v2: &Vector3D) -> Vector3D {
        // Java: return v1.crossProduct(v2);
        v1.crossProduct(v2)
    }

    // Java: static double distance1(Vector3D v1, Vector3D v2)
    pub fn distance1_static(v1: &Vector3D, v2: &Vector3D) -> f64 {
        // Java: return v1.distance1(v2);
        v1.distance1(v2)
    }

    // Java: static double distance(Vector3D v1, Vector3D v2)
    pub fn distance_static(v1: &Vector3D, v2: &Vector3D) -> f64 {
        // Java: return v1.distance(v2);
        v1.distance_point(v2)
    }

    // Java: static double distanceInf(Vector3D v1, Vector3D v2)
    pub fn distanceInf_static(v1: &Vector3D, v2: &Vector3D) -> f64 {
        // Java: return v1.distanceInf(v2);
        v1.distanceInf(v2)
    }

    // Java: static double distanceSq(Vector3D v1, Vector3D v2)
    pub fn distanceSq_static(v1: &Vector3D, v2: &Vector3D) -> f64 {
        // Java: return v1.distanceSq(v2);
        v1.distanceSq(v2)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return Vector3DFormat.getInstance().format(this);
        format!("{{{}, {}, {}}}", self.x, self.y, self.z)
    }

    // Java: String toString(final NumberFormat format)
    pub fn toString_formatted(&self, _format: &dyn std::any::Any) -> String {
        // Java: return new Vector3DFormat(format).format(this);
        format!("{{{}, {}, {}}}", self.x, self.y, self.z)
    }

}
