#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;

use super::Vector2D::Vector2D;
use super::Line::Line;

// Java: public class Segment
pub struct Segment {
    // Java field: Vector2D start
    pub(crate) start: Box<dyn std::any::Any>,
    // Java field: Vector2D end
    pub(crate) end: Box<dyn std::any::Any>,
    // Java field: Line line
    pub(crate) line: Box<dyn std::any::Any>,
}

// Java: public class Segment
impl Segment {
    // Java: Segment(final Vector2D start, final Vector2D end, final Line line)
    pub fn new(start: Box<dyn std::any::Any>, end: Box<dyn std::any::Any>, line: Box<dyn std::any::Any>) -> Self {
        // Java: this.start  = start;
        // Java: this.end    = end;
        // Java: this.line   = line;
        Segment { start, end, line }
    }

    // Java: Vector2D getStart()
    pub fn getStart(&self) -> &dyn std::any::Any {
        // Java: return start;
        &*self.start
    }

    // Java: Vector2D getEnd()
    pub fn getEnd(&self) -> &dyn std::any::Any {
        // Java: return end;
        &*self.end
    }

    // Java: Line getLine()
    pub fn getLine(&self) -> &dyn std::any::Any {
        // Java: return line;
        &*self.line
    }

    // Java: double distance(final Vector2D p)
    pub fn distance(&self, _p: &dyn std::any::Any) -> f64 {
        // Java: final double deltaX = end.getX() - start.getX();
        // Java: final double deltaY = end.getY() - start.getY();
        // Java: final double r = ((p.getX() - start.getX()) * deltaX + (p.getY() - start.getY()) * deltaY) /
        // Java: (deltaX * deltaX + deltaY * deltaY);
        // Java: // r == 0 => P = startPt
        // Java: // r == 1 => P = endPt
        // Java: // r < 0 => P is on the backward extension of the segment
        // Java: // r > 1 => P is on the forward extension of the segment
        // Java: // 0 < r < 1 => P is on the segment
        // Java: // if point isn't on the line segment, just return the shortest distance to the end points
        // Java: if (r < 0 || r > 1)
        // Java: final double dist1 = getStart().distance((Point<Euclidean2D>) p);
        // Java: final double dist2 = getEnd().distance((Point<Euclidean2D>) p);
        // Java: return FastMath.min(dist1, dist2);
        // Java: else
        // Java: // find point on line and see if it is in the line segment
        // Java: final double px = start.getX() + r * deltaX;
        // Java: final double py = start.getY() + r * deltaY;
        // Java: final Vector2D interPt = new Vector2D(px, py);
        // Java: return interPt.distance((Point<Euclidean2D>) p);
        todo!()
    }

}
