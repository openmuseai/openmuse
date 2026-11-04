#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.List
// Java: import org.apache.commons.math3.geometry.euclidean.threed.Vector3D
use crate::org::apache::commons::math3::geometry::euclidean::threed::Vector3D;
// Java: import org.apache.commons.math3.geometry.spherical.oned.Arc
use crate::org::apache::commons::math3::geometry::spherical::oned::Arc;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;

use std::any::Any;

// Java field: Vertex start
// Java field: Vertex end
// Java field: double length
// Java field: Circle circle

// Java: public class Edge
pub struct Edge {
    pub start: Box<dyn Any>,
    pub end: Box<dyn Any>,
    pub length: f64,
    pub circle: Box<dyn Any>,
}

// Java: public class Edge
impl Edge {
    // Java: Edge(final Vertex start, final Vertex end, final double length, final Circle circle)
    pub fn new(start: Box<dyn Any>, end: Box<dyn Any>, length: f64, circle: Box<dyn Any>) -> Self {
        // Java: this.start  = start;
        // Java: this.end    = end;
        // Java: this.length = length;
        // Java: this.circle = circle;
        // Java: start.setOutgoing(this);
        // Java: end.setIncoming(this);
        Edge {
            start,
            end,
            length,
            circle,
        }
    }

    // Java: Vertex getStart()
    pub fn getStart(&self) -> &Box<dyn Any> {
        // Java: return start;
        &self.start
    }

    // Java: Vertex getEnd()
    pub fn getEnd(&self) -> &Box<dyn Any> {
        // Java: return end;
        &self.end
    }

    // Java: double getLength()
    pub fn getLength(&self) -> f64 {
        // Java: return length;
        self.length
    }

    // Java: Circle getCircle()
    pub fn getCircle(&self) -> &Box<dyn Any> {
        // Java: return circle;
        &self.circle
    }

    // Java: Vector3D getPointAt(final double alpha)
    pub fn getPointAt(&self, _alpha: f64) -> Box<dyn Any> {
        // Java: return circle.getPointAt(alpha + circle.getPhase(start.getLocation().getVector()));
        todo!()
    }

    // Java: void setNextEdge(final Edge next)
    pub fn setNextEdge(&mut self, _next: &Edge) {
        // Java: end = next.getStart();
        // Java: end.setIncoming(this);
        // Java: end.bindWith(getCircle());
        todo!()
    }

    // Java: void split()
    pub fn split(&self, _splitCircle: &Box<dyn Any>, _outsideList: &mut Vec<Box<dyn Any>>, _insideList: &mut Vec<Box<dyn Any>>) {
        // Java: final double edgeStart        = circle.getPhase(start.getLocation().getVector());
        // Java: final Arc    arc              = circle.getInsideArc(splitCircle);
        // Java: final double arcRelativeStart = MathUtils.normalizeAngle(arc.getInf(), edgeStart + FastMath.PI) - edgeStart;
        // Java: final double arcRelativeEnd   = arcRelativeStart + arc.getSize();
        // Java: final double unwrappedEnd     = arcRelativeEnd - MathUtils.TWO_PI;
        // Java: final double tolerance = circle.getTolerance();
        // Java: if (unwrappedEnd >= length - tolerance)
        // Java: insideList.add(this);
        todo!()
    }

    // Java: Vertex addSubEdge()
    fn addSubEdge(&self, _subStart: Box<dyn Any>, _subEnd: Box<dyn Any>, _subLength: f64, _list: &mut Vec<Box<dyn Any>>, _splitCircle: &Box<dyn Any>) -> Box<dyn Any> {
        // Java: if (subLength <= circle.getTolerance())
        // Java: return subStart;
        // Java: subEnd.bindWith(splitCircle);
        // Java: final Edge edge = new Edge(subStart, subEnd, subLength, circle);
        // Java: list.add(edge);
        // Java: return subEnd;
        todo!()
    }
}
