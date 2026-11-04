#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import java.text.NumberFormat
// (no direct Rust equivalent)
// Java: import org.apache.commons.math3.exception.DimensionMismatchException
use crate::org::apache::commons::math3::exception::DimensionMismatchException;
// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats;
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

// Java field: Vector2D ZERO
static ZERO: std::sync::OnceLock<Vector2D> = std::sync::OnceLock::new();
// Java field: Vector2D NaN
static NaN: std::sync::OnceLock<Vector2D> = std::sync::OnceLock::new();
// Java field: Vector2D POSITIVE_INFINITY
static POSITIVE_INFINITY: std::sync::OnceLock<Vector2D> = std::sync::OnceLock::new();
// Java field: Vector2D NEGATIVE_INFINITY
static NEGATIVE_INFINITY: std::sync::OnceLock<Vector2D> = std::sync::OnceLock::new();
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 266938651998679754;

// Java: public class represents
pub struct represents {
}

// Java: public class Vector2D
#[derive(Clone, Debug)]
pub struct Vector2D {
    // Java field: double x
    pub(crate) x: f64,
    // Java field: double y
    pub(crate) y: f64,
}

// Java: public class represents
impl represents {
}

// Java: public class Vector2D
impl Vector2D {
    // Java: new Vector2D(Double.POSITIVE_INFINITY, Double.POSITIVE_INFINITY)
    pub fn get_POSITIVE_INFINITY() -> &'static Vector2D {
        POSITIVE_INFINITY.get_or_init(|| Vector2D { x: f64::INFINITY, y: f64::INFINITY })
    }

    // Java: new Vector2D(Double.NEGATIVE_INFINITY, Double.NEGATIVE_INFINITY)
    pub fn get_NEGATIVE_INFINITY() -> &'static Vector2D {
        NEGATIVE_INFINITY.get_or_init(|| Vector2D { x: f64::NEG_INFINITY, y: f64::NEG_INFINITY })
    }

    // Java: Vector2D(double x, double y)
    pub fn new(x: f64, y: f64) -> Self {
        // Java: this.x = x;
        // Java: this.y = y;
        Vector2D { x, y }
    }

    // Java: Vector2D(double[] v)
    pub fn new_from_array(v: &[f64]) -> Self {
        // Java: if (v.length != 2)
        if v.len() != 2
        {
            // Java: throw new DimensionMismatchException(v.length, 2);
            panic!("DimensionMismatchException: expected length 2, got {}", v.len());
        }
        // Java: this.x = v[0];
        // Java: this.y = v[1];
        Vector2D { x: v[0], y: v[1] }
    }

    // Java: Vector2D(double a, Vector2D u)
    pub fn new_scaled(a: f64, u: &Vector2D) -> Self {
        // Java: this.x = a * u.x;
        // Java: this.y = a * u.y;
        Vector2D { x: a * u.x, y: a * u.y }
    }

    // Java: Vector2D(double a1, Vector2D u1, double a2, Vector2D u2)
    pub fn new_linear_combination(a1: f64, u1: &Vector2D, a2: f64, u2: &Vector2D) -> Self {
        // Java: this.x = a1 * u1.x + a2 * u2.x;
        // Java: this.y = a1 * u1.y + a2 * u2.y;
        Vector2D { x: a1 * u1.x + a2 * u2.x, y: a1 * u1.y + a2 * u2.y }
    }

    // Java: Vector2D()
    pub fn get_ZERO() -> &'static Vector2D {
        ZERO.get_or_init(|| Vector2D { x: 0.0, y: 0.0 })
    }

    // Java: Vector2D()
    pub fn get_NaN() -> &'static Vector2D {
        NaN.get_or_init(|| Vector2D { x: f64::NAN, y: f64::NAN })
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

    // Java: double[] toArray()
    pub fn toArray(&self) -> Vec<f64> {
        // Java: return new double[] { x, y };
        vec![self.x, self.y]
    }

    // Java: Space getSpace()
    pub fn getSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return Euclidean2D.getInstance();
        todo!()
    }

    // Java: Vector2D getZero()
    pub fn getZero(&self) -> &'static Vector2D {
        // Java: return ZERO;
        Self::get_ZERO()
    }

    // Java: double getNorm1()
    pub fn getNorm1(&self) -> f64 {
        // Java: return FastMath.abs(x) + FastMath.abs(y);
        FastMath::abs(self.x) + FastMath::abs(self.y)
    }

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: return FastMath.sqrt (x * x + y * y);
        FastMath::sqrt(self.x * self.x + self.y * self.y)
    }

    // Java: double getNormSq()
    pub fn getNormSq(&self) -> f64 {
        // Java: return x * x + y * y;
        self.x * self.x + self.y * self.y
    }

    // Java: double getNormInf()
    pub fn getNormInf(&self) -> f64 {
        // Java: return FastMath.max(FastMath.abs(x), FastMath.abs(y));
        FastMath::max(FastMath::abs(self.x), FastMath::abs(self.y))
    }

    // Java: Vector2D add(Vector<Euclidean2D> v)
    pub fn add(&self, v: &Vector2D) -> Vector2D {
        // Java: Vector2D v2 = (Vector2D) v;
        // Java: return new Vector2D(x + v2.getX(), y + v2.getY());
        Vector2D::new(self.x + v.getX(), self.y + v.getY())
    }

    // Java: Vector2D add(double factor, Vector<Euclidean2D> v)
    pub fn add_scaled(&self, factor: f64, v: &Vector2D) -> Vector2D {
        // Java: Vector2D v2 = (Vector2D) v;
        // Java: return new Vector2D(x + factor * v2.getX(), y + factor * v2.getY());
        Vector2D::new(self.x + factor * v.getX(), self.y + factor * v.getY())
    }

    // Java: Vector2D subtract(Vector<Euclidean2D> p)
    pub fn subtract(&self, p: &Vector2D) -> Vector2D {
        // Java: Vector2D p3 = (Vector2D) p;
        // Java: return new Vector2D(x - p3.x, y - p3.y);
        Vector2D::new(self.x - p.x, self.y - p.y)
    }

    // Java: Vector2D subtract(double factor, Vector<Euclidean2D> v)
    pub fn subtract_scaled(&self, factor: f64, v: &Vector2D) -> Vector2D {
        // Java: Vector2D v2 = (Vector2D) v;
        // Java: return new Vector2D(x - factor * v2.getX(), y - factor * v2.getY());
        Vector2D::new(self.x - factor * v.getX(), self.y - factor * v.getY())
    }

    // Java: Vector2D normalize()
    pub fn normalize(&self) -> Vector2D {
        // Java: double s = getNorm();
        let s = self.getNorm();
        // Java: if (s == 0)
        if s == 0.0
        {
            // Java: throw new MathArithmeticException(LocalizedFormats.CANNOT_NORMALIZE_A_ZERO_NORM_VECTOR);
            panic!("MathArithmeticException: cannot normalize a zero norm vector");
        }
        // Java: return scalarMultiply(1 / s);
        self.scalarMultiply(1.0 / s)
    }

    // Java: static double angle(Vector2D v1, Vector2D v2)
    pub fn angle(v1: &Vector2D, v2: &Vector2D) -> f64 {
        // Java: double normProduct = v1.getNorm() * v2.getNorm();
        let normProduct = v1.getNorm() * v2.getNorm();
        // Java: if (normProduct == 0)
        if normProduct == 0.0
        {
            // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM);
            panic!("MathArithmeticException: zero norm");
        }
        // Java: double dot = v1.dotProduct(v2);
        let dot = v1.dotProduct(v2);
        // Java: double threshold = normProduct * 0.9999;
        let threshold = normProduct * 0.9999;
        // Java: if ((dot < -threshold) || (dot > threshold))
        if (dot < -threshold) || (dot > threshold)
        {
            // Java: // the vectors are almost aligned, compute using the sine
            // Java: final double n = FastMath.abs(MathArrays.linearCombination(v1.x, v2.y, -v1.y, v2.x));
            let n = FastMath::abs(MathArrays::linearCombination_2(v1.x, v2.y, -v1.y, v2.x));
            // Java: if (dot >= 0)
            if dot >= 0.0
            {
                // Java: return FastMath.asin(n / normProduct);
                return FastMath::asin(n / normProduct);
            }
            // Java: return FastMath.PI - FastMath.asin(n / normProduct);
            return std::f64::consts::PI - FastMath::asin(n / normProduct);
        }
        // Java: // the vectors are sufficiently separated to use the cosine
        // Java: return FastMath.acos(dot / normProduct);
        FastMath::acos(dot / normProduct)
    }

    // Java: Vector2D negate()
    pub fn negate(&self) -> Vector2D {
        // Java: return new Vector2D(-x, -y);
        Vector2D::new(-self.x, -self.y)
    }

    // Java: Vector2D scalarMultiply(double a)
    pub fn scalarMultiply(&self, a: f64) -> Vector2D {
        // Java: return new Vector2D(a * x, a * y);
        Vector2D::new(a * self.x, a * self.y)
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: return Double.isNaN(x) || Double.isNaN(y);
        self.x.is_nan() || self.y.is_nan()
    }

    // Java: boolean isInfinite()
    pub fn isInfinite(&self) -> bool {
        // Java: return !isNaN() && (Double.isInfinite(x) || Double.isInfinite(y));
        !self.isNaN() && (self.x.is_infinite() || self.y.is_infinite())
    }

    // Java: double distance1(Vector<Euclidean2D> p)
    pub fn distance1(&self, p: &Vector2D) -> f64 {
        // Java: Vector2D p3 = (Vector2D) p;
        // Java: final double dx = FastMath.abs(p3.x - x);
        let dx = FastMath::abs(p.x - self.x);
        // Java: final double dy = FastMath.abs(p3.y - y);
        let dy = FastMath::abs(p.y - self.y);
        // Java: return dx + dy;
        dx + dy
    }

    // Java: double distance(Vector<Euclidean2D> p)
    pub fn distance_vec(&self, p: &Vector2D) -> f64 {
        // Java: return distance((Point<Euclidean2D>) p);
        self.distance_point(p)
    }

    // Java: double distance(Point<Euclidean2D> p)
    pub fn distance_point(&self, p: &Vector2D) -> f64 {
        // Java: Vector2D p3 = (Vector2D) p;
        // Java: final double dx = p3.x - x;
        let dx = p.x - self.x;
        // Java: final double dy = p3.y - y;
        let dy = p.y - self.y;
        // Java: return FastMath.sqrt(dx * dx + dy * dy);
        FastMath::sqrt(dx * dx + dy * dy)
    }

    // Java: double distanceInf(Vector<Euclidean2D> p)
    pub fn distanceInf(&self, p: &Vector2D) -> f64 {
        // Java: Vector2D p3 = (Vector2D) p;
        // Java: final double dx = FastMath.abs(p3.x - x);
        let dx = FastMath::abs(p.x - self.x);
        // Java: final double dy = FastMath.abs(p3.y - y);
        let dy = FastMath::abs(p.y - self.y);
        // Java: return FastMath.max(dx, dy);
        FastMath::max(dx, dy)
    }

    // Java: double distanceSq(Vector<Euclidean2D> p)
    pub fn distanceSq(&self, p: &Vector2D) -> f64 {
        // Java: Vector2D p3 = (Vector2D) p;
        // Java: final double dx = p3.x - x;
        let dx = p.x - self.x;
        // Java: final double dy = p3.y - y;
        let dy = p.y - self.y;
        // Java: return dx * dx + dy * dy;
        dx * dx + dy * dy
    }

    // Java: double dotProduct(final Vector<Euclidean2D> v)
    pub fn dotProduct(&self, v: &Vector2D) -> f64 {
        // Java: final Vector2D v2 = (Vector2D) v;
        // Java: return MathArrays.linearCombination(x, v2.x, y, v2.y);
        MathArrays::linearCombination_2(self.x, v.x, self.y, v.y)
    }

    // Java: double crossProduct(final Vector2D p1, final Vector2D p2)
    pub fn crossProduct(&self, p1: &Vector2D, p2: &Vector2D) -> f64 {
        // Java: final double x1 = p2.getX() - p1.getX();
        let x1 = p2.getX() - p1.getX();
        // Java: final double y1 = getY() - p1.getY();
        let y1 = self.getY() - p1.getY();
        // Java: final double x2 = getX() - p1.getX();
        let x2 = self.getX() - p1.getX();
        // Java: final double y2 = p2.getY() - p1.getY();
        let y2 = p2.getY() - p1.getY();
        // Java: return MathArrays.linearCombination(x1, y1, -x2, y2);
        MathArrays::linearCombination_2(x1, y1, -x2, y2)
    }

    // Java: static double distance(Vector2D p1, Vector2D p2)
    pub fn distance_static(p1: &Vector2D, p2: &Vector2D) -> f64 {
        // Java: return p1.distance(p2);
        p1.distance_point(p2)
    }

    // Java: static double distanceInf(Vector2D p1, Vector2D p2)
    pub fn distanceInf_static(p1: &Vector2D, p2: &Vector2D) -> f64 {
        // Java: return p1.distanceInf(p2);
        p1.distanceInf(p2)
    }

    // Java: static double distanceSq(Vector2D p1, Vector2D p2)
    pub fn distanceSq_static(p1: &Vector2D, p2: &Vector2D) -> f64 {
        // Java: return p1.distanceSq(p2);
        p1.distanceSq(p2)
    }

    // Java: boolean equals(Object other)
    pub fn equals_obj(&self, other: &Vector2D) -> bool {
        // Java: if (this == other)
        if std::ptr::eq(self, other)
        {
            // Java: return true;
            return true;
        }
        // Java: if (other instanceof Vector2D)
        {
            // Java: final Vector2D rhs = (Vector2D)other;
            // Java: if (rhs.isNaN())
            if other.isNaN()
            {
                // Java: return this.isNaN();
                return self.isNaN();
            }
            // Java: return (x == rhs.x) && (y == rhs.y);
            return (self.x == other.x) && (self.y == other.y);
        }
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: if (isNaN())
        if self.isNaN()
        {
            // Java: return 542;
            return 542;
        }
        // Java: return 122 * (76 * MathUtils.hash(x) +  MathUtils.hash(y));
        122 * (76 * (self.x.to_bits() as i32) + (self.y.to_bits() as i32))
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return Vector2DFormat.getInstance().format(this);
        format!("{{{};{}}}", self.x, self.y)
    }

    // Java: String toString(final NumberFormat format)
    pub fn toString_formatted(&self, _format: &dyn std::any::Any) -> String {
        // Java: return new Vector2DFormat(format).format(this);
        format!("{{{};{}}}", self.x, self.y)
    }

}

impl PartialEq for Vector2D {
    fn eq(&self, other: &Self) -> bool {
        self.equals_obj(other)
    }
}
