#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Rotation
use crate::org::apache::commons::math3::geometry::euclidean::threed::Rotation;
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Vector3D
use crate::org::apache::commons::math3::geometry::euclidean::threed::Vector3D;
// Java: import org.apache.commons.math3.geometry.partitioning.Embedding
use crate::org::apache::commons::math3::geometry::partitioning::Embedding;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Transform
use crate::org::apache::commons::math3::geometry::partitioning::Transform;
// Java: import org.apache.commons.math3.geometry.spherical.oned.Arc
use crate::org::apache::commons::math3::geometry::spherical::oned::Arc;
// Java: import org.apache.commons.math3.geometry.spherical.oned.ArcsSet
use crate::org::apache::commons::math3::geometry::spherical::oned::ArcsSet;
// Java: import org.apache.commons.math3.geometry.spherical.oned.S1Point
use crate::org::apache::commons::math3::geometry::spherical::oned::S1Point;
// Java: import org.apache.commons.math3.geometry.spherical.oned.Sphere1D
use crate::org::apache::commons::math3::geometry::spherical::oned::Sphere1D;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;

// Java: public class represents
pub struct represents {
}

// Java: public class Circle
pub struct Circle {
    // Java field: Vector3D pole
    pub(crate) pole: Box<dyn std::any::Any>,
    // Java field: Vector3D x
    pub(crate) x: Box<dyn std::any::Any>,
    // Java field: Vector3D y
    pub(crate) y: Box<dyn std::any::Any>,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
}

// Java: public class CircleTransform
pub struct CircleTransform {
    // Java field: Rotation rotation
    pub(crate) rotation: Box<dyn std::any::Any>,
}

// Java: public class represents
impl represents {
}

// Java: public class Circle
impl Circle {
    // Java: Circle(final Vector3D pole, final double tolerance)
    pub fn new(_pole: Box<dyn std::any::Any>, tolerance: f64) -> Self {
        // Java: reset(pole);
        // Java: this.tolerance = tolerance;
        todo!()
    }

    // Java: Circle(final S2Point first, final S2Point second, final double tolerance)
    pub fn new_from_points(_first: &dyn std::any::Any, _second: &dyn std::any::Any, tolerance: f64) -> Self {
        // Java: reset(first.getVector().crossProduct(second.getVector()));
        // Java: this.tolerance = tolerance;
        todo!()
    }

    // Java: Circle()
    pub fn new_default() -> Self {
        todo!()
    }

    // Java: Circle(final Circle circle)
    pub fn new_copy(_circle: &Circle) -> Self {
        // Java: this(circle.pole, circle.x, circle.y, circle.tolerance);
        todo!()
    }

    // Java: Circle copySelf()
    pub fn copySelf(&self) -> Box<dyn std::any::Any> {
        // Java: return new Circle(this);
        todo!()
    }

    // Java: void reset(final Vector3D newPole)
    pub fn reset(&mut self, _new_pole: &dyn std::any::Any) {
        // Java: this.pole = newPole.normalize();
        // Java: this.x    = newPole.orthogonal();
        // Java: this.y    = Vector3D.crossProduct(newPole, x).normalize();
        todo!()
    }

    // Java: void revertSelf()
    pub fn revertSelf(&mut self) {
        // Java: // x remains the same
        // Java: y    = y.negate();
        // Java: pole = pole.negate();
        todo!()
    }

    // Java: Circle getReverse()
    pub fn getReverse(&self) -> Box<dyn std::any::Any> {
        // Java: return new Circle(pole.negate(), x, y.negate(), tolerance);
        todo!()
    }

    // Java: Point<Sphere2D> project(Point<Sphere2D> point)
    pub fn project(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return toSpace(toSubSpace(point));
        todo!()
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        self.tolerance
    }

    // Java: S1Point toSubSpace(final Point<Sphere2D> point)
    pub fn toSubSpace(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return new S1Point(getPhase(((S2Point) point).getVector()));
        todo!()
    }

    // Java: double getPhase(final Vector3D direction)
    pub fn getPhase(&self, _direction: &dyn std::any::Any) -> f64 {
        // Java: return FastMath.PI + FastMath.atan2(-direction.dotProduct(y), -direction.dotProduct(x));
        todo!()
    }

    // Java: S2Point toSpace(final Point<Sphere1D> point)
    pub fn toSpace(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return new S2Point(getPointAt(((S1Point) point).getAlpha()));
        todo!()
    }

    // Java: Vector3D getPointAt(final double alpha)
    pub fn getPointAt(&self, _alpha: f64) -> Box<dyn std::any::Any> {
        // Java: return new Vector3D(FastMath.cos(alpha), x, FastMath.sin(alpha), y);
        todo!()
    }

    // Java: Vector3D getXAxis()
    pub fn getXAxis(&self) -> &dyn std::any::Any {
        // Java: return x;
        &*self.x
    }

    // Java: Vector3D getYAxis()
    pub fn getYAxis(&self) -> &dyn std::any::Any {
        // Java: return y;
        &*self.y
    }

    // Java: Vector3D getPole()
    pub fn getPole(&self) -> &dyn std::any::Any {
        // Java: return pole;
        &*self.pole
    }

    // Java: Arc getInsideArc(final Circle other)
    pub fn getInsideArc(&self, _other: &Circle) -> Box<dyn std::any::Any> {
        // Java: final double alpha  = getPhase(other.pole);
        // Java: final double halfPi = 0.5 * FastMath.PI;
        // Java: return new Arc(alpha - halfPi, alpha + halfPi, tolerance);
        todo!()
    }

    // Java: SubCircle wholeHyperplane()
    pub fn wholeHyperplane(&self) -> Box<dyn std::any::Any> {
        // Java: return new SubCircle(this, new ArcsSet(tolerance));
        todo!()
    }

    // Java: SphericalPolygonsSet wholeSpace()
    pub fn wholeSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return new SphericalPolygonsSet(tolerance);
        todo!()
    }

    // Java: double getOffset(final Point<Sphere2D> point)
    pub fn getOffset(&self, _point: &dyn std::any::Any) -> f64 {
        // Java: return getOffset(((S2Point) point).getVector());
        todo!()
    }

    // Java: double getOffset(final Vector3D direction)
    pub fn getOffset_vec(&self, _direction: &dyn std::any::Any) -> f64 {
        // Java: return Vector3D.angle(pole, direction) - 0.5 * FastMath.PI;
        todo!()
    }

    // Java: boolean sameOrientationAs(final Hyperplane<Sphere2D> other)
    pub fn sameOrientationAs(&self, _other: &dyn std::any::Any) -> bool {
        // Java: final Circle otherC = (Circle) other;
        // Java: return Vector3D.dotProduct(pole, otherC.pole) >= 0.0;
        todo!()
    }

    // Java: static public static Transform<Sphere2D, Sphere1D> getTransform(final Rotation rotation)
    pub fn getTransform(_rotation: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: return new CircleTransform(rotation);
        todo!()
    }

}

// Java: public class CircleTransform
impl CircleTransform {
    // Java: S2Point apply(final Point<Sphere2D> point)
    pub fn apply_point(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return new S2Point(rotation.applyTo(((S2Point) point).getVector()));
        todo!()
    }

    // Java: Circle apply(final Hyperplane<Sphere2D> hyperplane)
    pub fn apply_hyperplane(&self, _hyperplane: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Circle circle = (Circle) hyperplane;
        // Java: return new Circle(rotation.applyTo(circle.pole),
        // Java: rotation.applyTo(circle.x),
        // Java: rotation.applyTo(circle.y),
        // Java: circle.tolerance);
        todo!()
    }

    // Java: SubHyperplane<Sphere1D> apply()
    pub fn apply_sub(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}
