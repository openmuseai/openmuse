// Java: import java.util.ArrayList
// Java: import java.util.Collection
// Java: import java.util.Comparator
// Java: import java.util.HashMap
// Java: import java.util.Iterator
// Java: import java.util.Map
// Java: import java.util.TreeSet
// Java: import org.apache.commons.math3.geometry.Point
use crate::org::apache::commons::math3::geometry::Point;
// Java: import org.apache.commons.math3.geometry.Space
use crate::org::apache::commons::math3::geometry::Space;
// Java: import org.apache.commons.math3.geometry.Vector
use crate::org::apache::commons::math3::geometry::Vector;

// Java: public class for
pub struct r#for {
}

// Java: public class AbstractRegion
pub struct AbstractRegion {
    // Java field: BSPTree<S> tree
    pub(crate) tree: Box<dyn std::any::Any>,
    // Java field: double tolerance
    pub(crate) tolerance: f64,
    // Java field: double size
    pub(crate) size: f64,
    // Java field: Point<S> barycenter
    pub(crate) barycenter: Box<dyn std::any::Any>,
}

// Java: public class for
impl r#for {
}

// Java: public class AbstractRegion
impl AbstractRegion {
    // Java: AbstractRegion(final double tolerance)
    pub fn AbstractRegion(&self) {
        // Java: this.tree      = new BSPTree<S>(Boolean.TRUE);
        // Java: this.tolerance = tolerance;
    }

    // Java: AbstractRegion(final BSPTree<S> tree, final double tolerance)
    pub fn AbstractRegion_2(&self) {
        // Java: this.tree      = tree;
        // Java: this.tolerance = tolerance;
    }

    // Java: AbstractRegion(final Collection<SubHyperplane<S>> boundary, final double tolerance)
    pub fn AbstractRegion_3(&self) {
        // Java: this.tolerance = tolerance;
        // Java: if (boundary.size() == 0)
        {
            // Java: // the tree represents the whole space
            // Java: tree = new BSPTree<S>(Boolean.TRUE);
        }
        // Java: else
        {
            // Java: // sort the boundary elements in decreasing size order
            // Java: // (we don't want equal size elements to be removed, so
            // Java: // we use a trick to fool the TreeSet)
            // Java: final TreeSet<SubHyperplane<S>> ordered = new TreeSet<SubHyperplane<S>>(new Comparator<SubHyperplane<S>>()
            {
                // Java: /** {@inheritDoc} */
                // Java: public int compare(final SubHyperplane<S> o1, final SubHyperplane<S> o2)
                {
                    // Java: final double size1 = o1.getSize();
                    // Java: final double size2 = o2.getSize();
                    // Java: return (size2 < size1) ? -1 : ((o1 == o2) ? 0 : +1);
                }
            }
            // Java: ordered.addAll(boundary);
            // Java: // build the tree top-down
            // Java: tree = new BSPTree<S>();
            // Java: insertCuts(tree, ordered);
            // Java: // set up the inside/outside flags
            // Java: tree.visit(new BSPTreeVisitor<S>()
            {
                // Java: /** {@inheritDoc} */
                // Java: public Order visitOrder(final BSPTree<S> node)
                {
                    // Java: return Order.PLUS_SUB_MINUS;
                }
                // Java: /** {@inheritDoc} */
                // Java: public void visitInternalNode(final BSPTree<S> node)
                {
                }
                // Java: /** {@inheritDoc} */
                // Java: public void visitLeafNode(final BSPTree<S> node)
                {
                    // Java: if (node.getParent() == null || node == node.getParent().getMinus())
                    {
                        // Java: node.setAttribute(Boolean.TRUE);
                    }
                    // Java: else
                    {
                        // Java: node.setAttribute(Boolean.FALSE);
                    }
                }
            }
        }
    }

    // Java: AbstractRegion(final Hyperplane<S>[] hyperplanes, final double tolerance)
    pub fn AbstractRegion_4(&self) {
        // Java: this.tolerance = tolerance;
        // Java: if ((hyperplanes == null) || (hyperplanes.length == 0))
        {
            // Java: tree = new BSPTree<S>(Boolean.FALSE);
        }
        // Java: else
        {
            // Java: // use the first hyperplane to build the right class
            // Java: tree = hyperplanes[0].wholeSpace().getTree(false);
            // Java: // chop off parts of the space
            // Java: BSPTree<S> node = tree;
            // Java: node.setAttribute(Boolean.TRUE);
            // Java: for (final Hyperplane<S> hyperplane : hyperplanes)
            {
                // Java: if (node.insertCut(hyperplane))
                {
                    // Java: node.setAttribute(null);
                    // Java: node.getPlus().setAttribute(Boolean.FALSE);
                    // Java: node = node.getMinus();
                    // Java: node.setAttribute(Boolean.TRUE);
                }
            }
        }
    }

    // Java: abstract public abstract AbstractRegion<S, T> buildNew(BSPTree<S> newTree)
    pub fn buildNew(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

    // Java: double getTolerance()
    pub fn getTolerance(&self) -> f64 {
        // Java: return tolerance;
        todo!()
    }

    // Java: void insertCuts(final BSPTree<S> node, final Collection<SubHyperplane<S>> boundary)
    pub fn insertCuts(&self) {
        // Java: final Iterator<SubHyperplane<S>> iterator = boundary.iterator();
        // Java: // build the current level
        // Java: Hyperplane<S> inserted = null;
        // Java: while ((inserted == null) && iterator.hasNext())
        {
            // Java: inserted = iterator.next().getHyperplane();
            // Java: if (!node.insertCut(inserted.copySelf()))
            {
                // Java: inserted = null;
            }
        }
        // Java: if (!iterator.hasNext())
        {
            // Java: return;
        }
        // Java: // distribute the remaining edges in the two sub-trees
        // Java: final ArrayList<SubHyperplane<S>> plusList  = new ArrayList<SubHyperplane<S>>();
        // Java: final ArrayList<SubHyperplane<S>> minusList = new ArrayList<SubHyperplane<S>>();
        // Java: while (iterator.hasNext())
        {
            // Java: final SubHyperplane<S> other = iterator.next();
            // Java: final SubHyperplane.SplitSubHyperplane<S> split = other.split(inserted);
            // Java: switch (split.getSide())
            {
            // Java: case PLUS:
                // Java: plusList.add(other);
                // Java: break;
            // Java: case MINUS:
                // Java: minusList.add(other);
                // Java: break;
            // Java: case BOTH:
                // Java: plusList.add(split.getPlus());
                // Java: minusList.add(split.getMinus());
                // Java: break;
            // Java: default:
                // Java: // ignore the sub-hyperplanes belonging to the cut hyperplane
            }
        }
        // Java: // recurse through lower levels
        // Java: insertCuts(node.getPlus(),  plusList);
        // Java: insertCuts(node.getMinus(), minusList);
    }

    // Java: public AbstractRegion<S, T> copySelf()
    pub fn copySelf(&self) -> Box<dyn std::any::Any> {
        // Java: return buildNew(tree.copySelf());
        todo!()
    }

    // Java: boolean isEmpty()
    pub fn isEmpty(&self) -> bool {
        // Java: return isEmpty(tree);
        todo!()
    }

    // Java: boolean isEmpty(final BSPTree<S> node)
    pub fn isEmpty_2(&self) -> bool {
        // Java: // we use a recursive function rather than the BSPTreeVisitor
        // Java: // interface because we can stop visiting the tree as soon as we
        // Java: // have found an inside cell
        // Java: if (node.getCut() == null)
        {
            // Java: // if we find an inside node, the region is not empty
            // Java: return !((Boolean) node.getAttribute());
        }
        // Java: // check both sides of the sub-tree
        // Java: return isEmpty(node.getMinus()) && isEmpty(node.getPlus());
        todo!()
    }

    // Java: boolean isFull()
    pub fn isFull(&self) -> bool {
        // Java: return isFull(tree);
        todo!()
    }

    // Java: boolean isFull(final BSPTree<S> node)
    pub fn isFull_2(&self) -> bool {
        // Java: // we use a recursive function rather than the BSPTreeVisitor
        // Java: // interface because we can stop visiting the tree as soon as we
        // Java: // have found an outside cell
        // Java: if (node.getCut() == null)
        {
            // Java: // if we find an outside node, the region does not cover full space
            // Java: return (Boolean) node.getAttribute();
        }
        // Java: // check both sides of the sub-tree
        // Java: return isFull(node.getMinus()) && isFull(node.getPlus());
        todo!()
    }

    // Java: boolean contains(final Region<S> region)
    pub fn contains(&self) -> bool {
        // Java: return new RegionFactory<S>().difference(region, this).isEmpty();
        todo!()
    }

    // Java: BoundaryProjection<S> projectToBoundary(final Point<S> point)
    pub fn projectToBoundary(&self) -> Box<dyn std::any::Any> {
        // Java: final BoundaryProjector<S, T> projector = new BoundaryProjector<S, T>(point);
        // Java: getTree(true).visit(projector);
        // Java: return projector.getProjection();
        todo!()
    }

    // Java: Location checkPoint(final Vector<S> point)
    pub fn checkPoint(&self) -> Box<dyn std::any::Any> {
        // Java: return checkPoint((Point<S>) point);
        todo!()
    }

    // Java: Location checkPoint(final Point<S> point)
    pub fn checkPoint_2(&self) -> Box<dyn std::any::Any> {
        // Java: return checkPoint(tree, point);
        todo!()
    }

    // Java: Location checkPoint(final BSPTree<S> node, final Vector<S> point)
    pub fn checkPoint_3(&self) -> Box<dyn std::any::Any> {
        // Java: return checkPoint(node, (Point<S>) point);
        todo!()
    }

    // Java: Location checkPoint(final BSPTree<S> node, final Point<S> point)
    pub fn checkPoint_4(&self) -> Box<dyn std::any::Any> {
        // Java: final BSPTree<S> cell = node.getCell(point, tolerance);
        // Java: if (cell.getCut() == null)
        {
            // Java: // the point is in the interior of a cell, just check the attribute
            // Java: return ((Boolean) cell.getAttribute()) ? Location.INSIDE : Location.OUTSIDE;
        }
        // Java: // the point is on a cut-sub-hyperplane, is it on a boundary ?
        // Java: final Location minusCode = checkPoint(cell.getMinus(), point);
        // Java: final Location plusCode  = checkPoint(cell.getPlus(),  point);
        // Java: return (minusCode == plusCode) ? minusCode : Location.BOUNDARY;
        todo!()
    }

    // Java: BSPTree<S> getTree(final boolean includeBoundaryAttributes)
    pub fn getTree(&self) -> Box<dyn std::any::Any> {
        // Java: if (includeBoundaryAttributes && (tree.getCut() != null) && (tree.getAttribute() == null))
        {
            // Java: // compute the boundary attributes
            // Java: tree.visit(new BoundaryBuilder<S>());
        }
        // Java: return tree;
        todo!()
    }

    // Java: double getBoundarySize()
    pub fn getBoundarySize(&self) -> f64 {
        // Java: final BoundarySizeVisitor<S> visitor = new BoundarySizeVisitor<S>();
        // Java: getTree(true).visit(visitor);
        // Java: return visitor.getSize();
        todo!()
    }

    // Java: double getSize()
    pub fn getSize(&self) -> f64 {
        // Java: if (barycenter == null)
        {
            // Java: computeGeometricalProperties();
        }
        // Java: return size;
        todo!()
    }

    // Java: void setSize(final double size)
    pub fn setSize(&self) {
        // Java: this.size = size;
    }

    // Java: Point<S> getBarycenter()
    pub fn getBarycenter(&self) -> Box<dyn std::any::Any> {
        // Java: if (barycenter == null)
        {
            // Java: computeGeometricalProperties();
        }
        // Java: return barycenter;
        todo!()
    }

    // Java: void setBarycenter(final Vector<S> barycenter)
    pub fn setBarycenter(&self) {
        // Java: setBarycenter((Point<S>) barycenter);
    }

    // Java: void setBarycenter(final Point<S> barycenter)
    pub fn setBarycenter_2(&self) {
        // Java: this.barycenter = barycenter;
    }

    // Java: abstract void computeGeometricalProperties()
    pub fn computeGeometricalProperties(&self) {
    }

    // Java: Side side(final Hyperplane<S> hyperplane)
    pub fn side(&self) -> Box<dyn std::any::Any> {
        // Java: final InsideFinder<S> finder = new InsideFinder<S>(this);
        // Java: finder.recurseSides(tree, hyperplane.wholeHyperplane());
        // Java: return finder.plusFound() ?
              // Java: (finder.minusFound() ? Side.BOTH  : Side.PLUS) :
              // Java: (finder.minusFound() ? Side.MINUS : Side.HYPER);
        todo!()
    }

    // Java: SubHyperplane<S> intersection(final SubHyperplane<S> sub)
    pub fn intersection(&self) -> Box<dyn std::any::Any> {
        // Java: return recurseIntersection(tree, sub);
        todo!()
    }

    // Java: SubHyperplane<S> recurseIntersection(final BSPTree<S> node, final SubHyperplane<S> sub)
    pub fn recurseIntersection(&self) -> Box<dyn std::any::Any> {
        // Java: if (node.getCut() == null)
        {
            // Java: return (Boolean) node.getAttribute() ? sub.copySelf() : null;
        }
        // Java: final Hyperplane<S> hyperplane = node.getCut().getHyperplane();
        // Java: final SubHyperplane.SplitSubHyperplane<S> split = sub.split(hyperplane);
        // Java: if (split.getPlus() != null)
        {
            // Java: if (split.getMinus() != null)
            {
                // Java: // both sides
                // Java: final SubHyperplane<S> plus  = recurseIntersection(node.getPlus(),  split.getPlus());
                // Java: final SubHyperplane<S> minus = recurseIntersection(node.getMinus(), split.getMinus());
                // Java: if (plus == null)
                {
                    // Java: return minus;
                }
                // Java: else if (minus == null)
                {
                    // Java: return plus;
                }
                // Java: else
                {
                    // Java: return plus.reunite(minus);
                }
            }
            // Java: else
            {
                // Java: // only on plus side
                // Java: return recurseIntersection(node.getPlus(), sub);
            }
        }
        // Java: else if (split.getMinus() != null)
        {
            // Java: // only on minus side
            // Java: return recurseIntersection(node.getMinus(), sub);
        }
        // Java: else
        {
            // Java: // on hyperplane
            // Java: return recurseIntersection(node.getPlus(),
                                       // Java: recurseIntersection(node.getMinus(), sub));
        }
        todo!()
    }

    // Java: public AbstractRegion<S, T> applyTransform(final Transform<S, T> transform)
    pub fn applyTransform(&self) -> Box<dyn std::any::Any> {
        // Java: // transform the tree, except for boundary attribute splitters
        // Java: final Map<BSPTree<S>, BSPTree<S>> map = new HashMap<BSPTree<S>, BSPTree<S>>();
        // Java: final BSPTree<S> transformedTree = recurseTransform(getTree(false), transform, map);
        // Java: // set up the boundary attributes splitters
        // Java: for (final Map.Entry<BSPTree<S>, BSPTree<S>> entry : map.entrySet())
        {
            // Java: if (entry.getKey().getCut() != null)
            {
                // Java: @SuppressWarnings("unchecked")
                // Java: BoundaryAttribute<S> original = (BoundaryAttribute<S>) entry.getKey().getAttribute();
                // Java: if (original != null)
                {
                    // Java: @SuppressWarnings("unchecked")
                    // Java: BoundaryAttribute<S> transformed = (BoundaryAttribute<S>) entry.getValue().getAttribute();
                    // Java: for (final BSPTree<S> splitter : original.getSplitters())
                    {
                        // Java: transformed.getSplitters().add(map.get(splitter));
                    }
                }
            }
        }
        // Java: return buildNew(transformedTree);
        todo!()
    }

    // Java: BSPTree<S> recurseTransform()
    pub fn recurseTransform(&self) -> Box<dyn std::any::Any> {
        todo!()
    }

}
