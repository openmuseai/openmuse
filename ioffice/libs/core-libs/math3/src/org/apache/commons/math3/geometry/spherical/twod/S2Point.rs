// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.OutOfRangeException
use crate::org::apache::commons::math3::exception::OutOfRangeException;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Vector3D
use crate::org::apache::commons::math3::geometry::euclidean::threed::Vector3D;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: S2Point PLUS_I
// Java field: S2Point PLUS_J
// Java field: S2Point PLUS_K
// Java field: S2Point MINUS_I
// Java field: S2Point MINUS_J
// Java field: S2Point MINUS_K
// Java field: S2Point NaN
// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 20131218;

// Java: public class represents
pub struct represents {
}

// Java: public class S2Point
pub struct S2Point {
    // Java field: double theta
    pub(crate) theta: f64,
    // Java field: double phi
    pub(crate) phi: f64,
    // Java field: Vector3D vector
    pub(crate) vector: Box<dyn std::any::Any + Send + Sync>,
}

// Java: public class represents
impl represents {
}

// Java: public class S2Point
#[allow(non_snake_case)]
impl S2Point {
    // Java: S2Point(final double theta, final double phi)
    pub fn new(theta: f64, phi: f64) -> Self {
        // Java: this(theta, phi, vector(theta, phi));
        let vector: Box<dyn std::any::Any + Send + Sync> = Box::new(0i32);
        S2Point { theta, phi, vector }
    }

    // Java: S2Point(final Vector3D vector)
    pub fn from_vector(_vector: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: this(FastMath.atan2(vector.getY(), vector.getX()), Vector3D.angle(Vector3D.PLUS_K, vector),
        // Java: vector.normalize());
        todo!()
    }

    // Java: S2Point(final double theta, final double phi, final Vector3D vector)
    pub fn from_components(theta: f64, phi: f64, vector: Box<dyn std::any::Any + Send + Sync>) -> Self {
        // Java: this.theta  = theta;
        // Java: this.phi    = phi;
        // Java: this.vector = vector;
        S2Point { theta, phi, vector }
    }

    // Java: static Vector3D vector(final double theta, final double phi)
    pub fn vector_from_angles(_theta: f64, _phi: f64) -> Box<dyn std::any::Any + Send + Sync> {
        // Java: if (phi < 0 || phi > FastMath.PI)
        // Java: throw new OutOfRangeException(phi, 0, FastMath.PI);
        // Java: final double cosTheta = FastMath.cos(theta);
        // Java: final double sinTheta = FastMath.sin(theta);
        // Java: final double cosPhi   = FastMath.cos(phi);
        // Java: final double sinPhi   = FastMath.sin(phi);
        // Java: return new Vector3D(cosTheta * sinPhi, sinTheta * sinPhi, cosPhi);
        todo!()
    }

    // Java: double getTheta()
    pub fn getTheta(&self) -> f64 {
        // Java: return theta;
        self.theta
    }

    // Java: double getPhi()
    pub fn getPhi(&self) -> f64 {
        // Java: return phi;
        self.phi
    }

    // Java: Vector3D getVector()
    pub fn getVector(&self) -> &Box<dyn std::any::Any + Send + Sync> {
        // Java: return vector;
        &self.vector
    }

    // Java: Space getSpace()
    pub fn getSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return Sphere2D.getInstance();
        todo!()
    }

    // Java: boolean isNaN()
    pub fn isNaN(&self) -> bool {
        // Java: return Double.isNaN(theta) || Double.isNaN(phi);
        self.theta.is_nan() || self.phi.is_nan()
    }

    // Java: S2Point negate()
    pub fn negate(&self) -> Box<dyn std::any::Any> {
        // Java: return new S2Point(-theta, FastMath.PI - phi, vector.negate());
        todo!()
    }

    // Java: double distance(final Point<Sphere2D> point)
    pub fn distance(&self, _point: &Box<dyn std::any::Any>) -> f64 {
        // Java: return distance(this, (S2Point) point);
        todo!()
    }

    // Java: static double distance(S2Point p1, S2Point p2)
    pub fn distance_between(_p1: &S2Point, _p2: &S2Point) -> f64 {
        // Java: return Vector3D.angle(p1.vector, p2.vector);
        todo!()
    }

    // Java: boolean equals(Object other)
    pub fn equals(&self, _other: &dyn std::any::Any) -> bool {
        // Java: if (this == other)
        {
            // Java: return true;
        }
        // Java: if (other instanceof S2Point)
        {
            // Java: final S2Point rhs = (S2Point) other;
            // Java: if (rhs.isNaN())
            {
                // Java: return this.isNaN();
            }
            // Java: return (theta == rhs.theta) && (phi == rhs.phi);
        }
        // Java: return false;
        todo!()
    }

    // Java: int hashCode()
    pub fn hashCode(&self) -> i32 {
        // Java: if (isNaN())
        {
            // Java: return 542;
        }
        // Java: return 134 * (37 * MathUtils.hash(theta) +  MathUtils.hash(phi));
        todo!()
    }

}
