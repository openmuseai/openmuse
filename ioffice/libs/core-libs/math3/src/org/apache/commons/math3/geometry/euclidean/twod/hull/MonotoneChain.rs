#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Collections
// Java: import java.util.Comparator
// Java: import java.util.List
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Line
use crate::org::apache::commons::math3::geometry::euclidean::twod::Line;
// Java: import org.apache.commons.math3.geometry.euclidean.twod.Vector2D
use crate::org::apache::commons::math3::geometry::euclidean::twod::Vector2D;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

use std::any::Any;

// Java: public class MonotoneChain extends AbstractConvexHullGenerator2D
pub struct MonotoneChain {
    pub(crate) includeCollinearPoints: bool,
    pub(crate) tolerance: f64,
}

// Java: public class MonotoneChain extends AbstractConvexHullGenerator2D
impl MonotoneChain {
    // Java: MonotoneChain()
    pub fn new() -> Self {
        // Java: this(false);
        MonotoneChain {
            includeCollinearPoints: false,
            tolerance: 1.0e-10,
        }
    }

    // Java: MonotoneChain(final boolean includeCollinearPoints)
    pub fn new_with_collinear(includeCollinearPoints: bool) -> Self {
        // Java: super(includeCollinearPoints);
        MonotoneChain {
            includeCollinearPoints,
            tolerance: 1.0e-10,
        }
    }

    // Java: MonotoneChain(final boolean includeCollinearPoints, final double tolerance)
    pub fn new_with_collinear_and_tolerance(includeCollinearPoints: bool, tolerance: f64) -> Self {
        // Java: super(includeCollinearPoints, tolerance);
        MonotoneChain {
            includeCollinearPoints,
            tolerance,
        }
    }

    // Java: Collection<Vector2D> findHullVertices(final Collection<Vector2D> points)
    pub fn findHullVertices(&self, _points: &[Box<dyn Any>]) -> Vec<Box<dyn Any>> {
        // Java: final List<Vector2D> pointsSortedByXAxis = new ArrayList<Vector2D>(points);
        // Java: // sort the points in increasing order on the x-axis
        // Java: Collections.sort(pointsSortedByXAxis, new Comparator<Vector2D>()
        // Java: /** {@inheritDoc} */
        // Java: public int compare(final Vector2D o1, final Vector2D o2)
        // Java: final double tolerance = getTolerance();
        // Java: // need to take the tolerance value into account, otherwise collinear points
        // Java: // will not be handled correctly when building the upper/lower hull
        // Java: final int diff = Precision.compareTo(o1.getX(), o2.getX(), tolerance);
        // Java: if (diff == 0)
        // Java: return Precision.compareTo(o1.getY(), o2.getY(), tolerance);
        // Java: else
        // Java: return diff;
        // Java: // build lower hull
        // Java: final List<Vector2D> lowerHull = new ArrayList<Vector2D>();
        // Java: for (Vector2D p : pointsSortedByXAxis)
        // Java: updateHull(p, lowerHull);
        // Java: // build upper hull
        // Java: final List<Vector2D> upperHull = new ArrayList<Vector2D>();
        // Java: for (int idx = pointsSortedByXAxis.size() - 1; idx >= 0; idx--)
        // Java: final Vector2D p = pointsSortedByXAxis.get(idx);
        // Java: updateHull(p, upperHull);
        // Java: // concatenate the lower and upper hulls
        // Java: // the last point of each list is omitted as it is repeated at the beginning of the other list
        // Java: final List<Vector2D> hullVertices = new ArrayList<Vector2D>(lowerHull.size() + upperHull.size() - 2);
        // Java: for (int idx = 0; idx < lowerHull.size() - 1; idx++)
        // Java: hullVertices.add(lowerHull.get(idx));
        // Java: for (int idx = 0; idx < upperHull.size() - 1; idx++)
        // Java: hullVertices.add(upperHull.get(idx));
        // Java: // special case: if the lower and upper hull may contain only 1 point if all are identical
        // Java: if (hullVertices.isEmpty() && ! lowerHull.isEmpty())
        // Java: hullVertices.add(lowerHull.get(0));
        // Java: return hullVertices;
        todo!()
    }

    // Java: void updateHull(final Vector2D point, final List<Vector2D> hull)
    pub fn updateHull(&self, _point: &Box<dyn Any>, _hull: &mut Vec<Box<dyn Any>>) {
        // Java: final double tolerance = getTolerance();
        // Java: if (hull.size() == 1)
        // Java: // ensure that we do not add an identical point
        // Java: final Vector2D p1 = hull.get(0);
        // Java: if (p1.distance(point) < tolerance)
        // Java: return;
        // Java: while (hull.size() >= 2)
        // Java: final int size = hull.size();
        // Java: final Vector2D p1 = hull.get(size - 2);
        // Java: final Vector2D p2 = hull.get(size - 1);
        // Java: final double offset = new Line(p1, p2, tolerance).getOffset(point);
        // Java: if (FastMath.abs(offset) < tolerance)
        // Java: // the point is collinear to the line (p1, p2)
        // Java: final double distanceToCurrent = p1.distance(point);
        // Java: if (distanceToCurrent < tolerance || p2.distance(point) < tolerance)
        // Java: // the point is assumed to be identical to either p1 or p2
        // Java: return;
        // Java: final double distanceToLast = p1.distance(p2);
        // Java: if (isIncludeCollinearPoints())
        // Java: final int index = distanceToCurrent < distanceToLast ? size - 1 : size;
        // Java: hull.add(index, point);
        // Java: else
        // Java: if (distanceToCurrent > distanceToLast)
        // Java: hull.remove(size - 1);
        // Java: hull.add(point);
        // Java: return;
        // Java: else if (offset > 0)
        // Java: hull.remove(size - 1);
        // Java: else
        // Java: break;
        // Java: hull.add(point);
        todo!()
    }
}
