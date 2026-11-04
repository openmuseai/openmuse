#![allow(unused_imports, dead_code, non_snake_case, unused_variables, unused_mut, unused_assignments)]

// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Iterator
// Java: import java.util.List
// Java: import java.util.NoSuchElementException
// Java: import org.apache.commons.math3.exception.MathIllegalArgumentException
use crate::org::apache::commons::math3::exception::MathIllegalArgumentException;
// Java: import org.apache.commons.math3.exception.MathInternalError
use crate::org::apache::commons::math3::exception::MathInternalError;
// Java: import org.apache.commons.math3.exception.NumberIsTooLargeException
use crate::org::apache::commons::math3::exception::NumberIsTooLargeException;
// Java: import org.apache.commons.math3.exception.util.LocalizedFormats
use crate::org::apache::commons::math3::exception::util::LocalizedFormats;
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.partitioning.AbstractRegion
use crate::org::apache::commons::math3::geometry::partitioning::AbstractRegion;
// Java: import org.apache.commons.math3.geometry.partitioning.BSPTree
use crate::org::apache::commons::math3::geometry::partitioning::BSPTree;
// Java: import org.apache.commons.math3.geometry.partitioning.BoundaryProjection
use crate::org::apache::commons::math3::geometry::partitioning::BoundaryProjection;
// Java: import org.apache.commons.math3.geometry.partitioning.Side
use crate::org::apache::commons::math3::geometry::partitioning::Side;
// Java: import org.apache.commons.math3.geometry.partitioning.SubHyperplane
use crate::org::apache::commons::math3::geometry::partitioning::SubHyperplane;
// Java: import org.apache.commons.math3.util.FastMath
use crate::org::apache::commons::math3::util::FastMath::FastMath;
// Java: import org.apache.commons.math3.util.MathUtils
use crate::org::apache::commons::math3::util::MathUtils;
// Java: import org.apache.commons.math3.util.Precision
use crate::org::apache::commons::math3::util::Precision;

// Java field: long serialVersionUID
pub(crate) const serialVersionUID: i64 = 7107764899289009;

// Java: public class represents
pub struct represents {
}

// Java: public class ArcsSet extends AbstractRegion implements Iterable
pub struct ArcsSet {
    pub(crate) tolerance: f64,
}

// Java: public class SubArcsIterator
pub struct SubArcsIterator {
    // Java field: BSPTree<Sphere1D> firstStart
    pub(crate) firstStart: Option<Box<dyn std::any::Any>>,
    // Java field: BSPTree<Sphere1D> current
    pub(crate) current: Option<Box<dyn std::any::Any>>,
    // Java field: double[] pending
    pub(crate) pending: Option<Vec<f64>>,
}

// Java: public class Split
pub struct Split {
    // Java field: ArcsSet plus
    pub(crate) plus: Option<Box<ArcsSet>>,
    // Java field: ArcsSet minus
    pub(crate) minus: Option<Box<ArcsSet>>,
}

// Java: public class InconsistentStateAt2PiWrapping
pub struct InconsistentStateAt2PiWrapping {
}

// Java: public class represents
impl represents {
}

// Java: public class ArcsSet extends AbstractRegion implements Iterable
impl ArcsSet {
    // Java: ArcsSet(final double tolerance)
    pub fn new(tolerance: f64) -> Self {
        // Java: super(tolerance);
        ArcsSet { tolerance }
    }

    // Java: ArcsSet(final double lower, final double upper, final double tolerance)
    pub fn new_from_bounds(_lower: f64, _upper: f64, tolerance: f64) -> Self {
        todo!()
    }

    // Java: ArcsSet(final BSPTree<Sphere1D> tree, final double tolerance)
    pub fn new_from_tree(_tree: Box<dyn std::any::Any>, tolerance: f64) -> Self {
        todo!()
    }

    // Java: ArcsSet(final Collection<SubHyperplane<Sphere1D>> boundary, final double tolerance)
    pub fn new_from_boundary(_boundary: Vec<Box<dyn std::any::Any>>, tolerance: f64) -> Self {
        todo!()
    }

    // Java: static BSPTree<Sphere1D> buildTree()
    pub fn buildTree() -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: void check2PiConsistency()
    pub fn check2PiConsistency(&self) {
        // Java: // start search at the tree root
        // Java: BSPTree<Sphere1D> root = getTree(false);
        // Java: if (root.getCut() == null)
        // Java: return;
        // Java: // find the inside/outside state before the smallest internal node
        // Java: final Boolean stateBefore = (Boolean) getFirstLeaf(root).getAttribute();
        // Java: // find the inside/outside state after the largest internal node
        // Java: final Boolean stateAfter = (Boolean) getLastLeaf(root).getAttribute();
        // Java: if (stateBefore ^ stateAfter)
        // Java: throw new InconsistentStateAt2PiWrapping();
        todo!()
    }

    // Java: BSPTree<Sphere1D> getFirstLeaf(final BSPTree<Sphere1D> root)
    pub fn getFirstLeaf(&self, _root: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: if (root.getCut() == null)
        // Java: return root;
        // Java: // find the smallest internal node
        // Java: BSPTree<Sphere1D> smallest = null;
        // Java: for (BSPTree<Sphere1D> n = root; n != null; n = previousInternalNode(n))
        // Java: smallest = n;
        // Java: return leafBefore(smallest);
        todo!()
    }

    // Java: BSPTree<Sphere1D> getLastLeaf(final BSPTree<Sphere1D> root)
    pub fn getLastLeaf(&self, _root: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: if (root.getCut() == null)
        // Java: return root;
        // Java: // find the largest internal node
        // Java: BSPTree<Sphere1D> largest = null;
        // Java: for (BSPTree<Sphere1D> n = root; n != null; n = nextInternalNode(n))
        // Java: largest = n;
        // Java: return leafAfter(largest);
        todo!()
    }

    // Java: BSPTree<Sphere1D> getFirstArcStart()
    pub fn getFirstArcStart(&self) -> Option<Box<dyn std::any::Any>> {
        // Java: // start search at the tree root
        // Java: BSPTree<Sphere1D> node = getTree(false);
        // Java: if (node.getCut() == null)
        // Java: return null;
        // Java: // walk tree until we find the smallest internal node
        // Java: node = getFirstLeaf(node).getParent();
        // Java: // walk tree until we find an arc start
        // Java: while (node != null && !isArcStart(node))
        // Java: node = nextInternalNode(node);
        // Java: return node;
        todo!()
    }

    // Java: boolean isArcStart(final BSPTree<Sphere1D> node)
    pub fn isArcStart(&self, _node: &dyn std::any::Any) -> bool {
        // Java: if ((Boolean) leafBefore(node).getAttribute())
        // Java: // it has an inside cell before it, it may end an arc but not start it
        // Java: return false;
        // Java: if (!(Boolean) leafAfter(node).getAttribute())
        // Java: // it has an outside cell after it, it is a dummy cut away from real arcs
        // Java: return false;
        // Java: // the cell has an outside before and an inside after it
        // Java: // it is the start of an arc
        // Java: return true;
        todo!()
    }

    // Java: boolean isArcEnd(final BSPTree<Sphere1D> node)
    pub fn isArcEnd(&self, _node: &dyn std::any::Any) -> bool {
        // Java: if (!(Boolean) leafBefore(node).getAttribute())
        // Java: // it has an outside cell before it, it may start an arc but not end it
        // Java: return false;
        // Java: if ((Boolean) leafAfter(node).getAttribute())
        // Java: // it has an inside cell after it, it is a dummy cut in the middle of an arc
        // Java: return false;
        // Java: // the cell has an inside before and an outside after it
        // Java: // it is the end of an arc
        // Java: return true;
        todo!()
    }

    // Java: BSPTree<Sphere1D> nextInternalNode(BSPTree<Sphere1D> node)
    pub fn nextInternalNode(&self, _node: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
        // Java: if (childAfter(node).getCut() != null)
        // Java: // the next node is in the sub-tree
        // Java: return leafAfter(node).getParent();
        // Java: // there is nothing left deeper in the tree, we backtrack
        // Java: while (isAfterParent(node))
        // Java: node = node.getParent();
        // Java: return node.getParent();
        todo!()
    }

    // Java: BSPTree<Sphere1D> previousInternalNode(BSPTree<Sphere1D> node)
    pub fn previousInternalNode(&self, _node: &dyn std::any::Any) -> Option<Box<dyn std::any::Any>> {
        // Java: if (childBefore(node).getCut() != null)
        // Java: // the next node is in the sub-tree
        // Java: return leafBefore(node).getParent();
        // Java: // there is nothing left deeper in the tree, we backtrack
        // Java: while (isBeforeParent(node))
        // Java: node = node.getParent();
        // Java: return node.getParent();
        todo!()
    }

    // Java: BSPTree<Sphere1D> leafBefore(BSPTree<Sphere1D> node)
    pub fn leafBefore(&self, _node: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: node = childBefore(node);
        // Java: while (node.getCut() != null)
        // Java: node = childAfter(node);
        // Java: return node;
        todo!()
    }

    // Java: BSPTree<Sphere1D> leafAfter(BSPTree<Sphere1D> node)
    pub fn leafAfter(&self, _node: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: node = childAfter(node);
        // Java: while (node.getCut() != null)
        // Java: node = childBefore(node);
        // Java: return node;
        todo!()
    }

    // Java: boolean isBeforeParent(final BSPTree<Sphere1D> node)
    pub fn isBeforeParent(&self, _node: &dyn std::any::Any) -> bool {
        // Java: final BSPTree<Sphere1D> parent = node.getParent();
        // Java: if (parent == null)
        // Java: return false;
        // Java: else
        // Java: return node == childBefore(parent);
        todo!()
    }

    // Java: boolean isAfterParent(final BSPTree<Sphere1D> node)
    pub fn isAfterParent(&self, _node: &dyn std::any::Any) -> bool {
        // Java: final BSPTree<Sphere1D> parent = node.getParent();
        // Java: if (parent == null)
        // Java: return false;
        // Java: else
        // Java: return node == childAfter(parent);
        todo!()
    }

    // Java: BSPTree<Sphere1D> childBefore(BSPTree<Sphere1D> node)
    pub fn childBefore(&self, _node: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: if (isDirect(node))
        // Java: // smaller angles are on minus side, larger angles are on plus side
        // Java: return node.getMinus();
        // Java: else
        // Java: // smaller angles are on plus side, larger angles are on minus side
        // Java: return node.getPlus();
        todo!()
    }

    // Java: BSPTree<Sphere1D> childAfter(BSPTree<Sphere1D> node)
    pub fn childAfter(&self, _node: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: if (isDirect(node))
        // Java: // smaller angles are on minus side, larger angles are on plus side
        // Java: return node.getPlus();
        // Java: else
        // Java: // smaller angles are on plus side, larger angles are on minus side
        // Java: return node.getMinus();
        todo!()
    }

    // Java: boolean isDirect(final BSPTree<Sphere1D> node)
    pub fn isDirect(&self, _node: &dyn std::any::Any) -> bool {
        // Java: return ((LimitAngle) node.getCut().getHyperplane()).isDirect();
        todo!()
    }

    // Java: double getAngle(final BSPTree<Sphere1D> node)
    pub fn getAngle(&self, _node: &dyn std::any::Any) -> f64 {
        // Java: return ((LimitAngle) node.getCut().getHyperplane()).getLocation().getAlpha();
        todo!()
    }

    // Java: ArcsSet buildNew(final BSPTree<Sphere1D> tree)
    pub fn buildNew(&self, _tree: Box<dyn std::any::Any>) -> Box<dyn std::any::Any> {
        // Java: return new ArcsSet(tree, getTolerance());
        todo!()
    }

    // Java: void computeGeometricalProperties()
    pub fn computeGeometricalProperties(&mut self) {
        // Java: if (getTree(false).getCut() == null)
        // Java: setBarycenter(S1Point.NaN);
        // Java: setSize(((Boolean) getTree(false).getAttribute()) ? MathUtils.TWO_PI : 0);
        // Java: else
        // Java: double size = 0.0;
        // Java: double sum  = 0.0;
        // Java: for (final double[] a : this)
        // Java: final double length = a[1] - a[0];
        // Java: size += length;
        // Java: sum  += length * (a[0] + a[1]);
        // Java: setSize(size);
        // Java: if (Precision.equals(size, MathUtils.TWO_PI, 0))
        // Java: setBarycenter(S1Point.NaN);
        // Java: else if (size >= Precision.SAFE_MIN)
        // Java: setBarycenter(new S1Point(sum / (2 * size)));
        // Java: else
        // Java: final LimitAngle limit = (LimitAngle) getTree(false).getCut().getHyperplane();
        // Java: setBarycenter(limit.getLocation());
        todo!()
    }

    // Java: BoundaryProjection<Sphere1D> projectToBoundary(final Point<Sphere1D> point)
    pub fn projectToBoundary(&self, _point: &dyn std::any::Any) -> Box<dyn std::any::Any> {
        // Java: // get position of test point
        // Java: final double alpha = ((S1Point) point).getAlpha();
        // Java: boolean wrapFirst = false;
        // Java: double first      = Double.NaN;
        // Java: double previous   = Double.NaN;
        // Java: for (final double[] a : this)
        // Java: if (Double.isNaN(first))
        // Java: // remember the first angle in case we need it later
        // Java: first = a[0];
        // Java: if (!wrapFirst)
        // Java: if (alpha < a[0])
        // Java: // the test point lies between the previous and the current arcs
        // Java: // offset will be positive
        // Java: if (Double.isNaN(previous))
        // Java: // we need to wrap around the circle
        // Java: wrapFirst = true;
        // Java: else
        // Java: final double previousOffset = alpha - previous;
        // Java: final double currentOffset  = a[0] - alpha;
        // Java: if (previousOffset < currentOffset)
        // Java: return new BoundaryProjection<Sphere1D>(point, new S1Point(previous), previousOffset);
        // Java: else
        // Java: return new BoundaryProjection<Sphere1D>(point, new S1Point(a[0]), currentOffset);
        // Java: else if (alpha <= a[1])
        // Java: // the test point lies within the current arc
        // Java: // offset will be negative
        // Java: final double offset0 = a[0] - alpha;
        // Java: final double offset1 = alpha - a[1];
        // Java: if (offset0 < offset1)
        // Java: return new BoundaryProjection<Sphere1D>(point, new S1Point(a[1]), offset1);
        // Java: else
        // Java: return new BoundaryProjection<Sphere1D>(point, new S1Point(a[0]), offset0);
        // Java: previous = a[1];
        // Java: if (Double.isNaN(previous))
        // Java: // there are no points at all in the arcs set
        // Java: return new BoundaryProjection<Sphere1D>(point, null, MathUtils.TWO_PI);
        // Java: else
        // Java: // the test point if before first arc and after last arc,
        // Java: // somewhere around the 0/2 \pi crossing
        // Java: if (wrapFirst)
        // Java: // the test point is between 0 and first
        // Java: final double previousOffset = alpha - (previous - MathUtils.TWO_PI);
        // Java: final double currentOffset  = first - alpha;
        // Java: if (previousOffset < currentOffset)
        // Java: return new BoundaryProjection<Sphere1D>(point, new S1Point(previous), previousOffset);
        // Java: else
        // Java: return new BoundaryProjection<Sphere1D>(point, new S1Point(first), currentOffset);
        // Java: else
        // Java: // the test point is between last and 2\pi
        // Java: final double previousOffset = alpha - previous;
        // Java: final double currentOffset  = first + MathUtils.TWO_PI - alpha;
        // Java: if (previousOffset < currentOffset)
        // Java: return new BoundaryProjection<Sphere1D>(point, new S1Point(previous), previousOffset);
        // Java: else
        // Java: return new BoundaryProjection<Sphere1D>(point, new S1Point(first), currentOffset);
        todo!()
    }

    // Java: List<Arc> asList()
    pub fn asList(&self) -> Vec<Box<dyn std::any::Any>> {
        // Java: final List<Arc> list = new ArrayList<Arc>();
        // Java: for (final double[] a : this)
        // Java: list.add(new Arc(a[0], a[1], getTolerance()));
        // Java: return list;
        todo!()
    }

    // Java: Iterator<double[]> iterator()
    pub fn iterator(&self) -> Box<dyn std::any::Any> {
        // Java: return new SubArcsIterator();
        todo!()
    }

    // Java: Side side(final Arc arc)
    pub fn side(&self, _arc: &super::Arc::Arc) -> Box<dyn std::any::Any> {
        // Java: return split(arc).getSide();
        todo!()
    }

    // Java: Split split(final Arc arc)
    pub fn split(&self, _arc: &super::Arc::Arc) -> Box<dyn std::any::Any> {
        // Java: final List<Double> minus = new ArrayList<Double>();
        // Java: final List<Double>  plus = new ArrayList<Double>();
        // Java: final double reference = FastMath.PI + arc.getInf();
        // Java: final double arcLength = arc.getSup() - arc.getInf();
        // Java: for (final double[] a : this)
        // Java: final double syncedStart = MathUtils.normalizeAngle(a[0], reference) - arc.getInf();
        // Java: final double arcOffset   = a[0] - syncedStart;
        // Java: final double syncedEnd   = a[1] - arcOffset;
        // Java: if (syncedStart < arcLength)
        // Java: // the start point a[0] is in the minus part of the arc
        // Java: minus.add(a[0]);
        // Java: if (syncedEnd > arcLength)
        // Java: // the end point a[1] is past the end of the arc
        // Java: // so we leave the minus part and enter the plus part
        // Java: final double minusToPlus = arcLength + arcOffset;
        // Java: minus.add(minusToPlus);
        // Java: plus.add(minusToPlus);
        // Java: if (syncedEnd > MathUtils.TWO_PI)
        // Java: // in fact the end point a[1] goes far enough that we
        // Java: // leave the plus part of the arc and enter the minus part again
        // Java: final double plusToMinus = MathUtils.TWO_PI + arcOffset;
        // Java: plus.add(plusToMinus);
        // Java: minus.add(plusToMinus);
        // Java: minus.add(a[1]);
        // Java: else
        // Java: // the end point a[1] is in the plus part of the arc
        // Java: plus.add(a[1]);
        // Java: else
        // Java: // the end point a[1] is in the minus part of the arc
        // Java: minus.add(a[1]);
        // Java: else
        // Java: // the start point a[0] is in the plus part of the arc
        // Java: plus.add(a[0]);
        // Java: if (syncedEnd > MathUtils.TWO_PI)
        // Java: // the end point a[1] wraps around to the start of the arc
        // Java: // so we leave the plus part and enter the minus part
        // Java: final double plusToMinus = MathUtils.TWO_PI + arcOffset;
        // Java: plus.add(plusToMinus);
        // Java: minus.add(plusToMinus);
        // Java: if (syncedEnd > MathUtils.TWO_PI + arcLength)
        // Java: // in fact the end point a[1] goes far enough that we
        // Java: // leave the minus part of the arc and enter the plus part again
        // Java: final double minusToPlus = MathUtils.TWO_PI + arcLength + arcOffset;
        // Java: minus.add(minusToPlus);
        // Java: plus.add(minusToPlus);
        // Java: plus.add(a[1]);
        // Java: else
        // Java: // the end point a[1] is in the minus part of the arc
        // Java: minus.add(a[1]);
        // Java: else
        // Java: // the end point a[1] is in the plus part of the arc
        // Java: plus.add(a[1]);
        // Java: return new Split(createSplitPart(plus), createSplitPart(minus));
        todo!()
    }

    // Java: void addArcLimit(final BSPTree<Sphere1D> tree, final double alpha, final boolean isStart)
    pub fn addArcLimit(&self, _tree: &dyn std::any::Any, _alpha: f64, _is_start: bool) {
        // Java: final LimitAngle limit = new LimitAngle(new S1Point(alpha), !isStart, getTolerance());
        // Java: final BSPTree<Sphere1D> node = tree.getCell(limit.getLocation(), getTolerance());
        // Java: if (node.getCut() != null)
        // Java: // this should never happen
        // Java: throw new MathInternalError();
        // Java: node.insertCut(limit);
        // Java: node.setAttribute(null);
        // Java: node.getPlus().setAttribute(Boolean.FALSE);
        // Java: node.getMinus().setAttribute(Boolean.TRUE);
        todo!()
    }

    // Java: ArcsSet createSplitPart(final List<Double> limits)
    pub fn createSplitPart(&self, _limits: &mut Vec<f64>) -> Option<Box<ArcsSet>> {
        // Java: if (limits.isEmpty())
        // Java: return null;
        // Java: else
        // Java: // collapse close limit angles
        // Java: for (int i = 0; i < limits.size(); ++i)
        // Java: final int    j  = (i + 1) % limits.size();
        // Java: final double lA = limits.get(i);
        // Java: final double lB = MathUtils.normalizeAngle(limits.get(j), lA);
        // Java: if (FastMath.abs(lB - lA) <= getTolerance())
        // Java: // the two limits are too close to each other, we remove both of them
        // Java: if (j > 0)
        // Java: // regular case, the two entries are consecutive ones
        // Java: limits.remove(j);
        // Java: limits.remove(i);
        // Java: i = i - 1;
        // Java: else
        // Java: // special case, i the the last entry and j is the first entry
        // Java: // we have wrapped around list end
        // Java: final double lEnd   = limits.remove(limits.size() - 1);
        // Java: final double lStart = limits.remove(0);
        // Java: if (limits.isEmpty())
        // Java: // the ends were the only limits, is it a full circle or an empty circle?
        // Java: if (lEnd - lStart > FastMath.PI)
        // Java: // it was full circle
        // Java: return new ArcsSet(new BSPTree<Sphere1D>(Boolean.TRUE), getTolerance());
        // Java: else
        // Java: // it was an empty circle
        // Java: return null;
        // Java: else
        // Java: // we have removed the first interval start, so our list
        // Java: // currently starts with an interval end, which is wrong
        // Java: // we need to move this interval end to the end of the list
        // Java: limits.add(limits.remove(0) + MathUtils.TWO_PI);
        // Java: // build the tree by adding all angular sectors
        // Java: BSPTree<Sphere1D> tree = new BSPTree<Sphere1D>(Boolean.FALSE);
        // Java: for (int i = 0; i < limits.size() - 1; i += 2)
        // Java: addArcLimit(tree, limits.get(i),     true);
        // Java: addArcLimit(tree, limits.get(i + 1), false);
        // Java: if (tree.getCut() == null)
        // Java: // we did not insert anything
        // Java: return null;
        // Java: return new ArcsSet(tree, getTolerance());
        todo!()
    }

    pub fn getTolerance(&self) -> f64 {
        self.tolerance
    }

}

// Java: public class SubArcsIterator
impl SubArcsIterator {
    // Java: void selectPending()
    pub fn selectPending(&mut self) {
        // Java: // look for the start of the arc
        // Java: BSPTree<Sphere1D> start = current;
        // Java: while (start != null && !isArcStart(start))
        // Java: start = nextInternalNode(start);
        // Java: if (start == null)
        // Java: // we have exhausted the iterator
        // Java: current = null;
        // Java: pending = null;
        // Java: return;
        // Java: // look for the end of the arc
        // Java: BSPTree<Sphere1D> end = start;
        // Java: while (end != null && !isArcEnd(end))
        // Java: end = nextInternalNode(end);
        // Java: if (end != null)
        // Java: // we have identified the arc
        // Java: pending = new double[]
        // Java: getAngle(start), getAngle(end)
        // Java: // prepare search for next arc
        // Java: current = end;
        // Java: else
        // Java: // the final arc wraps around 2\pi, its end is before the first start
        // Java: end = firstStart;
        // Java: while (end != null && !isArcEnd(end))
        // Java: end = previousInternalNode(end);
        // Java: if (end == null)
        // Java: // this should never happen
        // Java: throw new MathInternalError();
        // Java: // we have identified the last arc
        // Java: pending = new double[]
        // Java: getAngle(start), getAngle(end) + MathUtils.TWO_PI
        // Java: // there won't be any other arcs
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
        todo!()
    }

    // Java: void remove()
    pub fn remove(&self) {
        // Java: throw new UnsupportedOperationException();
        panic!("UnsupportedOperationException");
    }

}

// Java: public class Split
impl Split {
    // Java: Split(final ArcsSet plus, final ArcsSet minus)
    pub fn new(plus: Option<Box<ArcsSet>>, minus: Option<Box<ArcsSet>>) -> Self {
        // Java: this.plus  = plus;
        // Java: this.minus = minus;
        Split { plus, minus }
    }

    // Java: ArcsSet getPlus()
    pub fn getPlus(&self) -> &Option<Box<ArcsSet>> {
        // Java: return plus;
        &self.plus
    }

    // Java: ArcsSet getMinus()
    pub fn getMinus(&self) -> &Option<Box<ArcsSet>> {
        // Java: return minus;
        &self.minus
    }

    // Java: Side getSide()
    pub fn getSide(&self) -> Box<dyn std::any::Any> {
        // Java: if (plus != null)
        // Java: if (minus != null)
        // Java: return Side.BOTH;
        // Java: else
        // Java: return Side.PLUS;
        // Java: else if (minus != null)
        // Java: return Side.MINUS;
        // Java: else
        // Java: return Side.HYPER;
        todo!()
    }

}

// Java: public class InconsistentStateAt2PiWrapping
impl InconsistentStateAt2PiWrapping {
    // Java: InconsistentStateAt2PiWrapping()
    pub fn new() -> Self {
        // Java: super(LocalizedFormats.INCONSISTENT_STATE_AT_2_PI_WRAPPING);
        InconsistentStateAt2PiWrapping {}
    }

}
