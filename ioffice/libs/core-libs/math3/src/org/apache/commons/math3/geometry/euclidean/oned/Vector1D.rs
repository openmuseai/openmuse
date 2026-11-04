#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import java.text.NumberFormat
// (no direct Rust equivalent)
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
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: Vector1D ZERO
static ZERO: std::sync::OnceLock<Vector1D> = std::sync::OnceLock::new();
// Java field: Vector1D ONE
static ONE: std::sync::OnceLock<Vector1D> = std::sync::OnceLock::new();
// Java field: Vector1D NaN
static NaN: std::sync::OnceLock<Vector1D> = std::sync::OnceLock::new();
// Java field: Vector1D POSITIVE_INFINITY
static POSITIVE_INFINITY: std::sync::OnceLock<Vector1D> = std::sync::OnceLock::new();
// Java field: Vector1D NEGATIVE_INFINITY
static NEGATIVE_INFINITY: std::sync::OnceLock<Vector1D> = std::sync::OnceLock::new();
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 7556674948671647925;

// Java: public class represents
pub struct represents {
}

// Java: public class Vector1D
#[derive(Clone, Debug)]
pub struct Vector1D {
    // Java field: double x
    pub(crate) x: f64,
}

// Java: public class represents
impl represents {
}

// Java: public class Vector1D
impl Vector1D {
    // Java: new Vector1D(Double.POSITIVE_INFINITY)
    pub fn get_POSITIVE_INFINITY() -> &'static Vector1D {
        POSITIVE_INFINITY.get_or_init(|| Vector1D { x: f64::INFINITY })
    }

    // Java: new Vector1D(Double.NEGATIVE_INFINITY)
    pub fn get_NEGATIVE_INFINITY() -> &'static Vector1D {
        NEGATIVE_INFINITY.get_or_init(|| Vector1D { x: f64::NEG_INFINITY })
    }

    // Java: Vector1D(double x)
    pub fn new(x: f64) -> Self {
        // Java: this.x = x;
        Vector1D { x }
    }

    // Java: Vector1D(double a, Vector1D u)
    pub fn new_scaled(a: f64, u: &Vector1D) -> Self {
        // Java: this.x = a * u.x;
        Vector1D { x: a * u.x }
    }

    // Java: Vector1D(double a1, Vector1D u1, double a2, Vector1D u2)
    pub fn new_linear_combination(a1: f64, u1: &Vector1D, a2: f64, u2: &Vector1D) -> Self {
        // Java: this.x = a1 * u1.x + a2 * u2.x;
        Vector1D { x: a1 * u1.x + a2 * u2.x }
    }

    // Java: Vector1D()
    pub fn get_ZERO() -> &'static Vector1D {
        ZERO.get_or_init(|| Vector1D { x: 0.0 })
    }

    // Java: Vector1D()
    pub fn get_ONE() -> &'static Vector1D {
        ONE.get_or_init(|| Vector1D { x: 1.0 })
    }

    // Java: double getX()
    pub fn getX(&self) -> f64 {
        // Java: return x;
        self.x
    }

    // Java: Space getSpace()
    pub fn getSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return Euclidean1D.getInstance();
        todo!()
    }

    // Java: Vector1D getZero()
    pub fn getZero(&self) -> &'static Vector1D {
        // Java: return ZERO;
        Self::get_ZERO()
    }

    // Java: double getNorm1()
    pub fn getNorm1(&self) -> f64 {
        // Java: return FastMath.abs(x);
        FastMath::abs(self.x)
    }

    // Java: double getNorm()
    pub fn getNorm(&self) -> f64 {
        // Java: return FastMath.abs(x);
        FastMath::abs(self.x)
    }

    // Java: double getNormSq()
    pub fn getNormSq(&self) -> f64 {
        // Java: return x * x;
        self.x * self.x
    }

    // Java: double getNormInf()
    pub fn getNormInf(&self) -> f64 {
        // Java: return FastMath.abs(x);
        FastMath::abs(self.x)
    }

    // Java: Vector1D add(Vector<Euclidean1D> v)
    pub fn add(&self, v: &Vector1D) -> Vector1D {
        // Java: Vector1D v1 = (Vector1D) v;
        // Java: return new Vector1D(x + v1.getX());
        Vector1D::new(self.x + v.getX())
    }

    // Java: Vector1D add(double factor, Vector<Euclidean1D> v)
    pub fn add_scaled(&self, factor: f64, v: &Vector1D) -> Vector1D {
        // Java: Vector1D v1 = (Vector1D) v;
        // Java: return new Vector1D(x + factor * v1.getX());
        Vector1D::new(self.x + factor * v.getX())
    }

    // Java: Vector1D subtract(Vector<Euclidean1D> p)
    pub fn subtract(&self, p: &Vector1D) -> Vector1D {
        // Java: Vector1D p3 = (Vector1D) p;
        // Java: return new Vector1D(x - p3.x);
        Vector1D::new(self.x - p.x)
    }

    // Java: Vector1D subtract(double factor, Vector<Euclidean1D> v)
    pub fn subtract_scaled(&self, factor: f64, v: &Vector1D) -> Vector1D {
        // Java: Vector1D v1 = (Vector1D) v;
        // Java: return new Vector1D(x - factor * v1.getX());
        Vector1D::new(self.x - factor * v.getX())
    }

    // Java: Vector1D normalize()
    pub fn normalize(&self) -> Vector1D {
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

    // Java: Vector1D negate()
    pub fn negate(&self) -> Vector1D {
        // Java: return new Vector1D(-x);
        Vector1D::new(-self.x)
    }

    // Java: Vector1D scalarMultiply(double a)
    pub fn scalarMultiply(&self, a: f64) -> Vector1D {
        // Java: return new Vector1D(a * x);
        Vector1D::new(a * self.x)
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: return Double.isNaN(x);
        self.x.is_nan()
    }

    // Java: boolean isInfinite()
    pub fn isInfinite(&self) -> bool {
        // Java: return !isNaN() && Double.isInfinite(x);
        !self.isNaN() && self.x.is_infinite()
    }

    // Java: double distance1(Vector<Euclidean1D> p)
    pub fn distance1(&self, p: &Vector1D) -> f64 {
        // Java: Vector1D p3 = (Vector1D) p;
        // Java: final double dx = FastMath.abs(p3.x - x);
        let dx = FastMath::abs(p.x - self.x);
        // Java: return dx;
        dx
    }

    // Java: double distance(Vector<Euclidean1D> p)
    pub fn distance_vec(&self, p: &Vector1D) -> f64 {
        // Java: return distance((Point<Euclidean1D>) p);
        self.distance_point(p)
    }

    // Java: double distance(Point<Euclidean1D> p)
    pub fn distance_point(&self, p: &Vector1D) -> f64 {
        // Java: Vector1D p3 = (Vector1D) p;
        // Java: final double dx = p3.x - x;
        let dx = p.x - self.x;
        // Java: return FastMath.abs(dx);
        FastMath::abs(dx)
    }

    // Java: double distanceInf(Vector<Euclidean1D> p)
    pub fn distanceInf(&self, p: &Vector1D) -> f64 {
        // Java: Vector1D p3 = (Vector1D) p;
        // Java: final double dx = FastMath.abs(p3.x - x);
        let dx = FastMath::abs(p.x - self.x);
        // Java: return dx;
        dx
    }

    // Java: double distanceSq(Vector<Euclidean1D> p)
    pub fn distanceSq(&self, p: &Vector1D) -> f64 {
        // Java: Vector1D p3 = (Vector1D) p;
        // Java: final double dx = p3.x - x;
        let dx = p.x - self.x;
        // Java: return dx * dx;
        dx * dx
    }

    // Java: double dotProduct(final Vector<Euclidean1D> v)
    pub fn dotProduct(&self, v: &Vector1D) -> f64 {
        // Java: final Vector1D v1 = (Vector1D) v;
        // Java: return x * v1.x;
        self.x * v.x
    }

    // Java: static double distance(Vector1D p1, Vector1D p2)
    pub fn distance_static(p1: &Vector1D, p2: &Vector1D) -> f64 {
        // Java: return p1.distance(p2);
        p1.distance_point(p2)
    }

    // Java: static double distanceInf(Vector1D p1, Vector1D p2)
    pub fn distanceInf_static(p1: &Vector1D, p2: &Vector1D) -> f64 {
        // Java: return p1.distanceInf(p2);
        p1.distanceInf(p2)
    }

    // Java: static double distanceSq(Vector1D p1, Vector1D p2)
    pub fn distanceSq_static(p1: &Vector1D, p2: &Vector1D) -> f64 {
        // Java: return p1.distanceSq(p2);
        p1.distanceSq(p2)
    }

    // Java: boolean equals(Object other)
    pub fn equals_obj(&self, other: &Vector1D) -> bool {
        // Java: if (this == other)
        if std::ptr::eq(self, other)
        {
            // Java: return true;
            return true;
        }
        // Java: if (other instanceof Vector1D)
        {
            // Java: final Vector1D rhs = (Vector1D)other;
            // Java: if (rhs.isNaN())
            if other.isNaN()
            {
                // Java: return this.isNaN();
                return self.isNaN();
            }
            // Java: return x == rhs.x;
            return self.x == other.x;
        }
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: if (isNaN())
        if self.isNaN()
        {
            // Java: return 7785;
            return 7785;
        }
        // Java: return 997 * MathUtils.hash(x);
        997 * (self.x.to_bits() as i32)
    }

    // Java: String toString()
    pub fn toString(&self) -> String {
        // Java: return Vector1DFormat.getInstance().format(this);
        format!("{{{}}}", self.x)
    }

    // Java: String toString(final NumberFormat format)
    pub fn toString_formatted(&self, _format: &dyn std::any::Any) -> String {
        // Java: return new Vector1DFormat(format).format(this);
        format!("{{{}}}", self.x)
    }

}

impl PartialEq for Vector1D {
    fn eq(&self, other: &Self) -> bool {
        self.equals_obj(other)
    }
}
