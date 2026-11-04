#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments, non_upper_case_globals)]

// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Euclidean1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Euclidean1D;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Interval
use crate::org::apache::commons::math3::geometry::euclidean::oned::Interval;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.IntervalsSet
use crate::org::apache::commons::math3::geometry::euclidean::oned::IntervalsSet;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Vector1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Vector1D;
// Java: import org.apache.commons.math3.geometry.partitioning.Region.Location
use crate::org::apache::commons::math3::geometry::partitioning::Region::Location;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represents
pub struct represents {
}

// Java: public class SubLine
pub struct SubLine {
    // Java field: Line line
    pub(crate) line: Box<dyn std::any::Any>,
    // Java field: IntervalsSet remainingRegion
    pub(crate) remainingRegion: Box<dyn std::any::Any>,
}

// Java: public class represents
impl represents {
}

// Java: public class SubLine
impl SubLine {
    // Java: SubLine(final Line line, final IntervalsSet remainingRegion)
    pub fn new(line: Box<dyn std::any::Any>, remaining_region: Box<dyn std::any::Any>) -> Self {
        // Java: this.line            = line;
        // Java: this.remainingRegion = remainingRegion;
        SubLine { line, remainingRegion: remaining_region }
    }

    // Java: SubLine(final Vector3D start, final Vector3D end, final double tolerance)
    pub fn new_from_points(_start: &dyn std::any::Any, _end: &dyn std::any::Any, _tolerance: f64) -> Self {
        todo!()
    }

    // Java: SubLine(final Vector3D start, final Vector3D end)
    pub fn new_from_points_default(_start: &dyn std::any::Any, _end: &dyn std::any::Any) -> Self {
        todo!()
    }

    // Java: SubLine(final Segment segment)
    pub fn new_from_segment(_segment: &dyn std::any::Any) -> Self {
        // Java: this(segment.getLine(),
        // Java: buildIntervalSet(segment.getStart(), segment.getEnd(), segment.getLine().getTolerance()));
        todo!()
    }

    // Java: List<Segment> getSegments()
    pub fn getSegments(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final List<Interval> list = remainingRegion.asList();
        // Java: final List<Segment> segments = new ArrayList<Segment>(list.size());
        // Java: for (final Interval interval : list)
        // Java: final Vector3D start = line.toSpace((Point<Euclidean1D>) new Vector1D(interval.getInf()));
        // Java: final Vector3D end   = line.toSpace((Point<Euclidean1D>) new Vector1D(interval.getSup()));
        // Java: segments.add(new Segment(start, end, line));
        // Java: return segments;
        todo!()
    }

    // Java: Vector3D intersection(final SubLine subLine, final boolean includeEndPoints)
    pub fn intersection(&self, _sub_line: &SubLine, _include_end_points: bool) -> Option<Box<dyn std::any::Any>> {
        // Java: // compute the intersection on infinite line
        // Java: Vector3D v1D = line.intersection(subLine.line);
        // Java: if (v1D == null)
        // Java: return null;
        // Java: // check location of point with respect to first sub-line
        // Java: Location loc1 = remainingRegion.checkPoint((Point<Euclidean1D>) line.toSubSpace((Point<Euclidean3D>) v1D));
        // Java: // check location of point with respect to second sub-line
        // Java: Location loc2 = subLine.remainingRegion.checkPoint((Point<Euclidean1D>) subLine.line.toSubSpace((Point<Euclidean3D>) v1D));
        // Java: if (includeEndPoints)
        // Java: return ((loc1 != Location.OUTSIDE) && (loc2 != Location.OUTSIDE)) ? v1D : null;
        // Java: else
        // Java: return ((loc1 == Location.INSIDE) && (loc2 == Location.INSIDE)) ? v1D : null;
        todo!()
    }

    // Java: static IntervalsSet buildIntervalSet(final Vector3D start, final Vector3D end, final double tolerance)
    pub fn buildIntervalSet(_start: &dyn std::any::Any, _end: &dyn std::any::Any, _tolerance: f64) -> Box<dyn std::any::Any> {
        todo!()
    }

}
