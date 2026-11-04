// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.NoSuchElementException
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractRegion
use crate::org::apache::commons::math3::geometry::partitioning::AbstractRegion;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.BoundaryProjection
use crate::org::apache::commons::math3::geometry::partitioning::BoundaryProjection;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: double DEFAULT_TOLERANCE
pub(crate) const DEFAULT_TOLERANCE: f64 = 1.0e-10;

// Java: public class represents
// Java: public class IntervalsSet
pub struct IntervalsSet {
    tolerance: f64,
}

// Java: public class SubIntervalsIterator
pub struct SubIntervalsIterator {
    // Java field: BSPTree<Euclidean1D> current
    pub(crate) current: Option<usize>,
    // Java field: double[] pending
    pub(crate) pending: Option<Vec<f64>>,
}

// Java: public class represents

// Java: public class IntervalsSet
#[allow(non_snake_case)]
impl IntervalsSet {
    // Java: IntervalsSet(final double tolerance)
    pub fn new_with_tolerance(tolerance: f64) -> Self {
        // Java: super(tolerance);
        IntervalsSet { tolerance }
    }

    // Java: IntervalsSet(final double lower, final double upper, final double tolerance)
    pub fn new_with_bounds(lower: f64, upper: f64, tolerance: f64) -> Self {
        // Java: super(buildTree(lower, upper, tolerance), tolerance);
        IntervalsSet { tolerance }
    }

    // Java: IntervalsSet(final BSPTree<Euclidean1D> tree, final double tolerance)
    pub fn new_with_tree(tolerance: f64) -> Self {
        // Java: super(tree, tolerance);
        IntervalsSet { tolerance }
    }

    // Java: IntervalsSet()
    pub fn new_default() -> Self {
        // Java: this(DEFAULT_TOLERANCE);
        IntervalsSet { tolerance: DEFAULT_TOLERANCE }
    }


    // Java: IntervalsSet()
    pub fn new() -> Self {
        // Java: this(DEFAULT_TOLERANCE);
        IntervalsSet { tolerance: DEFAULT_TOLERANCE }
    }

    // Java: IntervalsSet(final double lower, final double upper)
    pub fn new_deprecated_bounds(lower: f64, upper: f64) -> Self {
        // Java: this(lower, upper, DEFAULT_TOLERANCE);
        Self::new_with_bounds(lower, upper, DEFAULT_TOLERANCE)
    }

    // Java: IntervalsSet(final BSPTree<Euclidean1D> tree)
    pub fn new_deprecated_tree() -> Self {
        // Java: this(tree, DEFAULT_TOLERANCE);
        Self::new_with_tree(DEFAULT_TOLERANCE)
    }

    // Java: IntervalsSet(final Collection<SubHyperplane<Euclidean1D>> boundary)
    pub fn new_deprecated_boundary() -> Self {
        // Java: this(boundary, DEFAULT_TOLERANCE);
        IntervalsSet { tolerance: DEFAULT_TOLERANCE }
    }

    // Java: static BSPTree<Euclidean1D> buildTree()
    pub fn buildTree(_lower: f64, _upper: f64, _tolerance: f64) {
        todo!()
    }

    // Java: IntervalsSet buildNew(final BSPTree<Euclidean1D> tree)
    pub fn buildNew(&self) -> IntervalsSet {
        // Java: return new IntervalsSet(tree, getTolerance());
        IntervalsSet { tolerance: self.tolerance }
    }

    // Java: void computeGeometricalProperties()
    pub fn computeGeometricalProperties(&self) {
        // Java: if (getTree(false).getCut() == null)
        // Java: setBarycenter((Point<Euclidean1D>) Vector1D.NaN);
        // Java: setSize(((Boolean) getTree(false).getAttribute()) ? Double.POSITIVE_INFINITY : 0);
        // Java: else
        // Java: double size = 0.0;
        // Java: double sum = 0.0;
        // Java: for (final Interval interval : asList())
        // Java: size += interval.getSize();
        // Java: sum  += interval.getSize() * interval.getBarycenter();
        // Java: setSize(size);
        // Java: if (Double.isInfinite(size))
        // Java: setBarycenter((Point<Euclidean1D>) Vector1D.NaN);
        // Java: else if (size >= Precision.SAFE_MIN)
        // Java: setBarycenter((Point<Euclidean1D>) new Vector1D(sum / size));
        // Java: else
        // Java: setBarycenter((Point<Euclidean1D>) ((OrientedPoint) getTree(false).getCut().getHyperplane()).getLocation());
        todo!()
    }

    // Java: double getInf()
    pub fn getInf(&self) -> f64 {
        // Java: BSPTree<Euclidean1D> node = getTree(false);
        // Java: double  inf  = Double.POSITIVE_INFINITY;
        // Java: while (node.getCut() != null)
        // Java: final OrientedPoint op = (OrientedPoint) node.getCut().getHyperplane();
        // Java: inf  = op.getLocation().getX();
        // Java: node = op.isDirect() ? node.getMinus() : node.getPlus();
        // Java: return ((Boolean) node.getAttribute()) ? Double.NEGATIVE_INFINITY : inf;
        todo!()
    }

    // Java: double getSup()
    pub fn getSup(&self) -> f64 {
        // Java: BSPTree<Euclidean1D> node = getTree(false);
        // Java: double  sup  = Double.NEGATIVE_INFINITY;
        // Java: while (node.getCut() != null)
        // Java: final OrientedPoint op = (OrientedPoint) node.getCut().getHyperplane();
        // Java: sup  = op.getLocation().getX();
        // Java: node = op.isDirect() ? node.getPlus() : node.getMinus();
        // Java: return ((Boolean) node.getAttribute()) ? Double.POSITIVE_INFINITY : sup;
        todo!()
    }

    // Java: BoundaryProjection<Euclidean1D> projectToBoundary(final Point<Euclidean1D> point)
    pub fn projectToBoundary(&self) {
        // Java: // get position of test point
        // Java: final double x = ((Vector1D) point).getX();
        // Java: double previous = Double.NEGATIVE_INFINITY;
        // Java: for (final double[] a : this)
        // Java: if (x < a[0])
        // Java: // the test point lies between the previous and the current intervals
        // Java: // offset will be positive
        // Java: final double previousOffset = x - previous;
        // Java: final double currentOffset  = a[0] - x;
        // Java: if (previousOffset < currentOffset)
        // Java: return new BoundaryProjection<Euclidean1D>(point, finiteOrNullPoint(previous), previousOffset);
        // Java: else
        // Java: return new BoundaryProjection<Euclidean1D>(point, finiteOrNullPoint(a[0]), currentOffset);
        // Java: else if (x <= a[1])
        // Java: // the test point lies within the current interval
        // Java: // offset will be negative
        // Java: final double offset0 = a[0] - x;
        // Java: final double offset1 = x - a[1];
        // Java: if (offset0 < offset1)
        // Java: return new BoundaryProjection<Euclidean1D>(point, finiteOrNullPoint(a[1]), offset1);
        // Java: else
        // Java: return new BoundaryProjection<Euclidean1D>(point, finiteOrNullPoint(a[0]), offset0);
        // Java: previous = a[1];
        // Java: // the test point if past the last sub-interval
        // Java: return new BoundaryProjection<Euclidean1D>(point, finiteOrNullPoint(previous), x - previous);
        todo!()
    }

    // Java: Vector1D finiteOrNullPoint(final double x)
    pub fn finiteOrNullPoint(&self, _x: f64) {
        // Java: return Double.isInfinite(x) ? null : new Vector1D(x);
        todo!()
    }

    // Java: List<Interval> asList()
    pub fn asList(&self) -> Vec<(f64, f64)> {
        // Java: final List<Interval> list = new ArrayList<Interval>();
        // Java: for (final double[] a : this)
        // Java: list.add(new Interval(a[0], a[1]));
        // Java: return list;
        todo!()
    }

    // Java: BSPTree<Euclidean1D> getFirstLeaf(final BSPTree<Euclidean1D> root)
    pub fn getFirstLeaf(&self) {
        // Java: if (root.getCut() == null)
        // Java: return root;
        // Java: // find the smallest internal node
        // Java: BSPTree<Euclidean1D> smallest = null;
        // Java: for (BSPTree<Euclidean1D> n = root; n != null; n = previousInternalNode(n))
        // Java: smallest = n;
        // Java: return leafBefore(smallest);
        todo!()
    }

    // Java: BSPTree<Euclidean1D> getFirstIntervalBoundary()
    pub fn getFirstIntervalBoundary(&self) {
        // Java: // start search at the tree root
        // Java: BSPTree<Euclidean1D> node = getTree(false);
        // Java: if (node.getCut() == null)
        // Java: return null;
        // Java: // walk tree until we find the smallest internal node
        // Java: node = getFirstLeaf(node).getParent();
        // Java: // walk tree until we find an interval boundary
        // Java: while (node != null && !(isIntervalStart(node) || isIntervalEnd(node)))
        // Java: node = nextInternalNode(node);
        // Java: return node;
        todo!()
    }

    // Java: boolean isIntervalStart(final BSPTree<Euclidean1D> node)
    pub fn isIntervalStart(&self) -> bool {
        // Java: if ((Boolean) leafBefore(node).getAttribute())
        // Java: // it has an inside cell before it, it may end an interval but not start it
        // Java: return false;
        // Java: if (!(Boolean) leafAfter(node).getAttribute())
        // Java: // it has an outside cell after it, it is a dummy cut away from real intervals
        // Java: return false;
        // Java: // the cell has an outside before and an inside after it
        // Java: // it is the start of an interval
        // Java: return true;
        todo!()
    }

    // Java: boolean isIntervalEnd(final BSPTree<Euclidean1D> node)
    pub fn isIntervalEnd(&self) -> bool {
        // Java: if (!(Boolean) leafBefore(node).getAttribute())
        // Java: // it has an outside cell before it, it may start an interval but not end it
        // Java: return false;
        // Java: if ((Boolean) leafAfter(node).getAttribute())
        // Java: // it has an inside cell after it, it is a dummy cut in the middle of an interval
        // Java: return false;
        // Java: // the cell has an inside before and an outside after it
        // Java: // it is the end of an interval
        // Java: return true;
        todo!()
    }

    // Java: BSPTree<Euclidean1D> nextInternalNode(BSPTree<Euclidean1D> node)
    pub fn nextInternalNode(&self) {
        // Java: if (childAfter(node).getCut() != null)
        // Java: // the next node is in the sub-tree
        // Java: return leafAfter(node).getParent();
        // Java: // there is nothing left deeper in the tree, we backtrack
        // Java: while (isAfterParent(node))
        // Java: node = node.getParent();
        // Java: return node.getParent();
        todo!()
    }

    // Java: BSPTree<Euclidean1D> previousInternalNode(BSPTree<Euclidean1D> node)
    pub fn previousInternalNode(&self) {
        // Java: if (childBefore(node).getCut() != null)
        // Java: // the next node is in the sub-tree
        // Java: return leafBefore(node).getParent();
        // Java: // there is nothing left deeper in the tree, we backtrack
        // Java: while (isBeforeParent(node))
        // Java: node = node.getParent();
        // Java: return node.getParent();
        todo!()
    }

    // Java: BSPTree<Euclidean1D> leafBefore(BSPTree<Euclidean1D> node)
    pub fn leafBefore(&self) {
        // Java: node = childBefore(node);
        // Java: while (node.getCut() != null)
        // Java: node = childAfter(node);
        // Java: return node;
        todo!()
    }

    // Java: BSPTree<Euclidean1D> leafAfter(BSPTree<Euclidean1D> node)
    pub fn leafAfter(&self) {
        // Java: node = childAfter(node);
        // Java: while (node.getCut() != null)
        // Java: node = childBefore(node);
        // Java: return node;
        todo!()
    }

    // Java: boolean isBeforeParent(final BSPTree<Euclidean1D> node)
    pub fn isBeforeParent(&self) -> bool {
        // Java: final BSPTree<Euclidean1D> parent = node.getParent();
        // Java: if (parent == null)
        // Java: return false;
        // Java: else
        // Java: return node == childBefore(parent);
        todo!()
    }

    // Java: boolean isAfterParent(final BSPTree<Euclidean1D> node)
    pub fn isAfterParent(&self) -> bool {
        // Java: final BSPTree<Euclidean1D> parent = node.getParent();
        // Java: if (parent == null)
        // Java: return false;
        // Java: else
        // Java: return node == childAfter(parent);
        todo!()
    }

    // Java: BSPTree<Euclidean1D> childBefore(BSPTree<Euclidean1D> node)
    pub fn childBefore(&self) {
        // Java: if (isDirect(node))
        // Java: // smaller abscissas are on minus side, larger abscissas are on plus side
        // Java: return node.getMinus();
        // Java: else
        // Java: // smaller abscissas are on plus side, larger abscissas are on minus side
        // Java: return node.getPlus();
        todo!()
    }

    // Java: BSPTree<Euclidean1D> childAfter(BSPTree<Euclidean1D> node)
    pub fn childAfter(&self) {
        // Java: if (isDirect(node))
        // Java: // smaller abscissas are on minus side, larger abscissas are on plus side
        // Java: return node.getPlus();
        // Java: else
        // Java: // smaller abscissas are on plus side, larger abscissas are on minus side
        // Java: return node.getMinus();
        todo!()
    }

    // Java: boolean isDirect(final BSPTree<Euclidean1D> node)
    pub fn isDirect(&self) -> bool {
        // Java: return ((OrientedPoint) node.getCut().getHyperplane()).isDirect();
        todo!()
    }

    // Java: double getAngle(final BSPTree<Euclidean1D> node)
    pub fn getAngle(&self) -> f64 {
        // Java: return ((OrientedPoint) node.getCut().getHyperplane()).getLocation().getX();
        todo!()
    }

    // Java: Iterator<double[]> iterator()
    pub fn iterator(&self) -> SubIntervalsIterator {
        // Java: return new SubIntervalsIterator();
        todo!()
    }
}

// Java: public class SubIntervalsIterator
#[allow(non_snake_case)]
impl SubIntervalsIterator {
    // Java: void selectPending()
    pub fn selectPending(&mut self) {
        // Java: // look for the start of the interval
        // Java: BSPTree<Euclidean1D> start = current;
        // Java: while (start != null && !isIntervalStart(start))
        // Java: start = nextInternalNode(start);
        // Java: if (start == null)
        // Java: // we have exhausted the iterator
        // Java: current = null;
        // Java: pending = null;
        // Java: return;
        // Java: // look for the end of the interval
        // Java: BSPTree<Euclidean1D> end = start;
        // Java: while (end != null && !isIntervalEnd(end))
        // Java: end = nextInternalNode(end);
        // Java: if (end != null)
        // Java: // we have identified the interval
        // Java: pending = new double[]
        // Java: getAngle(start), getAngle(end)
        // Java: // prepare search for next interval
        // Java: current = end;
        // Java: else
        // Java: // the final interval is open toward infinity
        // Java: pending = new double[]
        // Java: getAngle(start), Double.POSITIVE_INFINITY
        // Java: // there won't be any other intervals
        // Java: current = null;
        todo!()
    }

    // Java: boolean hasNext()
    pub fn hasNext(&self) -> bool {
        // Java: return pending != null;
        self.pending.is_some()
    }

    // Java: double[] next()
    pub fn next(&mut self) -> Vec<f64> {
        // Java: if (pending == null)
        // Java: throw new NoSuchElementException();
        // Java: final double[] next = pending;
        // Java: selectPending();
        // Java: return next;
        if self.pending.is_none() {
            panic!("NoSuchElementException");
        }
        let next = self.pending.take().unwrap();
        self.selectPending();
        next
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }
}
