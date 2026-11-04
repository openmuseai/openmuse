#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import org.apache.commons.math3.exception.MathArithmeticException
use crate::org::apache::commons::math3::exception::MathArithmeticException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Vector
use crate::org::apache::commons::math3::geometry::Vector;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Euclidean1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Euclidean1D;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Vector1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Vector1D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Euclidean2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Euclidean2D;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.PolygonsSet
use crate::org::apache::commons::math3::geometry::euclidean::twod::PolygonsSet;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.geometry.partitioning.Embedding
use crate::org::apache::commons::math3::geometry::partitioning::Embedding;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
use crate::org::apache::commons::math3::exception::util::LocalizedFormats::LocalizedFormats as LocalizedFormatsEnum;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represent
pub struct represent {
}

// Java: public class Plane
pub struct Plane {
    // Java field: double originOffset
    pub(crate) originOffset: f64,
    // Java field: Vector3D origin
    pub(crate) origin: Box<dyn std::any::Any>,
    // Java field: Vector3D u
    pub(crate) u: Box<dyn std::any::Any>,
    // Java field: Vector3D v
    pub(crate) v: Box<dyn std::any::Any>,
    // Java field: Vector3D w
    pub(crate) w: Box<dyn std::any::Any>,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
}

// Java: public class represent
impl represent {
}

// Java: public class Plane
impl Plane {
    // Java: Plane(final Vector3D normal, final double tolerance)
    pub fn new_from_normal(normal: Box<dyn std::any::Any>, tolerance: f64) -> Self {
        // Java: this(normal, DEFAULT_TOLERANCE);
        todo!()
    }

    // Java: Plane(final Vector3D p, final Vector3D normal, final double tolerance)
    pub fn new(p: Box<dyn std::any::Any>, normal: Box<dyn std::any::Any>, tolerance: f64) -> Self {
        todo!()
    }

    // Java: Plane(final Vector3D p1, final Vector3D p2, final Vector3D p3, final double tolerance)
    pub fn new_from_3points(p1: Box<dyn std::any::Any>, p2: Box<dyn std::any::Any>, p3: Box<dyn std::any::Any>, tolerance: f64) -> Self {
        todo!()
    }

    // Java: Plane(final Vector3D normal)
    pub fn new_from_normal_default(normal: Box<dyn std::any::Any>) -> Self {
        // Java: this(normal, DEFAULT_TOLERANCE);
        Self::new_from_normal(normal, DEFAULT_TOLERANCE)
    }

    // Java: Plane(final Vector3D p, final Vector3D normal)
    pub fn new_default(p: Box<dyn std::any::Any>, normal: Box<dyn std::any::Any>) -> Self {
        // Java: this(p, normal, DEFAULT_TOLERANCE);
        Self::new(p, normal, DEFAULT_TOLERANCE)
    }

    // Java: Plane(final Vector3D p1, final Vector3D p2, final Vector3D p3)
    pub fn new_from_3points_default(p1: Box<dyn std::any::Any>, p2: Box<dyn std::any::Any>, p3: Box<dyn std::any::Any>) -> Self {
        todo!()
    }

    // Java: Plane(final Plane plane)
    pub fn new_copy(plane: &Plane) -> Self {
        // Java: originOffset = plane.originOffset;
        // Java: origin       = plane.origin;
        // Java: u            = plane.u;
        // Java: v            = plane.v;
        // Java: w            = plane.w;
        // Java: tolerance    = plane.tolerance;
        todo!()
    }

    // Java: Plane copySelf()
    pub fn copySelf(&self) -> Box<dyn std::any::Any> {
        // Java: return new Plane(this);
        todo!()
    }

    // Java: void reset(final Vector3D p, final Vector3D normal)
    pub fn reset_from_point(&mut self, _p: &dyn std::any::Any, _normal: &dyn std::any::Any) {
        // Java: setNormal(normal);
        // Java: originOffset = -p.dotProduct(w);
        // Java: setFrame();
        todo!()
    }

    // Java: void reset(final Plane original)
    pub fn reset_from_plane(&mut self, original: &Plane) {
        // Java: originOffset = original.originOffset;
        // Java: origin       = original.origin;
        // Java: u            = original.u;
        // Java: v            = original.v;
        // Java: w            = original.w;
        todo!()
    }

    // Java: void setNormal(final Vector3D normal)
    pub fn setNormal(&mut self, _normal: &dyn std::any::Any) {
        // Java: final double norm = normal.getNorm();
        // Java: if (norm < 1.0e-10)
        // Java: throw new MathArithmeticException(LocalizedFormats.ZERO_NORM);
        // Java: w = new Vector3D(1.0 / norm, normal);
        todo!()
    }

    // Java: void setFrame()
    pub fn setFrame(&mut self) {
        // Java: origin = new Vector3D(-originOffset, w);
        // Java: u = w.orthogonal();
        // Java: v = Vector3D.crossProduct(w, u);
        todo!()
    }

    // Java: Vector3D getOrigin()
    pub fn getOrigin(&self) -> &Box<dyn std::any::Any> {
        // Java: return origin;
        &self.origin
    }

    // Java: Vector3D getNormal()
    pub fn getNormal(&self) -> &Box<dyn std::any::Any> {
        // Java: return w;
        &self.w
    }

    // Java: Vector3D getU()
    pub fn getU(&self) -> &Box<dyn std::any::Any> {
        // Java: return u;
        &self.u
    }

    // Java: Vector3D getV()
    pub fn getV(&self) -> &Box<dyn std::any::Any> {
        // Java: return v;
        &self.v
    }

    // Java: Point<Euclidean3D> project(Point<Euclidean3D> point)
    pub fn project(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return toSpace(toSubSpace(point));
        todo!()
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        self.tolerance
    }

    // Java: void revertSelf()
    pub fn revertSelf(&mut self) {
        // Java: final Vector3D tmp = u;
        // Java: u = v;
        // Java: v = tmp;
        // Java: w = w.negate();
        // Java: originOffset = -originOffset;
        todo!()
    }

    // Java: Vector2D toSubSpace(Vector<Euclidean3D> vector)
    pub fn toSubSpace_vec(&self, _vector: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return toSubSpace((Point<Euclidean3D>) vector);
        todo!()
    }

    // Java: Vector3D toSpace(Vector<Euclidean2D> vector)
    pub fn toSpace_vec(&self, _vector: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return toSpace((Point<Euclidean2D>) vector);
        todo!()
    }

    // Java: Vector2D toSubSpace(final Point<Euclidean3D> point)
    pub fn toSubSpace(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Vector3D p3D = (Vector3D) point;
        // Java: return new Vector2D(p3D.dotProduct(u), p3D.dotProduct(v));
        todo!()
    }

    // Java: Vector3D toSpace(final Point<Euclidean2D> point)
    pub fn toSpace(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Vector2D p2D = (Vector2D) point;
        // Java: return new Vector3D(p2D.getX(), u, p2D.getY(), v, -originOffset, w);
        todo!()
    }

    // Java: Vector3D getPointAt(final Vector2D inPlane, final double offset)
    pub fn getPointAt(&self, _inPlane: &dyn std::any::Any, _offset: f64) -> Box<dyn std::any::Any> {
        // Java: return new Vector3D(inPlane.getX(), u, inPlane.getY(), v, offset - originOffset, w);
        todo!()
    }

    // Java: boolean isSimilarTo(final Plane plane)
    pub fn isSimilarTo(&self, _plane: &Plane) -> bool {
        // Java: final double angle = Vector3D.angle(w, plane.w);
        // Java: return ((angle < 1.0e-10) && (FastMath.abs(originOffset - plane.originOffset) < tolerance)) ||
        // Java: ((angle > (FastMath.PI - 1.0e-10)) && (FastMath.abs(originOffset + plane.originOffset) < tolerance));
        todo!()
    }

    // Java: Plane rotate(final Vector3D center, final Rotation rotation)
    pub fn rotate(&self, _center: &dyn std::any::Any, _rotation: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Vector3D delta = origin.subtract(center);
        // Java: final Plane plane = new Plane(center.add(rotation.applyTo(delta)),
        // Java: rotation.applyTo(w), tolerance);
        // Java: // make sure the frame is transformed as desired
        // Java: plane.u = rotation.applyTo(u);
        // Java: plane.v = rotation.applyTo(v);
        // Java: return plane;
        todo!()
    }

    // Java: Plane translate(final Vector3D translation)
    pub fn translate(&self, _translation: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Plane plane = new Plane(origin.add(translation), w, tolerance);
        // Java: // make sure the frame is transformed as desired
        // Java: plane.u = u;
        // Java: plane.v = v;
        // Java: return plane;
        todo!()
    }

    // Java: Vector3D intersection(final Line line)
    pub fn intersection_line(&self, _line: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
        // Java: final Vector3D direction = line.getDirection();
        // Java: final double   dot       = w.dotProduct(direction);
        // Java: if (FastMath.abs(dot) < 1.0e-10)
        // Java: return null;
        // Java: final Vector3D point = line.toSpace((Point<Euclidean1D>) Vector1D.ZERO);
        // Java: final double   k     = -(originOffset + w.dotProduct(point)) / dot;
        // Java: return new Vector3D(1.0, point, k, direction);
        todo!()
    }

    // Java: Line intersection(final Plane other)
    pub fn intersection_plane(&self, _other: &Plane) -> Option<Box<dyn std::any::Any>> {
        // Java: final Vector3D direction = Vector3D.crossProduct(w, other.w);
        // Java: if (direction.getNorm() < tolerance)
        // Java: return null;
        // Java: final Vector3D point = intersection(this, other, new Plane(direction, tolerance));
        // Java: return new Line(point, point.add(direction), tolerance);
        todo!()
    }

    // Java: static Vector3D intersection(final Plane plane1, final Plane plane2, final Plane plane3)
    pub fn intersection_3planes(plane1: &Plane, plane2: &Plane, plane3: &Plane) -> Option<Box<dyn std::any::Any>> {
        // Java: // coefficients of the three planes linear equations
        // Java: final double a1 = plane1.w.getX();
        // Java: final double b1 = plane1.w.getY();
        // Java: final double c1 = plane1.w.getZ();
        // Java: final double d1 = plane1.originOffset;
        // Java: final double a2 = plane2.w.getX();
        // Java: final double b2 = plane2.w.getY();
        // Java: final double c2 = plane2.w.getZ();
        // Java: final double d2 = plane2.originOffset;
        // Java: final double a3 = plane3.w.getX();
        // Java: final double b3 = plane3.w.getY();
        // Java: final double c3 = plane3.w.getZ();
        // Java: final double d3 = plane3.originOffset;
        // Java: // direct Cramer resolution of the linear system
        // Java: // (this is still feasible for a 3x3 system)
        // Java: final double a23         = b2 * c3 - b3 * c2;
        // Java: final double b23         = c2 * a3 - c3 * a2;
        // Java: final double c23         = a2 * b3 - a3 * b2;
        // Java: final double determinant = a1 * a23 + b1 * b23 + c1 * c23;
        // Java: if (FastMath.abs(determinant) < 1.0e-10)
        // Java: return null;
        // Java: final double r = 1.0 / determinant;
        // Java: return new Vector3D(
        // Java: (-a23 * d1 - (c1 * b3 - c3 * b1) * d2 - (c2 * b1 - c1 * b2) * d3) * r,
        // Java: (-b23 * d1 - (c3 * a1 - c1 * a3) * d2 - (c1 * a2 - c2 * a1) * d3) * r,
        // Java: (-c23 * d1 - (b1 * a3 - b3 * a1) * d2 - (b2 * a1 - b1 * a2) * d3) * r);
        todo!()
    }

    // Java: SubPlane wholeHyperplane()
    pub fn wholeHyperplane(&self) -> Box<dyn std::any::Any> {
        // Java: return new SubPlane(this, new PolygonsSet(tolerance));
        todo!()
    }

    // Java: PolyhedronsSet wholeSpace()
    pub fn wholeSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return new PolyhedronsSet(tolerance);
        todo!()
    }

    // Java: boolean contains(final Vector3D p)
    pub fn contains(&self, _p: &dyn std::any::Any) -> bool {
        // Java: return FastMath.abs(getOffset(p)) < tolerance;
        todo!()
    }

    // Java: double getOffset(final Plane plane)
    pub fn getOffset_plane(&self, plane: &Plane) -> f64 {
        // Java: return originOffset + (sameOrientationAs(plane) ? -plane.originOffset : plane.originOffset);
        todo!()
    }

    // Java: double getOffset(Vector<Euclidean3D> vector)
    pub fn getOffset_vec(&self, _vector: &dyn std::any::Any) -> f64 {
        // Java: return getOffset((Point<Euclidean3D>) vector);
        todo!()
    }

    // Java: double getOffset(final Point<Euclidean3D> point)
    pub fn getOffset(&self, _point: &dyn std::any::Any) -> f64 {
        // Java: return ((Vector3D) point).dotProduct(w) + originOffset;
        todo!()
    }

    // Java: boolean sameOrientationAs(final Hyperplane<Euclidean3D> other)
    pub fn sameOrientationAs(&self, _other: &dyn std::any::Any) -> bool {
        // Java: return (((Plane) other).w).dotProduct(w) > 0.0;
        todo!()
    }

}
