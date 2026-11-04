#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Vector
use crate::org::apache::commons::math3::geometry::Vector;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Euclidean1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Euclidean1D;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.IntervalsSet
use crate::org::apache::commons::math3::geometry::euclidean::oned::IntervalsSet;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Vector1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Vector1D::Vector1D;
// Java: import org.apache.commons.math3.geometry.partitioning.Embedding
use crate::org::apache::commons::math3::geometry::partitioning::Embedding;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represent
pub struct represent {
}

// Java: public class Line
pub struct Line {
    // Java field: Vector3D direction
    pub(crate) direction: Box<dyn std::any::Any>,
    // Java field: Vector3D zero
    pub(crate) zero: Box<dyn std::any::Any>,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
}

// Java: public class represent
impl represent {
}

// Java: public class Line
impl Line {
    // Java: Line(final Vector3D p1, final Vector3D p2, final double tolerance)
    pub fn new(_p1: &dyn std::any::Any, _p2: &dyn std::any::Any, tolerance: f64) -> Self {
        // Java: reset(p1, p2); this.tolerance = tolerance;
        todo!()
    }

    // Java: Line(final Line line)
    pub fn new_copy(line: &Line) -> Self {
        // Java: this.direction = line.direction;
        // Java: this.zero      = line.zero;
        // Java: this.tolerance = line.tolerance;
        todo!()
    }

    // Java: Line(final Vector3D p1, final Vector3D p2)
    pub fn new_default_tol(_p1: &dyn std::any::Any, _p2: &dyn std::any::Any) -> Self {
        // Java: this(p1, p2, DEFAULT_TOLERANCE);
        todo!()
    }

    // Java: void reset(final Vector3D p1, final Vector3D p2)
    pub fn reset(&mut self, _p1: &dyn std::any::Any, _p2: &dyn std::any::Any) {
        // Java: final Vector3D delta = p2.subtract(p1);
        // Java: final double norm2 = delta.getNormSq();
        // Java: if (norm2 == 0.0)
        // Java: throw new MathIllegalArgumentException(LocalizedFormats.ZERO_NORM);
        // Java: this.direction = new Vector3D(1.0 / FastMath.sqrt(norm2), delta);
        // Java: zero = new Vector3D(1.0, p1, -p1.dotProduct(delta) / norm2, delta);
        todo!()
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        self.tolerance
    }

    // Java: Line revert()
    pub fn revert(&self) -> Box<dyn std::any::Any> {
        // Java: final Line reverted = new Line(this);
        // Java: reverted.direction = reverted.direction.negate();
        // Java: return reverted;
        todo!()
    }

    // Java: Vector3D getDirection()
    pub fn getDirection(&self) -> &dyn std::any::Any {
        // Java: return direction;
        &*self.direction
    }

    // Java: Vector3D getOrigin()
    pub fn getOrigin(&self) -> &dyn std::any::Any {
        // Java: return zero;
        &*self.zero
    }

    // Java: double getAbscissa(final Vector3D point)
    pub fn getAbscissa(&self, _point: &dyn std::any::Any) -> f64 {
        // Java: return point.subtract(zero).dotProduct(direction);
        todo!()
    }

    // Java: Vector3D pointAt(final double abscissa)
    pub fn pointAt(&self, _abscissa: f64) -> Box<dyn std::any::Any> {
        // Java: return new Vector3D(1.0, zero, abscissa, direction);
        todo!()
    }

    // Java: Vector1D toSubSpace(Vector<Euclidean3D> vector)
    pub fn toSubSpace_vec(&self, _vector: &dyn std::any::Any) -> Vector1D {
        // Java: return toSubSpace((Point<Euclidean3D>) vector);
        todo!()
    }

    // Java: Vector3D toSpace(Vector<Euclidean1D> vector)
    pub fn toSpace_vec(&self, _vector: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return toSpace((Point<Euclidean1D>) vector);
        todo!()
    }

    // Java: Vector1D toSubSpace(final Point<Euclidean3D> point)
    pub fn toSubSpace(&self, _point: &dyn std::any::Any) -> Vector1D {
        // Java: return new Vector1D(getAbscissa((Vector3D) point));
        todo!()
    }

    // Java: Vector3D toSpace(final Point<Euclidean1D> point)
    pub fn toSpace(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return pointAt(((Vector1D) point).getX());
        todo!()
    }

    // Java: boolean isSimilarTo(final Line line)
    pub fn isSimilarTo(&self, _line: &Line) -> bool {
        // Java: final double angle = Vector3D.angle(direction, line.direction);
        // Java: return ((angle < tolerance) || (angle > (FastMath.PI - tolerance))) && contains(line.zero);
        todo!()
    }

    // Java: boolean contains(final Vector3D p)
    pub fn contains(&self, _p: &dyn std::any::Any) -> bool {
        // Java: return distance(p) < tolerance;
        todo!()
    }

    // Java: double distance(final Vector3D p)
    pub fn distance_point(&self, _p: &dyn std::any::Any) -> f64 {
        // Java: final Vector3D d = p.subtract(zero);
        // Java: final Vector3D n = new Vector3D(1.0, d, -d.dotProduct(direction), direction);
        // Java: return n.getNorm();
        todo!()
    }

    // Java: double distance(final Line line)
    pub fn distance_line(&self, _line: &Line) -> f64 {
        // Java: final Vector3D normal = Vector3D.crossProduct(direction, line.direction);
        // Java: final double n = normal.getNorm();
        // Java: if (n < Precision.SAFE_MIN)
        // Java: // lines are parallel
        // Java: return distance(line.zero);
        // Java: // signed separation of the two parallel planes that contains the lines
        // Java: final double offset = line.zero.subtract(zero).dotProduct(normal) / n;
        // Java: return FastMath.abs(offset);
        todo!()
    }

    // Java: Vector3D closestPoint(final Line line)
    pub fn closestPoint(&self, _line: &Line) -> Box<dyn std::any::Any> {
        // Java: final double cos = direction.dotProduct(line.direction);
        // Java: final double n = 1 - cos * cos;
        // Java: if (n < Precision.EPSILON)
        // Java: // the lines are parallel
        // Java: return zero;
        // Java: final Vector3D delta0 = line.zero.subtract(zero);
        // Java: final double a        = delta0.dotProduct(direction);
        // Java: final double b        = delta0.dotProduct(line.direction);
        // Java: return new Vector3D(1, zero, (a - b * cos) / n, direction);
        todo!()
    }

    // Java: Vector3D intersection(final Line line)
    pub fn intersection(&self, _line: &Line) -> Option<Box<dyn std::any::Any>> {
        // Java: final Vector3D closest = closestPoint(line);
        // Java: return line.contains(closest) ? closest : null;
        todo!()
    }

    // Java: SubLine wholeLine()
    pub fn wholeLine(&self) -> Box<dyn std::any::Any> {
        // Java: return new SubLine(this, new IntervalsSet(tolerance));
        todo!()
    }

}
