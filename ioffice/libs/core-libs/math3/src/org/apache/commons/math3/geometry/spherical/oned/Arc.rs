#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.partitioning.Region::Location
use crate::org::apache::commons::math3::geometry::partitioning::Region;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java: public class represents
pub struct represents {
}

// Java: public class Arc
pub struct Arc {
    // Java field: double lower
    pub(crate) lower: f64,
    // Java field: double upper
    pub(crate) upper: f64,
    // Java field: double middle
    pub(crate) middle: f64,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
}

// Java: public class represents
impl represents {
}

// Java: public class Arc
impl Arc {
    // Java: Arc(final double lower, final double upper, final double tolerance)
    pub fn new(lower: f64, upper: f64, tolerance: f64) -> Self {
        // Java: this.tolerance = tolerance;
        // Java: if (Precision.equals(lower, upper, 0) || (upper - lower) >= MathUtils.TWO_PI)
        // Java: this.lower  = 0;
        // Java: this.upper  = MathUtils.TWO_PI;
        // Java: this.middle = FastMath.PI;
        // Java: else if (lower > upper)
        // Java: throw new NumberIsTooLargeException(...)
        todo!()
    }

    // Java: double getInf()
    pub fn getInf(&self) -> f64 {
        // Java: return lower;
        self.lower
    }

    // Java: double getSup()
    pub fn getSup(&self) -> f64 {
        // Java: return upper;
        self.upper
    }

    // Java: double getSize()
    pub fn getSize(&self) -> f64 {
        // Java: return upper - lower;
        self.upper - self.lower
    }

    // Java: double getBarycenter()
    pub fn getBarycenter(&self) -> f64 {
        // Java: return middle;
        self.middle
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        self.tolerance
    }

    // Java: Location checkPoint(final double point)
    pub fn checkPoint(&self, _point: f64) -> Box<dyn std::any::Any> {
        // Java: final double normalizedPoint = MathUtils.normalizeAngle(point, middle);
        // Java: if (normalizedPoint < lower - tolerance || normalizedPoint > upper + tolerance)
        // Java: return Location.OUTSIDE;
        // Java: else if (normalizedPoint > lower + tolerance && normalizedPoint < upper - tolerance)
        // Java: return Location.INSIDE;
        // Java: else
        // Java: return (getSize() >= MathUtils.TWO_PI - tolerance) ? Location.INSIDE : Location.BOUNDARY;
        todo!()
    }

}
