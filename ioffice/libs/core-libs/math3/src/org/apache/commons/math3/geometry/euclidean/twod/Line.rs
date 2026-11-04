#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

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
// Java: import org.apache.commons.math3.geometry.euclidean.oned.OrientedPoint
use crate::org::apache::commons::math3::geometry::euclidean::oned::OrientedPoint;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Vector1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Vector1D;
// Java: import org.apache.commons.math3.geometry.partitioning.Embedding
use crate::org::apache::commons::math3::geometry::partitioning::Embedding;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Transform
use crate::org::apache::commons::math3::geometry::partitioning::Transform;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.MathArrays
use crate::org::apache::commons::math3::util::MathArrays::MathArrays;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represents
pub struct represents {
}

// Java: public class Line
pub struct Line {
    // Java field: double angle
    pub(crate) angle: f64,
    // Java field: double cos
    pub(crate) cos: f64,
    // Java field: double sin
    pub(crate) sin: f64,
    // Java field: double originOffset
    pub(crate) originOffset: f64,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
    // Java field: Line reverse
    pub(crate) reverse: Option<Box<Line>>,
}

// Java: public class LineTransform
pub struct LineTransform {
    // Java field: double cXX
    pub(crate) cXX: f64,
    // Java field: double cYX
    pub(crate) cYX: f64,
    // Java field: double cXY
    pub(crate) cXY: f64,
    // Java field: double cYY
    pub(crate) cYY: f64,
    // Java field: double cX1
    pub(crate) cX1: f64,
    // Java field: double cY1
    pub(crate) cY1: f64,
    // Java field: double c1Y
    pub(crate) c1Y: f64,
    // Java field: double c1X
    pub(crate) c1X: f64,
    // Java field: double c11
    pub(crate) c11: f64,
}

// Java: public class represents
impl represents {
}

// Java: public class Line
impl Line {
    // Java: Line(final Vector2D p1, final Vector2D p2, final double tolerance)
    pub fn new_from_points(_p1: &dyn std::any::Any, _p2: &dyn std::any::Any, tolerance: f64) -> Self {
        // Java: reset(p1, p2);
        // Java: this.tolerance = tolerance;
        todo!()
    }

    // Java: Line(final Vector2D p, final double angle, final double tolerance)
    pub fn new_from_point_angle(_p: &dyn std::any::Any, _angle: f64, tolerance: f64) -> Self {
        // Java: reset(p, angle);
        // Java: this.tolerance = tolerance;
        todo!()
    }

    // Java: Line()
    pub fn new_default() -> Self {
        todo!()
    }

    fn new_internal(angle: f64, cos: f64, sin: f64, origin_offset: f64, tolerance: f64) -> Self {
        Line { angle, cos, sin, originOffset: origin_offset, tolerance, reverse: None }
    }

    // Java: Line(final Vector2D p1, final Vector2D p2)
    pub fn new_from_points_default(_p1: &dyn std::any::Any, _p2: &dyn std::any::Any) -> Self {
        // Java: this(p1, p2, DEFAULT_TOLERANCE);
        todo!()
    }

    // Java: Line(final Vector2D p, final double angle)
    pub fn new_from_point_angle_default(_p: &dyn std::any::Any, _angle: f64) -> Self {
        // Java: this(p, angle, DEFAULT_TOLERANCE);
        todo!()
    }

    // Java: Line(final Line line)
    pub fn new_copy(line: &Line) -> Self {
        // Java: angle        = MathUtils.normalizeAngle(line.angle, FastMath.PI);
        // Java: cos          = line.cos;
        // Java: sin          = line.sin;
        // Java: originOffset = line.originOffset;
        // Java: tolerance    = line.tolerance;
        // Java: reverse      = null;
        Line {
            angle: line.angle,
            cos: line.cos,
            sin: line.sin,
            originOffset: line.originOffset,
            tolerance: line.tolerance,
            reverse: None,
        }
    }

    // Java: Line copySelf()
    pub fn copySelf(&self) -> Box<dyn std::any::Any> {
        // Java: return new Line(this);
        todo!()
    }

    // Java: void reset(final Vector2D p1, final Vector2D p2)
    pub fn reset_from_points(&mut self, _p1: &dyn std::any::Any, _p2: &dyn std::any::Any) {
        // Java: unlinkReverse();
        // Java: final double dx = p2.getX() - p1.getX();
        // Java: final double dy = p2.getY() - p1.getY();
        // Java: final double d = FastMath.hypot(dx, dy);
        // Java: if (d == 0.0)
        // Java: angle        = 0.0;
        // Java: cos          = 1.0;
        // Java: sin          = 0.0;
        // Java: originOffset = p1.getY();
        // Java: else
        // Java: angle        = FastMath.PI + FastMath.atan2(-dy, -dx);
        // Java: cos          = dx / d;
        // Java: sin          = dy / d;
        // Java: originOffset = MathArrays.linearCombination(p2.getX(), p1.getY(), -p1.getX(), p2.getY()) / d;
        todo!()
    }

    // Java: void reset(final Vector2D p, final double alpha)
    pub fn reset_from_angle(&mut self, _p: &dyn std::any::Any, _alpha: f64) {
        // Java: unlinkReverse();
        // Java: this.angle   = MathUtils.normalizeAngle(alpha, FastMath.PI);
        // Java: cos          = FastMath.cos(this.angle);
        // Java: sin          = FastMath.sin(this.angle);
        // Java: originOffset = MathArrays.linearCombination(cos, p.getY(), -sin, p.getX());
        todo!()
    }

    // Java: void revertSelf()
    pub fn revertSelf(&mut self) {
        // Java: unlinkReverse();
        self.unlinkReverse();
        // Java: if (angle < FastMath.PI)
        if self.angle < std::f64::consts::PI {
            // Java: angle += FastMath.PI;
            self.angle += std::f64::consts::PI;
        }
        // Java: else
        else {
            // Java: angle -= FastMath.PI;
            self.angle -= std::f64::consts::PI;
        }
        // Java: cos          = -cos;
        self.cos = -self.cos;
        // Java: sin          = -sin;
        self.sin = -self.sin;
        // Java: originOffset = -originOffset;
        self.originOffset = -self.originOffset;
    }

    // Java: void unlinkReverse()
    pub fn unlinkReverse(&mut self) {
        // Java: if (reverse != null)
        // Java: reverse.reverse = null;
        // Java: reverse = null;
        self.reverse = None;
    }

    // Java: Line getReverse()
    pub fn getReverse(&self) -> Box<dyn std::any::Any> {
        // Java: if (reverse == null)
        // Java: reverse = new Line((angle < FastMath.PI) ? (angle + FastMath.PI) : (angle - FastMath.PI),
        // Java: -cos, -sin, -originOffset, tolerance);
        // Java: reverse.reverse = this;
        // Java: return reverse;
        todo!()
    }

    // Java: Vector1D toSubSpace(Vector<Euclidean2D> vector)
    pub fn toSubSpace_vec(&self, _vector: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return toSubSpace((Point<Euclidean2D>) vector);
        todo!()
    }

    // Java: Vector2D toSpace(Vector<Euclidean1D> vector)
    pub fn toSpace_vec(&self, _vector: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return toSpace((Point<Euclidean1D>) vector);
        todo!()
    }

    // Java: Vector1D toSubSpace(final Point<Euclidean2D> point)
    pub fn toSubSpace(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: Vector2D p2 = (Vector2D) point;
        // Java: return new Vector1D(MathArrays.linearCombination(cos, p2.getX(), sin, p2.getY()));
        todo!()
    }

    // Java: Vector2D toSpace(final Point<Euclidean1D> point)
    pub fn toSpace(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final double abscissa = ((Vector1D) point).getX();
        // Java: return new Vector2D(MathArrays.linearCombination(abscissa, cos, -originOffset, sin),
        // Java: MathArrays.linearCombination(abscissa, sin,  originOffset, cos));
        todo!()
    }

    // Java: Vector2D intersection(final Line other)
    pub fn intersection(&self, _other: &Line) -> Option<Box<dyn std::any::Any>> {
        // Java: final double d = MathArrays.linearCombination(sin, other.cos, -other.sin, cos);
        // Java: if (FastMath.abs(d) < tolerance)
        // Java: return null;
        // Java: return new Vector2D(MathArrays.linearCombination(cos, other.originOffset, -other.cos, originOffset) / d,
        // Java: MathArrays.linearCombination(sin, other.originOffset, -other.sin, originOffset) / d);
        todo!()
    }

    // Java: Point<Euclidean2D> project(Point<Euclidean2D> point)
    pub fn project(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: return toSpace(toSubSpace(point));
        todo!()
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        self.tolerance
    }

    // Java: SubLine wholeHyperplane()
    pub fn wholeHyperplane(&self) -> Box<dyn std::any::Any> {
        // Java: return new SubLine(this, new IntervalsSet(tolerance));
        todo!()
    }

    // Java: PolygonsSet wholeSpace()
    pub fn wholeSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return new PolygonsSet(tolerance);
        todo!()
    }

    // Java: double getOffset(final Line line)
    pub fn getOffset_line(&self, line: &Line) -> f64 {
        // Java: return originOffset +
        // Java: (MathArrays.linearCombination(cos, line.cos, sin, line.sin) > 0 ? -line.originOffset : line.originOffset);
        self.originOffset +
            if MathArrays::linearCombination_2(self.cos, line.cos, self.sin, line.sin) > 0.0 {
                -line.originOffset
            } else {
                line.originOffset
            }
    }

    // Java: double getOffset(Vector<Euclidean2D> vector)
    pub fn getOffset_vec(&self, _vector: &dyn std::any::Any) -> f64 {
        // Java: return getOffset((Point<Euclidean2D>) vector);
        todo!()
    }

    // Java: double getOffset(final Point<Euclidean2D> point)
    pub fn getOffset(&self, _point: &dyn std::any::Any) -> f64 {
        // Java: Vector2D p2 = (Vector2D) point;
        // Java: return MathArrays.linearCombination(sin, p2.getX(), -cos, p2.getY(), 1.0, originOffset);
        todo!()
    }

    // Java: boolean sameOrientationAs(final Hyperplane<Euclidean2D> other)
    pub fn sameOrientationAs(&self, _other: &dyn std::any::Any) -> bool {
        // Java: final Line otherL = (Line) other;
        // Java: return MathArrays.linearCombination(sin, otherL.sin, cos, otherL.cos) >= 0.0;
        todo!()
    }

    // Java: Vector2D getPointAt(final Vector1D abscissa, final double offset)
    pub fn getPointAt(&self, _abscissa: &dyn std::any::Any, _offset: f64) -> Box<dyn std::any::Any> {
        // Java: final double x       = abscissa.getX();
        // Java: final double dOffset = offset - originOffset;
        // Java: return new Vector2D(MathArrays.linearCombination(x, cos,  dOffset, sin),
        // Java: MathArrays.linearCombination(x, sin, -dOffset, cos));
        todo!()
    }

    // Java: boolean contains(final Vector2D p)
    pub fn contains(&self, _p: &dyn std::any::Any) -> bool {
        // Java: return FastMath.abs(getOffset(p)) < tolerance;
        todo!()
    }

    // Java: double distance(final Vector2D p)
    pub fn distance(&self, _p: &dyn std::any::Any) -> f64 {
        // Java: return FastMath.abs(getOffset(p));
        todo!()
    }

    // Java: boolean isParallelTo(final Line line)
    pub fn isParallelTo(&self, line: &Line) -> bool {
        // Java: return FastMath.abs(MathArrays.linearCombination(sin, line.cos, -cos, line.sin)) < tolerance;
        FastMath::abs(MathArrays::linearCombination_2(self.sin, line.cos, -self.cos, line.sin)) < self.tolerance
    }

    // Java: void translateToPoint(final Vector2D p)
    pub fn translateToPoint(&mut self, _p: &dyn std::any::Any) {
        // Java: originOffset = MathArrays.linearCombination(cos, p.getY(), -sin, p.getX());
        todo!()
    }

    // Java: double getAngle()
    pub fn getAngle(&self) -> f64 {
        // Java: return MathUtils.normalizeAngle(angle, FastMath.PI);
        self.angle
    }

    // Java: void setAngle(final double angle)
    pub fn setAngle(&mut self, _angle: f64) {
        // Java: unlinkReverse();
        // Java: this.angle = MathUtils.normalizeAngle(angle, FastMath.PI);
        // Java: cos        = FastMath.cos(this.angle);
        // Java: sin        = FastMath.sin(this.angle);
        todo!()
    }

    // Java: double getOriginOffset()
    pub fn getOriginOffset(&self) -> f64 {
        // Java: return originOffset;
        self.originOffset
    }

    // Java: void setOriginOffset(final double offset)
    pub fn setOriginOffset(&mut self, offset: f64) {
        // Java: unlinkReverse();
        self.unlinkReverse();
        // Java: originOffset = offset;
        self.originOffset = offset;
    }

    // Java: static public static Transform<Euclidean2D, Euclidean1D> getTransform()
    pub fn getTransform() -> Box<dyn std::any::Any> {
        todo!()
    }

}

// Java: public class LineTransform
impl LineTransform {
    // Java: Vector2D apply(final Point<Euclidean2D> point)
    pub fn apply_point(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Vector2D p2D = (Vector2D) point;
        // Java: final double  x   = p2D.getX();
        // Java: final double  y   = p2D.getY();
        // Java: return new Vector2D(MathArrays.linearCombination(cXX, x, cXY, y, cX1, 1),
        // Java: MathArrays.linearCombination(cYX, x, cYY, y, cY1, 1));
        todo!()
    }

    // Java: Line apply(final Hyperplane<Euclidean2D> hyperplane)
    pub fn apply_hyperplane(&self, _hyperplane: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: final Line   line    = (Line) hyperplane;
        // Java: final double rOffset = MathArrays.linearCombination(c1X, line.cos, c1Y, line.sin, c11, line.originOffset);
        // Java: final double rCos    = MathArrays.linearCombination(cXX, line.cos, cXY, line.sin);
        // Java: final double rSin    = MathArrays.linearCombination(cYX, line.cos, cYY, line.sin);
        // Java: final double inv     = 1.0 / FastMath.sqrt(rSin * rSin + rCos * rCos);
        // Java: return new Line(FastMath.PI + FastMath.atan2(-rSin, -rCos),
        // Java: inv * rCos, inv * rSin,
        // Java: inv * rOffset, line.tolerance);
        todo!()
    }

    // Java: SubHyperplane<Euclidean1D> apply()
    pub fn apply_sub(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}
