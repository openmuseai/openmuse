// Java: import org.apache.commons.math3.geometry.partitioning.Region.Location
use crate::org::apache::commons::math3::geometry::partitioning::Region::Location;
// Java: import org.apache.commons.math3.exception.NumberIsTooSmallException
use crate::org::apache::commons::math3::exception::NumberIsTooSmallException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;

// Java: public class represents
pub struct represents {
}

// Java: public class Interval
pub struct Interval {
    // Java field: double lower
    pub(crate) lower: f64,
    // Java field: double upper
    pub(crate) upper: f64,
}

// Java: public class represents
impl represents {
}

// Java: public class Interval
impl Interval {
    // Java: Interval(final double lower, final double upper)
    pub fn Interval(lower: f64, upper: f64) -> Self {
        // Java: if (upper < lower)
        if upper < lower {
            // Java: throw new NumberIsTooSmallException(LocalizedFormats.ENDPOINTS_NOT_AN_INTERVAL,
            // Java: upper, lower, true);
            panic!("NumberIsTooSmallException");
        }
        // Java: this.lower = lower;
        // Java: this.upper = upper;
        Interval { lower, upper }
    }

    // Java: double getInf()
    pub fn getInf(&self) -> f64 {
        // Java: return lower;
        self.lower
    }

    // Java: double getLower()
    pub fn getLower(&self) -> f64 {
        // Java: return getInf();
        self.getInf()
    }

    // Java: double getSup()
    pub fn getSup(&self) -> f64 {
        // Java: return upper;
        self.upper
    }

    // Java: double getUpper()
    pub fn getUpper(&self) -> f64 {
        // Java: return getSup();
        self.getSup()
    }

    // Java: double getSize()
    pub fn getSize(&self) -> f64 {
        // Java: return upper - lower;
        self.upper - self.lower
    }

    // Java: double getLength()
    pub fn getLength(&self) -> f64 {
        // Java: return getSize();
        self.getSize()
    }

    // Java: double getBarycenter()
    pub fn getBarycenter(&self) -> f64 {
        // Java: return 0.5 * (lower + upper);
        0.5 * (self.lower + self.upper)
    }

    // Java: double getMidPoint()
    pub fn getMidPoint(&self) -> f64 {
        // Java: return getBarycenter();
        self.getBarycenter()
    }

    // Java: Location checkPoint(final double point, final double tolerance)
    pub fn checkPoint(&self, point: f64, tolerance: f64) -> Location {
        // Java: if (point < lower - tolerance || point > upper + tolerance)
        if point < self.lower - tolerance || point > self.upper + tolerance {
            // Java: return Location.OUTSIDE;
            Location::OUTSIDE
        // Java: else if (point > lower + tolerance && point < upper - tolerance)
        } else if point > self.lower + tolerance && point < self.upper - tolerance {
            // Java: return Location.INSIDE;
            Location::INSIDE
        // Java: else
        } else {
            // Java: return Location.BOUNDARY;
            Location::BOUNDARY
        }
    }
}
