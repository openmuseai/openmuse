// Java: import java.util.ArrayList
// Java: import java.util.List
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Euclidean1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Euclidean1D;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Interval
use crate::org::apache::commons::math3::geometry::euclidean::oned::Interval;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.IntervalsSet
use crate::org::apache::commons::math3::geometry::euclidean::oned::IntervalsSet;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.OrientedPoint
use crate::org::apache::commons::math3::geometry::euclidean::oned::OrientedPoint;
// Java: import org.apache.commons.math3.geometry.euclidean.oned.Vector1D
use crate::org::apache::commons::math3::geometry::euclidean::oned::Vector1D;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractSubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::AbstractSubHyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.Hyperplane
use crate::org::apache::commons::math3::geometry::partitioning::Hyperplane;
// Java: import org.apache.commons.math3.geometry.partitioning.Region
use crate::org::apache::commons::math3::geometry::partitioning::Region;
// Java: import org.apache.commons.math3.geometry.partitioning.Region.Location
use crate::org::apache::commons::math3::geometry::partitioning::Region::Location;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represents
pub struct represents {
}

// Java: public class SubLine extends AbstractSubHyperplane
pub struct SubLine {
    pub hyperplane: Box<dyn std::any::Any>,
    pub remainingRegion: Box<dyn std::any::Any>,
}

// Java: public class represents
impl represents {
}

// Java: public class SubLine extends AbstractSubHyperplane
#[allow(non_snake_case)]
impl SubLine {
    // Java: SubLine()
    pub fn new(hyperplane: Box<dyn std::any::Any>, remainingRegion: Box<dyn std::any::Any>) -> Self {
        // Java: super(hyperplane, remainingRegion);
        SubLine { hyperplane, remainingRegion }
    }

    // Java: SubLine(final Vector2D start, final Vector2D end, final double tolerance)
    pub fn SubLine_from_points(start: &Box<dyn std::any::Any>, end: &Box<dyn std::any::Any>, tolerance: f64) -> Self {
        // Java: super(new Line(start, end, tolerance), buildIntervalSet(start, end, tolerance));
        todo!()
    }

    // Java: SubLine(final Vector2D start, final Vector2D end)
    pub fn SubLine_from_points_default(start: &Box<dyn std::any::Any>, end: &Box<dyn std::any::Any>) -> Self {
        // Java: this(start, end, DEFAULT_TOLERANCE);
        todo!()
    }

    // Java: SubLine(final Segment segment)
    pub fn SubLine_from_segment(segment: &Box<dyn std::any::Any>) -> Self {
        // Java: super(segment.getLine(),
        // Java: buildIntervalSet(segment.getStart(), segment.getEnd(), segment.getLine().getTolerance()));
        todo!()
    }

    // Java: List<Segment> getSegments()
    pub fn getSegments(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final Line line = (Line) getHyperplane();
        // Java: final List<Interval> list = ((IntervalsSet) getRemainingRegion()).asList();
        // Java: final List<Segment> segments = new ArrayList<Segment>(list.size());
        // Java: for (final Interval interval : list)
        {
            // Java: final Vector2D start = line.toSpace((Point<Euclidean1D>) new Vector1D(interval.getInf()));
            // Java: final Vector2D end   = line.toSpace((Point<Euclidean1D>) new Vector1D(interval.getSup()));
            // Java: segments.add(new Segment(start, end, line));
        }
        // Java: return segments;
        todo!()
    }

    // Java: Vector2D intersection(final SubLine subLine, final boolean includeEndPoints)
    pub fn intersection(&self, _subLine: &SubLine, _includeEndPoints: bool) -> Option<Box<dyn std::any::Any>> {
        // Java: // retrieve the underlying lines
        // Java: Line line1 = (Line) getHyperplane();
        // Java: Line line2 = (Line) subLine.getHyperplane();
        // Java: // compute the intersection on infinite line
        // Java: Vector2D v2D = line1.intersection(line2);
        // Java: if (v2D == null)
        {
            // Java: return null;
        }
        // Java: // check location of point with respect to first sub-line
        // Java: Location loc1 = getRemainingRegion().checkPoint(line1.toSubSpace((Point<Euclidean2D>) v2D));
        // Java: // check location of point with respect to second sub-line
        // Java: Location loc2 = subLine.getRemainingRegion().checkPoint(line2.toSubSpace((Point<Euclidean2D>) v2D));
        // Java: if (includeEndPoints)
        {
            // Java: return ((loc1 != Location.OUTSIDE) && (loc2 != Location.OUTSIDE)) ? v2D : null;
        }
        // Java: else
        {
            // Java: return ((loc1 == Location.INSIDE) && (loc2 == Location.INSIDE)) ? v2D : null;
        }
        todo!()
    }

    // Java: static IntervalsSet buildIntervalSet(final Vector2D start, final Vector2D end, final double tolerance)
    pub fn buildIntervalSet(_start: &Box<dyn std::any::Any>, _end: &Box<dyn std::any::Any>, _tolerance: f64) -> Box<dyn std::any::Any> {
        // Java: final Line line = new Line(start, end, tolerance);
        // Java: return new IntervalsSet(line.toSubSpace((Point<Euclidean2D>) start).getX(),
        // Java: line.toSubSpace((Point<Euclidean2D>) end).getX(),
        // Java: tolerance);
        todo!()
    }

    // Java: protected AbstractSubHyperplane<Euclidean2D, Euclidean1D> buildNew()
    pub fn buildNew(&self, _hyperplane: Box<dyn std::any::Any>, _remainingRegion: Box<dyn std::any::Any>) -> SubLine {
        // Java: return new SubLine(hyperplane, remainingRegion);
        todo!()
    }

    // Java: SplitSubHyperplane<Euclidean2D> split(final Hyperplane<Euclidean2D> hyperplane)
    pub fn split(&self, _hyperplane: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: final Line    thisLine  = (Line) getHyperplane();
        // Java: final Line    otherLine = (Line) hyperplane;
        // Java: final Vector2D crossing = thisLine.intersection(otherLine);
        // Java: final double tolerance  = thisLine.getTolerance();
        // Java: if (crossing == null)
        {
            // Java: // the lines are parallel
            // Java: final double global = otherLine.getOffset(thisLine);
            // Java: if (global < -tolerance)
            {
                // Java: return new SplitSubHyperplane<Euclidean2D>(null, this);
            }
            // Java: else if (global > tolerance)
            {
                // Java: return new SplitSubHyperplane<Euclidean2D>(this, null);
            }
            // Java: else
            {
                // Java: return new SplitSubHyperplane<Euclidean2D>(null, null);
            }
        }
        // Java: // the lines do intersect
        // Java: final boolean direct = FastMath.sin(thisLine.getAngle() - otherLine.getAngle()) < 0;
        // Java: final Vector1D x      = thisLine.toSubSpace((Point<Euclidean2D>) crossing);
        // Java: final SubHyperplane<Euclidean1D> subPlus  =
        // Java: new OrientedPoint(x, !direct, tolerance).wholeHyperplane();
        // Java: final SubHyperplane<Euclidean1D> subMinus =
        // Java: new OrientedPoint(x,  direct, tolerance).wholeHyperplane();
        // Java: final BSPTree<Euclidean1D> splitTree = getRemainingRegion().getTree(false).split(subMinus);
        // Java: final BSPTree<Euclidean1D> plusTree  = getRemainingRegion().isEmpty(splitTree.getPlus()) ?
        // Java: new BSPTree<Euclidean1D>(Boolean.FALSE) :
        // Java: new BSPTree<Euclidean1D>(subPlus, new BSPTree<Euclidean1D>(Boolean.FALSE),
        // Java: splitTree.getPlus(), null);
        // Java: final BSPTree<Euclidean1D> minusTree = getRemainingRegion().isEmpty(splitTree.getMinus()) ?
        // Java: new BSPTree<Euclidean1D>(Boolean.FALSE) :
        // Java: new BSPTree<Euclidean1D>(subMinus, new BSPTree<Euclidean1D>(Boolean.FALSE),
        // Java: splitTree.getMinus(), null);
        // Java: return new SplitSubHyperplane<Euclidean2D>(new SubLine(thisLine.copySelf(), new IntervalsSet(plusTree, tolerance)),
        // Java: new SubLine(thisLine.copySelf(), new IntervalsSet(minusTree, tolerance)));
        todo!()
    }

}
