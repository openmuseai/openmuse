#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

use std::sync::OnceLock;

// Java field: S1Point NaN
static NaN_POINT: OnceLock<S1Point> = OnceLock::new();
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 7530459720816214295;

// Java: public class represents
pub struct represents {
}

// Java: public class S1Point
pub struct S1Point {
    // Java field: double alpha
    pub(crate) alpha: f64,
    // Java field: Vector2D vector
    pub(crate) vector: Box<dyn std::any::Any + Send + Sync>,
}

// Java: public class represents
impl represents {
}

// Java: public class S1Point
impl S1Point {
    // Java: S1Point(final double alpha)
    pub fn new(alpha: f64) -> Self {
        // Java: this(MathUtils.normalizeAngle(alpha, FastMath.PI),
        // Java: new Vector2D(FastMath.cos(alpha), FastMath.sin(alpha)));
        S1Point {
            alpha,
            vector: Box::new(0i32),
        }
    }

    // Java: S1Point(final double alpha, final Vector2D vector)
    pub fn new_with_vector(alpha: f64, vector: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: this.alpha  = alpha;
        // Java: this.vector = vector;
        S1Point { alpha, vector }
    }

    pub fn NaN() -> &'static S1Point {
        NaN_POINT.get_or_init(|| S1Point::new(f64::NAN))
    }

    // Java: double getAlpha()
    pub fn getAlpha(&self) -> f64 {
        // Java: return alpha;
        self.alpha
    }

    // Java: Vector2D getVector()
    pub fn getVector(&self) -> &dyn std::any::Any {
        // Java: return vector;
        &*self.vector
    }

    // Java: Space getSpace()
    pub fn getSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return Sphere1D.getInstance();
        todo!()
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: return Double.isNaN(alpha);
        self.alpha.is_nan()
    }

    // Java: double distance(final Point<Sphere1D> point)
    pub fn distance(&self, _point: &dyn std::any::Any) -> f64 {
        // Java: return distance(this, (S1Point) point);
        todo!()
    }

    // Java: static double distance(S1Point p1, S1Point p2)
    pub fn distance_static(_p1: &S1Point, _p2: &S1Point) -> f64 {
        // Java: return Vector2D.angle(p1.vector, p2.vector);
        todo!()
    }

    // Java: boolean equals(Object other)
    pub fn equals_obj(&self, other: &S1Point) -> bool {
        // Java: if (this == other)
        // Java: return true;
        // Java: if (other instanceof S1Point)
        // Java: final S1Point rhs = (S1Point) other;
        // Java: if (rhs.isNaN())
        if other.isNaN() {
            // Java: return this.isNaN();
            return self.isNaN();
        }
        // Java: return alpha == rhs.alpha;
        self.alpha == other.alpha
        // Java: return false;
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: if (isNaN())
        if self.isNaN() {
            // Java: return 542;
            return 542;
        }
        // Java: return 1759 * MathUtils.hash(alpha);
        1759i32.wrapping_mul(self.alpha.to_bits() as i32)
    }

}
