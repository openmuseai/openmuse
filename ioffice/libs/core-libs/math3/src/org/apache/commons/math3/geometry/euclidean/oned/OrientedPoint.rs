#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Vector
use crate::org::apache::commons::math3::geometry::Vector;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represents
pub struct represents {
}

// Java: public class OrientedPoint
pub struct OrientedPoint {
    // Java field: Vector1D location
    pub(crate) location: Box<dyn std::any::Any>,
    // Java field: boolean direct
    pub(crate) direct: bool,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
}

// Java: public class represents
impl represents {
}

// Java: public class OrientedPoint
impl OrientedPoint {
    // Java: OrientedPoint(final Vector1D location, final boolean direct, final double tolerance)
    pub fn OrientedPoint_3(location: Box<dyn std::any::Any>, direct: bool, tolerance: f64) -> Self {
        // Java: this.location  = location;
        // Java: this.direct    = direct;
        // Java: this.tolerance = tolerance;
        OrientedPoint { location, direct, tolerance }
    }

    // Java: OrientedPoint(final Vector1D location, final boolean direct)
    pub fn OrientedPoint_2(location: Box<dyn std::any::Any>, direct: bool) -> Self {
        // Java: this(location, direct, DEFAULT_TOLERANCE);
        Self::OrientedPoint_3(location, direct, DEFAULT_TOLERANCE)
    }

    // Java: OrientedPoint copySelf()
    pub fn copySelf(&self) -> &Self {
        // Java: return this;
        self
    }

    // Java: double getOffset(Vector<Euclidean1D> vector)
    pub fn getOffset_vec(&self, vector: &dyn std::any::Any) -> f64 {
        // Java: return getOffset((Point<Euclidean1D>) vector);
        todo!()
    }

    // Java: double getOffset(final Point<Euclidean1D> point)
    pub fn getOffset(&self, point: &dyn std::any::Any) -> f64 {
        // Java: final double delta = ((Vector1D) point).getX() - location.getX();
        // Java: return direct ? delta : -delta;
        todo!()
    }

    // Java: SubOrientedPoint wholeHyperplane()
    pub fn wholeHyperplane(&self) -> Box<dyn std::any::Any> {
        // Java: return new SubOrientedPoint(this, null);
        todo!()
    }

    // Java: IntervalsSet wholeSpace()
    pub fn wholeSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return new IntervalsSet(tolerance);
        todo!()
    }

    // Java: boolean sameOrientationAs(final Hyperplane<Euclidean1D> other)
    pub fn sameOrientationAs(&self, other: &dyn std::any::Any) -> bool {
        // Java: return !(direct ^ ((OrientedPoint) other).direct);
        todo!()
    }

    // Java: Point<Euclidean1D> project(Point<Euclidean1D> point)
    pub fn project(&self, point: &dyn std::any::Any) -> &dyn std::any::Any {
        // Java: return location;
        todo!()
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        self.tolerance
    }

    // Java: Vector1D getLocation()
    pub fn getLocation(&self) -> &dyn std::any::Any {
        // Java: return location;
        &self.location
    }

    // Java: boolean isDirect()
    pub fn isDirect(&self) -> bool {
        // Java: return direct;
        self.direct
    }

    // Java: void revertSelf()
    pub fn revertSelf(&mut self) {
        // Java: direct = !direct;
        self.direct = !self.direct;
    }

}
