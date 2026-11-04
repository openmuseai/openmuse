#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;

// Java: public class represents
pub struct represents {
}

// Java: public class LimitAngle
pub struct LimitAngle {
    // Java field: S1Point location
    pub(crate) location: Box<dyn std::any::Any>,
    // Java field: boolean direct
    pub(crate) direct: bool,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
}

// Java: public class represents
impl represents {
}

// Java: public class LimitAngle
impl LimitAngle {
    // Java: LimitAngle(final S1Point location, final boolean direct, final double tolerance)
    pub fn new(location: Box<dyn std::any::Any>, direct: bool, tolerance: f64) -> Self {
        // Java: this.location  = location;
        // Java: this.direct    = direct;
        // Java: this.tolerance = tolerance;
        LimitAngle { location, direct, tolerance }
    }

    // Java: LimitAngle copySelf()
    pub fn copySelf(&self) -> Box<dyn std::any::Any> {
        // Java: return this;
        todo!()
    }

    // Java: double getOffset(final Point<Sphere1D> point)
    pub fn getOffset(&self, _point: &dyn std::any::Any) -> f64 {
        // Java: final double delta = ((S1Point) point).getAlpha() - location.getAlpha();
        // Java: return direct ? delta : -delta;
        todo!()
    }

    // Java: boolean isDirect()
    pub fn isDirect(&self) -> bool {
        // Java: return direct;
        self.direct
    }

    // Java: LimitAngle getReverse()
    pub fn getReverse(&self) -> Box<dyn std::any::Any> {
        // Java: return new LimitAngle(location, !direct, tolerance);
        todo!()
    }

    // Java: SubLimitAngle wholeHyperplane()
    pub fn wholeHyperplane(&self) -> Box<dyn std::any::Any> {
        // Java: return new SubLimitAngle(this, null);
        todo!()
    }

    // Java: ArcsSet wholeSpace()
    pub fn wholeSpace(&self) -> Box<dyn std::any::Any> {
        // Java: return new ArcsSet(tolerance);
        todo!()
    }

    // Java: boolean sameOrientationAs(final Hyperplane<Sphere1D> other)
    pub fn sameOrientationAs(&self, _other: &dyn std::any::Any) -> bool {
        // Java: return !(direct ^ ((LimitAngle) other).direct);
        todo!()
    }

    // Java: S1Point getLocation()
    pub fn getLocation(&self) -> &dyn std::any::Any {
        // Java: return location;
        &*self.location
    }

    // Java: Point<Sphere1D> project(Point<Sphere1D> point)
    pub fn project(&self, _point: &dyn std::any::Any) -> &dyn std::any::Any {
        // Java: return location;
        &*self.location
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        self.tolerance
    }

}
